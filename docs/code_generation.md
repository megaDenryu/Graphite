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
graphite::dynamic_graph_schema! {
    generated = "generated/org.rs";
    schema Org {
        node Person;
    }
}
```

指紋とは、schemaの内容と生成先パスから決定的に導かれる固定長の数値列であり、生成ファイルが最新かを判定する目印である (計算方法の詳細は後述「陳腐化の検出」参照)。`dynamic_graph_schema!`はschemaを解析・検証し、生成ファイルに埋め込まれた指紋との一致だけをコンパイル時に検査する。schema moduleの型や実装は生成しない。`Org::Graph`、`Org::Builder`、ID型、NodeRef、EdgeRef、役割アクセサ、探索API、公開trait実装は、`generated/org.rs`だけに存在する。

生成moduleへ付ける属性は上の2行で固定する。`non_snake_case`はschema名をそのままmodule名にするため、`dead_code`は利用側が使わない生成物を許すために要る。`private_interfaces`は、Graphite内部の型ではなく、利用者が非公開で宣言した値型 (辺の積み荷型など) が生成コードの公開API (公開フィールド・公開メソッドの引数と戻り値) に現れるために要る。schemaはノード値型・辺属性型の可視性を検査しないため、利用者が`pub`を付け忘れた値型がこの形で公開APIに漏れることがある (例: `crates/graphite/tests/edge_roles.rs`の`TransactionInfo`)。clippy側の4件は、機械が書いたコードを人手のコードと同じ書き味で判定しないための指定である (省略できる生存期間、「`from`で始まる名前なのに`self`を消費する」という命名規約に反した書き方、Copy型に対する`clone`、書式文字列へ渡す型名リテラル)。この4件を許さないと、schemaの内容によっては利用者のビルドに警告が出る。

生成moduleの読み込みは、schema宣言と同じファイルへ置く。宣言の直前と直後のどちらでもよい。`include!`の相対パスは宣言元ファイルの位置を基準に解決する。これは`mod foo;`のファイル探索 (module の入れ子に応じて探索先が変わる) とは基準が異なり、`include!`はファイル位置基準でmoduleの入れ子に影響されない (入れ子moduleの中へ`include!`を移しても基準は変わらない。実例は`crates/graphite/tests/graph_cross_module.rs`参照)。`#[path]`属性で基準を移動させることはしない。

## 宣言の種類

追跡可能な宣言は3種類あり、生成の入口はこの3つをまとめて1回の走査で扱う。

1. **動的グラフのschema** (`dynamic_graph_schema!`)。1宣言につき生成ファイル1件。
2. **静的グラフのschema** (`static_graph_schema!`)。1宣言につき生成ファイル1件 (種別ごとの辺値 `pub struct {種別}Edge<'a>` を持つ)。
3. **静的グラフのinstance** (schema名そのものを名前にしたマクロ、例: `Org! { .. }`)。1宣言につき生成ファイル1件。`Nodes`・`Edges`・`{個体名}Ref`・`{辺名}Ref`・グラフ本体の型 `Graph` を持つ。利用者はこの`Graph`を、instance宣言と同じ名前のmoduleを介した修飾パス (`{instance名}::Graph`) で参照する。

利用者は、静的グラフのschema・instanceも`generated = "..."`と生成moduleの配線を動的グラフと同じ形で書く (`docs/static_graph.md`「2層マクロの使い方」参照)。生成器は、生成の探索を2段階で行う: パッケージ内の全ファイルを1回ずつ構文解析して集めた `static_graph_schema!` の呼び出しから静的schema名簿を作り (schema名の重複はここでエラーにする)、名簿の名前と一致する残りのマクロ呼び出しをinstanceとみなして解決する。生成器は、名簿に無い名前で始まるのに`generated = "...";`から始まる呼び出しを「schemaが見つからない」エラーにする。

instanceの生成ファイルは、instanceの値の式 (ノードの初期値・積み荷の値) を含まない。値の式は宣言元ファイルのその場展開に残るため、値だけを書き換えた編集では再生成が要らない (指紋は構造 (名前・型・値の有無・端点・積み荷の有無) だけで決まる)。

生成器は、Rustとして解析できないファイルを走査から黙って除外せず違反にする (`generate`/`generate --check` を止める)。対象外にするのは`target`・`generated`・`ui`の各ディレクトリだけである。

この検査には及ばない範囲が3つある。(1) `quote!`・`quote_spanned!`・`parse_quote!`の入力の中に書かれた、instance宣言に似た形のトークン列は対象外にする (これらはトークン列を組み立てて返すマクロであり、その入力は文位置に直接書くinstance宣言ではなくデータであるため)。(2) 名簿に無い名前で始まり、かつ`generated = "...";`から始まらない呼び出しは対象外にする。schema名の綴り誤りと`generated`の書き忘れが同時に起きた宣言は、生成器のこの検査では検出できない (この場合は生成された`macro_rules!`が見つからず、通常の`cargo build`がコンパイルエラーとして検出する)。(3) 走査は各パッケージ直下の`src`・`tests`だけを対象にし、パッケージが個別に持てる`examples/`・`benches/`ディレクトリ (Cargoの規約による、`cargo run --example`・`cargo bench`向けのディレクトリ) は走査しない。

`generate`/`generate --check`はどちらも、読んだ宣言の内訳と件数を1行で表示する: `dynamic schema N件、static schema N件、static instance N件、生成 M件 (解析したファイル K件)`。

## 生成コマンド

生成の入口は2つある。外部crate向けの`cargo graphite generate`と、Graphite自身の開発用の`cargo xtask generate`である。

どちらの入口も`generate`と`generate --check`の2つの動作を持ち、動作の意味は入口によらず同じである。`generate`は全宣言を読み、期待する生成ファイルを更新する。`generate --check`はファイルを書き換えず、生成ファイルの不足と差分をエラーにする。

**外部crateから使う場合。** 一度だけ次のコマンドで生成器を入れる。

```powershell
cargo install --path <Graphiteのclone先>/crates/graphite-cli
```

以後は、生成したいパッケージのディレクトリ (`Cargo.toml`があるディレクトリ) で実行する。

```powershell
cargo graphite generate
cargo graphite generate --check
```

**Graphite自身の開発の場合。** 作業ディレクトリをリポジトリルート (このリポジトリの最上位ディレクトリ) にして実行する。

```powershell
cargo xtask generate
cargo xtask generate --check
```

2つの入口で違うのは走査開始点だけである。どちらもパッケージ直下の`src`と`tests`を走査し、宣言元の綴りをそのパッケージルートからの相対で記録する。違うのは対象にするパッケージの数だけで、`cargo graphite`は実行した場所のパッケージ1つを、`cargo xtask`は`crates/*`と`examples/*`の全パッケージを順に処理する。schema宣言の抽出・生成計画・書き込み・差分検査は`graphite-cli`が両方へ提供する同じ処理である (クレートの分け方は `docs/development/crate_architecture.md` 参照)。

基準ディレクトリを両方ともパッケージルートに揃えているため、同じパッケージをどちらの入口で生成しても本文はバイト単位で一致する。この一致は`xtask/tests/entry_point_agreement.rs`が`cargo test`の中で実測する。

外部crateからの経路が壊れていないことは`cargo xtask check-external`が実走で検査する。検査対象は`verification/external-crate`であり、ワークスペースの外に置いてある。

## 生成先

- 通常crateと例の `src/*.rs` にある宣言は、宣言元と同じ `src/generated/` に生成する。
- `crates/graphite/tests/*.rs` にある統合テストの宣言は、`crates/graphite/tests/generated/` に生成する。統合テストは1ファイルが1つのcrate rootであり、`src/` を持たないためである。`tests/generated/` 直下のファイルはどのテストのcrate rootにもならないので、`tests/` を走査する `cargo test` が生成物を単体のテストcrateとして拾うことはない。
- 宣言の `generated` は宣言元Rustファイルからの相対パス `generated/<名前>.rs` とする。絶対パスと `..` は許可しない。
- 生成ファイルはgitで管理する。ファイル先頭に手編集禁止、元DSLのパッケージ相対パスと行、再生成コマンドを記録する。

## 再生成の案内

生成ファイルの先頭に書く再生成コマンドの案内と、指紋が合わないときのコンパイルエラーの文言は、どの入口から生成しても同じにする。入口ごとに書き分けると、`cargo graphite generate`が書いたファイルを`cargo xtask generate --check`が古いと判定し、その逆も起きる。

このため案内は`cargo graphite generate`を主に書き、Graphite自身の開発では`cargo xtask generate`と括弧で添える形に固定する。

## 陳腐化の検出

`graphite-codegen`は検証済みschemaから決定的な指紋を作り、生成moduleへ埋め込む。指紋の実体はFNV-1a (64bit) を4種の初期値でそれぞれ計算した`[u64; 4]`であり、暗号強度のハッシュではなく偶発的な取り違え (schemaの位置移動・生成器の変更を含む) を検出するための目印である。`dynamic_graph_schema!`も同じ純粋層から指紋を得てconst評価で比較する。schemaの意味を変更して生成し忘れた場合、通常の`cargo build`がコンパイルエラーになるため、古い公開APIが黙って残らない。

`cargo xtask generate --check`は生成本文全体をバイト単位で比較する。schemaの位置移動、生成器の変更、コメントに記録する元DSL位置の変化も検出する。

宣言元ファイルの綴りは指紋の材料に入れない。生成物の doc へ書く宣言元への参照 (`docs/desugaring_reference.md` §26.6) とファイル先頭の案内コメントは、どちらも宣言元ファイルの綴りを含むが、指紋を計算する`dynamic_graph_schema!`は自分が書かれたファイルのパッケージ相対の綴りを知らない。綴りを指紋へ効かせると、生成ファイルの指紋とマクロが計算する指紋が一致しなくなる。宣言元ファイルを移動したときの綴りのずれは`generate --check`のバイト比較が検出する。

## 関連文書

生成物のどれが公開でどれが非公開かは `docs/desugaring_reference.md` §22 が一覧で定める。`graph!`の名前付きラッパーだけを生成ファイルへ事前生成しない理由は同 §26.5 にある。クレートごとの責務は `docs/development/crate_architecture.md` が定める。
