# ドキュメント索引

このファイルは `docs/` 配下の全ファイルを列挙する。索引に載っていない文書を
`docs/` へ置いてはならない。過不足は `cargo xtask check-docs` が検査する。

各文書の状態は次の4種類のいずれかである。

- **Current reference** とは、現行の仕様・契約を定義する文書のことである。
- **Current guide** とは、現行の機能の使い方を説明する文書のことである。
- **Development document** とは、Graphite 自身を実装・保守する人へ向けた文書の
  ことである。
- **Historical document** とは、過去の決定・経緯の記録であり、現行仕様ではない
  文書のことである。

版番号を名前に含む文書 (`schema_v4.md` ・ `edge_endpoints_v4_1.md` ・
`node_id_v4_2.md`) がある。この版番号は仕様が確定した版を指しており、内容は
現行仕様である。

## 現行の仕様とガイド (`docs/`)

| ファイル | 状態 | 内容 |
|---|---|---|
| `docs/README.md` | 索引 | このファイル。`docs/` 配下の全ファイルを列挙する |
| `docs/desugaring_reference.md` | Current reference | 脱糖の正本。どの構文がどの普通のRustの型・値・関数になるかを構文ごとに定義する |
| `docs/schema_v4.md` | Current reference | schema 宣言の構文と、辺の第一級化・`where` 制約・Graph中心の種別APIの仕様 |
| `docs/edge_endpoints_v4_1.md` | Current reference | 端点の役割名を必須とする有向辺と、無向辺の宣言の仕様 |
| `docs/node_id_v4_2.md` | Current reference | ノード・辺のID型を既定生成と明示指定から選ぶ規則 |
| `docs/reverse_query.md` | Current reference | 役割名からの逆引き探索メソッドの契約 |
| `docs/bulk_construction.md` | Current reference | 実行時データから構築する `Builder::extend` の契約 |
| `docs/graph_splice.md` | Current reference | `graph!` のスプライス構文 (`..式`) と `extend` への統一 |
| `docs/code_generation.md` | Current reference | 生成の配線・生成先・陳腐化の検出という生成規約 |
| `docs/static_graph.md` | Current reference | `static_schema!` (issue #24、全個体がコンパイル時に確定するグラフ) の構文・生成される名前の公開契約・コンパイル時検査の一覧 |
| `docs/modeling_guide.md` | Current guide | 何をグラフの要素として書き、何を構造体のフィールドとして書くかの判断基準 |
| `docs/flow_macro.md` | Current guide | `flow!` の構文と意味論 |
| `docs/compute_graph.md` | Current guide | `ComputeGraph<V>` の遅延評価と差分再計算の使い方 |

## 開発者向け (`docs/development/`)

| ファイル | 状態 | 内容 |
|---|---|---|
| `docs/development/design_principles.md` | Development document | Rust的な精神と型の strictness を具体化した6原則 |
| `docs/development/crate_architecture.md` | Development document | 5つのクレートの責務と、proc-macro クレートを分ける理由 |
| `docs/development/runtime_structure.md` | Development document | `crates/graphite/src` の実行時コードの配置と依存の向き |
| `docs/development/line_count_ledger.md` | Development document | 1ファイル100行原則のコード行の数え方と、超過を許したファイルの台帳 |
| `docs/development/ide_support_spec.md` | Development document | rust-analyzer 対応の仕様と、スパン継承の規範 |
| `docs/development/generated_vs_handwritten.md` | Development document | 生成コードが手書きテンプレートと分かれた7点の設計判断 |
| `docs/development/testing.md` | Development document | テストファイルの役割と実行手順、`.vscode` の運用ルール |

## 設計史 (`docs/history/`)

| ファイル | 状態 | 現行の置換先 | 内容 |
|---|---|---|---|
| `docs/history/design_journal.html` | Historical document | `docs/desugaring_reference.md` | v1〜v4.2 の全過程を通読するための読み物 |
| `docs/history/dev_history_2026-07-14_session1.md` | Historical document | `docs/desugaring_reference.md` | 立ち上げセッションの開発履歴 |
| `docs/history/dev_history_2026-07-14_session2.md` | Historical document | `docs/development/ide_support_spec.md` | rust-analyzer 対応セッションの開発履歴 |
| `docs/history/edge_syntax_v2.md` | Historical document | `docs/desugaring_reference.md` §2・§4 と `docs/schema_v4.md` §1 | ノード型・積み荷型を外部 struct 参照にした構文 v2 の決定 |
| `docs/history/edge_syntax_v3.md` | Historical document | `docs/schema_v4.md` | ラベルを型として扱う矢印式の構文 v3 の決定 |
| `docs/history/edge_view_api.md` | Historical document | `docs/desugaring_reference.md` §17・§18 | 全廃されたビュー6型によるエッジアクセスAPIの決定 |
| `docs/history/graph_literal_v3.md` | Historical document | `docs/schema_v4.md` | `graph!` リテラル構文 v3 とハンドシェイクマクロ全廃の決定 |
| `docs/history/phase4_open_questions.md` | Historical document | `docs/desugaring_reference.md` | フェーズ3終了時点の未決事項と、フェーズ4での対応関係 |
| `docs/history/phase5_candidates.md` | Historical document | GitHub の Issue | 実践example から報告されたAPI不足点9件と知見1件の記録。8件は解決済みで、平坦な名前空間の1件は設計判断として見送りが決まっている。未処置の項目は残っていない |
