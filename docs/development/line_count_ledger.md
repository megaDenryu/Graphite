# 1ファイル100行原則の例外台帳

> **Development document** — 索引: `docs/README.md`

この文書は2つを持つ。1つはコード行の数え方の定義であり、もう1つは100行を超える
ファイルの一覧 (台帳) である。グローバルの規約「1ファイル100行の原則と分割の質」の
条5は、超過を許したファイルを台帳へ登録し、台帳と実際の行数の両方を機械で見ることを
要求している。この台帳と `cargo xtask check-line-counts` がその要求に応える。

台帳が覆うのは、リポジトリの `crates`・`xtask`・`examples`・`verification` 配下の
Rust ソース全部である (生成物を除く。範囲の定義は「検査の範囲」節を読むこと)。

## コード行の数え方

コード行とは、次の3種類のどれでもない行のことである。

1. 空行 (空白だけの行を含む)
2. 行頭から始まる行コメント (`//`・`///`・`//!`) だけの行
3. ブロックコメント (`/* */`) の中にあり、コメントの外にコードを持たない行

行末に書いたコメントは、その行にコードがあるため、コード行として数える。
`#[cfg(test)]` を付けたテストモジュールの中もコード行として数える。規約が
テストを除外していないためである。テストの行数がファイルの原則を圧迫する場合の
正しい対処は、テストを別のファイルまたは `tests/` へ移すことであり、台帳へ専用の
区分を作ることではない。

数える処理は行の走査で行い、`syn` の構文解析は使わない (`crates/graphite/tests/ui`
配下のように、コンパイルが通らないことを目的にしたファイルも数える対象であるため)。
走査は次の判定を行い、行の途中の状態までは追わない。

- 空白を除いた先頭が `//` で始まる行は、コメントだけの行として数えない。
- 空白を除いた先頭が `/*` で始まる行は、同じ行に `*/` が現れ、その後ろにコードが
  残っていればコード行として数える。残っていなければ数えない。同じ行に `*/` が
  現れなければ、`*/` が現れる行までをブロックコメントの中とみなす。
- ブロックコメントの中の行は、`*/` の後ろにコードが残っていればコード行として
  数え、残っていなければ数えない。

この数え方には2つの限界がある。どちらも数え漏らし (実際より少なく数える) の側へ
働く。1つ目は、文字列リテラルの中身を解析しないことである。複数行の生文字列
リテラルの中に、行頭が `//` や `/*` の行を書くと、その行をコメントとみなして
数えない。2つ目は、Rust のブロックコメントが入れ子にできることである。走査は
最初の `*/` でブロックコメントを閉じたとみなす。どちらもリポジトリの現在の
ソースには現れていない。

## 区分

台帳の各行は区分を1つ持つ。区分は次の3種類である。

- **統合による超過** とは、分割すると1つの流れの一片になるファイルを統合した結果、
  100行を超えた状態のことである。規約はこの超過に150行の上限を置く。
- **宣言的データリテラル** とは、完成物の形をそのまま写す宣言的な式・リテラルが
  1つのデータを成しているファイルのことである。規約はこの区分に150行の上限を
  適用しない。行数で切ると宣言の一覧性そのものが壊れるためである。
- **再設計待ち** とは、150行を超えており、統合の判断が誤っているか型の責務が
  過多かの判定と再設計をこれから行うファイルのことである。issue #28 のやること4
  がこの区分の全件を対象にする。検査はこの区分を当面の猶予として違反にせず、
  件数だけを報告する。

## 台帳

各行は、ファイルの綴り・区分・超過を許す根拠を持つ。行数は書かない。行数は変更の
たびに動き、台帳へ書くと本体と台帳の二重管理になるためである。実際の行数は
`cargo xtask check-line-counts` が数えて報告する。

| ファイル | 区分 | 根拠 |
| --- | --- | --- |
| `crates/graphite-cli/src/generation_plan.rs` | 統合による超過 | 生成計画1つを所有し、追加・古いファイルの書き出し・差分検査・孤児の検出がその計画表への操作である。本体は105行で、残りは同居する単体テストである |
| `crates/graphite-cli/src/generation_tree.rs` | 統合による超過 | 走査対象のディレクトリ木1つを所有し、Rust ファイルと生成ファイルの収集がその木の操作である。本体は114行である |
| `crates/graphite-cli/src/package_root.rs` | 統合による超過 | パッケージルートの決定と走査対象の組み立てが1つの概念である。本体は72行で、残りは同居する単体テストである |
| `crates/graphite-cli/src/schema_source_file.rs` | 統合による超過 | schema 宣言を持つソースファイル1件の読み取りと宣言の切り出しが1つの流れである。本体は98行で、残りは同居する単体テストである |
| `crates/graphite-codegen/src/lib.rs` | 再設計待ち | 150行を超える。本体は99行であり、超過分は同居する単体テストである。テストの移設を含めて issue #28 のやること4 で判定する |
| `crates/graphite-codegen/src/schema/codegen/freeze/directed_edge.rs` | 統合による超過 | 有向辺1種別分の凍結処理 (辺表の構築・重複キーの検出・両端点の実在検査・端点対の重複検査) が1つの `for` ループを共有する1本の手続きである |
| `crates/graphite-codegen/src/schema/codegen/freeze/mod.rs` | 統合による超過 | 凍結の全工程の並び順そのものを持つ。並び順は生成ファイルの中身であり、切ると順序が読めなくなる |
| `crates/graphite-codegen/src/schema/codegen/freeze/undirected_edge.rs` | 統合による超過 | `directed_edge.rs` と同じ理由である。無向辺1種別分の凍結処理が1つの `for` ループを共有する |
| `crates/graphite-codegen/src/schema/codegen/insertable_trait/marker_traits.rs` | 統合による超過 | ノード側と辺側で対になる2つの生成関数が同じ形を共有しており、分けると対応が読めなくなる |
| `crates/graphite-codegen/src/schema/codegen/mod.rs` | 統合による超過 | 生成物1種別分の全体像を1箇所で見せる地図を兼ねる。地図を分けると読む場所が散る |
| `crates/graphite-codegen/src/schema/semantic/analyze.rs` | 統合による超過 | 検証済み構文からスキーマ定義を組み立てる1本の流れである。本体は54行で、残りは同居する単体テストである |
| `crates/graphite-codegen/src/schema/semantic/cardinality.rs` | 統合による超過 | 多重度という1つの概念とその判定を持つ。本体は84行で、残りは同居する単体テストである |
| `crates/graphite-codegen/src/schema/semantic/edge_definition.rs` | 再設計待ち | 150行を超える。辺の向き・有向端点・積み荷を1つの辺定義へ統合しているが、上限を超えたため責務の量を issue #28 のやること4 で判定する |
| `crates/graphite-codegen/src/schema/semantic/public_id_type.rs` | 統合による超過 | 公開ID型が既定生成と明示指定のどちらであるかを1つの判別共用体で持つ。本体は59行で、残りは同居する単体テストである |
| `crates/graphite-codegen/src/schema/semantic/schema_definition.rs` | 統合による超過 | スキーマ1つ分の意味モデル全体を所有し、添字ハンドルからの取り出しを提供する。本体は62行で、残りは同居する単体テストである |
| `crates/graphite-codegen/src/schema/semantic/traversal_plan.rs` | 統合による超過 | ノード参照へ生やす探索操作の並びを、生成する順のまま1つの表として持つ。本体は64行で、残りは同居する単体テストである |
| `crates/graphite-codegen/src/schema/semantic/violation_catalog.rs` | 統合による超過 | 生成する違反列挙型の種類と並びを1つの目録として持つ。本体は68行で、残りは同居する単体テストである |
| `crates/graphite-codegen/src/schema/syntax/edge_declaration.rs` | 統合による超過 | 有向・無向と役割名の整合の判定が、柄の向きと両端点の役割名の有無の組み合わせを1つの表として持つ |
| `crates/graphite-codegen/src/schema/validate/generated_name_collision.rs` | 統合による超過 | 全ての生成名を1つの表へ登録して重複をその場で診断する、1本の手続きである |
| `crates/graphite-macros/src/flow_dsl.rs` | 統合による超過 | `flow!` の入力 DSL の構文解析一式であり、分けると1つの文法が複数ファイルへ散る |
| `crates/graphite-macros/src/instance_codegen.rs` | 再設計待ち | 150行を超える。`graph!` のコード生成本体であり、責務の量を issue #28 のやること4 で判定する |
| `crates/graphite-macros/src/instance_dsl.rs` | 再設計待ち | 150行を超える。`graph!` の入力 DSL の構文解析一式であり、責務の量を issue #28 のやること4 で判定する |
| `crates/graphite-macros/src/instance_semantic.rs` | 統合による超過 | `graph!` の意味検査と並べ替えが1本の流れであり、途中で切ると検証途中の項の列を外へ晒す |
| `crates/graphite-macros/src/lib.rs` | 統合による超過 | proc-macro クレートの公開面である。6つのマクロ入口は同じ場所に並んでいることが公開面の一覧性そのものである |
| `crates/graphite/src/graph/mod.rs` | 再設計待ち | 150行を超える。公開契約の窓口としてメソッドを1画面へ集める設計だが、上限を超えたため issue #28 のやること4 で判定する |
| `crates/graphite/src/graph/topology/mod.rs` | 統合による超過 | `impl 有向トポロジー` を1ファイルへ集める規約 (`docs/development/runtime_structure.md` の分解の禁止事項1) の結果である |
| `crates/graphite/tests/allocation_contract.rs` | 再設計待ち | 150行を超える。確保契約の検証一式であり、テストの分け方を issue #28 のやること4 で判定する |
| `crates/graphite/tests/each_declaration_order.rs` | 統合による超過 | 検証対象1つ (`where each` の記述順) に対するテスト関数の列である。本体は29行である |
| `crates/graphite/tests/edge_roles.rs` | 統合による超過 | 検証対象1つ (辺の役割名) に対するテスト用スキーマとテスト関数の列である |
| `crates/graphite/tests/flow.rs` | 再設計待ち | 150行を超える。`flow!` の意味論の検証一式であり、テストの分け方を issue #28 のやること4 で判定する |
| `crates/graphite/tests/graph_construction.rs` | 統合による超過 | 検証対象1つ (汎用Graphの構築経路と構築時の失敗) に対するテスト関数の列である |
| `crates/graphite/tests/graph_cycle.rs` | 統合による超過 | 検証対象1つ (循環検出と閉路の内容) に対するテスト関数の列である |
| `crates/graphite/tests/graph_refs.rs` | 統合による超過 | 検証対象1つ (ノード参照と辺参照) に対するテスト用スキーマとテスト関数の列である |
| `crates/graphite/tests/graph_splice.rs` | 統合による超過 | 検証対象1つ (`graph!` のスプライス項) に対するテスト用スキーマとテスト関数の列である |
| `crates/graphite/tests/keyed_table_insertion_order.rs` | 統合による超過 | 検証対象1つ (挿入順の保持) に対するテスト用スキーマとテスト関数の列である。本体は27行である |
| `crates/graphite/tests/named_graph.rs` | 統合による超過 | 検証対象1つ (名前付き構築) に対するテスト用スキーマとテスト関数の列である |
| `crates/graphite/tests/node_id_shared_across_schemas.rs` | 統合による超過 | 検証対象1つ (明示ID型の複数スキーマ共有) に対する2つのスキーマとテスト関数の列である |
| `crates/graphite/tests/orgchart_handwritten.rs` | 再設計待ち | 150行を超える。マクロが生成すべきコードの手書きテンプレートであり、扱いを issue #28 のやること4 で判定する |
| `crates/graphite/tests/orgchart_macro.rs` | 再設計待ち | 150行を超える。`graph_schema!` の読み書き一式の検証であり、テストの分け方を issue #28 のやること4 で判定する |
| `crates/graphite/tests/role_query.rs` | 再設計待ち | 150行を超える。役割探索の検証一式であり、テストの分け方を issue #28 のやること4 で判定する |
| `crates/graphite/tests/schema_ids.rs` | 統合による超過 | 検証対象1つ (既定IDと明示ID型) に対するテスト用スキーマとテスト関数の列である |
| `crates/graphite/tests/traversal_api.rs` | 統合による超過 | 検証対象1つ (走査API) に対するテスト用スキーマとテスト関数の列である |
| `crates/graphite/tests/undirected_edges.rs` | 再設計待ち | 150行を超える。無向辺の検証一式であり、テストの分け方を issue #28 のやること4 で判定する |
| `crates/graphite/tests/unknown_endpoint_positions/unique_pair.rs` | 統合による超過 | 検証対象1つ (端点対の重複の診断文) に対する、有向4通りと無向2通りの網羅である |
| `examples/dialogue-engine/src/story.rs` | 宣言的データリテラル | 1本の `graph!` 呼び出しが30シーン・4エンディング・56本の選択肢を宣言する1つのデータである |
| `examples/graphitets-by-hand/src/bin/static_graph.rs` | 再設計待ち | 150行を超える。GraphiteTS の静的グラフ版を通常の Rust へ写す練習用ファイルであり、扱いを issue #28 のやること4 で判定する |
| `examples/graphitets-by-hand/src/main.rs` | 再設計待ち | 150行を超える。GraphiteTS の動的グラフ版を通常の Rust へ写す練習用ファイルであり、扱いを issue #28 のやること4 で判定する |
| `examples/org-analyzer/src/dataset.rs` | 統合による超過 | 1つのシードから1つの組織データを合成しきる1本の流れであり、途中で切ると生成途中の中間状態を外へ晒す |
| `examples/org-analyzer/src/reorg.rs` | 統合による超過 | 組織改編の1回分 (全要素の展開・対象部署の除外・再構築・報告の組み立て) が1本の流れである |
| `xtask/src/excerpt_inspection.rs` | 統合による超過 | 引用全件への判定と、検査が届いた範囲の集計が同じ走査の結果である。本体は64行で、残りは同居する単体テストである |
| `xtask/src/main.rs` | 統合による超過 | コマンドラインの引数解析と、各コマンドの使い方の説明文を1箇所へ統合している |
| `xtask/src/quoted_excerpt.rs` | 統合による超過 | 引用の収集と照合用の正規化が「文書に書かれた引用1件」という同じ概念に属する。本体は98行で、残りは同居する単体テストである |
| `xtask/src/quoted_excerpt_check.rs` | 統合による超過 | 引用1件へ2つの判定を掛け、その違反を整形する1本の流れである。本体は56行で、残りは同居する単体テストである |
| `xtask/src/reference_scan.rs` | 再設計待ち | 150行を超える。文書の走査と参照の収集を1つの型が持っており、責務の量を issue #28 のやること4 で判定する |
| `xtask/src/repository_root.rs` | 再設計待ち | 150行を超える。リポジトリルートからの綴りの組み立てを1つの型へ閉じているが、責務の量を issue #28 のやること4 で判定する |
| `xtask/src/source_reference.rs` | 再設計待ち | 150行を超える。ソース参照の綴りと行範囲の解析を1つの型が持っており、責務の量を issue #28 のやること4 で判定する |
| `xtask/src/source_reference_check.rs` | 統合による超過 | ソース参照1件の実在と行数範囲の判定と、その違反の整形が1本の流れである。本体は92行で、残りは同居する単体テストである |

## 検査の範囲

`cargo xtask check-line-counts` が数えるのは、`crates`・`xtask`・`examples`・
`verification` の配下にある拡張子 `.rs` のファイル全部である。除くのは次の2つだけで
ある。

1. ディレクトリ名が `generated` の場所にあるファイル。生成物であり、人が分割する
   対象ではない。
2. ディレクトリ名が `target` の場所にあるファイル。cargo のビルド生成物である。

読み込みに失敗したファイルは、対象から外さず違反として報告する。
