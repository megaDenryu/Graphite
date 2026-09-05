//! 値の型からストレージを決めるノード挿入の総称APIを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

// ノード挿入の総称API (`insert`/`insert_named`/`insert_with_id`/
// `insert_named_with_id`) を生成する。
pub(crate) fn gen_builder_node_insert_api(
    node_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
) -> TokenStream {
    quote! {
        /// 型名付きメソッド (`b.#accessor(id, value)` 群、上記
        /// `#node_methods`) の総称版。`graph!` の左辺名付きノード項は
        /// 下記 `insert_named` (名前付き位置を返す許可証付き経路) へ
        /// 脱糖するため、このメソッド自体は `graph!` を経由しない。
        /// 値の型を手書きで組み立てる場合 (プログラム的構築など) に使う。
        /// `graph!` はノード項の値の型を一切パースしないため
        /// (`key = 式` の「式」でしかない)、値の型 (`N: #node_trait_ident`)
        /// から正しい内部ストレージへの振り分けを rustc の型推論任せに
        /// する点は `insert_named` と共通。命名判断・trait の形は
        /// `gen_node_trait_and_impls` のドキュメントコメント参照。
        pub fn insert<N>(&mut self, key: impl Into<String>, value: N) -> N::Id
        where
            N: #node_trait_ident + #default_id_trait_ident,
        {
            value.insert_with_binding(self, key.into())
        }

        /// `graph!` が公開IDと名前付き要素の内部位置を同時に受け取る経路。
        /// [`graphite::NamedInsertPermit`] を要求する
        /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
        #[doc(hidden)]
        pub fn insert_named<N>(
            &mut self,
            key: impl Into<String>,
            value: N,
            permit: &graphite::NamedInsertPermit,
        ) -> (N::Id, N::NamedPosition)
        where
            N: #node_trait_ident + #default_id_trait_ident,
        {
            value.insert_named_with_binding(self, key.into(), permit)
        }

        /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
        /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
        /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
        /// `graph!` を経由しない。
        pub fn insert_with_id<N: #node_trait_ident>(&mut self, id: N::Id, value: N) -> N::Id {
            value.insert_with_id(self, id)
        }

        /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
        /// [`graphite::NamedInsertPermit`] を要求する
        /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
        #[doc(hidden)]
        pub fn insert_named_with_id<N: #node_trait_ident>(
            &mut self,
            id: N::Id,
            value: N,
            permit: &graphite::NamedInsertPermit,
        ) -> (N::Id, N::NamedPosition) {
            value.insert_named_with_id(self, id, permit)
        }
    }
}
