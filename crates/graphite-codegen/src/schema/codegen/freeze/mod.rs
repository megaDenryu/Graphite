//! 構築器から完成済みグラフを組み立てる凍結処理の全体を並べる。

pub(crate) mod directed_edge;
pub(crate) mod each_check;
pub(crate) mod node_table;
pub(crate) mod role_index_finalize;
pub(crate) mod undirected_edge;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::{construction_stamp_field_ident, edge_storage_ident, pair_index_field_ident};
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;
use crate::schema::semantic::{EachSide, 辺の向き};
use directed_edge::gen_directed_edge_freeze_block;
use node_table::gen_node_table_freeze_block;
use role_index_finalize::finalize_role_index;
use undirected_edge::gen_undirected_edge_freeze_block;

pub(crate) fn gen_freeze_body(
    schema_name: &Ident,
    violation_ident: &Ident,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_table_builds = nodes
        .iter()
        .map(|n| gen_node_table_freeze_block(violation_ident, n));

    let edge_blocks = edges.iter().map(|e| match e.shape() {
        辺の向き::有向 { 始点, 終点 } => {
            gen_directed_edge_freeze_block(violation_ident, e, 始点.役割名(), 終点.役割名())
        }
        辺の向き::無向 { .. } => gen_undirected_edge_freeze_block(violation_ident, e),
    });

    // 制約違反をすべて収集し終えてから、役割索引を公開クエリ向けの
    // 多重度別表現へ確定する。違反のある未完成索引を ExactlyOneRoleIndex/
    // OptionalRoleIndex 表現へ変換すると内部アサートになり、利用者向け
    // Violation を返せない。
    let edge_index_finalizers = edges.iter().flat_map(|edge| match edge.shape() {
        辺の向き::有向 { .. } => vec![
            finalize_role_index(
                edge,
                EachSide::Source,
                &edge.index_field_ident,
                &edge.from_node.field_ident,
                &edge.from_node.internal_position_ident(),
            ),
            finalize_role_index(
                edge,
                EachSide::Target,
                &edge.to_index_field_ident,
                &edge.to_node.field_ident,
                &edge.to_node.internal_position_ident(),
            ),
        ],
        辺の向き::無向 { .. } => vec![finalize_role_index(
            edge,
            EachSide::Source,
            &edge.index_field_ident,
            &edge.from_node.field_ident,
            &edge.from_node.internal_position_ident(),
        )],
    });

    let node_field_names = nodes.iter().map(|n| &n.field_ident);
    let edge_field_inits = edges.iter().map(|e| {
        let field = &e.accessor_ident;
        let storage = edge_storage_ident(field);
        quote! { #field: #storage }
    });
    // 有向辺は位置0索引 (`{accessor}_from_index`) と位置1索引
    // (`{accessor}_to_index`) の両方をフィールドとして持つ。無向辺は
    // `index_field_ident` (対称な単一索引) のみ (`gen_schema_struct` 参照)。
    let edge_index_names: Vec<Ident> = edges
        .iter()
        .flat_map(|e| {
            let pair = pair_index_field_ident(e.kind);
            match e.shape() {
                辺の向き::有向 { .. } => {
                    vec![
                        e.index_field_ident.clone(),
                        e.to_index_field_ident.clone(),
                        pair,
                    ]
                }
                辺の向き::無向 { .. } => vec![e.index_field_ident.clone(), pair],
            }
        })
        .collect();

    let stamp_field = construction_stamp_field_ident(schema_name.span());

    quote! {
        /// 検証ロジックの実体。最初の1件で打ち切らず全違反を `Vec` に
        /// 集めて返す。`freeze()` (単一エラー版) はこちらに委譲し先頭の1件を
        /// 取り出すだけの薄いラッパーにすることで、検証ロジックが二重実装に
        /// ならないようにしている。
        fn freeze_collecting(self) -> Result<#schema_name, Vec<#violation_ident>> {
            let mut __violations: Vec<#violation_ident> = Vec::new();
            let #stamp_field = self.#stamp_field;

            #(#node_table_builds)*
            #(#edge_blocks)*

            if !__violations.is_empty() {
                return Err(__violations);
            }

            #(#edge_index_finalizers)*

            Ok(#schema_name {
                #(#node_field_names,)*
                #(#edge_field_inits,)*
                #(#edge_index_names,)*
                #stamp_field,
            })
        }

        /// 最初の1件の違反で `Err` になる版。実装は
        /// `freeze_collecting` に委譲する。
        fn freeze(self) -> Result<#schema_name, #violation_ident> {
            self.freeze_collecting().map_err(|mut violations| violations.remove(0))
        }
    }
}
