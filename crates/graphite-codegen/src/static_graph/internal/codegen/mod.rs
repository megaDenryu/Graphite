//! schema+instanceから具象コードを並べる配線。並び順は「Nodes → Edges →
//! 個体参照 → 辺インスタンス参照 → 参照の層の集まり → グラフ本体」で、
//! 手書き到達点 (`examples/graphitets-by-hand/src/bin/static_graph.rs`) の
//! 層順 (実体の層 → 参照の層 → グラフ) と同じ。各生成物の中身は配下の
//! moduleが持ち、この module本体は並び順だけを知る。schemaだけから決まる
//! 生成物 (辺値struct群) は `static_schema!` の展開へ移した
//! (`schema::codegen`) ためここでは扱わない。`{種別}Edge` 型は
//! edge_entities/edge_ref が参照するだけで、ここでは定義しない。
//!
//! 生成される識別子の英語化 (issue #24 段階2、オーナー承認済み):
//! `Nodes`/`Edges` (旧 `ノード達`/`辺達`)・`new()` (旧 `初期値`/`張る`)・
//! `{個体名}Ref`/`{辺名}Ref` (旧 `{個体名}参照`/`{辺名}参照`)・`entity()`
//! (旧 `実体()`)・`NodeRefs`/`EdgeRefs` とその `node_refs`/`edge_refs`
//! フィールド (旧 `ノード参照達`/`辺参照達`)。利用者が書く個体名・辺名・
//! 役割名はそのまま (フィールド名・アクセサ名として echo する)。

mod edge_entities;
mod edge_ref;
mod graph_struct;
mod node_entities;
mod node_ref;
mod ref_collections;

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;

pub(crate) fn コードを生成する(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> TokenStream {
    let ノード達 = node_entities::ノード達を生成する(&instance.ノード宣言達);
    let 辺達 = edge_entities::辺達を生成する(schema, &instance.辺宣言達);
    let 個体参照達 = node_ref::個体参照達を生成する(instance);
    let 辺インスタンス参照達 = edge_ref::辺インスタンス参照達を生成する(schema, instance);
    let ノード参照達 = ref_collections::ノード参照達を生成する(instance);
    let 辺参照達 = ref_collections::辺参照達を生成する(instance);
    let グラフ本体 = graph_struct::グラフ本体を生成する(instance);

    quote! {
        #ノード達
        #辺達
        #個体参照達
        #辺インスタンス参照達
        #ノード参照達
        #辺参照達
        #グラフ本体
    }
}
