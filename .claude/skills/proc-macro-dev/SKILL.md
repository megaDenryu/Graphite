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

## エラー回復パーサと `ParseBuffer` の Drop 落とし穴

宣言単位でエラーを蓄積しつつ続行する回復パーサ (G4) を書くと、`syn::ParseBuffer`
特有の罠を踏みやすい。`ParseBuffer` は drop 時に「デリミタ内に未消費トークンが
残っていないか」を自動チェックし、残っていれば共有の `Unexpected` セルにその
位置を記録する。この記録は `syn::parse::Parser::parse2`/`parse` が呼び出し全体の
最終チェックで読み、**呼び出し元が最終的に `Ok` を返した場合にのみ**
「unexpected token」という無関係な幽霊エラーとして再浮上させる。

エラーが `?` でそのまま最後まで伝播する通常のパーサではこの問題は顕在化しない
(その場合は呼び出し元も `Err` を返すため)。一方、内側の `Err` を握りつぶして
次の境界まで読み飛ばし、パース全体としては `Ok` を返す回復パーサ (G4 のような
設計) は、まさにこの状況を作り出す。

対処: `parenthesized!`/`braced!`/`bracketed!` で開いた内側の `ParseBuffer`
(`content`) について、`Err` を返す前に必ず中身を空にする (drain_rest)。

```rust
fn drain_rest(content: ParseStream) {
    let _ = content.parse::<proc_macro2::TokenStream>();
}
```

`content.parse::<proc_macro2::TokenStream>()` は残りトークンを丸ごと読み捨てる
ので、これを `Err` を返す全ての分岐の直前に呼ぶ。分岐が多い関数では
「本体を別関数に切り出し、呼び出し元がエラー時に一括で drain する」形にすると
書き漏らしにくい (実例は `crates/graphite-macros/src/schema_dsl.rs` の
`parse_fields_block`/`Multiplicity::parse` を参照)。

## スパンポリシー

生成する識別子は必ず「由来するユーザートークンのスパン」を持たせる。これが
rust-analyzer の definition provider (F12 の着地点) の精度を決める。

- 型名・フィールド名は `decl.name` 系のスパンを継承する。エッジ派生名
  (`try_belongs_to`, `*_id`, `*_ids` 等) は `edge.label` のスパンを継承する。
- `format_ident!` は最初に補間された `Ident` のスパンを継承する
  (rust-analyzer の definition provider で実測確認済み、2026-07-14)。
  補間引数が `String`/`&str` のみの場合はこの継承が働かないため、
  `span = ..` を明示すること (例: `to_pascal_case` した文字列から
  `{Label}Attrs` のような型名を作る場合)。
- 新しいコード生成を追加したら、definition provider の
  `targetSelectionRange` がユーザートークンに着地することを確認する。
  計測には vscode-lsp-mcp が使える。

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
