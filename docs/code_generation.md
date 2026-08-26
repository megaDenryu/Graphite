# 追跡可能なRustコードの生成規約

> **Current reference** — 索引: `docs/README.md`

この文書は、Graphiteのschema DSLから通常のRustソースを生成し、公開APIの定義と実装を追跡可能に保つ規約を定める文書であり、実装が変わるたびに追随して更新する。

生成物の中身 (どの構文がどの型・値・関数になるか) は `docs/desugaring_reference.md` が正本である。この文書は生成の配線・生成先・陳腐化の検出・可視性の境界という規約の側を定める。

## 宣言と配線

利用者はschema宣言に生成先を指定し、同じRustファイルで生成moduleを明示的に読み込む。

```rust
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Org {
    include!("generated/org.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/org.rs";
    schema Org {
        node Person;
    }
}
```

指紋とは、schemaの内容と生成先パスから決定的に導かれる固定長の数値列であり、生成ファイルが最新かを判定する目印である (計算方法の詳細は後述「陳腐化の検出」参照)。`graph_schema!`はschemaを解析・検証し、生成ファイルに埋め込まれた指紋との一致だけをコンパイル時に検査する。schema moduleの型や実装は生成しない。`Org::Graph`、`Org::Builder`、ID型、NodeRef、EdgeRef、役割アクセサ、探索API、公開trait実装は、`generated/org.rs`だけに存在する。

生成moduleへ付ける属性は上の2行で固定する。`non_snake_case`はschema名をそのままmodule名にするため、`dead_code`は利用側が使わない生成物を許すために要る。`private_interfaces`は、Graphite内部の型ではなく、利用者が非公開で宣言した値型 (辺の積み荷型など) が生成コードの公開API (公開フィールド・公開メソッドの引数と戻り値) に現れるために要る。schemaはノード値型・辺属性型の可視性を検査しないため、利用者が`pub`を付け忘れた値型がこの形で公開APIに漏れることがある (例: `crates/graphite/tests/edge_roles.rs`の`TransactionInfo`)。clippy側の4件は、機械が書いたコードを人手のコードと同じ書き味で判定しないための指定である (省略できる生存期間、「`from`で始まる名前なのに`self`を消費する」という命名規約に反した書き方、Copy型に対する`clone`、書式文字列へ渡す型名リテラル)。この4件を許さないと、schemaの内容によっては利用者のビルドに警告が出る。

生成moduleの読み込みは、schema宣言と同じファイルへ置く。宣言の直前と直後のどちらでもよい。`include!`の相対パスは宣言元ファイルの位置を基準に解決する。これは`mod foo;`のファイル探索 (module の入れ子に応じて探索先が変わる) とは基準が異なり、`include!`はファイル位置基準でmoduleの入れ子に影響されない (入れ子moduleの中へ`include!`を移しても基準は変わらない。実例は`crates/graphite/tests/graph_cross_module.rs`参照)。`#[path]`属性で基準を移動させることはしない。

## 生成コマンド

作業ディレクトリをリポジトリルート (このリポジトリの最上位ディレクトリ) にして実行する。

```powershell
cargo xtask generate
cargo xtask generate --check
```

`generate`は全宣言を読み、期待する生成ファイルを更新する。`generate --check`はファイルを書き換えず、生成ファイルの不足と差分をエラーにする。

## 生成先

- 通常crateと例の `src/*.rs` にある宣言は、宣言元と同じ `src/generated/` に生成する。
- `crates/graphite/tests/*.rs` にある統合テストの宣言は、`crates/graphite/tests/generated/` に生成する。統合テストは1ファイルが1つのcrate rootであり、`src/` を持たないためである。`tests/generated/` 直下のファイルはどのテストのcrate rootにもならないので、`tests/` を走査する `cargo test` が生成物を単体のテストcrateとして拾うことはない。
- 宣言の `generated` は宣言元Rustファイルからの相対パス `generated/<名前>.rs` とする。絶対パスと `..` は許可しない。
- 生成ファイルはgitで管理する。ファイル先頭に手編集禁止、元DSLのリポジトリ相対パスと行、実行場所つきの再生成コマンドを記録する。

## 陳腐化の検出

`graphite-codegen`は検証済みschemaから決定的な指紋を作り、生成moduleへ埋め込む。指紋の実体はFNV-1a (64bit) を4種の初期値でそれぞれ計算した`[u64; 4]`であり、暗号強度のハッシュではなく偶発的な取り違え (schemaの位置移動・生成器の変更を含む) を検出するための目印である。`graph_schema!`も同じ純粋層から指紋を得てconst評価で比較する。schemaの意味を変更して生成し忘れた場合、通常の`cargo build`がコンパイルエラーになるため、古い公開APIが黙って残らない。

`cargo xtask generate --check`は生成本文全体をバイト単位で比較する。schemaの位置移動、生成器の変更、コメントに記録する元DSL位置の変化も検出する。

## 関連文書

生成物のどれが公開でどれが非公開かは `docs/desugaring_reference.md` §22 が一覧で定める。`graph!`の名前付きラッパーだけを生成ファイルへ事前生成しない理由は同 §26.5 にある。クレートごとの責務は `docs/development/crate_architecture.md` が定める。
