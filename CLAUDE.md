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

## 2 クレート構成とその理由

```
crates/graphite/         # ランタイムクレート。利用者が唯一 depend するクレート
crates/graphite-macros/  # proc-macro クレート (graph_schema!, graph! を実装する)
```

`graphite-macros` はなぜ分離が必要か: proc-macro クレート (`proc-macro = true`) は
手続き型マクロ=コンパイラプラグインの一種であり、生成する側 (マクロ) と生成された
コードが依存する側 (ランタイム型) を同じクレートに置けない、という **Rust の技術的
制約**です。選択の余地はありません (serde/serde_derive、diesel、sqlx が全て同じ
2 分割を採用しているのはこのため)。

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
```

**ビルドコマンドは必ず** `cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50`
**の形で実行する。** 素の `cargo build` を使うと大量の警告で出力が埋まってレビュー
不能になる。

## リポジトリ固有のルール

- **コミットメッセージは日本語**

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
