# graph! リテラル構文 v3 — `=` 束縛・式ペイロード・ハンドシェイク全廃

> **[v4 (`docs/schema_v4.md`) で置換済み]** このファイルは歴史的記録として残す。
> 現行のリテラル構文 (辺行も含め全行 `名前 = 値`、辺は `key = Kind(from -> to)` /
> `key = Kind(from -[式]-> to)` でタプル struct を構築) は `docs/schema_v4.md`
> を参照すること。

2026-07-14 セッション2 でのユーザー決定。`docs/edge_syntax_v2.md` (スキーマ宣言
構文 v2) と対になる、リテラル側の改訂。決定3 (`../Bullet/docs/graph_design_sketches.md`)
のリテラル形 `key: Type { .. }` / `-[label { .. }]->` を置き換える。

## 1. 動機 (ユーザー提案の趣旨)

> グラフインスタンスの中のノードインスタンスなのだから、`alice: Person { .. }` ではなく
> `alice = Person { .. }` のほうが自然。そうすればグラフの外で `let alice1 = Person { .. };`
> と作ってから `alice = alice1` のような変数渡しもできる。

- `alice` は `Org` のフィールドではなく、リテラルが導入する**束縛**である。実際
  G1 の脱糖は `let alice = PersonId(..)` という let 束縛であり、`=` が意味論に忠実。
- 統一規則が立つ: **schema は `:` (型付け)、リテラルは `=` (代入)**。
- 外部で構築済みの値 (ノード値・エッジペイロードとも) をリテラルへ渡せる。

## 2. 新構文

```rust
let alice1 = Person { name: "Alice".into() };
let promo = BossEdge { since: 2023 };

let g = graphite::graph!(Org {
    alice = alice1,                          // 外で作った値を move
    bob   = Person { name: "Bob".into() },   // インライン構築 (ただの式)
    eng   = Team  { name: "Engineering".into() },

    alice -[belongs_to]-> eng,               // 属性なしエッジは不変
    bob   -[boss = BossEdge { since: 2021 }]-> alice,  // ペイロードは `label = 式`
    carol -[boss = promo]-> alice,           // 式なので外の値も渡せる
});
```

- ノード項: `key = 式,`。式は任意の Rust 式 (型はマクロではなく rustc が推論する。§3)。
- 属性ありエッジ: `-[label = 式]->`。式は任意の Rust 式で、そのエッジのペイロード型
  に型付けされる。旧糖衣 `-[label { fields }]->` は**廃止** (検出・診断なし。素の
  パースエラーに任せる — 旧構文の方針と同じ)。
- 属性なしエッジ: `-[label]->` 不変。
- 旧ノード形 `key: Type { fields }` も**廃止** (同上)。

## 3. 型推論の設計 — マクロは型名を知らない

旧実装は `Person` という型名をトークンとして読み `b.person(..)` / `PersonId(..)` を
機械生成していた。v3 ではマクロは型を一切知らず、rustc の型推論に載せる:

- `graph_schema!` がスキーマごとにノード用 trait を生成する (形は実装時に調整可):

```rust
pub trait OrgNode: Sized {
    type Id;
    fn insert_into(self, b: &mut OrgBuilder, key: String) -> Self::Id;
}
impl OrgNode for Person { type Id = PersonId; fn insert_into(..) { /* person 格納 */ } }
impl OrgNode for Team   { type Id = TeamId;   fn insert_into(..) { /* team 格納 */ } }

impl OrgBuilder {
    pub fn insert<N: OrgNode>(&mut self, key: impl Into<String>, value: N) -> N::Id { .. }
}
```

- `graph!` の脱糖:

```rust
Org::create(|__graphite_b| {
    let alice = __graphite_b.insert("alice", /* ユーザーの式そのまま */ alice1);
    // alice: PersonId は rustc が推論する
    ...
    __graphite_b.boss(alice.clone(), ..., /* ユーザーの式そのまま */ BossEdge { since: 2021 });
})
```

- 単相化されるためゼロコスト (原則5)。`alice = 42` は「`i32: OrgNode` が満たされない」
  という正しい trait 境界エラーになる。
- 既存の型名付き builder メソッド (`b.person(id, value)` 等) は**従来どおり維持**
  (examples の合成データ生成などプログラム的構築の主要 API)。`insert` はそれらの
  総称版として公開してよい (命名は原則3 に従い実装時に決定)。

## 4. ハンドシェイクマクロの全廃

`__graphite_edge_{Schema}!` を**完全に削除**する。

- attrs アーム: ペイロードが式渡しになったため不要 (これで proc-macro → macro_rules
  の二段展開が消え、`docs/ide_support_spec.md` §1.7 の「graph! リテラル内の属性
  フィールドが定義ジャンプ不能」問題が構造的に解消する見込み。実装後に実測)。
- check アーム (未知ラベル診断): rustc の method-not-found に任せる
  (`no method named 'no_such_label' found for &mut OrgBuilder` + 類似名の
  `help:` 提示)。自前診断の「利用可能一覧」は失うが許容、というユーザー決定。
- **効果: schema と graph! の同一ファイル制約 (G5) が消滅する。** graph! が参照する
  のは通常の型・メソッドだけになるため、schema がスコープに `use` されていれば
  別ファイル・別モジュールから使える。ハンドシェイクマクロの補完候補への露出も消える。
- 別モジュールの schema に対して graph! が動くことを示すテストを追加する。

## 5. 維持するもの

- 重複キー診断 (最初の宣言位置の併記つき)・未宣言キー参照診断は現行のまま
  (マクロレベルの早期診断として rustc エラーより質が良い)。
- G1 の let 束縛方式・スパン規約 (宣言出現/エッジ内出現)・G4 のエラー回復。
- ノード項とエッジの 2 段並べ替え。

## 6. 移行対象

- crates/graphite/tests/ の graph! 使用箇所・trybuild UI テスト
  (`graph_unknown_edge_label` は rustc E0599 ベースの期待値に書き換えるか、
  役目を終えたとして削除を検討。`graph_duplicate_node_key` は維持)
- examples: hello-graph (§3/§4 の本文とエラー引用の**再採取**、README の表)、
  dialogue-engine (graph! の主要ユーザー)
- README (root): graph! の構文説明、「同一ファイル制約」に関する記述の削除
  (クリーンに: 制約が消えたので言及ごと消す)
- docs/ide_support_spec.md: G5 節を「v3 で制約自体が消滅」と更新。§1.7 の
  二段展開制約も実測後に更新
