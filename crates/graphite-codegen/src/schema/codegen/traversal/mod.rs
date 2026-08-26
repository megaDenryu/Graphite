//! ノード参照へ生やす探索メソッドを、意味モデルの探索計画の順に並べる。

pub(crate) mod between_traversal;
pub(crate) mod incident_traversal;
pub(crate) mod role_traversal;

use proc_macro2::TokenStream;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::{ノードの探索計画, 探索操作};
use between_traversal::gen_between_traversal_methods;
use incident_traversal::gen_incident_traversal_method;
use role_traversal::gen_role_traversal_method;

/// 1つのノード種別の `NodeRef` へ生やす辺関連メソッドをすべて生成する。
///
/// `NodeRef` は親 `Graph` と内部位置を保持する Graph 束縛の参照なので、
/// 一度取得した後の関係の探索は親 `Graph` を再注入せずこの参照自身から辿る
/// (issue #9)。生成するのは次の3種類である。
///
/// - 有向辺の役割探索 `{kind}_as_{役割}()`
/// - 無向辺の接続探索 `{kind}_incident()`
/// - 端点対検索 `{kind}_between(other)` / `{kind}_try_between(other)`
///
/// どの操作をどの順で生やすかは意味モデルの探索計画が確定済みなので、ここは
/// 操作1つずつをRustへ写すだけである。
pub(crate) fn gen_node_traversal_methods(
    探索計画: &ノードの探索計画,
    edges: &[EdgeInfo<'_>],
) -> Vec<TokenStream> {
    探索計画
        .操作の列()
        .iter()
        .map(|操作| match 操作 {
            探索操作::役割による探索 {
                辺,
                役割名,
                側,
                多重度,
            } => gen_role_traversal_method(&edges[辺.添字()], 役割名, *側, *多重度),
            探索操作::接続による探索 { 辺 } => {
                gen_incident_traversal_method(&edges[辺.添字()])
            }
            探索操作::端点対による探索 { 辺 } => {
                gen_between_traversal_methods(&edges[辺.添字()])
            }
        })
        .collect()
}
