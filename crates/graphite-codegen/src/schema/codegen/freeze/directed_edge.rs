//! 有向辺1種別分の凍結検査と索引の構築を生成する。
//!
//! このファイルは1ファイル100行の原則の例外である。生成する凍結処理は
//! 「辺表の構築・重複キーの検出・両端点の実在検査・端点対の重複検査・
//! 2つの役割索引への積み込み・多重度検査」が1つの `for` ループを共有する
//! 1本の手続きであり、途中で切ると隣のファイルを読まないと意味が取れなく
//! なるため、まとめて置いている。単一の責務が自然に大きくなった例である。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::{each_violation_ident, edge_storage_ident, pair_index_field_ident};
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::freeze::each_check::gen_each_type_check;
use crate::schema::codegen::pair_index::gen_pair_index_map_type;
use crate::schema::semantic::EachSide;

// 有向辺1種別分の凍結検査本体を生成する。
//
// 凍結は次の手順で行う。
//
// 1. `Vec<(KindId, Kind)>` から `KeyedTable<KindId, Kind>` を構築 (重複キー
//    は `{Kind}DuplicateKey` 違反として記録し、その要素は捨てる)。
// 2. 生き残った各辺について端点 (位置0/1) がそれぞれのノード表に実在するか
//    検査する (`{Kind}UnknownSource`/`{Kind}UnknownTarget`)。両端点とも
//    正当な辺だけを位置0索引 (`{accessor}_from_index`) と位置1索引
//    (`{accessor}_to_index`) の両方に積む。後者は `docs/reverse_query.md`
//    により構造体フィールドとして永続化する。この索引は終点役割クエリと
//    入次数 each 検証の両方に使う。
//    `unique pair` 制約があれば、同じ (位置0, 位置1) の対が2回目に現れた
//    時点で `{Kind}UniquePairViolation` を記録する。
// 3. `each` 制約があれば、`each_side` に応じて出次数 (位置0索引) または
//    入次数 (位置1索引、手順2で作った永続化済みのものをそのまま使う) を
//    検査する。
pub(crate) fn gen_directed_edge_freeze_block(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
    from_role: &Ident,
    to_role: &Ident,
) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let storage = edge_storage_ident(accessor);
    let record = edge.record_ident();
    let edge_position = edge.internal_position_ident();
    let from_position_type = edge.from_node.internal_position_ident();
    let to_position_type = edge.to_node.internal_position_ident();
    let from_index = &edge.index_field_ident;
    let to_index = &edge.to_index_field_ident;
    let from_field = &edge.from_node.field_ident;
    let to_field = &edge.to_node.field_ident;
    let dup_key = edge.duplicate_key_variant();
    let unk_src = edge.unknown_source_variant();
    let unk_dst = edge.unknown_target_variant();
    let kind = edge.kind;
    let pair_index = pair_index_field_ident(kind);
    let pair_index_type = gen_pair_index_map_type(edge);

    let (destructure_value, build_record) = match edge.payload() {
        Some(payload) => {
            let payload_role = payload.役割名();
            (
                quote! {
                    let #kind { #from_role: from, #to_role: to, #payload_role } = value;
                },
                quote! {
                    #record { #from_role: from_position, #to_role: to_position, #payload_role }
                },
            )
        }
        None => (
            quote! {
                let #kind { #from_role: from, #to_role: to } = value;
            },
            quote! {
                #record { #from_role: from_position, #to_role: to_position }
            },
        ),
    };

    let (unique_pair_check, pair_insert) = if edge.unique_pair() {
        let v = edge.unique_pair_violation_variant();
        (
            quote! {
                if #pair_index.contains_key(&(from_position, to_position)) {
                    __violations.push(#violation_ident::#v {
                        source: from.clone(),
                        target: to.clone(),
                    });
                }
            },
            quote! { #pair_index.insert((from_position, to_position), internal_edge_position); },
        )
    } else {
        (
            quote! {},
            quote! { #pair_index.entry((from_position, to_position)).or_default().push(internal_edge_position); },
        )
    };

    let each_type_check = gen_each_type_check(edge);
    let each_checks = edge.定義.記述順の役割の多重度制約().iter().map(|constraint| {
        let spec = constraint.指定された範囲();
        let min = spec.min();
        let invalid_count = match spec.max() {
            Some(max) if min == max => quote! { count != #min },
            Some(max) => quote! { !(#min..=#max).contains(&count) },
            None => quote! { count < #min },
        };
        let v = each_violation_ident(edge.kind, constraint.役割名());
        match constraint.側() {
            EachSide::Source => quote! {
                for position in #from_field.positions() {
                    let internal_position = #from_position_type(position);
                    let key = #from_field.get_at(position).expect("列挙した内部位置はノード表に存在する").0;
                    let count = #from_index.get(&internal_position).map(Vec::len).unwrap_or(0);
                    if #invalid_count {
                        __violations.push(#violation_ident::#v { source: key.clone(), count });
                    }
                }
            },
            EachSide::Target => quote! {
                for position in #to_field.positions() {
                    let internal_position = #to_position_type(position);
                    let key = #to_field.get_at(position).expect("列挙した内部位置はノード表に存在する").0;
                    let count = #to_index.get(&internal_position).map(Vec::len).unwrap_or(0);
                    if #invalid_count {
                        __violations.push(#violation_ident::#v { target: key.clone(), count });
                    }
                }
            },
        }
    });

    quote! {
        let mut #storage: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut #from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #pair_index: #pair_index_type = std::collections::HashMap::new();
        for (id, value) in self.#accessor {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(#violation_ident::#dup_key(id));
                continue;
            }
            #destructure_value
            let from_position = #from_field.position(&from).map(#from_position_type);
            let to_position = #to_field.position(&to).map(#to_position_type);
            if from_position.is_none() {
                __violations.push(#violation_ident::#unk_src { edge: id.clone(), source: from.clone() });
            }
            if to_position.is_none() {
                __violations.push(#violation_ident::#unk_dst { edge: id.clone(), target: to.clone() });
            }
            if let (Some(from_position), Some(to_position)) = (from_position, to_position) {
                #unique_pair_check
                let internal_edge_position = #edge_position(graphite::TablePosition::from_index(#storage.len()));
                #pair_insert
                #from_index.entry(from_position).or_default().push(internal_edge_position);
                #to_index.entry(to_position).or_default().push(internal_edge_position);
                let inserted = #storage.insert(id, #build_record);
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        #each_type_check
        #(#each_checks)*
    }
}
