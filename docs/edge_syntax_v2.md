# エッジ宣言構文 v2 — 矢印形 + 事前宣言属性型 (決定3 の改訂)

2026-07-14 セッション2 でのユーザー決定。`../Bullet/docs/graph_design_sketches.md`
決定3 (矢印記法) の**宣言側の改訂**にあたる。

## 1. 動機

1. **無名ブロックの廃止 (ユーザー決定)**: 現行の `edge boss: Employee -> Employee
   (0..1) { since: i32 }` は、見た目が TypeScript 的な構造的レコードなのに、実体は
   マクロが裏で `BossAttrs` という nominal 型を生成する「見た目は構造的、実体は
   隠された nominal」であり、Rust の哲学 (無名 struct 型は存在しない) にも
   ユーザーの美学にも反する。利用側では `BossAttrs` という「どこにも書かれていない
   名前」が突然現れ、「{ since: i32 } に名前ないの? 何こいつ?」となる。
2. **rename 取り残しの根治 (G7-(a) の全面採用)**: `docs/ide_support_spec.md`
   §1.5 の通り、隠された生成名 `{Label}Attrs` はケース変換を挟むため rust-analyzer
   の rename がカスケードできない。属性型がユーザー自身のトークンになれば、
   この問題クラスごと消滅する (ラベルの rename は型に触れず、型の rename は
   普通の struct rename)。
3. **宣言もリテラルと同じ矢印形に**: 決定3 はリテラル側だけ `-[ラベル]->` に統一し、
   宣言側は `edge ラベル: A -> B` のままだった。宣言・リテラルの形を完全に揃える。

## 2. 新構文

### 2.1 属性型はマクロの外で普通の Rust struct として宣言する

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}
```

- derive・可視性・メソッドは全てユーザーの自由 (原則6: 消去可能な拡張)。
- graph_schema! はこの型を**生成せず、参照するだけ**。
- 制約: `graph!` リテラルや builder 呼び出し側から struct リテラル構築できる
  可視性が必要 (普通の Rust の可視性規則そのまま)。

### 2.2 schema のエッジ宣言は矢印形

```rust
graphite::graph_schema! {
    schema OrgChart {
        node Employee { name: String, id: u32 }
        node Department { name: String }

        edge Employee -[belongs_to]-> Department (1);
        edge Employee -[boss: BossEdge]-> Employee (0..1);
        edge Employee -[reports]-> Employee (0..*);
    }
}
```

- 矢印内は `ラベル` (属性なし) または `ラベル: 型パス` (属性あり)。Rust の
  フィールド宣言 `name: Type` と同じ顔。型パスは `syn::Path` として受け、
  `edges::BossEdge` のようなモジュール修飾も許す。
- 多重度は矢印の後ろ (決定3 の原則「矢印の中は辺が運ぶもの」— 多重度は制約
  なので外に置く)。
- `edge` キーワードは維持する (G4 エラー回復パーサの宣言境界が `node`/`edge`
  キーワードに依存しているため。削ると回復境界が消える)。
- **旧構文 `edge label: From -> To (mult) { fields }` は廃止** (v0 につき互換層
  なし)。ただし移行親切診断を出す: 宣言先頭が `edge ident :` の形なら旧構文と
  判定し、「新構文: edge From -[label]-> To (mult);」を span 付き compile_error
  で提示する (旧: 2トークン目が ident + `:`、新: 2トークン目がノード型 ident +
  `-[` なので曖昧なく判別できる)。

### 2.3 graph! リテラルは現状維持

```rust
graphite::graph!(OrgChart {
    tanaka: Employee { name: "田中".into(), id: 1 },
    sato: Employee { name: "佐藤".into(), id: 2 },
    tanaka -[boss { since: 2020 }]-> sato,
})
```

- `-[boss { since: 2020 }]->` の `{ .. }` は、schema が `boss` に対応させた
  ユーザー宣言型 (`BossEdge`) の struct リテラル本体として脱糖される。
  リテラルに型名を書かせる案 (`-[boss: BossEdge { .. }]->`) は冗長なので不採用
  (ラベル→型の対応は schema が一意に知っている)。

## 3. 実装上の要点

### 3.1 graph! のラベル→型解決 (ハンドシェイクの拡張)

`graph!` はスキーマの中身を知らないため、`boss` → `BossEdge` を自力で解決
できない (現行は命名規則 `{Label}Attrs` で機械導出していたが、その規則自体が
廃止される)。解決策: graph_schema! が生成するハンドシェイクマクロを拡張し、
「エッジラベル → 属性 struct リテラル構築」のディスパッチを持たせる:

```rust
// graph_schema! が生成 (イメージ)
macro_rules! __graphite_edge_OrgChart {
    (check belongs_to) => {};
    (check boss) => {};
    (check reports) => {};
    (check $other:ident) => { compile_error!("...利用可能エッジ一覧...") };
    (attrs boss { $($f:ident : $v:expr),* $(,)? }) => { BossEdge { $($f: $v),* } };
    // 属性なしエッジに attrs を渡した場合の親切エラー等も同様に生成
}
```

- graph! は `__graphite_edge_{Schema}!(attrs boss { since: 2020 })` を式として
  埋め込む。macro_rules 展開でもトークンの span は保持されるので、`since` から
  `BossEdge.since` フィールドへの定義ジャンプは機能するはず (実装後に実測する)。
- 同一ファイル制約 (G5) は従来と不変: graph! は現行でも check マクロ呼び出しを
  無条件に埋め込むため、schema と同一スコープ必須という条件は変わらない。
- 未知ラベル診断 (項目5) は check アームとして統合する (マクロを2本生成しない)。

### 3.2 生成コードの変更点

- `{Label}Attrs` struct の生成を全廃。builder メソッド・アクセサ・pairs イテレータ
  の属性型は schema 宣言に書かれた型パスをそのまま参照する。
- 属性型への trait 要求は生成コードが実際に必要とする最小に留め、README に
  明記する (現行生成型は Clone/Debug/PartialEq derive だったが、builder が値を
  保持し freeze で move し参照を返すだけなら Clone は不要のはず。実装時に確認)。
- 違反 enum のバリアント (`BossMultiplicity` 等)・アクセサ命名 (`boss`/`try_boss`/
  `boss_pairs` 等) は従来通りラベルから導出 (変更なし)。

### 3.3 スパン規約 (G3 の適用)

- ラベル ident・型パス・ノード型 ident は全てユーザートークンをそのまま使う。
- 型パスはユーザーの `BossEdge` トークンが生成コード中の型参照になるため、
  型の rename は普通の struct rename として全参照 (schema 内の出現含む) に
  カスケードするはず (実装後に F2 で実測する)。

## 4. 移行対象

- crates/graphite/tests/orgchart_macro.rs ほか全テスト
- crates/graphite/tests/ui/ の trybuild テスト (旧構文のものは新構文へ移行 +
  旧構文検出の移行診断テストを新設)
- examples 3本 (build-pipeline / org-analyzer / dialogue-engine)
- README の構文説明・「手書きテンプレートとの差異」節
- orgchart_handwritten.rs (フェーズ2手書きテンプレート) は歴史的資料として
  変更せず残し、README の差異節に「v2 構文では属性型はユーザー宣言」と追記

## 5. 将来の一貫性課題 (未着手)

- ノード宣言 `node Employee { .. }` も同じ理屈で外部 struct 参照形
  (`node Employee;` = 事前宣言した struct を参照) を許すか。ノードは宣言に
  名前が見えている (無名性の問題がない) ため今回は対象外としたが、
  「マクロが型を生成するか参照するか」の一貫性としては検討余地がある。
