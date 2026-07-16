# スキーマ宣言構文 v2 — ノード/エッジ属性の外部 struct 参照化 (決定3 の改訂)

> **注記 (2026-07-16)**: エッジ宣言部 (2.2「エッジ宣言」および関連する
> `-[label]->`/`-[label: 型パス]->` 記法) は `docs/edge_syntax_v3.md` で
> `label: From -> To (mult)` / `label: From -[型パス]-> To (mult)` の形へ
> 改訂済みです。本ファイルはノード/エッジ属性の外部 struct 参照化という
> v2 の主眼 (2.1 節以下) については引き続き有効な資料であり、歴史的資料
> として残します。

2026-07-14 セッション2 でのユーザー決定 (エッジ属性型の外部宣言化に着手し、
その後の設計確認でノード型にも同じ思想を拡張)。`../Bullet/docs/graph_design_sketches.md`
決定3 (矢印記法) の**宣言側の改訂**にあたる。ファイル名は `edge_syntax_v2.md`
のままだが、実際にはエッジだけでなくノードの宣言構文も含む改訂である。

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
4. **ノードへの一貫性拡張 (ユーザー決定)**: エッジ属性型を外部 struct 参照に
   した以上、ノード型 `node Employee { name: String }` だけがマクロ生成の
   nominal 型として残るのは一貫性を欠く。ノードは宣言に名前が見えている分
   「無名性」の問題は無いが、「マクロが型を生成するか参照するか」を
   エッジ・ノードで揃えることを優先し、ノードも外部 struct 参照形へ揃えた
   (旧版の §5 で「将来の一貫性課題」として保留していた論点を本改訂で解消)。

## 2. 新構文

### 2.1 ノード型・エッジ属性型はいずれもマクロの外で普通の Rust struct として宣言する

```rust
pub struct Employee {
    pub name: String,
    pub id: u32,
}

pub struct Department {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}
```

- derive・可視性・メソッドは全てユーザーの自由 (原則6: 消去可能な拡張)。
- `graph_schema!` はこれらの型を**生成せず、参照するだけ**。
- 制約: `graph!` リテラルや builder 呼び出し側から struct リテラル構築できる
  可視性が必要 (普通の Rust の可視性規則そのまま)。
- **ノード型名とエッジ属性型名で受け付ける構文が異なる** (後述 2.2/2.3):
  ノード型名は単純 `Ident` のみ、エッジ属性型は `syn::Path` (モジュール修飾可)。
  理由は「端点の同一性照合に使うかどうか」の違い。ノード型名はエッジの
  `from`/`to` 端点の型名との文字列照合に使われる (`Employee` という同じ
  トークンが `node` 宣言と `edge` 宣言の両方に現れて初めて同一ノード種別だと
  分かる)。`syn::Path` にすると `crate::Employee` と `Employee` を同一視でき
  ず照合が破綻するため、単純 `Ident` に制限する (モジュール修飾したい場合は
  `use` でこのスコープに名前を持ち込むのが Rust の作法どおりの解決)。エッジ
  属性型はこの照合に使われず、schema 側からユーザー宣言型への単方向の参照
  でしかないため、モジュール修飾を許して構わない。

### 2.2 schema のノード宣言・エッジ宣言

```rust
pub struct Employee { pub name: String, pub id: u32 }
pub struct Department { pub name: String }
pub struct BossEdge { pub since: i32 }

graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;

        edge Employee -[belongs_to]-> Department (1);
        edge Employee -[boss: BossEdge]-> Employee (0..1);
        edge Employee -[reports]-> Employee (0..*);
    }
}
```

ノード宣言:

- `node 型名;` — 型名はユーザーが宣言した struct への参照。フィールド列は
  書かない (マクロは値の型を生成しないので書く場所が無い)。
- `node 型名(複数形);` — 内部ストレージの複数形フィールド名を明示指定する
  省略可能な構文 (旧版から維持)。省略時は素朴な `+ "s"` にフォールバックする。
- **インライン生成形 `node Employee { name: String }` は完全廃止**。旧構文と
  同様、専用の検出・移行診断は設けない (まだ配布していない言語であり互換
  配慮は不要、言語をクリーンに保つというユーザー決定)。書いた場合は syn の
  素のパースエラーに任せる。
- `node` 宣言は (エッジから推論するのではなく) 引き続き必須。理由:
  (a) どのエッジにも接続しない孤立ノード種別を宣言できる必要がある、
  (b) エッジ端点の typo が「暗黙の新ノード種別」に化けて検出不能になるのを
  防ぐ、(c) schema がノード種別一覧の図式ドキュメントであり続ける。

エッジ宣言:

- 矢印内は `ラベル` (属性なし) または `ラベル: 型パス` (属性あり)。Rust の
  フィールド宣言 `name: Type` と同じ顔。型パスは `syn::Path` として受け、
  `edges::BossEdge` のようなモジュール修飾も許す (2.1 参照)。
- 多重度は矢印の後ろ (決定3 の原則「矢印の中は辺が運ぶもの」— 多重度は制約
  なので外に置く)。
- `edge` キーワードは維持する (G4 エラー回復パーサの宣言境界が `node`/`edge`
  キーワードに依存しているため。削ると回復境界が消える)。
- **旧構文 `edge label: From -> To (mult) { fields }` は廃止** (v0 につき互換層
  なし)。旧構文専用の検出・移行診断は設けない。まだ配布していない言語なので
  互換配慮は不要であり、「旧構文のにおいを一切残さず言語をクリーンに保つ」
  というユーザー決定に基づく。旧構文を書いた場合は syn の素のパースエラー
  (`expected` 系) がそのまま出るだけで十分とする。

### 2.3 graph! リテラルは現状維持

```rust
graphite::graph!(OrgChart {
    tanaka: Employee { name: "田中".into(), id: 1 },
    sato: Employee { name: "佐藤".into(), id: 2 },
    tanaka -[boss { since: 2020 }]-> sato,
})
```

- `tanaka: Employee { .. }` の `Employee { .. }` は、ユーザーが外部で宣言した
  ノード型の struct リテラルとして脱糖される (旧版はマクロ生成型の struct
  リテラルだったが、参照先が外部宣言型に変わるだけで構文・脱糖規則自体は
  不変)。
- `-[boss { since: 2020 }]->` の `{ .. }` は、schema が `boss` に対応させた
  ユーザー宣言型 (`BossEdge`) の struct リテラル本体として脱糖される。
  リテラルに型名を書かせる案 (`-[boss: BossEdge { .. }]->`) は冗長なので不採用
  (ラベル→型の対応は schema が一意に知っている)。

## 3. 実装上の要点

### 3.1 graph! のラベル→型解決 (ハンドシェイクの拡張)

`graph!` はスキーマの中身を知らないため、`boss` → `BossEdge` を自力で解決
できない (旧版は命名規則 `{Label}Attrs` で機械導出していたが、その規則自体が
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
- 未知ラベル診断は check アームとして統合する (マクロを2本生成しない)。

### 3.2 生成コードの変更点

- `{Label}Attrs` struct、およびノード値の struct (`Employee`/`Department` 等)
  の生成を全廃。builder メソッド・アクセサ・pairs イテレータの値の型・属性型は
  いずれも schema 宣言に書かれた型 (ノードは `Ident`、エッジ属性は型パス) を
  そのまま参照する。マクロが生成するのはグラフ機械のみ (`{Node}Id` newtype
  キー・ストレージ・builder・アクセサ・違反 enum)。
- ノード値の型・エッジ属性型への trait 要求は生成コードが実際に必要とする
  最小に留め、README に明記する。生成コードは値を builder が保持し、freeze
  で move し、アクセサが参照を返すだけなので、いずれの型にも
  Clone/Debug/PartialEq 等の trait を一切要求しない (newtype キー型は
  `HashMap` キーとして使うため `Hash + Eq` を要求するが、これはノード値の
  型とは別物)。
- 違反 enum のバリアント (`BossMultiplicity` 等)・アクセサ命名 (`boss`/`try_boss`/
  `boss_pairs` 等) は従来通りラベルから導出 (変更なし)。

### 3.3 スパン規約 (G3 の適用)

- ラベル ident・型パス・ノード型 ident は全てユーザートークンをそのまま使う。
- 型パスはユーザーの `BossEdge` トークンが生成コード中の型参照になるため、
  型の rename は普通の struct rename として全参照 (schema 内の出現含む) に
  カスケードするはず (実装後に F2 で実測する)。ノード型についても同様。

### 3.4 同一モジュール内の複数 schema がノード型を共有する場合の制約

同じ struct を同一モジュール内の複数 schema が `node` として共有すると、
両方の schema が同じ `{Node}Id` newtype (例: 両方が `pub struct EmployeeId(pub String);`)
を生成しようとして名前衝突になる。schema ごとにモジュールを分けて運用する
ことを README に明記する。

## 4. 移行対象

- crates/graphite/tests/orgchart_macro.rs ほか全テスト
- crates/graphite/tests/ui/ の trybuild テスト (旧構文のものは新構文へ移行。
  旧構文専用の検出テストは新設しない — 上記の通り診断そのものを設けないため)
- examples 3本 (build-pipeline / org-analyzer / dialogue-engine)
- README の構文説明・「手書きテンプレートとの差異」節
- orgchart_handwritten.rs (フェーズ2手書きテンプレート) は歴史的資料として
  変更せず残し、README の差異節に「v2 構文ではノード値の型・属性型はいずれも
  ユーザー宣言」と追記

## 5. 将来の一貫性課題

旧版ではここで「ノードも外部 struct 参照形にするか」を将来課題として保留
していたが、本改訂で採用済み (2.1〜2.2 参照)。現時点で残る一貫性課題は無い。
