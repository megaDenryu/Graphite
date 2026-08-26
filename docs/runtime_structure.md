# ランタイムクレートの内部構造 (`crates/graphite/src`)

`crates/graphite` の実行時コードがどの概念をどのファイルへ置いているかを記録する。
公開APIの使い方は README を、生成コードとの対応は `docs/desugaring_reference.md` を
参照する。ここは「どこに何があるか」と「何が何に依存してよいか」だけを扱う。

## 4つの実行時概念と、それぞれが現役である理由

1. **汎用 `Graph<N, E, K>`** (`graph/`) — 水準1の同種グラフ。`has_cycle`・
   `topological_sort`・`topological_levels`・`critical_path_by`・`reachable_from`・
   `path`・`map_nodes`・`filter_nodes`・`from_edges` を提供する。図式グラフ
   (`graph_schema!`) から射影して汎用アルゴリズムを使う経路と、計算グラフの構造検証の
   両方がこれを使う。
2. **計算グラフ `ComputeGraph<V>`** (`compute/`) — 計算グラフを実行時の値として保持し、
   pull 型で遅延評価・差分再計算するランタイムエンジン。**リポジトリ内に呼び出し側は無い**
   (2026-08-26 時点。使っているのは自身のテスト
   `crates/graphite/tests/compute_graph_*.rs` と `compute/mod.rs` の doctest だけで、
   想定利用者だった `examples/reactive-cells` は `ComputeGraph` を使わず、汎用 `Graph`
   の上に自前の `Engine` を実装している)。ライブラリとして公開しているAPIであり、
   `flow!` とも図式グラフとも別概念なので残す。
3. **schema生成コード向けの実行時契約** (`schema_runtime/`) — `graph_schema!`/`graph!`
   が生成したコードだけが名指しする内部契約。役割索引・辺リテラル・名前付き構築・
   構築印の採番・`GraphMismatch` から成る。
4. **キー付き要素表 `KeyedTable<K, V>`** (`keyed_table.rs`) と
   **順序を持たない対 `UnorderedPair<T>`** (`unordered_pair.rs`) — 生成コードがノード表・
   辺表・無向辺の端点で共有する土台。

## ファイルと責務

各ファイルの1行目のモジュールdocが正本であり、下表はその索引である。

| ファイル | 何を所有するか |
| --- | --- |
| `lib.rs` | 公開facade。モジュールの配線と再公開だけ |
| `graph/mod.rs` | 汎用Graphの公開API窓口。キーと内部位置の翻訳と各部品への委譲 |
| `graph/build_error.rs` | 構築時に検出する失敗 (キー重複・未知端点) の型と表示 |
| `graph/cycle_error.rs` | 循環検出時に返す閉路の型と表示 |
| `graph/builder.rs` | `Graph::create` がクロージャへ貸し出す構築用builder |
| `graph/assembly.rs` | 構築中のグラフ。検査しながらトポロジーとキー対応表を組み立てる |
| `graph/key_correspondence.rs` | キーと内部位置の対応表。正引きと逆引きが互いの逆であること |
| `graph/structure_graph.rs` | 値なし構造グラフの構築 (`from_edges`) |
| `graph/topology/mod.rs` | 有向トポロジー。`petgraph` を包む中核データと位置水準の基本操作 |
| `graph/topology/position.rs` | ノード位置のnewtypeと、作り直しの前後を対応づける表 |
| `graph/topology/cycle_search.rs` | 循環の有無判定と、閉路を含む強連結成分の選別 |
| `graph/topology/simple_cycle_extraction.rs` | 強連結成分から単純閉路を1本切り出す反復DFS |
| `graph/topology/topological_order.rs` | 全体のトポロジカル順序。循環時は閉路の探索へ切り替える |
| `graph/topology/dependency_levels.rs` | 依存レベル分割。レベル内順序が挿入順であることの保証 |
| `graph/topology/longest_path.rs` | ノード重み付き最長経路の動的計画法 |
| `graph/topology/traversal.rs` | 始点から辿る走査 (深さ優先の到達集合・幅優先の最短経路) |
| `graph/topology/transform.rs` | トポロジーの作り直しと、位置の対応の生成 |
| `compute/mod.rs` | 計算グラフの公開API窓口。3部品の配線と遅延評価の入口 |
| `compute/node_kind.rs` | ノード種別 (入力/計算) と、値を求める関数 |
| `compute/node_table.rs` | 計算ノード表。キーから種別・依存キー列・計算を引く |
| `compute/builder.rs` | 凍結前の半端な宣言列を唯一所有する builder |
| `compute/dependency_structure.rs` | 検証済みの依存グラフとトポロジカル位置 (凍結後不変) |
| `compute/evaluation_state.rs` | 評価状態。現在値と未再計算集合の整合 |
| `compute/recomputation.rs` | 再計算器。収集→整列→評価を1回分実行する |
| `compute/error.rs` | `ComputeGraphError` |
| `schema_runtime/mod.rs` | 実行時契約の配線 |
| `schema_runtime/role_index.rs` | 多重度ごとの役割索引3種 |
| `schema_runtime/edge_literal.rs` | `graph!` の柄から辺値を構築する契約 |
| `schema_runtime/graph_mismatch.rs` | 異なるGraph由来の参照を混ぜた契約違反 |
| `schema_runtime/named_element.rs` | 名前付き位置をGraphの借用へ束縛する契約 |
| `schema_runtime/construction_stamp.rs` | 構築を識別する印の採番 (クレート唯一のカウンタ) |
| `schema_runtime/named_construction.rs` | 名前付き構築の唯一の経路と許可証 |
| `keyed_table.rs` | キー付き要素表。挿入順保証と内部位置の安定性 |
| `unordered_pair.rs` | 順序を持たない同型値の対 |

## 依存の向き

- `graph/` は他のどのモジュールにも依存しない。
- `compute/` は `graph/` に依存する (依存構造の検証に汎用Graphを使う)。逆向きは無い。
- `schema_runtime/` は `graph/`・`compute/` に依存しない。
- `keyed_table.rs`・`unordered_pair.rs` は何にも依存しない。

**`petgraph` を名指しできるのは `graph/topology/` 配下だけである。** `graph/` 直下は
`topology::ノード位置` (`NodeIndex` を包むnewtype) を通してのみトポロジーへ触れ、
`NodeIndex` を名指ししない。この規則は次で機械的に検査できる。

```powershell
# C:\devs\Graphite で実行する。graph/topology/ 以外が出たら違反。
Get-ChildItem crates\graphite\src -Recurse -Filter *.rs |
  Select-String -Pattern "petgraph::" |
  Where-Object { $_.Line -notmatch "^\s*(//|///|//!)" }
```

可視性は、トポロジーの基本操作が `pub(in crate::graph)`、`petgraph` 本体を返す内部
アクセサが `pub(in crate::graph::topology)` である。`pub(crate)`/`pub(super)` は使わない。

## 分解の禁止事項

行数だけを理由に分けない。次の3つは特に禁じる。

1. `impl Graph<N, E, K>` を複数ファイルへ散らすこと。`graph/mod.rs` だけに置く
   (例外は `structure_graph.rs` の `impl Graph<(), (), K>` 1つ)。同様に
   `impl 有向トポロジー` は `graph/topology/mod.rs` だけに置く。
2. アルゴリズムを自由関数にすること。アルゴリズムは「`&有向トポロジー` を
   コンストラクタで受ける型のメソッド」にする。
3. 私有フィールドの可視性を緩めて外の自由関数から触れるようにすること。

## 100行原則の例外

1ファイル100行原則の例外は次の3つで、理由は各ファイルの冒頭にも書いてある。

- `graph/mod.rs` — 公開契約20メソッドを1画面で読む場所。分岐・ループ・アルゴリズムを
  書かず、1メソッドの本体は翻訳と委譲だけにする規則で運用する。行数の実体は公開API
  のrustdocである。
- `compute/mod.rs` — 公開API3つと、doctest付きのモジュールdoc。設計判断の説明は該当
  ファイル (動的ディスパッチ→`node_kind.rs`、再利用と内製→`dependency_structure.rs`、
  glitch-free→`recomputation.rs`) へ分配済み。
- `keyed_table.rs` — 1つの概念が1つの不変条件 (挿入順と内部位置の安定性) を所有する
  ので分けない。

`graph/topology/mod.rs` と `graph/assembly.rs` も100行を超えるが、どちらも1つの型の
メソッド群であり、分けると上の禁止事項1に当たる。

## 生成コードが依存する再公開

生成コードは `::graphite::` 直下の綴りを出力する。次の12件は移動しても必ず
`lib.rs` から再公開し続ける。`次の構築印を発行する` を落とすと全ての生成コードが壊れる。

`KeyedTable` / `UnorderedPair` / `MultipleRoleIndex` / `ExactlyOneRoleIndex` /
`OptionalRoleIndex` / `DirectedEdgeLiteral` / `UndirectedEdgeLiteral` / `GraphMismatch` /
`NamedGraphElement` / `NamedInsertPermit` / `FreezableBuilder` / `build_named_graph` /
`次の構築印を発行する`

再公開文には `#[doc(hidden)]` を付ける。`GraphMismatch` だけは利用者が受け取る
エラー型なので付けない。

## テストの配置

`crates/graphite/src` には `#[cfg(test)]` を原則として置かず、対象概念ごとの統合テスト
(`crates/graphite/tests/`) へ置く。例外は `unordered_pair.rs` で、公開APIが2メソッド
しかなく分けても読みやすくならないため同居させている。

| テストファイル | 対象 |
| --- | --- |
| `graph_construction.rs` | 汎用Graphの構築経路と構築時の失敗 |
| `graph_cycle.rs` | 循環検出と閉路の内容 |
| `graph_order.rs` | トポロジカル順序と依存レベル分割の順序保証 |
| `graph_critical_path.rs` | ノード重み付き最長経路 |
| `graph_traversal_paths.rs` | 走査 (全件・近傍・到達可能性) と経路探索 |
| `graph_transform.rs` | ノード値の写像と述語による絞り込み |
| `compute_graph_evaluation.rs` | 遅延評価と差分再計算の評価回数 |
| `compute_graph_validation.rs` | 凍結時検証 (循環・未宣言依存・キー重複) |
| `compute_graph_contract.rs` | 呼び出し規約違反のパニック |
| `keyed_table.rs` | キー付き要素表の挿入・検索・走査・内部位置 |
| `common/mod.rs` | 汎用Graphのテストが共有する人物ノードの標本データ |
