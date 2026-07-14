---
name: proc-macro-dev
description: Graphite の proc-macro (graph_schema!, graph! 等) を開発・デバッグする際の注意点
---

# proc-macro 開発の注意点

Graphite の `graphite-macros` クレートで手続き型マクロを開発する際に踏みやすい
落とし穴と、その対処をまとめる。

## 2 クレート分離と re-export パターン

proc-macro クレート (`proc-macro = true`) はランタイム型を直接持てない
(生成する側と生成されたコードが依存する側の型を同じクレートに置けない、という
Rust の技術的制約)。そのため `graphite-macros` (マクロ) と `graphite`
(ランタイム型・アルゴリズム) は分離されている。**利用者が直接依存するのは
`graphite` だけ**であり、マクロは `graphite` から re-export する
(serde/serde_derive と同じ構成)。新しいマクロを `graphite-macros` に追加したら、
`graphite::lib.rs` に `pub use graphite_macros::新マクロ;` を必ず追加すること。
`graphite-macros` への直接依存を利用者に要求する変更は避ける。

## カスタムトークン列と rustfmt

`-[label { attrs }]->` のような矢印記法は、`-`, `[`, ident, `{`, `}`, `]`, `-`,
`>` の並びとしてそれぞれ単独では合法な Rust トークンだが、rustfmt はこれを
「知らない構文」として扱い、整形が崩れる (意図しない改行・インデント) 可能性が
高い。`graph!` のようなマクロ呼び出しを含むテストコードや example には
`#[rustfmt::skip]` を付けて、rustfmt がマクロ引数の中身に手を出さないようにする。

## エラー報告は panic させず span を保つ

マクロのパース・検証エラーは `panic!` してはいけない (コンパイラ全体がクラッシュ
したような分かりにくい表示になる)。`syn::Error::new_spanned(&node, "メッセージ")`
で元のトークンの span を保った `syn::Error` を作り、`.to_compile_error()` で
`TokenStream` に変換して返す。これにより rust-analyzer 上でも該当箇所に赤波線が
出て、通常のコンパイルエラーと同じ体験になる。

## 展開結果の確認手段

- マクロ展開後の実際のコードを目視で確認したいときは `cargo expand` を使う
  (`cargo install cargo-expand` が必要)。手で書いた「展開後の予想コード」と
  実際の展開結果を突き合わせる。
- 「このマクロ呼び出しはコンパイルエラーになるべき」というテストは `trybuild`
  クレートを使う (`tests/ui/*.rs` + 期待する `.stderr` を用意し、実際のエラー
  メッセージと突き合わせる)。

## ビルドログ

proc-macro のコンパイルエラーは展開失敗も含めて警告・エラーが大量に出やすい。
CLAUDE.md 記載の `cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50`
形式を必ず使うこと。素の `cargo build` は出力が埋まってレビューできない。

## petgraph のキー制約

`petgraph::graphmap::GraphMap` (および `DiGraphMap`/`UnGraphMap`) はノードの
キー型に `Copy` を要求する。`String` のような `Copy` でないキーをそのままノード
キーにはできない。文字列キー (ユーザーキー、決定1で採用した方式) を扱う場合は
`petgraph::graph::DiGraph` (キーは内部の `NodeIndex`) を使い、
`HashMap<K, NodeIndex>` の索引テーブルを別途持ってユーザーキーと `NodeIndex` を
相互変換する構成にする。
