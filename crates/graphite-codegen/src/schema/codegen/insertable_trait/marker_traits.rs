//! ノード専用・辺専用の型境界となるマーカートレイトと、その実装一式を生成する。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。こ
//! のファイルはノード側と辺側で対になる2つの生成関数を持つ。この2つは同じ形を
//! 共有しており、分けると読み手は対応を追えなくなる。超過を許す根拠の台帳は
//! `docs/development/line_count_ledger.md` にある。

use proc_macro2::TokenStream;
use quote::quote;

use crate::naming::{construction_stamp_field_ident, 固定生成名の予約表};
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::insertable_trait::default_id_implementation::gen_default_id_impl;
use crate::schema::codegen::insertable_trait::element_implementation::{
    gen_insertable_and_named_impl, InsertableNamedSpec,
};
use crate::schema::codegen::node_names::NodeInfo;
use crate::schema::codegen::reference::node_reference::gen_node_reference_type;
use crate::schema::semantic::ノードの探索計画;

// v4 (`docs/schema_v4.md` §3.2) が要求する「ノード挿入用トレイト」
// とその各ノード型への impl、およびノード種別ごとの `NodeRef` 型を生成する。
//
// このトレイトが必要になる背景は次のとおりである。
//
// `graph!` はノード項を `key = 式` と書かせ、値の型をマクロが一切パース
// しない (式の型は rustc の型推論に委ねる、という設計上の決定)。その結果
// `graph!` はもはや「どのビルダーメソッドを呼ぶべきか」を型名から逆引き
// できないため、値の型さえ分かれば正しい内部ストレージへ振り分けられる
// 総称メソッドが要る。この trait 境界を介した単相化がそれを実現する
// (実行時のリフレクション・型判別・`dyn` ディスパッチは一切無い。
// `docs/development/design_principles.md` 原則5: ゼロコスト志向)。
//
// 読み取り側をここへ置かないのには理由がある。
//
// 公開IDからの検索と種別全体への操作 (`{node}_by_id`/`{node}_ids`/
// `{node}_iter`/`{node}_len`/`{node}_value_mut`) は、個体と索引を所有する
// `Graph` のメソッドとして `gen_schema_impl` が生成する。ノード型
// (`Person` 等) はユーザーが `graph_schema!` の外で宣言する型であり複数
// schema 間で共有されうるため、ユーザー struct への固有 impl は追加しない。
// schema module 内にノード名の空 struct (読み取り用マーカー) も置かない
// (issue #9: `Graph` を外から引数で渡す型名前空間を作らない)。
//
// `{Schema}Insertable` と `{Schema}DefaultId` には次のように役割が分かれている。
//
// 型付き挿入と関連型 `Id` は `{Schema}Insertable` に置く。文字列の束縛名から
// IDを作る操作は自動生成IDだけが実装する `{Schema}DefaultId` に置く。
// `{Schema}Node` はノード専用の型境界を保つマーカートレイトである。
//
// 命名は、`docs/development/design_principles.md` 原則3 (std 命名規約準拠) に
// 沿って決めた名前である。
//
// 内部 trait 名は `{Schema}Node` とした。生成 module に移した後も
// `node Node;` や `edge Edge = ..;` と生成基盤名が衝突する可能性を増やさず、
// コンパイラ診断から所属 schema を判別できる名前を維持する。
pub(crate) fn gen_node_trait_and_impls(
    予約表: &固定生成名の予約表,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
    探索計画の列: &[ノードの探索計画],
) -> TokenStream {
    let node_trait_ident = 予約表.ノード挿入トレイト名();
    let insertable_trait_ident = 予約表.挿入可能トレイト名();
    let default_id_trait_ident = 予約表.既定id生成トレイト名();
    let builder_ident = 予約表.構築器型名();
    let graph_ident = 予約表.グラフ型名();
    debug_assert_eq!(
        nodes.len(),
        探索計画の列.len(),
        "探索計画の列はノード定義の列と同じ並びで1件ずつ対応する"
    );
    let node_impls = nodes.iter().zip(探索計画の列).map(|(n, 探索計画)| {
        let ty = &n.type_ident;
        let reference = n.reference_ident();
        let internal_position = n.internal_position_ident();
        let named_position = n.named_position_ident();
        let stamp_field = construction_stamp_field_ident(ty.span());
        // IDE 支援 (`docs/development/ide_support_spec.md` §1.9, G3 ポリシー): このノード
        // 型への `{Schema}Node`/`{Schema}Insertable` impl が生やすメソッド名は
        // `n.type_ident` (ノード型そのもののトークン) のスパンを持たせる。
        // トレイト定義自体 (下の `pub trait #node_trait_ident { .. }`) は
        // 単一の由来トークンを持たない schema 全体のインフラなので call_site
        // のままでよい (指示どおり、impl 側だけに適用する)。
        let span = ty.span();
        let value_type = quote! { super::#ty };
        let common_impl = gen_insertable_and_named_impl(InsertableNamedSpec {
            insertable_trait_ident,
            builder_ident,
            graph_ident,
            value_type: value_type.clone(),
            id_ty: n.id_ty,
            named_position: &named_position,
            internal_position: &internal_position,
            storage: &n.field_ident,
            accessor: &n.accessor_ident,
            reference: &reference,
            stamp_field: &stamp_field,
            span,
        });
        let default_id_impl = gen_default_id_impl(
            default_id_trait_ident,
            insertable_trait_ident,
            builder_ident,
            &value_type,
            n.id_ty,
        );
        let node_reference = gen_node_reference_type(graph_ident, n, 探索計画, edges);
        quote! {
            #common_impl

            #default_id_impl
            impl #node_trait_ident for super::#ty {}

            #node_reference
        }
    });

    quote! {
        /// ノード挿入で使うトレイト境界。読み取りは `Graph` の種別メソッドと
        /// `NodeRef` のメソッドが提供する。利用者がこのトレイトのメソッドを
        /// 直接呼ぶことは想定しない。
        pub trait #node_trait_ident: #insertable_trait_ident {}

        #(#node_impls)*
    }
}

// エッジ挿入用トレイト (書き込み側専用)。`graph!` の辺行
// `key = Kind(from -> to)` は名前付きフィールドの辺値型を関連コンストラクタで
// 構築したあと、この trait 境界を介した総称 `{Builder}::add` に脱糖する
// (`docs/schema_v4.md` §2/§3.2)。読み取り側は `Graph` の種別メソッド
// (`{kind}_by_id`/`{kind}_iter`/`{kind}_ids`/`{kind}_len`、`gen_schema_impl`
// 参照) と `NodeRef` のメソッド (`{kind}_as_{役割}`/`{kind}_incident`/
// `{kind}_between`、`gen_node_traversal_methods` 参照) が提供するため、
// このトレイトには含めない。
//
// 型付き挿入と関連型 `Id` は `{Schema}Insertable` に集約する。このトレイトは
// エッジ専用の型境界を保つマーカーになる。
pub(crate) fn gen_edge_trait_and_impls(
    予約表: &固定生成名の予約表,
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let edge_trait_ident = 予約表.辺挿入トレイト名();
    let insertable_trait_ident = 予約表.挿入可能トレイト名();
    let default_id_trait_ident = 予約表.既定id生成トレイト名();
    let builder_ident = 予約表.構築器型名();
    let graph_ident = 予約表.グラフ型名();
    let edge_impls = edges.iter().map(|e| {
        let kind = e.kind;
        let accessor = &e.accessor_ident;
        let reference = e.reference_ident();
        let internal_position = e.internal_position_ident();
        let named_position = e.named_position_ident();
        let stamp_field = construction_stamp_field_ident(kind.span());
        let value_type = quote! { #kind };
        // 必須ではないが (このメソッドはユーザーが直接呼ぶ想定ではない)、
        // 他の生成メソッドとの一貫性のため `edge.kind` のスパンを付ける
        // (`docs/development/ide_support_spec.md` §1.9 の指示: 余裕があれば付けてよい)。
        let common_impl = gen_insertable_and_named_impl(InsertableNamedSpec {
            insertable_trait_ident,
            builder_ident,
            graph_ident,
            value_type: value_type.clone(),
            id_ty: e.id_ty,
            named_position: &named_position,
            internal_position: &internal_position,
            storage: accessor,
            accessor,
            reference: &reference,
            stamp_field: &stamp_field,
            span: kind.span(),
        });
        let default_id_impl = gen_default_id_impl(
            default_id_trait_ident,
            insertable_trait_ident,
            builder_ident,
            &value_type,
            e.id_ty,
        );
        quote! {
            #common_impl

            #default_id_impl
            impl #edge_trait_ident for #kind {}
        }
    });

    quote! {
        /// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
        /// この trait のメソッドを直接呼ぶことは想定しない
        /// (`{Builder}::add` 経由で使う)。
        pub trait #edge_trait_ident: #insertable_trait_ident {}

        #(#edge_impls)*
    }
}
