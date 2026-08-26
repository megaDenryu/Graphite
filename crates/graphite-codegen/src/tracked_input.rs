//! 追跡形式の `graph_schema!` 入力の先頭行 (`generated = "..."`) と schema 本体を切り分ける。

use proc_macro2::TokenStream;
use syn::{Ident, LitStr, Token};

pub(crate) struct TrackedInput {
    pub(crate) generated_path: LitStr,
    pub(crate) schema_tokens: TokenStream,
}

impl syn::parse::Parse for TrackedInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if key != "generated" {
            return Err(syn::Error::new_spanned(
                key,
                "追跡可能な生成先が指定されていません。最初の行に `generated = \"...\";` を書いてください",
            ));
        }
        input.parse::<Token![=]>()?;
        let generated_path = input.parse()?;
        input.parse::<Token![;]>()?;
        let schema_tokens = input.parse()?;
        Ok(Self {
            generated_path,
            schema_tokens,
        })
    }
}
