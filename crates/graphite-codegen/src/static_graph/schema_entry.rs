// `static_graph_schema!` の展開本体 (issue #41 段階3)。schema
// (`generated = "..."` 付き) を構文解析・検証し、生成ファイルとの指紋照合・
// node型アンカーと、`macro_rules! {schema名}` (内部マクロへの転送) を並べて
// 返す。公開の `{種別}Edge` struct群は生成ファイルだけにあり、ここでは出力
// しない。

use proc_macro2::TokenStream;
use quote::quote;

use crate::fingerprint_check::{指紋照合コードを生成する, 静的schema対象文言, 静的schema指紋定数名};

use super::{parse_tracked_static_schema, schema};

pub fn parse_and_expand_static_graph_schema(input: TokenStream) -> TokenStream {
    let tracked = match parse_tracked_static_schema(input) {
        Ok(tracked) => tracked,
        Err(errors) => {
            return errors.iter().map(syn::Error::to_compile_error).collect();
        }
    };
    let schema名 = tracked.schema_name();
    let 定数名 = 静的schema指紋定数名();
    let 指紋照合 = 指紋照合コードを生成する(
        quote! { #schema名::#定数名 },
        tracked.fingerprint(),
        &静的schema対象文言(schema名),
        tracked.generated_path().span(),
    );
    let 型アンカー = schema::codegen::骨組みを生成する(tracked.型入力());
    let 生スキーマトークン = tracked.schema_tokens();
    quote! {
        #指紋照合
        #型アンカー
        macro_rules! #schema名 {
            ($($t:tt)*) => {
                ::graphite::__static_graph_impl! {
                    #生スキーマトークン
                    instance { $($t)* }
                }
            };
        }
    }
}
