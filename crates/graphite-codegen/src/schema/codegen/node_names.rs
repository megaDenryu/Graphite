//! ノード種別1つ分について、生成コードが使う識別子一式をまとめて持つ。

use syn::Ident;

use crate::naming::{
    accessor_ident, duplicate_node_key_variant_ident, internal_position_ident,
    named_position_ident, node_storage_ident, reference_ident,
};
use crate::schema::codegen::public_id_type::PublicIdType;
use crate::schema::semantic::ノード定義;

pub(crate) struct NodeInfo<'a> {
    /// ノード値の型名 (`Person`)。ユーザー宣言型への参照。
    pub(crate) type_ident: Ident,
    /// スキーマ内限定で既定生成するID、または `(id: 型パス)` で指定された既存型。
    pub(crate) id_ty: PublicIdType<'a>,
    /// 内部ストレージのフィールド名 (`__graphite_node_person`)。
    pub(crate) field_ident: Ident,
    /// builder のノード追加メソッド名 = 単数形 snake_case (`person`)。
    pub(crate) accessor_ident: Ident,
}

impl<'a> NodeInfo<'a> {
    pub(crate) fn new(定義: &'a ノード定義) -> Self {
        let 型名 = 定義.ノード値型名();
        NodeInfo {
            type_ident: 型名.clone(),
            id_ty: PublicIdType::new(定義.公開id型()),
            field_ident: node_storage_ident(型名),
            accessor_ident: accessor_ident(型名),
        }
    }

    pub(crate) fn dup_variant(&self) -> Ident {
        duplicate_node_key_variant_ident(&self.type_ident)
    }

    pub(crate) fn internal_position_ident(&self) -> Ident {
        internal_position_ident(&self.type_ident)
    }

    pub(crate) fn reference_ident(&self) -> Ident {
        reference_ident(&self.type_ident)
    }

    pub(crate) fn named_position_ident(&self) -> Ident {
        named_position_ident(&self.type_ident)
    }
}
