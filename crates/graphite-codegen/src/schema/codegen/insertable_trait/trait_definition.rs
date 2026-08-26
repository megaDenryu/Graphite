//! ノードと辺に共通する挿入トレイトそのものの定義を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// ノード用/エッジ用の挿入トレイトの**共通 supertrait**
/// (`docs/graph_splice.md` §2「extend の統一」)。
///
/// ## 背景: なぜ統一 `extend` にこの trait が要るか
///
/// `graph!` のスプライス項 (`..式`) と builder の一括構築 API は、渡された
/// イテレータの要素の型 (ノード型かエッジ種別か) を見て正しい内部ストレージへ
/// 振り分ける必要がある。この判別も他の総称メソッド (`insert`/`add`) と同様
/// rustc の型推論 (単相化) に委ねたいので、`extend<K, T>` の `T` に対する
/// **単一の**トレイト境界が要る。しかし `insert`/`add` はそれぞれ「ノード専用」
/// 「エッジ専用」の型境界を保つ必要がある (`docs/graph_splice.md` §2「これも
/// 統一できるか? しない」)。この2つの要求を両立させるため、型付き挿入と
/// `Id` を本トレイトに集約し、文字列から既定IDを作る能力だけを別トレイトに
/// 分ける。
///
/// ## 検討した代替案: 2本の blanket impl
///
/// ```text
/// impl<T: {Schema}Node> {Schema}Insertable for T { .. }
/// impl<T: {Schema}Edge> {Schema}Insertable for T { .. }
/// ```
/// という2本の blanket impl にすれば、ノード/エッジの型ごとに追加の impl
/// ブロックを生成せずに済む (schema 内の型数に関わらず定数個の impl で
/// 橋渡しできる) ため、生成コード量そのものはこちらの方が小さくなる場合が
/// 多い。しかし rustc の coherence 検査は「ある型が `{Schema}Node` と
/// `{Schema}Edge` を両方実装する可能性」を型システムのレベルでは否定できない
/// (この2つは無関係な独立したトレイトであり、将来のある型が両方を実装しない
/// 保証が無い) ため、この2本の blanket impl は素の stable Rust では
/// **E0119 (conflicting implementations)** になる。したがって、型ごとに
/// `{Schema}Insertable` を直接 impl する (= supertrait 関係にして、ノード型
/// への impl ブロックを1つ増やす) 方式を採用する。
pub(crate) fn gen_insertable_traits(
    insertable_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
    builder_ident: &Ident,
) -> TokenStream {
    quote! {
        /// 型付き ID を受け取るノード・エッジ共通の挿入トレイト。
        ///
        /// 署名が `insert_with_id(self, b, id)` と、挿入される値を receiver に
        /// して `Builder` を引数で受ける向きなのは、`graph!` がノード項の値の
        /// 型を解析せず、正しい内部ストレージへの振り分けを値の型の trait
        /// ディスパッチに頼るためである。利用者向けの公開入口は
        /// `Builder::insert`/`Builder::add` の側にある。
        ///
        /// `insert_named_with_id` は [`graphite::NamedInsertPermit`] を要求する
        /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
        /// `insert_with_id` (許可証不要、名前付き位置を返さない) は独立した
        /// 実装を持ち、`insert_named_with_id` を経由しない
        /// (`create` のクロージャから許可証なしで呼べる必要があるため)。
        pub trait #insertable_trait_ident: Sized {
            type Id;
            #[doc(hidden)]
            type NamedPosition;

            #[doc(hidden)]
            fn insert_named_with_id(
                self,
                b: &mut #builder_ident,
                id: Self::Id,
                permit: &graphite::NamedInsertPermit,
            ) -> (Self::Id, Self::NamedPosition);

            fn insert_with_id(self, b: &mut #builder_ident, id: Self::Id) -> Self::Id;
        }

        /// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
        /// 実装する。明示ID型には実装せず、文字列変換を要求しない。
        pub trait #default_id_trait_ident: #insertable_trait_ident {
            #[doc(hidden)]
            fn insert_named_with_binding(
                self,
                b: &mut #builder_ident,
                binding: String,
                permit: &graphite::NamedInsertPermit,
            ) -> (Self::Id, Self::NamedPosition);

            fn insert_with_binding(self, b: &mut #builder_ident, binding: String) -> Self::Id;
        }
    }
}
