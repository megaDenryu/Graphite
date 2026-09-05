//! ノード種別1つ分について、生成コードが使う識別子一式をまとめて持つ。

use syn::Ident;

use crate::naming::{
    accessor_ident, duplicate_node_key_variant_ident, internal_position_ident,
    named_position_ident, node_storage_ident, reference_ident,
};
use crate::schema::codegen::declaration_doc::宣言元への参照;
use crate::schema::codegen::public_id_type::PublicIdType;
use crate::schema::semantic::ノード定義;

// ノード種別1つ分の、生成コードで使う識別子一式。
//
// `宣言元への参照` だけは生成名ではないが、この種別の生成物すべてが同じ参照を doc へ
// 書くため、識別子と同じ場所で持つ。
pub(crate) struct NodeInfo<'a> {
    pub(crate) type_ident: Ident, // ノード値の型名 (`Person`)。利用者の宣言型を指す
    pub(crate) 宣言元への参照: 宣言元への参照, // 生成物の doc へ足す `node` 宣言元への参照
    pub(crate) id_ty: PublicIdType<'a>, // 既定生成するID、または `(id: 型パス)` 指定の既存型
    pub(crate) field_ident: Ident, // 内部ストレージのフィールド名 (`__graphite_node_person`)
    pub(crate) accessor_ident: Ident, // builder のノード追加メソッド名 (`person`)
}

impl<'a> NodeInfo<'a> {
    pub(crate) fn new(定義: &'a ノード定義, 宣言元への参照: 宣言元への参照) -> Self {
        let 型名 = 定義.ノード値型名();
        NodeInfo {
            type_ident: 型名.clone(),
            宣言元への参照,
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
