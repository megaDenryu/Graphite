//! 値の型からストレージを決める辺挿入の総称APIを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// 辺挿入の総称API (`add`/`add_named`/`add_with_id`/`add_named_with_id`) を生成する。
pub(crate) fn gen_builder_edge_insert_api(
    edge_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
) -> TokenStream {
    quote! {
        /// `insert` のエッジ版。`graph!` の辺行 `key = Kind(from -> to)`
        /// は名前付きフィールドの辺値型を関連コンストラクタで構築したあと、
        /// 下記 `add_named` へ脱糖する (`docs/schema_v4.md` §2/§3.2)。
        /// このメソッド自体は値の型から内部ストレージへ振り分ける総称
        /// ディスパッチを提供する手書き用APIで、`graph!` を直接経由しない。
        pub fn add<E>(&mut self, key: impl Into<String>, value: E) -> E::Id
        where
            E: #edge_trait_ident + #default_id_trait_ident,
        {
            value.insert_with_binding(self, key.into())
        }

        /// `graph!` が公開IDと名前付き辺の内部位置を同時に受け取る経路。
        /// [`graphite::NamedInsertPermit`] を要求する
        /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
        #[doc(hidden)]
        pub fn add_named<E>(
            &mut self,
            key: impl Into<String>,
            value: E,
            permit: &graphite::NamedInsertPermit,
        ) -> (E::Id, E::NamedPosition)
        where
            E: #edge_trait_ident + #default_id_trait_ident,
        {
            value.insert_named_with_binding(self, key.into(), permit)
        }

        /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
        /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
        /// `add_named_with_id` へ脱糖するため、このメソッド自体は
        /// `graph!` を経由しない。
        pub fn add_with_id<E: #edge_trait_ident>(&mut self, id: E::Id, value: E) -> E::Id {
            value.insert_with_id(self, id)
        }

        /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
        /// [`graphite::NamedInsertPermit`] を要求する
        /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
        #[doc(hidden)]
        pub fn add_named_with_id<E: #edge_trait_ident>(
            &mut self,
            id: E::Id,
            value: E,
            permit: &graphite::NamedInsertPermit,
        ) -> (E::Id, E::NamedPosition) {
            value.insert_named_with_id(self, id, permit)
        }
    }
}
