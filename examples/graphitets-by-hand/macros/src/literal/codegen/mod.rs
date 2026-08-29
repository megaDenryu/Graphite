//! 静的グラフ入力から生成物を並べる配線。並び順は「個体タグ → ノード達 →
//! 辺達 → 型別名 → 所属辺メソッド → 多重度・対一意のconst assert → 参照の層と
//! グラフ本体」で、手書き到達点 (`static_graph.rs`) の層順 (仕組み→実体の層→
//! 参照の層) を踏まえつつ、多重度検査を参照の層の手前に置く。各生成物の中身は
//! 配下の module が持ち、この module 本体は並び順だけを知る。

mod aliases;
mod collections;
mod edge_entities;
mod multiplicity_asserts;
mod node_edge_methods;
mod node_entities;
mod tags;

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::literal::input::静的グラフ入力;

pub(crate) fn 生成する(入力: &静的グラフ入力) -> TokenStream {
    let 実体型索引: HashMap<String, proc_macro2::Ident> =
        入力.ノード宣言達.iter().map(|ノード| (ノード.名前.to_string(), ノード.実体型.clone())).collect();

    let 個体タグ = tags::個体タグ達を生成する(&入力.ノード宣言達);
    let ノード達 = node_entities::ノード達を生成する(&入力.ノード宣言達);
    let 辺達 = edge_entities::辺達を生成する(&入力.辺宣言達, &実体型索引);
    let 型別名達 = aliases::型別名達を生成する(入力);
    let 所属辺メソッド達 = node_edge_methods::所属辺メソッド達を生成する(&入力.ノード宣言達, &入力.辺宣言達);
    let 多重度検査達 = multiplicity_asserts::多重度検査達を生成する(入力);
    let 参照達とグラフ = collections::参照達とグラフを生成する(入力);

    quote! {
        #個体タグ
        #ノード達
        #辺達
        #型別名達
        #所属辺メソッド達
        #多重度検査達
        #参照達とグラフ
    }
}
