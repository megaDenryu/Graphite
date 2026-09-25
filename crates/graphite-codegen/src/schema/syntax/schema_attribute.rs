//! `schema` の前に書く外側属性を読み、DSL が受理する唯一の属性 `#[derive(Clone)]` の有無にする。
//!
//! 属性は schema のヘッダの一部であり、ヘッダの構文エラーは回復しない
//! (`schema_declaration.rs` の `parse_recovering` の方針)。受理しない属性を
//! 黙って読み飛ばすと、利用者は書いた属性が効いたと誤解するため、属性は
//! 1つ残らずここで受理するか拒否するかを決める。

use syn::punctuated::Punctuated;
use syn::{Attribute, Path, Token};

// schema 宣言の前に `#[derive(Clone)]` が書かれたかどうか
// (`docs/schema_v4.md` §3.1.3)。
pub enum CloneDerive {
    NotDeclared,
    Declared,
}

const ACCEPTED_ATTRIBUTE: &str = "schema 宣言の前に書ける属性は `#[derive(Clone)]` だけです";

impl CloneDerive {
    // `schema` の前に並んだ外側属性の列を読む。受理しない属性と、
    // `#[derive(Clone)]` の重複は構文エラーにする。
    pub fn from_attributes(attributes: Vec<Attribute>) -> syn::Result<Self> {
        let mut clone_derive = Self::NotDeclared;
        for attribute in attributes {
            validate_derive_clone(&attribute)?;
            if let Self::Declared = clone_derive {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "`#[derive(Clone)]` が2回書かれています。1回だけ書いてください",
                ));
            }
            clone_derive = Self::Declared;
        }
        Ok(clone_derive)
    }
}

// 1つの属性がちょうど `#[derive(Clone)]` であることを確かめる。
// `Clone` 以外の導出 (`Debug` 等) を受理しないのは、生成器がその導出を
// 生成物のどこへ付けるかという意味を定めていないためである。
fn validate_derive_clone(attribute: &Attribute) -> syn::Result<()> {
    if !attribute.path().is_ident("derive") {
        return Err(syn::Error::new_spanned(attribute, ACCEPTED_ATTRIBUTE));
    }
    let derived = attribute.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
    let mut paths = derived.iter();
    match (paths.next(), paths.next()) {
        (Some(path), None) if path.is_ident("Clone") => Ok(()),
        _ => Err(syn::Error::new_spanned(attribute, ACCEPTED_ATTRIBUTE)),
    }
}
