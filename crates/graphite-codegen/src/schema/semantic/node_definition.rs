//! ノード宣言を意味として確定した定義と、その定義を指す添字ハンドルを持つ。

use proc_macro2::Ident;

use super::public_id_type::公開ID型;
use crate::naming::generated_id_ident;
use crate::schema::syntax::NodeDecl;

/// スキーマ定義が持つノード定義の列の中の1件を指すハンドル。
///
/// 辺定義が端点のノードを指すのに使う。生の `usize` を使うと始点と終点を
/// 取り違えても型が通るため newtype にする。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ノード定義番号(usize);

impl ノード定義番号 {
    pub(super) fn 添字から作る(添字: usize) -> Self {
        Self(添字)
    }

    /// ノード定義の列を引くための添字へ戻す。
    ///
    /// 注意: 生の添字へ戻してよいのは、ノード定義の列 (またはそれと同じ順で
    /// 作った列) を引く場所だけである。別の列の添字として使うと取り違えになる。
    pub fn 添字(self) -> usize {
        self.0
    }
}

/// ノード種別1つ分の意味。ノード値の型は利用者が schema の外で宣言するため、
/// ここが持つのはその型名への参照である。
pub struct ノード定義 {
    ノード値型名: Ident,
    公開id型: 公開ID型,
}

impl ノード定義 {
    pub(super) fn 宣言から作る(宣言: &NodeDecl) -> Self {
        Self {
            ノード値型名: 宣言.name.clone(),
            公開id型: 公開ID型::宣言から作る(
                generated_id_ident(&宣言.name),
                宣言.id_ty.clone(),
            ),
        }
    }

    pub fn ノード値型名(&self) -> &Ident {
        &self.ノード値型名
    }

    pub fn 公開id型(&self) -> &公開ID型 {
        &self.公開id型
    }
}
