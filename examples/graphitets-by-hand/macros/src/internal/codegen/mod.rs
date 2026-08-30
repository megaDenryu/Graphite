//! schema+instanceから具象コードを並べる配線。並び順は「辺値struct (種別) →
//! ノード達 → 辺達 → 個体参照 → 辺インスタンス参照 → 参照の層の集まり →
//! グラフ本体」で、手書き到達点 (`static_graph.rs`) の層順 (実体の層 →
//! 参照の層 → グラフ) と同じ。各生成物の中身は配下のmoduleが持ち、この
//! module本体は並び順だけを知る。

mod edge_entities;
mod edge_ref;
mod edge_value;
mod graph_struct;
mod node_entities;
mod node_ref;
mod ref_collections;

use proc_macro2::TokenStream;
use quote::quote;

use crate::literal::input::静的グラフ入力;
use crate::schema::input::静的グラフ型入力;

pub(crate) fn コードを生成する(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> TokenStream {
    let 辺値struct達 = edge_value::辺値struct達を生成する(schema);
    let ノード達 = node_entities::ノード達を生成する(&instance.ノード宣言達);
    let 辺達 = edge_entities::辺達を生成する(schema, &instance.辺宣言達);
    let 個体参照達 = node_ref::個体参照達を生成する(instance);
    let 辺インスタンス参照達 = edge_ref::辺インスタンス参照達を生成する(schema, instance);
    let ノード参照達 = ref_collections::ノード参照達を生成する(instance);
    let 辺参照達 = ref_collections::辺参照達を生成する(instance);
    let グラフ本体 = graph_struct::グラフ本体を生成する(instance);

    quote! {
        #辺値struct達
        #ノード達
        #辺達
        #個体参照達
        #辺インスタンス参照達
        #ノード参照達
        #辺参照達
        #グラフ本体
    }
}
