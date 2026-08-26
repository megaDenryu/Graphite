//! 完成済みグラフの構築経路 (`create` 系) と種別APIを1つの impl へまとめる。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::kind_api::edge_kind_api::gen_edge_kind_api_methods;
use crate::schema::codegen::kind_api::node_kind_api::gen_node_kind_api_methods;
use crate::schema::codegen::node_names::NodeInfo;

/// スキーマ struct 本体の impl。構築経路 (`create` 系) と、種別APIを置く。
///
/// 種別APIとは、ある種別に属する個体の全体を対象にする読み取り・可変操作の
/// ことである。完成済みの `Graph` が個体と索引を所有するため、これらは
/// `Graph` のメソッドになる (issue #9: `Org::Person::get(&graph, &id)` の
/// ように所有者を外から引数で渡す型名前空間は作らない)。名前は
/// `{accessor}_{固定接尾辞}` の機械的連結で、`{種別名}_` で始まるため補完に
/// 種別ごとの操作が並ぶ (`kind_api_method_ident` 参照)。
///
/// - ノード種別: `{node}_by_id` / `{node}_value_mut` / `{node}_ids` /
///   `{node}_iter` / `{node}_len`
/// - 辺種別: `{kind}_by_id` / `{kind}_payload_mut` (積み荷がある場合のみ) /
///   `{kind}_ids` / `{kind}_iter` / `{kind}_len`
///
/// 可変APIの主語は `&mut Graph` だけである。`NodeRef`/`EdgeRef` は共有借用の
/// ハンドルなのでそこから可変借用は作れず、引数も公開IDのままにする
/// (可変借用中は `Ref` を生かせないため内部位置をキーにできない)。
///
/// `graph!` 左辺名の静的読み取りは呼び出し箇所のラッパーへ生成するため
/// ここには含まれない。一度 `Ref` を得た後の関係の探索は `NodeRef`/`EdgeRef`
/// 自身のメソッドが担う。
pub(crate) fn gen_schema_impl(
    schema_name: &Ident,
    violation_ident: &Ident,
    builder_ident: &Ident,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_kind_apis = nodes.iter().map(gen_node_kind_api_methods);
    let edge_kind_apis = edges.iter().map(gen_edge_kind_api_methods);
    quote! {
        impl #schema_name {
            #(#node_kind_apis)*
            #(#edge_kind_apis)*

            /// builder をクロージャに貸し出し、戻ったら凍結して図式適合
            /// (端点種別・where 制約) を一括検査する。最初の1件の違反で
            /// `Err` になる (複数の違反を全件見たい場合は
            /// [`Self::create_collecting`] を使う)。
            pub fn create<F>(f: F) -> Result<Self, #violation_ident>
            where
                F: for<'b> FnOnce(&'b mut #builder_ident),
            {
                let mut builder = #builder_ident::new();
                f(&mut builder);
                builder.freeze()
            }

            /// `graph!` が名前付き要素の名前付き位置を凍結境界の外へ運ぶための
            /// 内部構築経路。`Graph` の凍結に成功した場合だけ名前付き位置を返す。
            /// [`graphite::build_named_graph`] へ薄く委譲するだけで、
            /// [`graphite::NamedInsertPermit`] はそちらでしか作らない
            /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
            #[doc(hidden)]
            pub fn create_named<F, N>(f: F) -> Result<(Self, N), #violation_ident>
            where
                F: for<'b> FnOnce(&'b mut #builder_ident, &'b graphite::NamedInsertPermit) -> N,
            {
                graphite::build_named_graph(#builder_ident::new, f)
            }

            /// [`Self::create`] の複数違反収集版。builder をクロージャに
            /// 貸し出し、戻ったら凍結して図式適合を検査する点は `create` と
            /// 同じだが、最初の1件で打ち切らず全違反を `Vec` に集めて返す。
            pub fn create_collecting<F>(f: F) -> Result<Self, Vec<#violation_ident>>
            where
                F: for<'b> FnOnce(&'b mut #builder_ident),
            {
                let mut builder = #builder_ident::new();
                f(&mut builder);
                builder.freeze_collecting()
            }
        }
    }
}
