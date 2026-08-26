# クレート構成とその理由

> **Development document** — 索引: `docs/README.md`

この文書は、Graphite を構成する4つのクレートが何を担い、なぜ分かれているのかを
定める文書であり、実装が変わるたびに追随して更新する。Graphite を使うだけなら
読む必要はない。

## 4つのクレート

```
crates/graphite/         # ランタイムクレート。利用者が唯一 depend するクレート
crates/graphite-codegen/ # schemaの構文解析・検証・指紋・Rust生成を担う純粋層
crates/graphite-macros/  # コンパイル時検証・指紋照合とgraph!/flow!を担うproc-macroクレート
xtask/                   # 生成ファイルの探索・読み書き・差分検査、文書参照と索引の検査(check-docs)を担う開発用入口
```

## proc-macro クレートを分ける理由

proc-macro クレート (`proc-macro = true`) は手続き型マクロ、つまりコンパイラ
プラグインの一種である。生成する側 (マクロ) と、生成されたコードが依存する側
(ランタイム型) を同じクレートへ置くことはできない。これは Rust の技術的制約で
あって設計の選択ではない。serde と serde_derive、diesel、sqlx が同じ2分割を
採用しているのもこの制約による。

Graphite ではさらに2つの分割を加えている。マクロが生成する内容と通常ファイルの
内容が一致することは `graphite-codegen` を共有することで保証し、ファイルの
読み書きは `xtask` だけが行う。

利用者は `graphite` だけに依存する。マクロは `graphite::graph_schema!` /
`graphite::graph!` / `graphite::flow!` として re-export されたものを使い、
`graphite-macros` へ直接依存させることはしない。

## クレートの責務

- `graphite-codegen` は schema の構文解析、意味検査、指紋計算、Rustコード生成を
  行う純粋層である。
- `graphite-macros` はコンパイル時の schema 検査と指紋照合、および `graph!` と
  `flow!` の展開を行う。
- `xtask` は宣言元の探索、生成先の検査、生成ファイルの読み書きと差分検査、文書参照と
  索引の検査 (`cargo xtask check-docs`) を行う。
- `graphite` はグラフの実行時型を持ち、利用者が依存する入口としてマクロを
  再公開する。

## 関連文書

`crates/graphite` の実行時コードがどの概念をどのファイルへ置いているかは
`docs/development/runtime_structure.md` に記録している。生成の配線・生成先・
陳腐化の検出という規約の側は `docs/code_generation.md` が定める。
