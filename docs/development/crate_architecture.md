# クレート構成とその理由

> **Development document** — 索引: `docs/README.md`

この文書は、Graphite を構成する5つのクレートが何を担い、なぜ分かれているのかを
定める文書であり、実装が変わるたびに追随して更新する。Graphite を使うだけなら
読む必要はない。

## 5つのクレート

```
crates/graphite/         # ランタイムクレート。利用者が唯一 depend するクレート
crates/graphite-codegen/ # schemaの構文解析・検証・指紋・Rust生成を担う純粋層
crates/graphite-macros/  # コンパイル時検証・指紋照合とgraph!/flow!を担うproc-macroクレート
crates/graphite-cli/     # 生成コア(宣言の探索・生成計画・ファイルの読み書きと差分検査)と cargo-graphite バイナリ
xtask/                   # Graphite リポジトリ自身の開発用入口。全パッケージの生成と、文書参照と索引の検査(check-docs)
```

## proc-macro クレートを分ける理由

proc-macro クレート (`proc-macro = true`) は手続き型マクロ、つまりコンパイラ
プラグインの一種である。生成する側 (マクロ) と、生成されたコードが依存する側
(ランタイム型) を同じクレートへ置くことはできない。これは Rust の技術的制約で
あって設計の選択ではない。serde と serde_derive、diesel、sqlx が同じ2分割を
採用しているのもこの制約による。

Graphite ではさらに2つの分割を加えている。マクロが生成する内容と通常ファイルの
内容が一致することは `graphite-codegen` を共有することで保証し、ファイルの
読み書きは `graphite-cli` とその利用者 (`xtask`) だけが行う。

利用者は `graphite` だけに依存する。マクロは `graphite::graph_schema!` /
`graphite::graph!` / `graphite::flow!` として re-export されたものを使い、
`graphite-macros` へ直接依存させることはしない。

## 生成の入口を2つに分ける理由

生成ファイルを書き出す入口は2つある。Graphite リポジトリ自身の `cargo xtask
generate` と、外部 crate 向けの `cargo graphite generate` である。この2つで違うのは
**どのパッケージを対象にするかだけ**であり、schema宣言の抽出・生成計画・書き込み・
差分検査は `graphite-cli` の `GenerationTree` (基準ディレクトリと走査開始点の一覧を
持つ、生成対象のディレクトリ木を表す型) を通して完全に共有する。

| 入口 | 対象を決める型 | 対象のパッケージ | 1パッケージあたりの走査開始点 |
|---|---|---|---|
| `cargo xtask generate` | `xtask::RepositoryRoot` | `crates/*` ・ `examples/*` の全部 | パッケージ直下の `src` ・ `tests` |
| `cargo graphite generate` | `graphite_cli::PackageRoot` | 実行した場所のパッケージ1つ | パッケージ直下の `src` ・ `tests` |

分けるのは、Graphite リポジトリが複数パッケージを1回の実行で処理する必要があり、
外部 crate は自分のパッケージだけを対象にするためである。走査開始点を設定で
切り替える形にはしない。どちらも規約で決まる開始点であり、選択の自由度は要らない。

`xtask` も1パッケージにつき1つ `GenerationTree` を作り、基準ディレクトリを
パッケージルートにする。リポジトリルートを基準にした1つの木で全パッケージを
まとめて処理すると、生成ファイルの2行目へ書く宣言元の綴りが両入口で食い違い、
一方が書いたファイルをもう一方が古いと判定する。

共有する側を分けないと、生成した内容が入口ごとにずれる。生成ファイルの先頭に書く
案内コメントを入口ごとに書き分けないのも同じ理由である (`docs/code_generation.md`
「再生成の案内」参照)。両入口の本文が一致することは
`xtask/tests/entry_point_agreement.rs` が `cargo test` の中で実測する。

## クレートの責務

- `graphite-codegen` は schema の構文解析、意味検査、指紋計算、Rustコード生成を
  行う純粋層である。ファイルの読み書きは行わない。
- `graphite-macros` はコンパイル時の schema 検査と指紋照合、および `graph!` と
  `flow!` の展開を行う。
- `graphite-cli` は宣言元の探索、生成先の検査、生成ファイルの読み書きと差分検査を
  行い、`cargo graphite generate [--check]` のバイナリ (`cargo-graphite`) を提供する。
- `xtask` は Graphite リポジトリ自身の開発用入口である。`crates/*` と `examples/*`
  の全パッケージを順に `graphite-cli` へ渡し、加えて文書参照と索引の検査
  (`cargo xtask check-docs`)、外部 crate からの生成経路の実走検査
  (`cargo xtask check-external`) を行う。
- `graphite` はグラフの実行時型を持ち、利用者が依存する入口としてマクロを
  再公開する。

## 関連文書

`crates/graphite` の実行時コードがどの概念をどのファイルへ置いているかは
`docs/development/runtime_structure.md` に記録している。生成の配線・生成先・
陳腐化の検出という規約の側は `docs/code_generation.md` が定める。
