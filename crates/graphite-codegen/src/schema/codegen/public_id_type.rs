//! 意味モデルの公開ID型を生成コードの型位置へ写し、既定生成ID型の定義を作る。

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;
use crate::schema::semantic::公開ID型;

// 意味モデルの公開ID型を、生成コードの型位置へそのまま置けるトークンとして扱う。
//
// `self::` → `super::` の読み替えは意味モデルの構築時に1回だけ済ませてある。
// ここは確定済みの名前を書き出すだけで、意味の判断はしない。
#[derive(Clone, Copy)]
pub(crate) struct PublicIdType<'a>(&'a 公開ID型);

impl<'a> PublicIdType<'a> {
    pub(crate) fn new(id_ty: &'a 公開ID型) -> Self {
        Self(id_ty)
    }

    // スキーマが生成するID型ならその型名。明示ID型なら `None`。
    pub(crate) fn generated_ident(self) -> Option<&'a Ident> {
        self.0.スキーマが生成する型名()
    }

    pub(crate) fn is_debug_printable(self) -> bool {
        self.0.デバッグ表示に使えるか()
    }
}

impl ToTokens for PublicIdType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            公開ID型::スキーマが生成するID型 { 型名 } => 型名.to_tokens(tokens),
            公開ID型::利用者が宣言した既存のID型 {
                生成module内から見たパス,
            } => 生成module内から見たパス.to_tokens(tokens),
        }
    }
}

// 明示ID型がないノード・エッジのスキーマ内限定の型付き文字列IDを生成する。
pub(crate) fn gen_default_id_types(
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
) -> Vec<TokenStream> {
    let ノードの公開id型 = nodes.iter().map(|node| {
        (
            node.id_ty,
            format!(" `{}` ノードの公開ID。", node.type_ident),
            &node.宣言元への参照,
        )
    });
    let 辺の公開id型 = edges.iter().map(|edge| {
        (
            edge.id_ty,
            format!(" `{}` 辺の公開ID。", edge.kind),
            &edge.宣言元への参照,
        )
    });
    ノードの公開id型
        .chain(辺の公開id型)
        .filter_map(|(id_ty, doc, 宣言元への参照)| {
            let ident = id_ty.generated_ident()?;
            Some(quote! {
                #[doc = #doc]
                #宣言元への参照
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #ident(pub String);
            })
        })
        .collect()
}
