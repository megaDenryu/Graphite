# build-pipeline

Graphite を使った「ビルドパイプライン・オーケストレータ」の実践example。
CI/タスクランナーが内部で持っているような実行計画ツールを、`Task` と
`Artifact` という異種ノード + `produces`/`consumes` の型付きエッジで表現する。

## 概要

現実の Rust プロジェクトのビルドを模した、20タスク・23アーティファクトから
なる多段パイプライン (`fetch -> codegen -> build -> test / lint -> doc ->
package -> deploy`) を `pipeline.txt` (簡易行形式) として同梱している。これを
実行時にパースし、`graphite::graph_schema!` で宣言したグラフスキーマへ
組み立てたうえで、以下を行う CLI ツール。

- **validate**: 図式適合 (`graph_schema!` が保証する形の正しさ) に加えて、
  「誰も produce しない artifact を consume している (孤児成果物)」「同じ
  artifact を2つのタスクが produce している (競合)」「タスク依存が循環して
  いる」というドメイン固有の妥当性検査を行う
- **plan**: 依存関係から「並列実行可能なタスクの波」を計算する (無限並列
  ワーカーを仮定した理想的なスケジュール)
- **critical-path**: タスクの想定実行時間を重みとした最長経路 (クリティカル
  パス) と、全体並列度 (総作業量 / クリティカルパス長) を計算する
- **mermaid**: グラフを mermaid flowchart として出力する (Task/Artifact で
  ノード形状を描き分ける)

### スキーマ

```rust
pub struct Task { pub name: String, pub cmd: String, pub secs: u32 }
pub struct Artifact { pub path: String }

graphite::graph_schema! {
    schema BuildPipeline {
        node Task;
        node Artifact;

        edge Task -[produces]-> Artifact (0..*);
        edge Task -[consumes]-> Artifact (0..*);
    }
}
```

`src/schema.rs` に定義がある。

## ディレクトリ構成

| ファイル | 行数目安 | 役割 |
|---|---|---|
| `src/schema.rs` | 約60行 | `graph_schema!` によるスキーマ宣言 + `graph!` リテラルのショーケース (固定の小さなパイプライン) |
| `src/parser.rs` | 約240行 | `pipeline.txt` の簡易行形式パーサ (行番号付きエラー) |
| `src/builder.rs` | 約100行 | パース結果から `BuildPipeline` グラフを構築 (成果物ノードの暗黙生成を含む) |
| `src/analysis.rs` | 約380行 | ドメイン検証・実行計画 (波)・クリティカルパスの計算ロジック |
| `src/report.rs` | 約190行 | CLI 出力の整形 (表・mermaid) |
| `src/lib.rs` | 数行 | 上記モジュールを re-export するライブラリクレート (統合テストから使うため) |
| `src/main.rs` | 約110行 | CLI エントリポイント (サブコマンド振り分け) |
| `pipeline.txt` | - | 同梱のサンプルパイプライン定義 (20タスク・23アーティファクト) |
| `tests/integration.rs` | 約190行 | 統合テスト |

## 使い方

作業ディレクトリは `examples/build-pipeline/` (同梱の `pipeline.txt` を
カレントディレクトリから読み込むため)。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run -- validate
cargo run -- plan
cargo run -- critical-path
cargo run -- mermaid
```

`pipeline-file` 引数を省略すると `./pipeline.txt` を読み込む。別のファイルを
検査したい場合は `cargo run -- validate my_pipeline.txt` のように第2引数で
指定する。

### `validate` の実行例

```
$ cargo run -q -- validate
ドメイン検証: 違反なし (孤児成果物 / produce競合 / 循環依存のいずれも検出されませんでした)
```

循環依存を仕込んだファイルを渡すと (`examples/` 直下ではなく検証用に用意した
壊れたファイルの例):

```
$ cargo run -q -- validate broken_cycle.txt
ドメイン検証: 1件の違反を検出しました
  [1] 循環依存: タスク b を経由する依存の循環が検出されました
```

孤児成果物・produce競合も同様に `[番号] 種別: 詳細` の形式で1行ずつ報告する
(`src/analysis.rs::DomainIssue` の `Display` 実装)。

### `plan` の実行例

```
$ cargo run -q -- plan
波  所要時間   タスク (この波の中で並列実行可能)
--  --------   --------------------------------
1       30s   fetch_deps
2       45s   codegen_grpc, codegen_proto
3      120s   build_core, lint_fmt
4       90s   build_net, build_util, test_core
5      100s   build_api, test_net, test_util
6      110s   build_cli, build_server, lint_clippy, test_api
7      150s   doc_build, test_integration
8       35s   package_cli, package_docs, package_server
9       25s   deploy_staging, publish_docs
10      30s   deploy_prod

波の合計 (逐次実行した場合の下限見積り): 735秒 / 10波
```

各波は「依存タスクが全て完了済みで、今すぐ並列実行を開始できるタスクの
集合」。波の所要時間は波内タスクの `max(secs)` (無限並列ワーカーを仮定)。

### `critical-path` の実行例

```
$ cargo run -q -- critical-path
クリティカルパス (依存関係上、最も時間がかかる経路):
  fetch_deps (30s)  -> codegen_proto (45s)  -> build_core (120s)  -> build_net (90s)  -> build_api (100s)  -> build_server (110s)  -> test_integration (150s)  -> package_server (35s)  -> deploy_staging (25s)  -> deploy_prod (30s)

合計時間: 735秒
全タスクの所要時間合計 (総作業量): 1387秒
全体並列度 (総作業量 / クリティカルパス長): 1.89倍
```

同梱の `pipeline.txt` ではクリティカルパスの合計 (735秒) が波の合計ともちょうど
一致する。これは偶然ではなく、各波の最大所要時間タスクがちょうど次段への
唯一の継続経路になるようにサンプルデータを組んだため (`build_net` が
`build_core` と `codegen_grpc` の双方を待つ、のように分岐が波の途中で合流する
構成)。一般のパイプラインではこの2値は一致しない。

### `mermaid` の実行例 (抜粋)

```
$ cargo run -q -- mermaid
flowchart TD
    T_build_core["build_core (120s)"]
    ...
    A_target_core_rlib[("target/core.rlib")]
    ...
    T_build_core -->|produces| A_target_core_rlib
    A_target_core_rlib -->|consumes| T_build_net
```

`Task` は矩形 (`["..."]`)、`Artifact` は円柱形 (`[("...")]`、「保存された
成果物」を表す慣用のノード形状) で描き分けている。`consumes` は可読性を
優先し、矢印を `Artifact -> Task` 方向 (成果物がタスクへ流れ込む向き) に
描いている (スキーマ上の `from`/`to` の向きとは逆。詳細は `src/report.rs`
のコメント参照)。

## `pipeline.txt` の文法

```
# コメント行・空行は無視される
task <名前>: <コマンド...> (<秒数>s)      タスク定義
<タスク名> produces <パス>                そのタスクが生成する成果物
<タスク名> consumes <パス>                そのタスクが読み込む成果物
```

`Artifact` ノードは専用の宣言行を持たず、`produces`/`consumes` 行に現れた
パスの集合から `src/builder.rs` が暗黙的に生成する (成果物ごとに宣言行を
書かせるのは冗長なため)。パースエラーは行番号付きで報告される
(`src/parser.rs` の `ParseError`)。

## Graphite を使う意味

このアプリの本質的な難しさは「`Task` と `Artifact` という異種ノード」+
「`produces`/`consumes` という2種類の型付きエッジ」+「タスク間依存はエッジの
直接の中身ではなく、エッジ2本 (produces と consumes) を artifact 経由で
合成して初めて得られる導出情報」という点にある。もし `HashMap` と `Vec` を
素朴に自作していたら、以下を自分で手書きする羽目になっていたはずである。

- **型付きアクセサの手書き**: `HashMap<String, Task>` と
  `HashMap<String, Artifact>` を別々に持つと、「このキーは Task 用か
  Artifact 用か」を呼び出し側が毎回意識する必要がある。`graphite` では
  `g.task(&TaskId)` / `g.artifact(&ArtifactId)` のようにキー型自体が newtype
  で区別されるため、`ArtifactId` を `task()` に渡すコードはコンパイルが通ら
  ない。素朴な `HashMap<String, _>` 2枚構成ではこの区別が実行時までに検出
  されない (例: `produces` のキーに誤って artifact のパスではなくタスク名を
  入れてしまう、といったバグを型で防げない)。
- **freeze による一括検証**: `produces`/`consumes` が指すタスク名・パスが
  必ず宣言済みであること (`UnknownTask`/`UnknownArtifact`) は、素の
  `HashMap<String, Vec<String>>` 自作では「エッジ追加のたびに毎回両端の
  存在確認を書く」か「確認を省略して後で謎の `panic`/データ不整合に悩む」
  かの二択になりがちである。`BuildPipeline::create` は builder に積んだ
  内容をクロージャの外に一切漏らさず (借用検査器が保証)、戻ってきた瞬間に
  一括で凍結・検証するため、「検証を書き忘れたパス」が原理的に存在しない。
- **多重度 API と `Vec` の使い分け**: `produces`/`consumes` は `(0..*)` なので
  `g.produces().of(&TaskId) -> Vec<&Artifact>` が自動生成される。自作なら
  `HashMap<String, Vec<String>>` を用意したうえで `.get(key).cloned()
  .unwrap_or_default()` のような空デフォルト処理を毎回書く必要がある
  (書き忘れると未知キーで `panic`)。
- **`{label}().iter()`**: 本アプリのドメイン検証 (孤児成果物・
  produce競合・循環依存) はどれも「全 produces ペア」「全 consumes ペア」を
  俯瞰して初めて判定できる (1本のエッジだけを見ても分からない)。
  `g.produces().iter()`/`g.consumes().iter()` が無ければ、内部の
  `HashMap<K, Vec<V>>` を手でフラット化するイテレータをアプリ側に毎回書く
  ことになる。ここでは `analysis.rs` の `validate`/`task_dependency_graph`
  がこれをそのまま使い、artifactごとの producer/consumer 集合を
  `HashMap<&ArtifactId, Vec<&TaskId>>` へ畳み込むだけで済んでいる。
- **`{node_snake}_ids()` による全件列挙**: `plan`/`critical-path` は
  「全タスク」を起点にした波・経路の計算が要る。`g.task_ids()` が無ければ、
  ノード用 `HashMap` のキー列挙を毎回 `.keys()` で取り出したうえで、それが
  「タスクのキーである」という前提をコメントで祈るしかない。
- **タスク依存グラフへの射影と汎用アルゴリズムの再利用**: 「タスク A は
  タスク B に依存する」という関係は `produces`/`consumes` のどちらの
  フィールドにも直接存在せず、`consumes ∘ produces⁻¹` として2つのエッジ種別
  を artifact をキーに合成して初めて得られる (README 本体の「導出エッジ」の
  考え方そのもの)。素朴な自作なら、この合成ロジックとグラフアルゴリズム
  (循環検出・トポロジカルソート) の両方を1から書く必要がある。ここでは
  合成だけをアプリ側 (`analysis::task_dependency_graph`) で書き、循環検出・
  トポロジカルソートは汎用 `graphite::Graph<(), (), TaskId>` の
  `has_cycle`/`topological_sort`/`out_neighbors` にそのまま委譲している
  (`Graph` はノード型 `N`・キー型 `K` を自由に選べる設計になっているため、
  「値はいらないが依存関係の形だけ知りたい」というこの用途に `N = ()` が
  ぴったりはまる)。

まとめると、Graphite は「型付きアクセサ」「freeze一括検証」「多重度ごとの
戻り値」「pairs/ids イテレータ」「汎用グラフアルゴリズムへの射影」という
5点セットを生成・提供することで、CI パイプラインのような「異種ノード +
型付きエッジ + 導出関係」を持つドメインの手書きコストを大きく下げている。

## テスト

```powershell
cargo test
```

- 各モジュール内の単体テスト (`parser`/`builder`/`analysis`/`report`/`schema`)
- `tests/integration.rs`:
  - 同梱 `pipeline.txt` がボリューム要件 (20タスク・15アーティファクト以上)
    を満たし、図式適合・ドメイン違反ゼロで波計画・クリティカルパスを計算
    できること
  - 仕込みエラー (循環依存・孤児成果物・produce競合) をそれぞれ独立に検出
    できること
  - パーサの異常系 (コロンなし・秒数欠落・秒数の単位なし・未知キーワード・
    複数行にまたがる行番号の正しさ) が行番号付きで報告されること
  - 手計算できる小さな既知データ (`fetch -> {build_a, build_b} -> link`) での
    波数・クリティカルパス長の一致
