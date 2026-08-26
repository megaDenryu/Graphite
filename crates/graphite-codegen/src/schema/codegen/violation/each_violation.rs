//! 多重度制約に反したことを表す違反 variant とその表示を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::each_violation_ident;
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::EachSide;

/// `each` 制約違反 (`{Kind}{Role}EachViolation`)。制約が指す側に応じて
/// `source` (出次数) または `target` (入次数) を持つ。
pub(crate) fn gen_each_violation_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
    side: EachSide,
    role: &Ident,
) -> (TokenStream, TokenStream) {
    let kind_str = edge.kind.to_string();
    let spec = edge
        .定義
        .側の役割の多重度制約(side)
        .expect("違反定義に載る多重度制約は同じ辺定義が保持している")
        .指定された範囲();
    let expected_str = match spec.max() {
        Some(max) if spec.min() == max => format!("ちょうど{}", spec.min()),
        Some(max) => format!("{}..{}", spec.min(), max),
        None => format!("{}..*", spec.min()),
    };
    let v = each_violation_ident(edge.kind, role);
    match side {
        EachSide::Source => {
            let from_id = &edge.from_node.id_ty;
            let from_type_str = edge.from_node.type_ident.to_string();
            let variant = quote! {
                /// このエッジ種別の `each` 制約違反 (出次数)。
                #v { source: #from_id, count: usize }
            };
            let display_arm = if edge.from_node.id_ty.is_debug_printable() {
                quote! {
                    #violation_ident::#v { source, count } => write!(
                        f,
                        "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                        #kind_str, #from_type_str, source, #expected_str, count
                    )
                }
            } else {
                quote! {
                    #violation_ident::#v { count, .. } => write!(
                        f,
                        "多重度制約違反: 辺 `{}` は {} の出次数 {} を期待しますが実際は {} 本です",
                        #kind_str, #from_type_str, #expected_str, count
                    )
                }
            };
            (variant, display_arm)
        }
        EachSide::Target => {
            let to_id = &edge.to_node.id_ty;
            let to_type_str = edge.to_node.type_ident.to_string();
            let variant = quote! {
                /// このエッジ種別の `each` 制約違反 (入次数)。
                #v { target: #to_id, count: usize }
            };
            let display_arm = if edge.to_node.id_ty.is_debug_printable() {
                quote! {
                    #violation_ident::#v { target, count } => write!(
                        f,
                        "多重度制約違反: 辺 `{}` は {} {:?} について入次数 {} を期待しますが実際は {} 本です",
                        #kind_str, #to_type_str, target, #expected_str, count
                    )
                }
            } else {
                quote! {
                    #violation_ident::#v { count, .. } => write!(
                        f,
                        "多重度制約違反: 辺 `{}` は {} の入次数 {} を期待しますが実際は {} 本です",
                        #kind_str, #to_type_str, #expected_str, count
                    )
                }
            };
            (variant, display_arm)
        }
    }
}
