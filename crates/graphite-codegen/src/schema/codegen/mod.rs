//! 意味モデルを Rust の生成物へ写し、生成物の並び順を決める配線を持つ。
//!
//! 生成物ごとの中身は配下の module が持ち、この module 本体は「何をどの順で
//! 並べるか」だけを知る。並び順は追跡可能な生成ファイルの中身そのものなので、
//! 並べ替えると生成物が変わる。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。
//! このファイルは生成物1種別分の全体像を1箇所で見せる地図を兼ねる。この地図
//! の分割は、読み手の読む場所探しを増やす。超過を許す根拠の台帳は
//! `docs/development/line_count_ledger.md` にある。
//!
//! 以下は生成物の全体像であり、配下の module を読むときの地図として置く
//! (v4、`docs/schema_v4.md` §3 参照。
//! v4.1 の役割名・無向辺は `docs/edge_endpoints_v4_1.md`、ID 型の既定生成と
//! 明示指定は `docs/node_id_v4_2.md` 参照)。名前付きラッパー・名前付き位置型・
//! 呼び出し箇所・凍結の用語定義は `docs/schema_v4.md` §3.1.1 参照。
//!
//! ## 生成物の全体像 (1エッジ種別分)
//!
//! `schema Org` 内の
//! `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;` から:
//!
//! ```text
//! pub mod Org {
//! pub struct BossId(pub String);
//! pub struct Boss {
//!     pub subordinate: PersonId,
//!     pub superior: PersonId,
//!     pub appointment: super::BossEdge,
//! }
//! pub struct PersonRef<'graph> { /* &Graph + private position */ }
//! pub struct BossRef<'graph> { /* &Graph + private position */ }
//! impl<'graph> PersonRef<'graph> {
//!     pub fn boss_as_subordinate(self) -> Option<BossRef<'graph>> { .. }
//!     pub fn boss_as_superior(self) -> impl Iterator<Item = BossRef<'graph>> { .. }
//!     pub fn boss_between(self, other: PersonRef<'graph>) -> .. { .. }
//! }
//! pub struct Graph { .. }
//! impl Graph {
//!     pub fn person_by_id(&self, id: &PersonId) -> Option<PersonRef<'_>> { .. }
//!     pub fn boss_by_id(&self, id: &BossId) -> Option<BossRef<'_>> { .. }
//!     pub fn boss_iter(&self) -> impl Iterator<Item = BossRef<'_>> { .. }
//! }
//! pub struct Builder { .. }
//! pub enum Violation { .. }
//! }
//! ```
//!
//! 以下、ノード種別ごとの `{Node}Ref<'graph>` を NodeRef、辺種別ごとの
//! `{Kind}Ref<'graph>` を EdgeRef と総称する。
//!
//! 構築用の有向辺値 (`(subordinate: Employee) -> (superior: Employee)`) は役割名の
//! 公開IDフィールドを保持する。完成済みの `EdgeRef` は役割名のメソッドで `NodeRef` を返す。
//! 無向辺 (`Person -- Person`) は
//! `.endpoints() -> (PersonRef<'_>, PersonRef<'_>)` を生やし、`{kind}_incident` と
//! `{kind}_between` はどちらの位置に置かれても対称に検索できる。内部の凍結処理
//! (`freeze()`) と検索処理は名前付きフィールドを直接使う。
//!
//! 個体と索引を所有するのは完成済みの `Graph` なので、公開IDからの検索と
//! 種別全体への操作は `Graph` のメソッドとして生やす (`graph.person_by_id(&id)`、
//! `graph.boss_iter()`)。`Graph` を外から引数で渡す型名前空間の関連関数
//! (`Org::Person::get(&graph, &id)`) は作らない (issue #9)。一度 `NodeRef` を
//! 得た後の関係の探索は、その参照が親 `Graph` と内部位置を保持しているため
//! `NodeRef` 自身のメソッド (`person.boss_as_subordinate()`) で辿る。
//! ノード値型はユーザーが module 外に宣言し、複数 schema 間で共有できる。
//! ユーザー型への固有 impl は追加しない。
//! ID 型を省略した `node Person;` は schema module 内に `PersonId(String)` を
//! 生成する。既存型を使う場合は `node Person(id: ExistingId);` と明示する。
//!
//! where 制約 → 役割探索メソッドの戻り型の対応表 (`docs/schema_v4.md` §3.2):
//! - `each X: 1`    -> `node.{kind}_as_X()` は `EdgeRef`
//! - `each X: 0..1` -> `node.{kind}_as_X()` は `Option<EdgeRef>`
//! - その他の範囲または制約なし -> iterator
//! - `unique pair`  -> `a.{kind}_between(b)` は `Option`、それ以外は iterator
//!
//! 判定する制約は問い合わせた役割そのものの `each`。無向辺は役割名を
//! 持たないため `node.{kind}_incident()` を生成し、常に iterator を返す。
//!
//! 有向辺の両側は役割名を使う対称なAPIになる。たとえば
//! `person.boss_as_subordinate()` と `person.boss_as_superior()` はどちらも
//! `EdgeRef` を返し、相手端点や積み荷はその参照から辿る。
//!
//! 実装は凍結時に構築・永続化する終点索引 `{accessor}_to_index`
//! (`NodePosition -> EdgePosition/Option/連続範囲`、`gen_schema_struct`/`gen_directed_edge_freeze_block`
//! 参照) を検索するだけなので O(1)。この索引は入次数 each 検証にも使う
//! (参照: `docs/reverse_query.md`)。

mod builder;
mod declaration_doc;
mod edge_names;
mod edge_record;
mod edge_value;
mod freeze;
mod graph_construction_api;
mod graph_storage;
mod insertable_trait;
mod internal_position_type;
mod kind_api;
mod named_position_type;
mod node_names;
mod pair_index;
mod public_id_type;
mod reference;
mod traversal;
mod violation;

use proc_macro2::TokenStream;
use quote::quote;

use crate::naming::固定生成名の予約表;
use crate::schema::semantic::スキーマ定義;
use builder::gen_builder_impl;
use builder::struct_definition::gen_builder_struct;
pub(crate) use declaration_doc::宣言元ファイルの綴り;
use edge_names::{build_edge_info, EdgeInfo};
use edge_record::gen_edge_record_structs;
use edge_value::gen_edge_value_structs;
use graph_construction_api::gen_schema_impl;
use graph_storage::gen_schema_struct;
use insertable_trait::marker_traits::{gen_edge_trait_and_impls, gen_node_trait_and_impls};
use insertable_trait::trait_definition::gen_insertable_traits;
use internal_position_type::gen_internal_position_types;
use named_position_type::gen_named_position_types;
use node_names::NodeInfo;
use public_id_type::gen_default_id_types;
use reference::gen_edge_reference_types;
use violation::gen_violation_enum;

pub(crate) fn generate_module_body(
    schema: &スキーマ定義,
    宣言元の綴り: &宣言元ファイルの綴り,
) -> TokenStream {
    let schema_name = schema.スキーマ名();
    // 固定生成名は衝突検査 (`schema::validate::generated_name_collision`) と
    // 同じ予約表から取り出す。
    let 予約表 = 固定生成名の予約表::schema名から導出する(schema_name);
    let graph_ident = 予約表.グラフ型名().clone();
    let violation_ident = 予約表.違反列挙型名().clone();
    let builder_ident = 予約表.構築器型名().clone();
    let insertable_trait_ident = 予約表.挿入可能トレイト名().clone();
    let default_id_trait_ident = 予約表.既定id生成トレイト名().clone();

    // schema 全体に属する生成物 (`Graph`・`Builder`・`Violation`) は
    // `schema Name` の宣言を指す。種別ごとの生成物は `NodeInfo`/`EdgeInfo` が
    // 持つ、その種別の宣言への参照を指す。
    let スキーマ宣言元への参照 = 宣言元の綴り.宣言への参照(&schema.宣言の形());

    let node_infos: Vec<NodeInfo> = schema
        .ノード定義の列()
        .iter()
        .map(|定義| NodeInfo::new(定義, 宣言元の綴り.宣言への参照(&定義.宣言の形())))
        .collect();

    let edge_infos: Vec<EdgeInfo> = schema
        .辺定義の列()
        .iter()
        .map(|定義| {
            build_edge_info(
                定義,
                &node_infos,
                宣言元の綴り.宣言への参照(&schema.辺の宣言の形(定義)),
            )
        })
        .collect();

    let default_id_defs = gen_default_id_types(&node_infos, &edge_infos);
    let internal_position_defs = gen_internal_position_types(&node_infos, &edge_infos);
    let named_position_defs = gen_named_position_types(&node_infos, &edge_infos);
    let edge_value_struct_defs = gen_edge_value_structs(&edge_infos);
    let edge_record_defs = gen_edge_record_structs(&edge_infos);
    let edge_reference_defs = gen_edge_reference_types(&graph_ident, &edge_infos);
    let violation_def = gen_violation_enum(
        &violation_ident,
        &node_infos,
        &edge_infos,
        schema.違反定義の列(),
        &スキーマ宣言元への参照,
    );
    let schema_struct_def = gen_schema_struct(
        &graph_ident,
        &node_infos,
        &edge_infos,
        &スキーマ宣言元への参照,
    );
    let schema_impl = gen_schema_impl(
        &graph_ident,
        &violation_ident,
        &builder_ident,
        &node_infos,
        &edge_infos,
    );
    let builder_struct_def = gen_builder_struct(
        &builder_ident,
        &node_infos,
        &edge_infos,
        &スキーマ宣言元への参照,
    );
    let builder_impl = gen_builder_impl(&予約表, &node_infos, &edge_infos);
    let insertable_trait_def = gen_insertable_traits(
        &insertable_trait_ident,
        &default_id_trait_ident,
        &builder_ident,
    );
    let node_trait_and_impls = gen_node_trait_and_impls(
        &予約表,
        &node_infos,
        &edge_infos,
        schema.ノードごとの探索計画(),
    );
    let edge_trait_and_impls = gen_edge_trait_and_impls(&予約表, &edge_infos);
    quote! {
        #(#default_id_defs)*
        #(#internal_position_defs)*
        #(#named_position_defs)*
        #(#edge_value_struct_defs)*
        #(#edge_record_defs)*
        #violation_def
        #schema_struct_def
        #schema_impl
        #(#edge_reference_defs)*
        #builder_struct_def
        #insertable_trait_def
        #node_trait_and_impls
        #edge_trait_and_impls
        #builder_impl
    }
}

pub(crate) fn generate(schema: &スキーマ定義, 宣言元の綴り: &宣言元ファイルの綴り) -> TokenStream {
    let schema_name = schema.スキーマ名();
    let body = generate_module_body(schema, 宣言元の綴り);
    quote! {
        #[allow(non_snake_case)]
        pub mod #schema_name {
            use super::*;
            #body
        }
    }
}
