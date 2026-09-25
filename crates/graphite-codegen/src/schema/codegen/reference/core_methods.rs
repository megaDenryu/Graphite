//! 有向・無向で共通する辺参照のメソッド (内部レコードの取得と `id()`) を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::declaration_doc::宣言元への参照;
use crate::schema::codegen::public_id_type::PublicIdType;

// 生成物は利用側の crate で `clippy::expect_used` の検査を受けるため、到達したら
// バグである箇所を `.expect()` でなく、破られた不変条件を書いた `panic!` で表す (issue #48)。
const 辺参照の内部位置の不変条件: &str = "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)";

// 辺参照値の共通メソッド (内部レコードの取得、`id()`) を生成する。
// 有向/無向のどちらの `impl<'graph> #reference<'graph> { .. }` 本体からも
// 同形で使うため共有する。`id()` のスパンは辺種別トークンを継承する。
pub(crate) fn edge_reference_core_methods(
    accessor: &Ident,
    record: &Ident,
    id_ty: &PublicIdType,
    kind_span: proc_macro2::Span,
    辺宣言元への参照: &宣言元への参照,
) -> TokenStream {
    let id_ident = Ident::new("id", kind_span);
    let 内部位置の不変条件 = 辺参照の内部位置の不変条件;
    quote! {
        fn record(self) -> &'graph #record {
            let Some((_, record)) = self.graph.#accessor.get_at(self.internal_position.0) else {
                panic!(#内部位置の不変条件)
            };
            record
        }

        /// この辺個体の公開IDを借用する。
        #辺宣言元への参照
        pub fn #id_ident(self) -> &'graph #id_ty {
            let Some((id, _)) = self.graph.#accessor.get_at(self.internal_position.0) else {
                panic!(#内部位置の不変条件)
            };
            id
        }
    }
}
