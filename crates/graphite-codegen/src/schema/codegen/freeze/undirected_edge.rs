//! 無向辺1種別分の凍結検査と対称な索引の構築を生成する。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。
//! このファイルは `directed_edge.rs` と同じ理由で超過する。このファイルは
//! 無向辺1種別分の凍結処理を持ち、その工程は1つの `for` ループを共有する。
//! 超過を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::{edge_storage_ident, pair_index_field_ident};
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::pair_index::gen_pair_index_map_type;

// 無向辺1種別分の凍結検査本体を生成する
// (`docs/edge_endpoints_v4_1.md` §2)。
//
// 位置0/1索引 (`{accessor}_index`) は「その位置0キーに (有向の from_index
// と同じ形で) 接続するエッジキーの一覧」だが、無向のため対称に構築する:
// 位置0・位置1のどちらにも (自己ループなら1回だけ) 積む。これにより
// - `{kind}_incident`/`{kind}_between` はどちらの位置に置かれてもこの索引から検索できる。
// - 格納順 (挿入順) は `KeyedTable` の `iter()` の走査順そのままなので、索引の
//   `push` もその順で行われ、`docs/edge_endpoints_v4_1.md` §2 の
//   「挿入順保持」がそのまま満たされる。
pub(crate) fn gen_undirected_edge_freeze_block(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let storage = edge_storage_ident(accessor);
    let record = edge.record_ident();
    let edge_position = edge.internal_position_ident();
    let node_position_type = edge.from_node.internal_position_ident();
    let index = &edge.index_field_ident;
    let node_field = &edge.from_node.field_ident;
    let dup_key = edge.duplicate_key_variant();
    let unk = edge.unknown_endpoint_variant();
    let kind = edge.kind;
    let pair_index = pair_index_field_ident(kind);
    let pair_index_type = gen_pair_index_map_type(edge);

    let (destructure_value, build_record) = match edge.payload() {
        Some(payload) => {
            let payload_role = payload.役割名();
            (
                quote! {
                    let #kind { endpoints, #payload_role } = value;
                },
                quote! {
                    #record {
                        endpoints: graphite::UnorderedPair::new(first_position, second_position),
                        #payload_role,
                    }
                },
            )
        }
        None => (
            quote! {
                let #kind { endpoints } = value;
            },
            quote! {
                #record {
                    endpoints: graphite::UnorderedPair::new(first_position, second_position),
                }
            },
        ),
    };

    // 無向辺の `unique pair` は `UnorderedPair` に同一性判定を委譲し、
    // ID型へ順序比較を要求せず (p0, p1) と (p1, p0) を同一視する。
    let (unique_pair_check, pair_insert) = if edge.unique_pair() {
        let v = edge.unique_pair_violation_variant();
        (
            quote! {
                if #pair_index.contains_key(&graphite::UnorderedPair::new(first_position, second_position)) {
                    __violations.push(#violation_ident::#v {
                        a: p0.clone(),
                        b: p1.clone(),
                    });
                }
            },
            quote! { #pair_index.insert(graphite::UnorderedPair::new(first_position, second_position), internal_edge_position); },
        )
    } else {
        (
            quote! {},
            quote! { #pair_index.entry(graphite::UnorderedPair::new(first_position, second_position)).or_default().push(internal_edge_position); },
        )
    };

    quote! {
        let mut #storage: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut #index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #pair_index: #pair_index_type = std::collections::HashMap::new();
        for (id, value) in self.#accessor {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(#violation_ident::#dup_key(id));
                continue;
            }
            #destructure_value
            let (p0, p1) = endpoints.endpoints();
            let p0 = p0.clone();
            let p1 = p1.clone();
            let first_position = #node_field.position(&p0).map(#node_position_type);
            let second_position = #node_field.position(&p1).map(#node_position_type);
            if first_position.is_none() {
                __violations.push(#violation_ident::#unk { edge: id.clone(), endpoint: p0.clone() });
            }
            if p1 != p0 && second_position.is_none() {
                __violations.push(#violation_ident::#unk { edge: id.clone(), endpoint: p1.clone() });
            }
            if let (Some(first_position), Some(second_position)) = (first_position, second_position) {
                #unique_pair_check
                let internal_edge_position = #edge_position(graphite::TablePosition::from_index(#storage.len()));
                #pair_insert
                #index.entry(first_position).or_default().push(internal_edge_position);
                if second_position != first_position {
                    #index.entry(second_position).or_default().push(internal_edge_position);
                }
                let inserted = #storage.insert(id, #build_record);
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
    }
}
