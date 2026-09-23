//! 全個体がコンパイル時に確定する静的グラフの構文解析・検証・コード生成
//! (issue #24 段階2、`examples/graphitets-by-hand/macros/` からの移設)。
//! 公開する入口は `static_graph_schema!` (`graphite_macros::static_graph_schema`) 1個
//! だけ。schemaを構文解析・検証し、(1) schemaだけから決まる生成物
//! (辺値struct群・node型アンカー、`schema::codegen`) と (2) schemaの生
//! トークンを本体に焼き込んだ `macro_rules! {schema名}` を同じ展開の中で
//! 並べて出力する。(1) を macro_rules! の外へ出すことで、schema トークンが
//! macro_rules! 本体に埋もれた不活性なトークン列ではなく rust-analyzer が
//! 解釈できる実際のRustアイテムになる。生成されるmacro_rulesがschema名
//! そのものをマクロ名にする (利用側は `<schema名>! { graph <名前>; .. }` と
//! 書く) ので、利用側が別途schema名を書く必要はない。
//!
//! macro_rules!が実際に個体宣言を受け取ると、schemaの生トークンと個体宣言の
//! 生トークンを束ねて `#[doc(hidden)]` の内部proc macro `__static_graph_impl!`
//! (`internal` module、`graphite_macros::__static_graph_impl` から呼ばれる)
//! へ転送する。schemaとinstanceを1回の展開で同時に見ることで、多重度・
//! 対一意といった「両方が揃わないと検査できない」制約を迂回機構なしで、
//! 通常のcompile_error!として検出できる。
//!
//! 生成されたmacro_rulesは通常のmacro_rules!と同じテキスト順の制約を持つ:
//! `static_graph_schema! { schema <名前> { .. } }` より後ろの行でしか
//! `<名前>! { .. }` を呼べない (詳細は `docs/static_graph.md` を参照)。
//!
//! この機構はインライン展開のまま完結する (`dynamic_graph_schema!` のようなファイル
//! 生成トラッキングには参加しない)。ファイルI/Oを持たないため
//! `graphite-cli`/`cargo xtask generate` の対象にもならない (`flow!` と
//! 同じ位置づけ)。

mod file;
// `錨を組み立てる`・`個体供給関数を組み立てる`・`積み荷供給関数を組み立てる`
// は段階3でその場展開へ配線するまで、各ファイル内の単体試験だけが呼ぶ
// (issue #41 段階2、`inline::mod` のdoc参照)。
#[allow(dead_code)]
mod inline;
mod internal;
mod literal;
mod naming;
mod schema;
mod semantic;
mod tracked;
mod trace;

use proc_macro2::TokenStream;
use quote::quote;

pub use tracked::{
    parse_tracked_static_instance, parse_tracked_static_schema, TrackedStaticInstance,
    TrackedStaticSchema,
};

// `static_graph_schema!` の展開本体。schemaを構文解析・検証し、schemaだけから
// 決まる生成物と `macro_rules! {schema名}` (内部マクロへの転送) を並べて
// 返す。
pub fn parse_and_expand_static_graph_schema(input: TokenStream) -> TokenStream {
    let 生トークン = input.clone();
    let 解析済み = match syn::parse2::<schema::input::静的グラフ型入力>(input) {
        Ok(解析済み) => 解析済み,
        Err(エラー) => return エラー.to_compile_error(),
    };
    if let Err(エラー) = 解析済み.検証する() {
        return エラー.to_compile_error();
    }

    let 骨組み = schema::codegen::骨組みを生成する(&解析済み);
    let macro_rules名 = &解析済み.schema名;
    quote! {
        #骨組み
        macro_rules! #macro_rules名 {
            ($($t:tt)*) => {
                ::graphite::__static_graph_impl! {
                    #生トークン
                    instance { $($t)* }
                }
            };
        }
    }
}

// `__static_graph_impl!` の展開本体。`static_graph_schema!` がmacro_rules!転送で
// 焼き込んだschemaの生トークンと、利用側が `<schema名>! { .. }` で書いた
// instanceの生トークンを1回の展開で同時に受け取り、両者の相互検証と
// 具象コード生成を行う。
pub fn expand_static_graph_internal(input: TokenStream) -> TokenStream {
    let 入力 = match syn::parse2::<internal::静的グラフ内部入力>(input) {
        Ok(入力) => 入力,
        Err(エラー) => return エラー.to_compile_error(),
    };
    let 検証済み = match 入力.検証する() {
        Ok(検証済み) => 検証済み,
        Err(エラー) => return エラー.to_compile_error(),
    };
    検証済み.コードを生成する()
}
