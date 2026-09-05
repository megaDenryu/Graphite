//! syn の項目ごとに散らばった可視性・属性・名前の取り出しを1つの形へ揃える。
//!
//! doc コメントを要求される項目だけを `Some` で返し、それ以外は `None` を返す。

use syn::{Attribute, ImplItem, Item, TraitItem, Visibility};

// 可視性・属性・名前を持ち、doc コメントを要求される項目だけを返す。
// `use`・`impl`・`extern crate`・マクロ呼び出しは要求されないため `None` を返す。
pub(super) fn item_facts(item: &Item) -> Option<(&Visibility, &[Attribute], String)> {
    match item {
        Item::Const(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::Enum(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::Fn(i) => Some((&i.vis, &i.attrs, i.sig.ident.to_string())),
        Item::Mod(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::Static(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::Struct(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::Trait(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::TraitAlias(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::Type(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        Item::Union(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        _ => None,
    }
}

pub(super) fn trait_item_facts(item: &TraitItem) -> Option<(&[Attribute], String)> {
    match item {
        TraitItem::Const(i) => Some((&i.attrs, i.ident.to_string())),
        TraitItem::Fn(i) => Some((&i.attrs, i.sig.ident.to_string())),
        TraitItem::Type(i) => Some((&i.attrs, i.ident.to_string())),
        _ => None,
    }
}

pub(super) fn impl_item_facts(item: &ImplItem) -> Option<(&Visibility, &[Attribute], String)> {
    match item {
        ImplItem::Const(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        ImplItem::Fn(i) => Some((&i.vis, &i.attrs, i.sig.ident.to_string())),
        ImplItem::Type(i) => Some((&i.vis, &i.attrs, i.ident.to_string())),
        _ => None,
    }
}
