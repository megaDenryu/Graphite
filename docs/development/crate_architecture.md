# クレート構成とその理由

この文書は、Graphite を構成する4つのクレートが何を担い、なぜ分かれているのかを
定める生存型文書である。Graphite を使うだけなら読む必要はない。

## 4つのクレート

```
crates/graphite/         # ランタイムクレート。利用者が唯一 depend するクレート
crates/graphite-codegen/ # schemaの構文解析・検証・指紋・Rust生成を担う純粋層
crates/graphite-macros/  # コンパイル時検証・指紋照合とgraph!/flow!を担うproc-macroクレート
xtask/                   # 生成ファイルの探索・読み書き・差分検査を担う開発用入口
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

## 関連文書

`crates/graphite` の実行時コードがどの概念をどのファイルへ置いているかは
`docs/development/runtime_structure.md` に記録している。生成の配線・生成先・
陳腐化の検出という規約の側は `docs/code_generation.md` が定める。
