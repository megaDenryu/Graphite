//! graphitets-by-hand 用のマクロ (issue #24 段階2)。公開するのは
//! `静的グラフ型!` 1個だけ。schemaを構文解析・検証し、(1) schemaだけから
//! 決まる生成物 (辺値struct群・node型アンカー、`schema::codegen`) と
//! (2) schemaの生トークンを本体に焼き込んだ `macro_rules! {schema名}` を
//! 同じ展開の中で並べて出力する。(1) を macro_rules! の外へ出すことで、
//! schema トークンが macro_rules! 本体に埋もれた不活性なトークン列ではなく
//! rust-analyzer が解釈できる実際のRustアイテムになる (段階2コミット時点の
//! 変更、以前は instance毎に `静的グラフ内部!` 側で生成していた)。
//! 生成されるmacro_rulesがschema名そのものをマクロ名にする (利用側は
//! `<schema名>! { graph <名前>; .. }` と書く) ので、利用側が別途schema名を
//! 書く必要はない。
//!
//! macro_rules!が実際に個体宣言を受け取ると、schemaの生トークンと個体宣言の
//! 生トークンを束ねて `#[doc(hidden)]` の内部proc macro `静的グラフ内部!`
//! (`internal` module) へ転送する。schemaとinstanceを1回の展開で同時に見る
//! ことで、多重度・対一意といった「両方が揃わないと検査できない」制約を
//! 迂回機構 (位置キーtrait+const assert) なしで、通常のcompile_error!として
//! 検出できる。
//!
//! 生成されたmacro_rulesは通常のmacro_rules!と同じテキスト順の制約を持つ:
//! `静的グラフ型! { schema <名前> { .. } }` より後ろの行でしか
//! `<名前>! { .. }` を呼べない (詳細はREADME参照)。

mod internal;
mod literal;
mod schema;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn 静的グラフ型(入力: TokenStream) -> TokenStream {
    let 生トークン: proc_macro2::TokenStream = 入力.clone().into();
    let 解析済み = parse_macro_input!(入力 as schema::input::静的グラフ型入力);
    if let Err(エラー) = 解析済み.検証する() {
        return エラー.to_compile_error().into();
    }

    let 骨組み = schema::codegen::骨組みを生成する(&解析済み);
    let macro_rules名 = &解析済み.schema名;
    quote! {
        #骨組み
        macro_rules! #macro_rules名 {
            ($($t:tt)*) => {
                ::graphitets_by_hand_macros::静的グラフ内部! {
                    #生トークン
                    instance { $($t)* }
                }
            };
        }
    }
    .into()
}

// 利用者が直接書くことを想定しない内部マクロ。`静的グラフ型!` が生成する
// macro_rules!からだけ呼ばれる (詳細はmodule doc参照)。
#[doc(hidden)]
#[proc_macro]
pub fn 静的グラフ内部(入力: TokenStream) -> TokenStream {
    let 入力 = parse_macro_input!(入力 as internal::静的グラフ内部入力);
    if let Err(エラー) = 入力.検証する() {
        return エラー.to_compile_error().into();
    }
    入力.コードを生成する().into()
}
