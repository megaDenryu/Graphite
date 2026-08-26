//! イテレータから一括で挿入するAPIを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// 一括構築API (`extend`) を生成する。
pub(crate) fn gen_builder_extend_api(default_id_trait_ident: &Ident) -> TokenStream {
    quote! {
        /// `insert`/`add` のイテレータ版 (`docs/bulk_construction.md`、
        /// `docs/graph_splice.md` §2)。実行時データからの構築で for
        /// ループが構築コードに残るのを避けるため、要素単位 API の反復に
        /// 完全に一致する意味論 (挿入順保持・検証は凍結時) をまとめて
        /// 提供する。ノード用・エッジ用の呼び分けが要らない単一の総称
        /// メソッドに統一している (v4 破壊的変更、旧 `extend_nodes`/
        /// `extend_edges` は廃止): 値の型が既定IDを生成できれば
        /// ノードでもエッジでもよい (どちらになるかは rustc の
        /// 型推論任せ)。`graph!` のスプライス項 (`..式`) もこのメソッドへ
        /// 脱糖する。`insert`/`add` と同じ理由 (トレイトが schema ごとに
        /// 名前が異なる) で、graphite ランタイム側の共通機構ではなく
        /// ここに生成する。
        pub fn extend<K, T>(&mut self, items: impl IntoIterator<Item = (K, T)>) -> Vec<T::Id>
        where
            K: Into<String>,
            T: #default_id_trait_ident,
        {
            // スプライス要素は公開IDだけを持ち、名前付き位置を返さない。
            items.into_iter().map(|(k, v)| v.insert_with_binding(self, k.into())).collect()
        }
    }
}
