//! 違反列挙型と、その表示を意味モデルの違反定義の順に組み立てる。

pub(crate) mod each_violation;
pub(crate) mod edge_duplicate_key;
pub(crate) mod node_duplicate;
pub(crate) mod unique_pair_violation;
pub(crate) mod unknown_endpoint;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::declaration_doc::宣言元への参照;
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;
use crate::schema::semantic::違反定義;
use each_violation::gen_each_violation_case;
use edge_duplicate_key::gen_edge_duplicate_key_case;
use node_duplicate::gen_node_duplicate_key_case;
use unique_pair_violation::gen_unique_pair_violation_case;
use unknown_endpoint::{
    gen_unknown_endpoint_case, gen_unknown_source_case, gen_unknown_target_case,
};

// 違反 enum を生成する。
//
// - ノード重複 (`Duplicate{Node}`) は v3 から維持。
// - 辺キー重複 (`{Kind}DuplicateKey`) は v4 で新規追加 (辺も第一級キーを
//   持つため)。
// - 未知の端点参照: 有向は `{Kind}UnknownSource`/`{Kind}UnknownTarget`
//   (どの辺がどちらの端点で未知キーを参照したかを型付きで持つ)、無向は
//   位置の区別が無いため `{Kind}UnknownEndpoint` 1種類。
// - `each` 制約違反 (`{Kind}{Role}EachViolation`) は解決された側
//   (出次数/入次数) に応じて `source` または `target` を持つ。
// - `unique pair` 違反 (`{Kind}UniquePairViolation`) は有向なら
//   `source`/`target`、無向なら順序の意味が無いため `a`/`b`。
pub(crate) fn gen_violation_enum(
    violation_ident: &Ident,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
    違反定義の列: &[違反定義],
    スキーマ宣言元への参照: &宣言元への参照,
) -> TokenStream {
    let mut variants: Vec<TokenStream> = Vec::new();
    let mut display_arms: Vec<TokenStream> = Vec::new();
    for 違反 in 違反定義の列 {
        let (variant, display_arm) = gen_violation_case(violation_ident, nodes, edges, 違反);
        variants.push(variant);
        display_arms.push(display_arm);
    }

    quote! {
        /// 凍結時の図式適合検査が見つけた違反。
        #スキーマ宣言元への参照
        #[allow(clippy::enum_variant_names)]
        #[derive(Clone, PartialEq, Eq)]
        pub enum #violation_ident {
            #(#variants,)*
        }

        impl std::fmt::Display for #violation_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms,)*
                }
            }
        }

        impl std::fmt::Debug for #violation_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }

        impl std::error::Error for #violation_ident {}
    }
}

// 違反定義1件を、違反列挙型の variant とその `Display` の分岐へ写す。
pub(crate) fn gen_violation_case(
    violation_ident: &Ident,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
    違反: &違反定義,
) -> (TokenStream, TokenStream) {
    match 違反 {
        違反定義::ノードのキーが重複した { ノード } => {
            gen_node_duplicate_key_case(violation_ident, &nodes[ノード.添字()])
        }
        違反定義::辺のキーが重複した { 辺 } => {
            gen_edge_duplicate_key_case(violation_ident, &edges[辺.添字()])
        }
        違反定義::未知の始点を参照した { 辺 } => {
            gen_unknown_source_case(violation_ident, &edges[辺.添字()])
        }
        違反定義::未知の終点を参照した { 辺 } => {
            gen_unknown_target_case(violation_ident, &edges[辺.添字()])
        }
        違反定義::未知の端点を参照した { 辺 } => {
            gen_unknown_endpoint_case(violation_ident, &edges[辺.添字()])
        }
        違反定義::多重度に反した {
            辺, 側, 役割名
        } => gen_each_violation_case(violation_ident, &edges[辺.添字()], *側, 役割名),
        違反定義::端点対が重複した { 辺 } => {
            gen_unique_pair_violation_case(violation_ident, &edges[辺.添字()])
        }
    }
}
