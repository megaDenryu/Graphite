# static_graph_schema! — 全個体がコンパイル時に確定するグラフ (issue #24)

> **Current reference** — 索引: `docs/README.md`

`static`/`dynamic` は、**そのschemaから作られるグラフの個体集合・トポロジーが
宣言時に確定するか、実行時に構築できるか**を表す名前である。この命名は
「schemaがstatic/dynamicか」を意味しない。schema自体の型は、どちらも通常の
コンパイル時マクロとして静的に定義される。`static`/`dynamic` という名前は、
Rustの `static`/`'static` (静的な記憶域・生存期間) とは無関係な、別の語彙
である。

`dynamic_graph_schema!`/`graph!` は、`freeze()` を呼ぶまで個体を実行時に追加できる
グラフを扱う。制約 (多重度・対一意) は `freeze()` の実行時検証で確かめる。
`static_graph_schema!` はこれと対照的に、**個体・辺の集合自体がソースコードの
時点で固定されているグラフ**を対象にし、同じ制約をコンパイルエラーとして
検出する。利用例は `examples/static-org` を参照。

## 2層マクロの使い方

`static_graph_schema!` はschemaを検証し、**schema名そのものを名前にした
`macro_rules!`** を生成する。利用側はこの生成された `macro_rules!` へ
個体宣言を渡して具体グラフを組み立てる。

schema・instanceは共に `generated = "..."` を持ち、動的グラフ (`dynamic_graph_schema!`) と同じ生成ファイル・指紋照合の方式で公開APIを追跡する (issue #41)。
利用者は、宣言と同じファイルへ、生成先を読み込む `mod <名前> { include!("generated/<名前>.rs"); }` を置く (配線の書式は `docs/code_generation.md` を参照)。
この`mod`の置き場所はinstance宣言の置き場所と無関係である。instance展開はimplを一切使わず、値の橋渡しを素の関数 (呼び出し位置に生成する組み立て関数、下の「生成される名前の公開契約」参照) だけで行うため、利用者が`mod`を最上位に置いたままinstance宣言を関数の中に置いても警告は出ない (実測は `crates/graphite/tests/static_mod_outside_instance_inside_fn.rs` を参照。`#![deny(warnings)]`で警告0件を固定している)。
公開の `Nodes`・`Edges`・`{個体名}Ref`・`{辺名}Ref`・`Graph` はすべてこの生成ファイルの中にあり、schema・instanceマクロのその場展開には現れない。組み立て関数 (`{グラフ名}の個体を組み立てる`・`{グラフ名}の辺を組み立てる`) だけは例外で、その場展開の出力としてinstance宣言の呼び出し位置に生成される。既定の可視性は `pub(crate)` であり、利用者は同じcrateの中からこの関数を呼ぶ (呼び出し位置がライブラリの最上位でも、`mod`が私有なら生成ファイルの公開型が外部に漏れないため、組み立て関数も同じ可視性に合わせる)。

schemaとinstanceを別ファイルに分ける場合、利用者は2つの配線を行う。
1つ目はschema moduleの`use`である。
instance側のファイルは、schemaを宣言したファイルのmodule越しに `use 組織のファイル::組織;` のように`use`し、生成物が参照する型 (`組織::所属Edge`等) を解決できるようにする。
2つ目は、schema側の`mod <名前> { include!(..); }`宣言への`#[macro_use]`と、instance側の`mod`宣言より前に置く順序である。
`static_graph_schema!`が生成する`<schema名>!`という`macro_rules!`は`#[macro_export]`も`pub(crate) use`も持たない、通常の`macro_rules!`と同じテキスト順の可視性しか持たないため、`use`だけではこの`macro_rules!`を解決できず (「`組織`はマクロではなくmoduleです」という趣旨のエラーになる)、`#[macro_use]`によるテキスト順の伝播が要る。
利用者は、crateの入口ファイル (`main.rs`/`lib.rs`等) で `#[macro_use] mod organization; mod dev_team;`のように、schema側の`mod`宣言をinstance側の`mod`宣言より前に置く。
この配線は`crates/graphite/tests/static_multi_module.rs`が実際に固定している。

```rust
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 組織 {
    include!("generated/組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 開発チーム {
    include!("generated/開発チーム.rs");
}

組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 太郎 = 社員 { 名前: "太郎".into() };
    node 開発部: 部署;
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}
```

生成ファイルは `cargo graphite generate` (Graphiteリポジトリ自身の開発では
`cargo xtask generate`) で作る・更新する。生成の探索は2段階を踏む:
パッケージ内の全ファイルを1回ずつ見て静的schema名簿を作り (schema名の重複は
ここで検出する)、次に名簿の名前と一致するinstance宣言を解決する
(`docs/code_generation.md` 「宣言と配線」参照)。

Graphiteの開発者は、`静的グラフ型!`/`静的グラフ!` という2マクロ構成 (issue #24
段階1) を実装途中で全廃した。この転換の理由は、2つの proc マクロが互いの
展開結果を見えない制約を橋渡しするための機構 (タグ型・trait・
`PhantomData`) が、生成物を全部具象にできないという弱点を持っていたことで
ある。現行の1マクロ + macro_rules!転送構成は、schemaとinstanceのトークンを
1回の展開で同時に見ることで、この橋渡し機構を不要にした。

## DSL構文

### schema宣言

```text
static_graph_schema! {
    generated = "<生成先への相対パス>";
    schema <schema名> {
        (node <名前>;)*
        (edge <名前> = (<役割>: <型>) -> (<役割>: <型>) [where <制約>(, <制約>)*];)*
        (edge <名前> = (<役割>: <型>) -[<役割>: <型>]-> (<役割>: <型>) [where ...];)*
        (edge <名前> = (<役割>: <型>) -- (<役割>: <型>) [where <制約>(, <制約>)*];)*
        (edge <名前> = (<役割>: <型>) -[<役割>: <型>]- (<役割>: <型>) [where <制約>(, <制約>)*];)*
    }
}
```

**役割名は有向・無向を問わず必須**。両端を必ず `(役割名: 型)` の形で書く
(役割名を省いた裸形 `社員 -- 社員` は拒否する)。無向辺の積み荷付き記法
`-[役割: 型]-` は、有向の積み荷付き記法 `-[役割: 型]->` から矢尻 (`>`) を
落とした形 (`dynamic_graph_schema!` の慣習に倣う)。

`where` 節:

```text
where each <役割>: N | N..M | N..*  (, ...)*
where unique pair
```

`each` は役割位置ごとの多重度、`unique pair` は端点の組の対一意を宣言する。
無向辺は役割名を左右対称に持つため `each` を付けられない (`schema::validate`
が拒否する)。

### instance宣言

schema名がそのままマクロ名になるため、instance宣言はschema名を書かない。
行の種類は先頭の `node`/`edge` キーワードで確定する (右辺の形からの推測
判別はしない)。

```text
<schema名>! {
    generated = "<生成先への相対パス>";
    graph <グラフ名>;
    (node <名前> = <型> { <フィールド式, ...> };)*
    (node <名前>: <型> = <式>;)*
    (node <名前>: <型>;)*
    (edge <名前> = <種別>(<始点> -> <終点>);)*
    (edge <名前> = <種別>(<始点> -[<積み荷式>]-> <終点>);)*
    (edge <名前> = <種別>(<始点> -- <終点>);)*
    (edge <名前> = <種別>(<始点> -[<積み荷式>]- <終点>);)*
}
```

`node` は3形態を受理する (`examples/static-org/src/main.rs:64-67`):

1. `名前 = 型 { .. };` — 構造体リテラル。実体型はリテラルのパスから読む
2. `名前: 型 = 式;` — 型を明示すれば右辺は構造体リテラルに限らない任意の式で
   よい (関数呼び出し等)
3. `名前: 型;` — 宣言のみ。実体値は、instance展開が生成する組み立て関数
   `{グラフ名}の個体を組み立てる(..)` の引数として実行時に渡す (次節)

型注釈も構造体リテラルも無い場合は `` `node 名前: 型 = 式;` か
`node 名前 = 型 { ... };` の形で書いてください `` という展開時エラーになる。

## 生成される名前の公開契約

`<schema名>!` が展開時に生成する名前のうち、次は安定した公開名として
利用者が依存してよい。生成物は全部具象の struct (タグ・trait・
`PhantomData`・仕組みへの依存なし) であり、孤児規則 (E0116) に落ちない。
実体は生成ファイル (`generated/<instance名>.rs`) の中にあり、宣言と同じ
ファイルに置いた `mod <instance名> { include!(..); }` を通して参照する
(`{instance名}::Nodes` のようにmodule越しの修飾パスで使う)。

分類は issue #41 が定める2種類である。
**Aとは、利用者が書いたschema・instanceのトークンを直接または機械的に派生して作る名前のことである。** 生成物のdocは「宣言:」段落を持ち、その段落がDSLへの由来を文章で示す。
**Bとは、利用者のDSLに同名のトークンが存在しない、Graphite自身が定める固定語彙の名前のことである。** 生成物のdocは「固定語彙:」の1行を持つ。
A・Bのどちらも、F12は生成ファイルの中の人間が読める定義へ着地する (契約追跡・実装追跡が目的であり、DSLトークンそのものへは戻らない。由来追跡は「宣言:」段落のテキストが担う)。
ただしschema側が持つ役割アクセサ・積み荷アクセサ (下表参照) は例外で、その名前自体がinstanceのDSLトークンから機械的に作られるため、F12がDSLトークンへ正確に着地する (実測は`docs/development/ide_support_spec.md`§1.16参照)。
分類・追跡の詳細は下の「追跡の契約」節を参照。

| 生成されるもの | 名前 | 分類 | 修飾パスの例 | 備考 |
|---|---|---|---|---|
| 個体を組み立てる関数 | `{グラフ名}の個体を組み立てる` | A | `{グラフ名}の個体を組み立てる(..)` | instance展開がその場展開の出力として呼び出し位置に生成する (生成ファイルの中には無い)。値なし宣言 (`node 名前: 型;`) の個体だけを宣言順の位置引数に取り、値ありの個体はinstance宣言の式から計算した値を`Nodes::new`へ渡す。`Nodes`を返す。既定の可視性は`pub(crate)` |
| 辺を組み立てる関数 | `{グラフ名}の辺を組み立てる` | A | `{グラフ名}の辺を組み立てる(&nodes)` | instance展開がその場展開の出力として呼び出し位置に生成する (生成ファイルの中には無い)。`&Nodes`を受け取り、積み荷ありの具体辺の値をinstance宣言の式から計算して`Edges::new`へ渡す。`Edges<'_>`を返す。既定の可視性は`pub(crate)` |
| ノードの実体の唯一の所有者 | `Nodes` | B | `{instance名}::Nodes` | `Nodes::new(..)` は値の計算を持たない素の構築子であり、全個体を宣言順の位置引数にそのまま取る (引数名は個体名、型はその個体の実体型)。値ありの個体の値も計算済みの値として渡す必要があるため、通常は直接呼ばず `{グラフ名}の個体を組み立てる` を使う |
| 辺の実体の唯一の所有者 | `Edges<'a>` | B | `{instance名}::Edges` | `Edges::new(&nodes, ..)` も値の計算を持たない素の構築子であり、`&'a Nodes` に加え積み荷ありの具体辺すべてを宣言順の位置引数にそのまま取る。通常は直接呼ばず `{グラフ名}の辺を組み立てる` を使う |
| グラフ本体 | `Graph` | B | `{instance名}::Graph` | `{instance名}::Graph::new(&nodes, &edges)` で構築する。フィールドは `node_refs`/`edge_refs` |
| 個体参照の集まり | `NodeRefs<'a>` | B | `{instance名}::NodeRefs` | `Graph::node_refs` フィールドの型 |
| 辺参照の集まり | `EdgeRefs<'a>` | B | `{instance名}::EdgeRefs` | `Graph::edge_refs` フィールドの型 |
| `Graph`のフィールド | `node_refs`/`edge_refs` | B | `g.node_refs`/`g.edge_refs` | それぞれ `NodeRefs`/`EdgeRefs` を持つ |
| `Nodes`・`Edges`・`NodeRefs`・`EdgeRefs`が持つ個体名・辺名のフィールド | 個体名/辺名そのまま | A | `nodes.{個体名}`・`edges.{辺名}`・`g.node_refs.{個体名}`・`g.edge_refs.{辺名}` | 型 (`Nodes`等) はB分類の固定語彙だが、フィールド名は利用者が書いた個体名・辺名をそのまま使う |
| 個体ごとの具象参照 | `{個体名}Ref` | A | `{instance名}::{個体名}Ref` | `g.node_refs.{個体名}` からアクセスする |
| 具象参照から実体を取り出すメソッド | `entity()` | B | `{個体名}Ref::entity()` | `&実体型` を返す |
| 辺インスタンスごとの具象参照 | `{辺名}Ref` | A | `{instance名}::{辺名}Ref` | `g.edge_refs.{辺名}` からアクセスする |
| 個体参照から具体辺参照を返すメソッド | 辺名そのまま | A | `{個体名}Ref::{辺名}()` | 個体が端点になっている具体辺ごとに、その辺名をメソッド名にして生える (`太郎Ref::太郎の所属() -> 太郎の所属Ref`)。端点でない具体辺のメソッドは生えない (「存在しない辿り」検査、下の「コンパイル時検査の一覧」参照) |
| 種別ごとの辺値 struct | `{種別名}Edge` | A | `{schema名}::{種別名}Edge` | 役割名・積み荷フィールドをそのまま持つ。schemaファイルの中にあり、同じschemaから作った複数のinstanceで共有する |
| 辺参照のロールアクセサ | 役割名そのまま | A | `{辺名}Ref::{役割名}()` | 有向・無向を問わず、schema宣言の役割名がそのままアクセサ名になる (`所属(member: 社員) -> (team: 部署)` なら `.member()`/`.team()`、`友人 = (甲: 社員) -- (乙: 社員)` なら `.甲()`/`.乙()`)。無向辺専用の固定名は存在しない |
| 積み荷アクセサ | 積み荷の役割名そのまま | A | `{辺名}Ref::{積み荷役割名}()` | `&積み荷型` を返す |

**`{個体名}Ref` は生成ファイルの中の具象 `pub` struct なので、利用者は
マクロの外から後付けで自由にメソッドを生やせる。**
`impl<'a> 開発チーム::太郎Ref<'a> { fn あだ名(&self) -> String { .. } }`
のように、module越しの修飾パスで書け、生成されたチェーンの末尾へ通常の
メソッドと同じ形で継ぎ足せる (`examples/static-org/src/main.rs`)。

これらの名前は英語である。マクロ名 (`static_graph_schema!`) と生成される固定名
だけを英語化した方針 (issue #24 段階2、オーナー承認済み) であり、
**利用者が書く名前 (schema名・graph名・個体名・辺名・実体型・種別名・役割名)
と診断メッセージの日本語はそのまま**残る。旧日本語名との対応は次のとおり
(移設元 `examples/graphitets-by-hand/macros/` の設計記録):

| 旧 (日本語) | 新 (英語) |
|---|---|
| `静的グラフ型!` | `static_graph_schema!` |
| `ノード達` | `Nodes` |
| `辺達` | `Edges` |
| `初期値()` (ノード達) / `張る()` (辺達) | どちらも `new()` |
| `ノード参照達`/`辺参照達` (struct名とそのコンストラクタ `作る()`) | `NodeRefs`/`EdgeRefs` とその `new()` |
| グラフ本体のフィールド `ノード参照達`/`辺参照達` | `node_refs`/`edge_refs` |
| `{個体名}参照`/`{辺名}参照` | `{個体名}Ref`/`{辺名}Ref` |
| `{種別}の辺` | `{種別}Edge` |
| 参照の `実体()` | `entity()` |

## 追跡の契約 (issue #41)

生成APIを利用者が理解可能な範囲で追跡できることを、Graphiteは3つの経路で保証する。

1. **由来追跡**: なぜこの名前・このAPIが存在するのか、どのDSL宣言から来た
   のかを確認できること。
2. **契約追跡**: この型・メソッドが何を表し、どんな値を返し、どの具体個体・
   辺・役割に対応するのかを確認できること。
3. **実装追跡**: 実際にどんなRustコードへ展開され、どう実装されているのか
   を確認できること。

生成器は、この3つを1種類のF12操作へ無理に押し込めない。
利用者は、A分類・B分類どちらの名前でもF12で人間が読める生成ファイルの定義へ着地し (schema側の役割アクセサ・積み荷アクセサはDSLトークンへも正確に着地する。上の「生成される名前の公開契約」参照)、意味カード (doc) の「宣言:」/「固定語彙:」段落が由来を、意味の箇条が契約を、それぞれ文脈として補う。
実装追跡は生成ファイルそのものが正式経路であり、A分類・B分類どちらのF12も生成ファイルへ着地するため、実装追跡には追加の操作が要らない。
その場展開に残る部分 (指紋照合・DSLトークンの型参照・値供給関数、「macro_rules!転送の仕組みとテキスト順の制約」節参照) を読みたい場合の補助として、`cargo expand` (`cargo install cargo-expand`が必要) を使う。

**組み立て関数 (`{グラフ名}の個体を組み立てる`・`{グラフ名}の辺を組み立てる`) は、この3経路のどれについても例外である。**
組み立て関数はその場展開の出力であり生成ファイルの中に無いため、F12は生成ファイルの定義ではなくinstance宣言そのものへ着地する (`docs/development/ide_support_spec.md`§1.16で実測)。
組み立て関数の意味カードは「宣言:」段落を持たない (instance展開がマクロ実行中に呼び出し元のソースファイルパスを取得できないため、`instance_entry.rs`が宣言元を「分かっていない」に固定しており、意味カードの由来は「概要の1文」と「意味の箇条」だけになる)。
本体のコードは生成ファイルに写らずその場展開に残るため、実装追跡には`cargo expand`が要る (上の段落の「補助」ではなく、組み立て関数についてはこれが唯一の経路である)。

### 意味カードの書式

生成する公開型・公開メソッドのdocは、`crate::static_graph::trace`が組み立てる
「意味カード」であり、次の段落から成る。

1. 概要の1文 (固定語彙なら「(Graphite の固定語彙)」を含める)
2. 意味の箇条 (`graph`・個体・具体辺・辺種別・役割・実体型などのうち該当する項目)
3. 由来と文脈 — A分類は「宣言: `<宣言元ファイルのパッケージ相対の綴り>` の
   `<宣言の形>`」、B分類は「固定語彙: `<名前>` (`docs/static_graph.md`
   「生成される名前の公開契約」)」。どちらも宣言元ファイルの行番号は含めない
   (行番号を意味カードへ入れると、宣言の行が動くだけで再生成が必要になる)
4. schemaとinstanceの両方に由来する生成物 (role/payloadアクセサ・具体辺参照等) は、「関係する schema 宣言: `<...>`」または「関係する instance 宣言: `<...>`」を追加で持つ。1つのspanでは表現できない、2つの宣言から合成された意味を説明するためである。B分類でも、instanceが決めた具体的なgraph・個体と結び付く生成物 (`Nodes::new`等) は同様に「関係する instance 宣言: `<...>`」を持つ (`examples/static-org/src/generated/開発チーム.rs`の`Nodes::new`が実例)

実測例は `docs/development/ide_support_spec.md` §1.16「静的グラフの受理
マトリクス」を参照する。

### 制約

- **schema名は同じCargo targetの中で一意である。** 生成器 (`graphite-cli`)
  は、パッケージ内の全ファイルを構文解析して静的schema名簿を作るが、
  名簿の単位はパッケージ全体ではなくCargo targetである。`src/`配下
  (lib・bin) はまとめて1つのtarget、`tests/`配下は最初の1階層
  (`tests/foo.rs`・`tests/foo/`はどちらも`foo`というtarget) ごとに別の
  targetとして扱う。同じtargetの中に同じ名前の`static_graph_schema!`が
  2つ以上あれば`cargo graphite generate`/`cargo xtask generate`をエラーで
  止めるが、targetが違えば同名のschemaを許す。instanceの照合
  (`{schema名}! { .. }`という呼び出しがどのschemaのinstanceか) も、
  呼び出しと同じtargetの名簿だけを見て行う。
  **裁定 (2026-09-24、PR #45レビュー対応):** 生成器はRustのmodule解決を
  再実装しない。`src/lib.rs`と`tests/foo.rs`のように、本来は別クレート
  (別のマクロスコープ) である場所を1つの名簿へ混ぜて同名衝突を検出したり、
  無関係な同名マクロ呼び出しをinstanceと誤認したりしないよう、Cargoの
  ビルド単位 (target) を生成器が機械的に判定できる境界として採用した。
  この近似は、`tests/共通ヘルパー/mod.rs`のように複数のtest実行ファイルへ
  `#[path]`/`mod`で読み込まれる補助ファイルの中にschema宣言を置いた場合、
  実際に読み込む側のtargetと生成器が判定するtargetが食い違いうるという
  既知の制約を持つが、Rustの完全な名前解決をCLIへ持ち込むコストと比べて
  許容する。回帰試験: `crates/graphite-cli/src/static_resolution/tests/`
  の`別のcargo_targetなら同名のschemaを許す`・
  `別のcargo_targetにある同名schemaはinstanceとして解決しない`。
- **instanceを他のマクロの入力の中に書いてはならない。** `println!("{}", 組織! { .. })`のように、名簿の名前を他のマクロの引数の中へ埋め込む書き方は生成器がエラーにする (`docs/code_generation.md`参照)。利用者は、instanceを文の位置 (または関数の中の文の位置) に直接書く。
- **生成moduleを読み込む`mod`の置き場所は、instance宣言の置き場所と無関係である。** instance展開はimplを一切使わず、値の橋渡しを素の関数 (呼び出し位置に生成する組み立て関数) だけで行うため、利用者が`mod`を最上位に置いたままinstance宣言だけを関数の中に置いても`non_local_definitions`警告は出ない。`examples/static-org/src/main.rs`の`mod 経理チーム`(最上位)と`経理チームの花子の所属先を求める`関数(instance宣言はこの中)が、この配置の実例である。
- **schemaとinstanceを別ファイルに分けるときは、instance側のファイルがschema moduleを`use`し、schema側の`mod`宣言に`#[macro_use]`を付けてinstance側の`mod`宣言より前に置く。** 例: instance側のファイルは`use crate::organization::組織;`のように、schemaを宣言したファイルのmoduleを`use`する。crateの入口ファイルは`#[macro_use] mod organization; mod dev_team;`のように、schema側の`mod`をinstance側の`mod`より前に置く (理由と実測は上の「2層マクロの使い方」節を参照)。
- **同じスコープに、同名の値ありの個体・積み荷ありの辺を持つinstanceを複数置いてよい。** 組み立て関数は、値の供給関数 (`__graphite_initial_value_{名前}`・`__graphite_payload_{辺名}`) を自分の本体の中へ入れ子で定義する。供給関数の名前は個体名・辺名だけから決まりグラフ名を含まないため、この入れ子が無いと同名の個体・辺を持つ複数のinstanceを同じスコープ (ファイルの最上位、または同じ関数の中) に置いたとき名前が衝突する。組み立て関数ごとに別スコープを持つことで、instanceが違えば同名でも衝突しない。回帰試験: `crates/graphite/tests/static_same_individual_name_multiple_instances.rs` (最上位)・`static_same_individual_name_inside_function.rs` (関数の中)。個体の値の式が組み立て関数を呼ぶたびに1回だけ評価されることの回帰試験は `crates/graphite/tests/static_individual_value_evaluated_once_per_assembly.rs`。

### renameの手順

利用者は、生成名のrenameを、元DSLを正本として行う。
rust-analyzer自身のrename機能を生成APIの名前へ直接使うのではなく、次の順で行う。

1. 元DSL (schemaの`node`/`edge`宣言名、instanceの`node`/`edge`/`graph`宣言名)
   を書き換える。
2. `cargo graphite generate` (Graphiteリポジトリ自身の開発では
   `cargo xtask generate`) を実行し、生成ファイルを作り直す。
3. 利用箇所 (生成APIを呼ぶ側のコード) を新しい名前へ書き換える。

生成APIの名前は元DSLのトークンから決定的に導かれるため、元DSLを直さずに
生成ファイルだけを手で書き換えても、次の`cargo graphite generate`で上書き
される (ファイル先頭に手編集禁止の案内がある)。

## コンパイル時検査の一覧

相互検証はschema・instance双方の構造検証 (名前の重複・端点の宣言漏れ) が
通っている前提で、両者を突き合わせないと検出できない誤りを見る
(`crates/graphite-codegen/src/static_graph/internal/validate/`)。全て通常の
`compile_error!` (instance側の該当トークンを指す) として展開時に検出する。
`examples/static-org/src/main.rs:115-141` に実測コメントがある。

| 検査 | 誤りの例 | 実測した文言 |
|---|---|---|
| 未知の種別 | instanceが宣言していない種別を使う | `` `{種別}` はschemaに無い種別です `` |
| 向きの不一致 | 有向種別を無向で (または逆に) 書く | `向きの不一致: 種別 `{種別}` は有向 (`->`) ですが、辺 `{辺名}` は無向 (`--`) で書かれています` |
| 積み荷有無の不一致 | 積み荷ありの種別に積み荷を書かない (または逆) | `積み荷の有無の不一致: 種別 `{種別}` は積み荷が必要ですが、辺 `{辺名}` に積み荷がありません` |
| 端点の実体型不一致 | 個体の実体型が役割の要求する型と違う | `端点の実体型の不一致: `{個体名}` の実体型は `{実体型}` ですが、この役割は `{期待型}` 型を要求します` |
| 多重度違反 (`each`) | 役割位置での本数が範囲外 | `多重度制約違反: `一郎` の `所属` (役割 `member`) の本数が0件で、範囲 1..1 の外です` |
| 対一意違反 (`unique pair`) | 端点の組が (順序に依らず正規化して) 重複 | `対一意制約違反: 種別 `友人` の辺 `次郎と太郎` は端点の組 (太郎, 次郎) が既出の辺と重複しています` |
| 存在しない辿り | 端点でない個体からロールアクセサを呼ぶ | rustcの通常のE0599 (`太郎Ref` に `次郎の所属` というメソッドは無い、と類似名を提示する) |

多重度違反は、その種別の辺を1本も持たない個体も含め、instanceが宣言する
全個体を走査対象にする (schemaが宣言した全ての辺種別を起点に走査するため、
「宣言されているが1回も使われない種別」も検査対象から漏れない)。

### 生成ファイルが古いときの診断の並び (既知の制約)

利用者が個体・辺を足してからinstanceの生成ファイルを再生成し忘れると、
新しく足した個体・辺を指すDSLトークンの型参照・値供給関数が、古い生成
ファイルの中に存在しない型を参照する (`crates/graphite/tests/ui/static_stale_generated_instance_with_individuals.rs`
に固定した回帰試験がある)。この場合、rustcは次の順で診断を出す。

1. 「型が見つからない」E0425が、DSLに出現するが古い生成ファイルには無い名前 (足りない個体・辺) の、DSLでの出現箇所ごとに1件出る。rustcは「似た名前の構造体があります」という自動修正の提案を添えることがあり、この提案はGraphiteが生成した公開名 (例: 既存個体の`一郎Ref`) への書き換えを勧める。これは利用者の語彙ではなく、利用者が採用すべき修正でもない。
2. 再生成を促す「生成ファイルが古いため、cargo graphite generate を実行
   してください」というE0080 (指紋照合のpanic) が出る。個体を足した場合は
   この後ろに、組み立て関数が古い`Nodes::new`/`Edges::new`を引数の
   個数不一致で呼び出すE0061がもう1件続くことがある
   (`crates/graphite/tests/ui/static_stale_generated_instance_with_individuals.rs`
   に固定した回帰試験がある)。いずれにせよE0080は再生成を促す結論として
   一貫して現れ、この診断だけが実際に取るべき対処 (再生成) を示す。

Graphiteの開発者は、この並びを変えられないか2つの案を検証し、どちらも
採用しなかった。

- **案1: 指紋照合と型参照・値供給関数を同じ`const _: () = { .. };`ブロック
  へ同梱する。** rustcの診断順は変わらなかった (E0425が先、E0080が最後
  のまま)。名前解決 (E0425の由来) と定数評価 (E0080の由来) はrustcの
  別々のコンパイラパスであり、同じ構文ブロックへまとめても、名前解決
  パスは定数評価パスより必ず先に走る。この事実は
  `crates/graphite-codegen/src/static_graph/instance_entry.rs`の実装から
  読み取れる設計判断ではなく、rustc自体のパス構成に由来する。
- **案2: DSLトークンの型参照のspanを、instanceのトークン自身ではなく`generated = "..."`リテラルのspanにする。** この変更は、issue #41 §5.4が要求するDSLトークンの錨 (個体名・辺名のトークンからF12でschema・instanceの定義へ戻れる仕組み) 自体を壊すため採用しなかった (`static_instance_missing_module.rs`のように、span変更を要する別のUIテストで実際に確認した限りでは、rustcの「似た名前の構造体があります」という自動修正の提案は`generated = "..."`リテラルや`graph`トークンなど実際のspanを正確に指しており、マクロ呼び出し全体を覆う範囲へ悪化する現象は再現しなかった。それでもDSLトークンの錨を壊す欠点だけで不採用の結論は変わらない)。

この制約は、instanceの再生成忘れという1種類の古さについてのIDE体験上の
劣化であり、健全性には関与しない (最終的に必ずE0080が出て、利用者は
再生成が必要なことを知ることができる)。利用者が生成ファイルの古さに
遭遇したら、E0425の「似た名前」提案を採用せず、E0080の指示に従って
`cargo graphite generate` (Graphiteリポジトリ自身の開発では
`cargo xtask generate`) を実行することが正しい対処である。

### instance moduleの宣言を忘れたときの無関係なimport提案 (既知の制約)

instance moduleの`mod`宣言そのものを書き忘れると (`crates/graphite/tests/ui/static_instance_missing_module.rs`)、`{instance名}::..`を参照するコード全てが解決に失敗しE0433が4件出る。
このうち組み立て関数の戻り値型 (`-> {instance名}::Nodes`・`-> {instance名}::Edges<'_>`) が原因の2件には、rustcが`help: consider importing this struct`として`use petgraph::graphmap::Nodes;`・`use petgraph::csr::Edges;`等の無関係な提案を添える。

原因は名前解決の失敗そのものではなく、`Nodes`・`Edges`という名前がGraphiteの依存クレートpetgraphの複数の構造体名と一致することである。
rustcは最初のセグメント (`{instance名}`) の解決に失敗すると、パスの最後のセグメントと同じ名前を持つ、インポート可能な項目をクレート依存グラフ全体から探して提案する。
これはrustc組み込みの診断ヒューリスティックであり、パスに付けるspanをどう変えても (`{instance名}`を`generated = "..."`リテラルのspanへ付け替える案を実験したが、提案は消えず単にエラーの行が移動しただけだった)、名前の一致という原因を取り除かない限り消えない。
`Nodes`・`Edges`はissue #41が定めた固定語彙であり、この提案を消すためだけに公開名を変える変更はしない。
利用者がこの提案に遭遇したら、`use petgraph::..;`を追加するのではなく、`mod {instance名} { include!(..); }`宣言が無いことを疑うのが正しい対処である。

## macro_rules!転送の仕組みとテキスト順の制約

`static_graph_schema!` はschemaを検証したうえで、schemaの生トークンを本体へ
焼き込んだ `macro_rules! {schema名}` を生成する。個体宣言は
**schema名がそのままマクロ名**になり、この生成された `macro_rules!` が
schemaとinstance両方の生トークンを束ねて `#[doc(hidden)]` の内部マクロ
`__static_graph_impl!` (`::graphite::__static_graph_impl!` という絶対パスで
呼ぶ) へ転送する。そこでschemaとinstanceを1回の展開で同時に見て、相互検証と
具象コード生成を行う。展開は1回で全部が見えるため、生成物にタグ型・
`PhantomData`・橋渡し用traitは一切現れない。

生成された `macro_rules!` は通常の `macro_rules!` と同じテキスト順の制約を
受ける。**`static_graph_schema! { schema <名前> { .. } }` の呼び出しより後の行で
しか `<名前>! { .. }` を呼べない** (同じファイル内で `static_graph_schema!` の
呼び出しを先に書く必要がある)。

その場展開に残るのは、schema側が指紋照合・node型アンカー・
`macro_rules! {schema名}`、instance側が相互検証の診断・指紋照合・値の供給
関数・組み立て関数 (`{グラフ名}の個体を組み立てる`・`{グラフ名}の
辺を組み立てる`)・DSLトークンの型参照だけである。
公開生成物のうち `Nodes`・`Edges`・`{個体名}Ref`・`{辺名}Ref`・`Graph` は
すべて生成ファイルの中にあり、`cargo graphite generate`/
`cargo xtask generate` の対象になる (issue #41)。組み立て関数だけは
その場展開の出力としてinstance宣言の呼び出し位置に生成される。

## 実装の配置

- 構文解析・検証・意味モデル・生成: `crates/graphite-codegen/src/static_graph/`
  (`schema`/`literal` は構文解析、`internal` は相互検証、`semantic` は
  意味モデル、`trace`/`naming` は追跡情報と生成名 (`naming::assembly_names`
  が組み立て関数の名前、`naming::construction_card_names`が
  `Nodes::new`/`Edges::new`の意味カードを持つ)、`file` は生成ファイル
  本文、`inline` はその場展開に残す部分 (`inline::value_supply`が値の
  供給を担う素の関数、`inline::assembly`が組み立て関数)、`tracked` は
  追跡対象のschema/instance宣言、`schema_entry`/`instance_entry` はマクロ
  展開の入口)
- doc属性の組み立て: `crates/graphite-codegen/src/static_graph/doc_render.rs`
  (`file`と`inline::assembly`が共有する)
- proc マクロ入口: `crates/graphite-macros/src/lib.rs` の `static_graph_schema`/
  `__static_graph_impl`
- 生成ファイルの探索・書き込み・差分検査: `crates/graphite-cli/src/static_resolution/`
  (静的schema名簿と2段階の解決、`docs/code_generation.md` 参照)
- 公開: `crates/graphite/src/lib.rs` から re-export

移設元の設計記録 (2マクロ構成からの転換の経緯、IDE対応、実測ログ) は
GitHub issue #24 のコメント、および凍結した `examples/graphitets-by-hand`
にある。
