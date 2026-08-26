# Graphite

型付きの図式グラフ (ノード種別・役割名つきの辺・多重度の制約) を Rust の型システムと
所有権に乗せる proc-macro DSL とランタイムです。DSL は普通の Rust のコードへ脱糖され、
生成物は追跡できる通常の Rust ファイルとして残ります。

図式グラフとは、`graph_schema!` でノード種別・役割名つきの辺・多重度の制約を宣言し、
その宣言に沿う形だけを型で受け付けるグラフのことです。

> **実験的プロジェクトです (v0)。API は予告なく変わります。**

自作言語 Vertex のグラフ機能の設計検討から派生した独立した Rust プロジェクトです
(Vertex 言語処理系のコードには一切依存しません)。v1 から v4.2 までの設計過程を
通読したい場合は `docs/history/design_journal.html` をブラウザで開いてください。

## 何ができるライブラリか

- **ドメインの関係を schema として宣言できます。** ノード種別・辺種別・端点の役割名・
  多重度の制約を型で表します。辺は第一級の要素であり、独立したキーと積み荷を持ちます
  (積み荷とは辺そのものが持つ属性値のことです。詳細は「主要な概念」)。
- **宣言から通常の Rust ソースを生成します。** `cargo graphite generate` が書き出した
  ファイルが唯一の実装であり、利用コードから定義ジャンプするとその実装行へ着地します。
  マクロは宣言を検証したうえで、生成ファイルとの指紋の一致をコンパイル時に検査します。
- **完成したグラフを凍結して読みます。** `graph.alice()` や
  `person.belongs_to_as_member()` のように、関係を型のついたメソッドで辿ります。
  戻り値の型は宣言した多重度が決めるため、「1本に定まる」「無いかもしれない」
  「複数ある」が呼び出し側の型に現れます。
- **制約の違反は構築時に `Result` で返ります。** 完成後のグラフには制約を満たした
  状態だけが存在します。

「ある値をグラフの要素として書くべきか、普通の構造体のフィールドとして書くべきか」の
判断基準は `docs/modeling_guide.md` にあります。

## どれを使えばよいか

Graphite には意図的に別の概念が同居しています。用途から選んでください。

| やりたいこと | 使うもの | 何をするものか |
|---|---|---|
| ドメイン固有の型付きグラフ構造を宣言したい | `graph_schema!` | ノード種別・辺種別・役割名・制約を型付きで宣言する。実体は `cargo graphite generate` が書き出す通常の Rust ファイルで、マクロは宣言の検証と指紋の照合を行う |
| そのschemaの具体的な Graph 値を作りたい | `graph!` / 生成された `Builder` | 静的な項と実行時データの両方からグラフを構築し、凍結時に制約を検査する |
| 同種ノードの汎用グラフアルゴリズムを使いたい | `Graph<N, E, K>` | ノード型が1種類の汎用不変グラフ。`has_cycle` / `topological_sort` / `topological_levels` / `critical_path_by` / `reachable_from` / `path` を持つ。マクロを使わない |
| 値を独立した関数へ順に流したい | `flow!` | Graph の値を作らない即時実行の糖衣。`x -[f]-> y` は `let y = (f)(x);` へ脱糖するだけ |
| 計算の依存を実行時の値として保持し、遅延評価・差分再計算したい | `ComputeGraph<V>` | 依存関係をランタイムの値として持ち、必要になった分だけ計算し、変わった入力の影響が及ぶ範囲だけを再計算する |

読み分けの要点は4つです。

1. `graph_schema!` / `graph!` の schema は静的に型付けされますが、内容は静的な項だけ
   でなく `Builder` ・ `extend` ・ `..式` により実行時データからも構築できます。完成
   した後、トポロジーは凍結されます。
2. `flow!` は Graph を保存しません。`ComputeGraph` は Graph を実行時の値として
   保存します。
3. `Graph<N, E, K>` と図式グラフは別のAPIです。図式グラフから汎用アルゴリズムへ
   渡したいときは `Graph::<(), (), K>::from_edges(nodes, edges)` で構造だけを射影します。
4. `ComputeGraph` はライブラリとして公開していますが、**このリポジトリの中に利用例が
   ありません** (2026-08-26 時点)。動く使い方は `crates/graphite/src/compute/mod.rs` の
   doctest と `crates/graphite/tests/compute_graph_*.rs` にあります。
   `examples/reactive-cells` は `ComputeGraph` を使わず、汎用 `Graph` の上に独自の
   `Engine` を実装しているため、`ComputeGraph` の実例ではありません。

## クイックスタート

ノード型を普通の struct として書き、schema を宣言し、生成ファイルを読み込み、
`graph!` でグラフを作り、名前から辿る、という一周です。

```rust
// ノード型・積み荷型は普通の Rust struct として宣言する。
// graph_schema! はこれらの型を生成せず、参照するだけ。
pub struct Person { pub name: String }
pub struct Team { pub name: String }
pub struct BossEdge { pub since: i32 }

graphite::graph_schema! {
    generated = "generated/main_org.rs";
    schema Org {
        node Person;
        node Team;

        edge BelongsTo = (member: Person) -> (team: Team) where each member: 1;
        edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Org {
    include!("generated/main_org.rs");
}

#[rustfmt::skip]
let g = graphite::graph!(Org {
    alice = Person { name: "Alice".into() },
    bob   = Person { name: "Bob".into() },
    eng   = Team { name: "Engineering".into() },

    alice_eng = BelongsTo(alice -> eng),
    bob_eng   = BelongsTo(bob -> eng),
    bob_boss  = Boss(bob -[BossEdge { since: 2021 }]-> alice),
})?;

// 左辺名がそのまま静的アクセサになる。
let alice = g.alice();                            // Org::PersonRef<'_>

// each member: 1 と宣言したので、辺は1本に定まる。
let team = alice.belongs_to_as_member().team();   // Org::TeamRef<'_>

// each subordinate: 0..1 と宣言したので、Option で返る。
let boss_edge = g.bob().boss_as_subordinate().unwrap();
let boss = boss_edge.superior();                  // 相手の端点
let appointment = boss_edge.payload();            // 辺そのものが持つ値
```

この宣言から生成されるのは、ノード種別と辺種別ごとの newtype のキー
(`Org::PersonId` ・ `Org::BossId` など)、構築用の辺の値型 (`Org::Boss`)、完成した
グラフを読む参照型 (`Org::PersonRef<'graph>` ・ `Org::BossRef<'graph>`)、グラフ本体
(`Org::Graph`)、`Org::Builder`、制約違反の enum (`Org::Violation`) です。ノードの
値型と積み荷の型は生成せず、宣言に書かれた型をそのまま参照します。

生成ファイルは、そのパッケージのディレクトリで `cargo graphite generate` を実行して
更新します (導入は「導入方法」を参照)。
schema を変えて生成し忘れると、指紋が合わず通常の `cargo build` がコンパイル
エラーになります。schema 宣言と `include!` はモジュール直下へ置いてください。
関数の中で宣言したローカルな型は、生成された module から参照できません。

動く完全な例は `examples/hello-graph` にあります。`edge Kind = ...` が何を定義して
いるのか、何ができて何ができないのかを、実際のコンパイルエラー付きで1つずつ確認
できます。

## 主要な概念

- **ノードの値は普通の Rust の型です。** Graphite は値型を生成せず、宣言に書かれた
  型を参照するだけです。`Clone` ・ `Debug` ・ `PartialEq` などを要求しません。
- **ノードと辺は ID による同一性を持ちます。** `node Person;` は `PersonId(pub String)`
  を schema module 内に生成します。自分で用意した型を使いたい場合は
  `node Person(id: EmployeeNumber);` と書きます。IDは内部位置ではありません。詳細は
  `docs/node_id_v4_2.md` にあります。
- **辺は第一級の要素です。** 辺種別は新しい名前の型として生成され、独自のキーと
  積み荷を持ちます。同じ形でも `Boss` と `Mentor` は別の型です。
- **端点の役割名が関係の意味を表します。** `(subordinate: Person) -> (superior: Person)`
  のように両端へ名前を付け、探索メソッドは `person.boss_as_subordinate()` のように
  役割名を指定して参照します。始点・終点という固定語彙は使いません。
- **積み荷は端点ではなく関係そのものの属性です。** `-[appointment: BossEdge]->` の
  `BossEdge` が積み荷で、`edge.payload()` で読みます。
- **`where each` と `where unique pair` が構造の制約です。** `each 役割名: 1` は
  ちょうど1本、`0..1` は高々1本、`unique pair` は同じ端点の対に2本目を張ることの
  禁止です。制約を書かなければ平行辺を許す多重グラフになります。
- **構築して凍結します。** `graph!` か `Builder` で組み立て、凍結時に全制約を検査
  します。最初の1件で止めず全違反を集めたい場合は `{Schema}::Graph::create_collecting` を
  使います。
- **完成後は `NodeRef` / `EdgeRef` から辿ります。** どちらも `&Graph` の参照と
  内部位置 (整数1個) だけを持つ、スタック上でコピーできる値です。種別全体への操作
  (`graph.boss_iter()` ・ `graph.person_by_id(&id)`) の主語は `Graph`、関係の探索の
  主語は参照自身です。
- **制約なしの辺は挿入順を保ちます。** 同じ始点キーに対する複数の終点の相対順序は、
  構築時に追加した順 (`graph!` では記述順) のままです。分岐ノベルの選択肢のように
  順序そのものが意味を持つ場面で依存できます。
- **`graph!` の左辺名は1つの平坦な名前空間です。** ノードと辺で種別が違っても、同じ
  識別子を2回使うとコンパイルエラーになります。
- **無向辺の端点は `graphite::UnorderedPair<T>` で表します。** 役割名を持たないため
  `where each` は使えず、探索は `node.<kind>_incident()` になります。
- **違反は種別ごとに型のついた値で返ります。** `Org::Violation` のバリアントは辺種別
  ごとに生成されるため、どの制約がどの要素で破れたのかを `String` へ落とさずに
  受け取れます。
- **schema が違えば生成される型も別です。** 同じ `Person` を参照しても `Org::PersonId`
  と `Approval::PersonId` は別の型になります。IDを共有したい場合は両方で
  `node Person(id: PersonId);` と明示します。

完全な構文・脱糖の対応・計算量は README へ複製しません。正本は
`docs/desugaring_reference.md` です。

## 実践例

`examples/` 配下に、`graphite` だけに依存する、他から独立したクレートを7本置いています。
それぞれ独立したワークスペースなので、`cd` して `cargo run` します。

- 最初に読む → `examples/hello-graph`
- ビルド依存の DAG とクリティカルパス → `examples/build-pipeline`
- 型付きドメインのグラフ → `examples/org-analyzer` ・ `examples/dialogue-engine`
- 状態遷移 → `examples/state-machine`
- 依存の再計算 → `examples/reactive-cells`
- 波に分けた並列実行 → `examples/async-dag`

`state-machine` ・ `async-dag` ・ `reactive-cells` の3本は「暗黙の制御フローで表現
されていた構造を、宣言されたグラフデータに変え、性質の検証をグラフアルゴリズムへ
任せる」という同じ変換の実証です。

- `examples/state-machine` — bool フラグの組み合わせ爆発を、状態をノード・イベントを
  辺種別・決定性を `where each before: 0..1` として再定式化し、到達不能状態を検出します。
- `examples/async-dag` — `.await` の順序へ溶け込んでいた依存を辺として宣言し、循環を
  ハングではなく構築時のエラーに変え、`topological_levels` の波を実際に並列実行します。
- `examples/reactive-cells` — observer パターンのグリッチと登録順依存を、依存を辺として
  宣言し `topological_sort` の順で解決します。

各ディレクトリの使い方とサブコマンドは、それぞれの `README.md` にあります。

## ドキュメント

`docs/README.md` が `docs/` 配下の全ファイルを状態つきで列挙する索引です。主要な
入口は次のとおりです。

| 知りたいこと | 読む文書 |
|---|---|
| 構文・生成型・公開API・計算量の正本 | `docs/desugaring_reference.md` |
| schema 宣言の構文と制約 | `docs/schema_v4.md` |
| 何をノード・辺として置くかの判断 | `docs/modeling_guide.md` |
| 端点の役割名と無向辺 | `docs/edge_endpoints_v4_1.md` |
| ID の設計 | `docs/node_id_v4_2.md` |
| 役割からの探索 | `docs/reverse_query.md` |
| 実行時データからの一括構築とスプライス | `docs/bulk_construction.md` ・ `docs/graph_splice.md` |
| 生成の配線・生成先・陳腐化の検出 | `docs/code_generation.md` |
| `flow!` | `docs/flow_macro.md` |
| `ComputeGraph` | `docs/compute_graph.md` |

## 導入方法

crates.io へは未公開です。`crates/graphite/Cargo.toml` に `license` フィールドが無く、
`cargo publish` の要求を満たしません。下記のライセンスの状態を先に読んでください。
そのため、いずれの使い方でもこのリポジトリを clone するか Git 依存で参照します。

### schema を使わない場合

`Graph<N, E, K>` ・ `ComputeGraph<V>` ・ `flow!` だけならコード生成が要りません。
依存を足すだけで使えます。

```toml
[dependencies]
graphite = { git = "https://github.com/megaDenryu/Graphite" }
```

### schema を使う場合

`graph_schema!` は実装を展開せず、生成された通常の Rust ファイルとの指紋の一致を
検査します。そのため、生成器を1つ入れる必要があります。

```powershell
git clone https://github.com/megaDenryu/Graphite
cargo install --path Graphite/crates/graphite-cli
```

これで `cargo graphite` が使えます。自分のパッケージへ依存を足し、

```toml
[dependencies]
graphite = { git = "https://github.com/megaDenryu/Graphite" }
```

schema 宣言と生成moduleの `include!` を書いたら、そのパッケージのディレクトリで
生成します。

```powershell
cargo graphite generate          # 生成ファイルを更新する
cargo graphite generate --check  # 差分と孤児をエラーにする (CI 向け)
```

走査するのはパッケージ直下の `src` と `tests` の配下です。生成ファイルは
git で管理してください。実際に動く最小の crate は `verification/external-crate`
にあります。生成の規約は `docs/code_generation.md` が定めます。

## 開発者向け

Graphite 自身を実装・保守する人向けの文書は `docs/development/` にあります。設計の
6原則は `docs/development/design_principles.md`、クレートの分け方は
`docs/development/crate_architecture.md`、実行時コードの配置は
`docs/development/runtime_structure.md`、テストの構成と実行は
`docs/development/testing.md` にあります。作業時の規約は `CLAUDE.md` が定めます。

## ライセンス

**ライセンス未定 (TBD)。** リポジトリに `LICENSE` ファイルが無く、法的には全ての権利を
留保した状態です。公開リポジトリですが、現時点で利用の許諾を与えていません。利用を
検討する場合は Issue で連絡してください。
