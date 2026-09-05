//! 属性と可視性の判定。2つの検査 (内部領域の撤去・生成コードの網羅) が共有する。

use syn::{AttrStyle, Attribute, Meta, Visibility};

// `///` と `#[doc = "..."]` がこれに当たる。`//!` は Inner なので外れ、
// `#[doc(hidden)]` は Meta::List なので外れる。
pub(super) fn is_outer_doc_comment(attribute: &Attribute) -> bool {
    matches!(attribute.style, AttrStyle::Outer)
        && attribute.path().is_ident("doc")
        && matches!(attribute.meta, Meta::NameValue(_))
}

pub(super) fn has_doc_comment(attributes: &[Attribute]) -> bool {
    attributes.iter().any(is_outer_doc_comment)
}

// `#[doc(hidden)]` が付いた項目は利用者の rustdoc に出ないため検査の対象外である。
pub(super) fn is_doc_hidden(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("doc")
            && match &attribute.meta {
                Meta::List(list) => list.tokens.to_string().trim() == "hidden",
                _ => false,
            }
    })
}

// 手続き型マクロの入口は `graphite` から re-export され利用者の rustdoc に出る
// ため、内部領域の中でも公開面として扱う。
pub(super) fn is_procedural_macro_entry(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        ["proc_macro", "proc_macro_derive", "proc_macro_attribute"]
            .iter()
            .any(|name| attribute.path().is_ident(name))
    })
}

pub(super) fn is_public(visibility: &Visibility) -> bool {
    matches!(visibility, Visibility::Public(_))
}
