//! 検証済みの構文モデルを読み、スキーマ定義を組み立てる。
//!
//! 検証を通過した構文だけを受け取る前提なので失敗しない署名にする。端点の名前
//! 解決に失敗した場合は「検証が漏れている」というバグなので `expect` で落とす。

use proc_macro2::Ident;

use super::edge_definition::{有向端点, 辺の向き, 辺定義};
use super::node_definition::{ノード定義, ノード定義番号};
use super::schema_definition::スキーマ定義;
use crate::schema_dsl::{EdgeDecl, EdgeShape, NodeDecl, SchemaInput};

/// 検証済みの `schema` 宣言からスキーマ定義を組み立てる。
pub fn 検証済み構文からスキーマ定義を組み立てる(
    構文: &SchemaInput,
) -> スキーマ定義 {
    let ノード定義の列: Vec<ノード定義> = 構文.nodes.iter().map(ノード定義::宣言から作る).collect();
    let 辺定義の列: Vec<辺定義> = 構文
        .edges
        .iter()
        .map(|宣言| {
            辺定義::宣言と向きから作る(宣言, 辺の向きを組み立てる(宣言, &構文.nodes))
        })
        .collect();
    スキーマ定義::定義の列から作る(
        構文.schema_name.clone(),
        ノード定義の列,
        辺定義の列,
    )
}

/// 辺宣言の端点に書かれたノード型名を、ノード定義番号へ解決した向きにする。
fn 辺の向きを組み立てる(
    宣言: &EdgeDecl, ノード宣言の列: &[NodeDecl]
) -> 辺の向き {
    match &宣言.shape {
        EdgeShape::Directed { from, to, .. } => 辺の向き::有向 {
            始点: 有向端点::役割名とノードから作る(
                from.role.clone(),
                型名からノード定義番号を求める(ノード宣言の列, &from.ty),
            ),
            終点: 有向端点::役割名とノードから作る(
                to.role.clone(),
                型名からノード定義番号を求める(ノード宣言の列, &to.ty),
            ),
        },
        // 無向辺の両端は同じノード型であることを `validate_undirected_same_type`
        // が保証済みなので、片方だけを見れば足りる。
        EdgeShape::Undirected { first, .. } => 辺の向き::無向 {
            端点のノード: 型名からノード定義番号を求める(
                ノード宣言の列,
                first,
            ),
        },
    }
}

fn 型名からノード定義番号を求める(
    ノード宣言の列: &[NodeDecl],
    型名: &Ident,
) -> ノード定義番号 {
    let 添字 = ノード宣言の列
        .iter()
        .position(|宣言| 宣言.name == *型名)
        .expect("validate() を通過していれば必ず見つかるはず");
    ノード定義番号::添字から作る(添字)
}
