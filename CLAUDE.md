# CLAUDE.md

このファイルは Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイドです。

## プロジェクト概要

Graphite は、自作言語 Vertex (`../Bullet`) のグラフ機能の設計検討から派生した、
**独立した Rust プロジェクト**です。Vertex 本体 (`../Bullet`) とは切り離されており、
Vertex 言語処理系のコードには一切依存しません。

Vertex 側では「グラフ指向」を独立言語の構文・型システムとして実装する道を選びましたが、
その設計を壁打ちする過程で「グラフはあくまで既存言語 (Rust) の型システムと所有権に
乗るデータ構造として実装でき、DSL 部分だけを proc マクロ + ライブラリとして切り出せる
のではないか」という仮説が生まれました。Graphite はその仮説を実際に検証するプロジェクト
です。

**設計の一次資料** (実装で判断に迷ったら必ずこの 2 つを読み直すこと):

- `../Bullet/docs/rust_graph_extension_sketch.md` — 全体構成 (2 クレート構成)・
  水準1/水準2 の Rust での実現方針・`graph_schema!`/`graph!` の展開イメージ・
  未決の問い・最初の一歩の提案
- `../Bullet/docs/graph_design_sketches.md` — グラフ型そのものの設計決定 1〜6
  (ノード同一性、可変性、矢印記法、多重度検査、可視性、型推論) とその論拠

これらは Vertex (独立言語) 側の文書であり Graphite の一部ではありませんが、
Graphite の設計判断のほとんどはここで既に検討済みです。車輪の再発明をする前に
必ず参照してください。

## 5 クレート構成とその理由

```
crates/graphite/         # ランタイムクレート。利用者が唯一 depend するクレート
crates/graphite-codegen/ # schemaの構文解析・検証・指紋・Rust生成を担う純粋層
crates/graphite-macros/  # コンパイル時検証・指紋照合と graph!/flow! を担うproc-macroクレート
crates/graphite-cli/     # 生成コア(宣言の探索・生成計画・読み書きと差分検査)と cargo-graphite バイナリ
xtask/                   # Graphite リポジトリ自身の開発用入口。ワークスペース全体の走査と文書検査
```

`graphite-macros` はなぜ分離が必要か: proc-macro クレート (`proc-macro = true`) は
手続き型マクロ=コンパイラプラグインの一種であり、生成する側 (マクロ) と生成された
コードが依存する側 (ランタイム型) を同じクレートに置けない、という **Rust の技術的
制約**です。選択の余地はありません (serde/serde_derive、diesel、sqlx が全て同じ
2 分割を採用しているのはこのため)。Graphiteではさらに、マクロとファイル生成が
同じ処理を使うように`graphite-codegen`へ純粋な生成処理を分離し、ファイルI/Oは
`graphite-cli`だけへ閉じ込める。

生成の入口は2つある。外部crate向けの`cargo graphite generate`と、Graphite自身の
`cargo xtask generate`である。この2つで違うのは走査開始点だけであり (前者は
パッケージ直下の`src`・`tests`、後者はワークスペース全体)、抽出・生成計画・
書き込み・差分検査は`graphite-cli`の`GenerationTree`を通して共有する。
生成ファイルの案内コメントと指紋エラーの文言を入口ごとに書き分けてはならない。
書き分けると、一方が書いたファイルをもう一方が古いと判定する。

利用者は `graphite` だけに依存し、`graphite-macros` のマクロは `graphite` から
re-export される想定です (`graphite::graph_schema!` のように使う。serde が
`serde_derive` を `serde::Serialize` として re-export しているのと同じ構成)。
`graphite-macros` に直接依存させることはしません。

## 開発コマンド

```powershell
# ビルド (エラー出力を短く保つ運用形式。素の cargo build は使わない)
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50

# テスト
cargo test

# 追跡可能なschema Rustコードを生成・検査
cargo xtask generate
cargo xtask generate --check

# 文書参照の綴りの実在と docs/README.md 索引の網羅を検査
cargo xtask check-docs

# 外部crateからの生成経路 (verification/external-crate) を実走で検査
cargo xtask check-external

# 外部crate向けの生成器を入れる (利用者が行う手順。開発中は cargo run で足りる)
cargo install --path crates/graphite-cli
cargo graphite generate [--check]
```

**ビルドコマンドは必ず** `cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50`
**の形で実行する。** 素の `cargo build` を使うと大量の警告で出力が埋まってレビュー
不能になる。

## リポジトリ固有のルール

- **コミットメッセージは日本語**
- 新機能・API設計で判断に迷ったら `docs/development/design_principles.md` (Rust的な精神・
  型のstrictnessを具体化した6原則) を必ず参照する
- Graphite自身の開発ツールの入口は`cargo xtask`へ集約する。現在のコマンドは
  `cargo xtask generate [--check]`・`cargo xtask check-docs`・
  `cargo xtask check-external`である。外部crateの利用者向けの入口は
  `cargo graphite generate [--check]` (バイナリ名`cargo-graphite`) である
- 文書の配置は3つに分ける。現行の仕様とガイド (実装に追随して更新し続ける文書) は
  `docs/`直下、Graphite自身を実装・保守する人向けの文書は`docs/development/`、
  設計史・旧仕様・開発記録は`docs/history/`へ置く
- `docs/`配下へファイルを足したら`docs/README.md`の索引へ1行加える。索引に載って
  いない文書を置いてはならない。各文書の冒頭には状態 (Current reference /
  Current guide / Development document / Historical document。定義は
  `docs/README.md` を参照) を1行で表示する
- 文書間の参照の綴りと索引の網羅は`cargo xtask check-docs`が検査する。文書を移動・
  改名したら、このコマンドが通る状態にしてからコミットする
- `docs/history/`の文書の本文は書き換えない。参照先の綴りの追随だけを行う
- READMEの第一読者はGraphiteを使うRust利用者である。内部実装・開発規約・設計史を
  READMEへ書かず、`docs/development/`と`docs/history/`へ置いて案内だけを残す

## 運用ポリシー (重要): モデル委譲

開発コストの高いオーケストレータモデル (Fable/Opus) は **方針策定・タスク分解・
レビューに徹し、コードは書かない**。以下は必ず Sonnet subagent (`model: sonnet`,
`effort: high`) に委譲する:

- 実装 (Rust コードの追加・変更)
- テストの作成・実行
- git 操作 (add / commit / branch 操作など)
- 複数エージェントを並行運用した際の競合解消・収斂作業

オーケストレータは委譲した subagent の成果物をレビューし、次の方針を決めることに
専念する。これは「開発コストが高いモデルに実装労働をさせるのは無駄」という判断に
基づく、Bullet プロジェクトの運用方針を踏襲したもの。

実装エージェントの定義は `.claude/agents/impl.md` に、proc-macro 開発時の注意点は
`.claude/skills/proc-macro-dev/SKILL.md` に集約されている。

## 実装フェーズ計画

1. **① 足場 (完了)** — cargo workspace 構成、2 クレートの骨格、CLAUDE.md/エージェント
   定義/スキル定義。グラフ実装はまだ無い。
2. **② 水準1ランタイム + 水準2手書きターゲット** — マクロ無しでジェネリック
   `Graph<N, E, K>` (petgraph ラッパー: `has_cycle`/`topological_sort`/
   `reachable_from` 等) を `graphite` に実装する。続けて `OrgChart` 相当の
   図式グラフ (水準2) を**マクロを使わず手書き**し、`graph_schema!` が生成すべき
   コードの実際の形・量を確認する。
3. **③ `graph_schema!`/`graph!` マクロ実装** — ②で確認した手書きコードの形を
   テンプレートに、`graphite-macros` で宣言マクロ (`graph_schema!`) と
   インスタンスリテラルマクロ (`graph!`) を実装する。

各フェーズの詳細な設計判断は `rust_graph_extension_sketch.md` の「最初の一歩の提案」
と「未決の問い」を参照。
