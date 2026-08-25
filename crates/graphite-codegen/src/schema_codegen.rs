//! `graph_schema!` のコード生成本体 (v4、`docs/schema_v4.md` §3 参照。
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
//! 個体と索引を所有するのは完成済みの `Graph` なので、公開IDからの検索と種別
//! 全体への操作は `Graph` のメソッドとして生やす (`graph.person_by_id(&id)`、
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

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Path};

use crate::naming::{
    construction_stamp_field_ident, each_violation_ident, edge_record_ident, edge_storage_ident,
    generated_id_ident, graph_type_ident, incident_method_ident, internal_position_ident,
    kind_api_method_ident, named_position_ident, node_storage_ident, pair_index_field_ident,
    reference_ident, to_snake_case, traversal_method_ident,
};
use crate::schema_dsl::{
    EachSpec, EdgeDecl, EdgePayload, EdgeShape, NodeDecl, RoleCardinality, SchemaInput,
};
use crate::schema_validate::{self, EachSide};

/// 宣言を省略した既定生成ID、または明示された既存ID型を表す。
/// 生成コード内でどちらも同じ型位置へ展開できるよう、パスの調整もここへ集約する。
struct PublicIdType {
    generated_ident: Ident,
    explicit_path: Option<Path>,
}

impl PublicIdType {
    fn is_generated(&self) -> bool {
        self.explicit_path.is_none()
    }
}

impl ToTokens for PublicIdType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Some(path) = &self.explicit_path else {
            self.generated_ident.to_tokens(tokens);
            return;
        };

        let first = path
            .segments
            .first()
            .map(|segment| segment.ident.to_string());
        if path.leading_colon.is_some() || first.as_deref() == Some("crate") {
            path.to_tokens(tokens);
        } else if first.as_deref() == Some("self") {
            let mut relative = path.clone();
            relative.segments = path.segments.iter().skip(1).cloned().collect();
            quote!(super::#relative).to_tokens(tokens);
        } else {
            // `use super::*` が呼び出し側の型名を取り込み、外部crate名と
            // プリミティブ型は生成moduleでも同じパスで解決できる。
            path.to_tokens(tokens);
        }
    }
}

struct NodeInfo {
    /// ノード値の型名 (`Person`)。ユーザー宣言型への参照。
    type_ident: Ident,
    /// スキーマ内限定で既定生成するID、または `(id: 型パス)` で指定された既存型。
    id_ty: PublicIdType,
    /// 内部ストレージのフィールド名 (`__graphite_node_person`)。
    field_ident: Ident,
    /// builder のノード追加メソッド名 = 単数形 snake_case (`person`)。
    accessor_ident: Ident,
}

impl NodeInfo {
    fn new(decl: &NodeDecl) -> Self {
        let type_name = decl.name.to_string();
        let span = decl.name.span();
        let field_ident = node_storage_ident(&decl.name);
        NodeInfo {
            type_ident: decl.name.clone(),
            id_ty: PublicIdType {
                generated_ident: generated_id_ident(&decl.name),
                explicit_path: decl.id_ty.clone(),
            },
            field_ident,
            accessor_ident: Ident::new(&to_snake_case(&type_name), span),
        }
    }

    fn dup_variant(&self) -> Ident {
        format_ident!("Duplicate{}", self.type_ident)
    }

    fn internal_position_ident(&self) -> Ident {
        internal_position_ident(&self.type_ident)
    }

    fn reference_ident(&self) -> Ident {
        reference_ident(&self.type_ident)
    }

    fn named_position_ident(&self) -> Ident {
        named_position_ident(&self.type_ident)
    }
}

/// エッジ宣言 1 つ分の、生成コードで使う識別子一式。
///
/// `from_node`/`to_node` は `node_infos` (呼び出し元 `generate` のローカル変数)
/// への参照であり、両者の借用が同じ関数スコープに収まるよう単一のライフタイム
/// パラメータで表現する。無向辺では `from_node`/`to_node` は常に同一の
/// `NodeInfo` (両端同型、`schema_validate::validate_undirected_same_type` で
/// 検証済み) を指す。
struct EdgeInfo<'a> {
    kind: &'a Ident,
    /// エッジ種別の newtype キー型名 (`BossId`)。
    id_ty: PublicIdType,
    /// 内部ストレージのフィールド名 = builder 追加メソッド名 = 単数形
    /// snake_case (`boss`)。`Kind` は既に PascalCase (型名) なので
    /// ノードと同じ `to_snake_case` 変換で導出できる。
    accessor_ident: Ident,
    /// 位置0キー -> その位置0からの (有向: 出る / 無向: 接続する) エッジキー
    /// 一覧の内部フィールド名。凍結時に構築する (`docs/schema_v4.md`
    /// §3.2)。有向辺は始点を表す `{accessor}_from_index`、無向辺は方向の
    /// 意味を持たないため `{accessor}_index` とする。
    index_field_ident: Ident,
    /// 位置1キー (終点) -> そこへ入るエッジキー一覧の内部フィールド名
    /// (`{accessor}_to_index`)。**有向辺のみ**構造体フィールドとして持つ
    /// (無向辺は `index_field_ident` が既に対称なので不要)。凍結時に
    /// 構築・永続化する (`docs/reverse_query.md`)。終点役割クエリの索引であり、
    /// v4.1 で入次数 each 検証のためだけに一時構築していた索引を
    /// これに統合した。
    to_index_field_ident: Ident,
    from_node: &'a NodeInfo,
    to_node: &'a NodeInfo,
    shape: EdgeInfoShape,
    /// 端点の役割名ごとの多重度。両端へ独立に指定できる。
    each: Vec<EdgeEach>,
    unique_pair: bool,
}

enum EdgeInfoShape {
    Directed {
        from_role: Ident,
        to_role: Ident,
        payload: Option<EdgePayload>,
    },
    Undirected {
        payload: Option<EdgePayload>,
    },
}

struct EdgeEach {
    role: Ident,
    spec: EachSpec,
    side: EachSide,
}

impl<'a> EdgeInfo<'a> {
    fn payload(&self) -> Option<&EdgePayload> {
        match &self.shape {
            EdgeInfoShape::Directed { payload, .. } | EdgeInfoShape::Undirected { payload } => {
                payload.as_ref()
            }
        }
    }

    fn duplicate_key_variant(&self) -> Ident {
        format_ident!("{}DuplicateKey", self.kind)
    }
    fn unknown_source_variant(&self) -> Ident {
        format_ident!("{}UnknownSource", self.kind, span = self.kind.span())
    }
    fn unknown_target_variant(&self) -> Ident {
        format_ident!("{}UnknownTarget", self.kind, span = self.kind.span())
    }
    /// 無向辺用: 位置の区別が無いため未知端点は1種類の variant で足りる。
    fn unknown_endpoint_variant(&self) -> Ident {
        format_ident!("{}UnknownEndpoint", self.kind, span = self.kind.span())
    }

    fn each_for(&self, side: EachSide) -> Option<&EdgeEach> {
        self.each.iter().find(|constraint| constraint.side == side)
    }
    fn unique_pair_violation_variant(&self) -> Ident {
        format_ident!("{}UniquePairViolation", self.kind, span = self.kind.span())
    }

    fn internal_position_ident(&self) -> Ident {
        internal_position_ident(self.kind)
    }

    fn reference_ident(&self) -> Ident {
        reference_ident(self.kind)
    }

    fn record_ident(&self) -> Ident {
        edge_record_ident(self.kind)
    }

    fn named_position_ident(&self) -> Ident {
        named_position_ident(self.kind)
    }
}

pub fn generate_module_body(schema: &SchemaInput) -> TokenStream {
    let schema_name = &schema.schema_name;
    let graph_ident = graph_type_ident(schema_name);
    let violation_ident = format_ident!("Violation", span = schema_name.span());
    let builder_ident = format_ident!("Builder", span = schema_name.span());
    // `graph!` が値の型名を一切知らずに済むようにするための、ノード挿入用
    // トレイト。名前は schema ごとにユニークにする
    // (`gen_node_trait_and_impls` のドキュメントコメント参照)。
    let node_trait_ident = format_ident!("{}Node", schema_name);
    // 同じ理由でエッジ挿入用にも生やす (書き込み側専用。読み取り側は
    // 各エッジ種別型への固有 impl なのでトレイトを介さない)。
    let edge_trait_ident = format_ident!("{}Edge", schema_name);
    // ノード用/エッジ用の挿入 trait を単一の `extend` に橋渡しするための
    // 共通 supertrait (`gen_insertable_trait` のドキュメントコメント参照、
    // `docs/graph_splice.md` §2)。
    let insertable_trait_ident = format_ident!("{}Insertable", schema_name);
    let default_id_trait_ident = format_ident!("{}DefaultId", schema_name);

    let node_infos: Vec<NodeInfo> = schema.nodes.iter().map(NodeInfo::new).collect();

    let edge_infos: Vec<EdgeInfo> = schema
        .edges
        .iter()
        .map(|edge| build_edge_info(edge, &node_infos))
        .collect();

    let default_id_defs = gen_default_id_types(&node_infos, &edge_infos);
    let internal_position_defs = gen_internal_position_types(&node_infos, &edge_infos);
    let named_position_defs = gen_named_position_types(&node_infos, &edge_infos);
    let edge_value_struct_defs = gen_edge_value_structs(&edge_infos);
    let edge_record_defs = gen_edge_record_structs(&edge_infos);
    let edge_reference_defs = gen_edge_reference_types(&graph_ident, &edge_infos);
    let violation_def = gen_violation_enum(&violation_ident, &node_infos, &edge_infos);
    let schema_struct_def = gen_schema_struct(&graph_ident, &node_infos, &edge_infos);
    let schema_impl = gen_schema_impl(
        &graph_ident,
        &violation_ident,
        &builder_ident,
        &node_infos,
        &edge_infos,
    );
    let builder_struct_def = gen_builder_struct(&builder_ident, &node_infos, &edge_infos);
    let builder_impl = gen_builder_impl(
        &builder_ident,
        &violation_ident,
        &node_trait_ident,
        &edge_trait_ident,
        &default_id_trait_ident,
        &graph_ident,
        &node_infos,
        &edge_infos,
    );
    let insertable_trait_def = gen_insertable_traits(
        &insertable_trait_ident,
        &default_id_trait_ident,
        &builder_ident,
    );
    let node_trait_and_impls = gen_node_trait_and_impls(
        &node_trait_ident,
        &insertable_trait_ident,
        &default_id_trait_ident,
        &builder_ident,
        &graph_ident,
        &node_infos,
        &edge_infos,
    );
    let edge_trait_and_impls = gen_edge_trait_and_impls(
        &edge_trait_ident,
        &insertable_trait_ident,
        &default_id_trait_ident,
        &builder_ident,
        &graph_ident,
        &edge_infos,
    );
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

pub fn generate(schema: &SchemaInput) -> TokenStream {
    let schema_name = &schema.schema_name;
    let body = generate_module_body(schema);
    quote! {
        #[allow(non_snake_case)]
        pub mod #schema_name {
            use super::*;
            #body
        }
    }
}

fn build_edge_info<'a>(decl: &'a EdgeDecl, node_infos: &'a [NodeInfo]) -> EdgeInfo<'a> {
    let (from_type, to_type, shape) = match &decl.shape {
        EdgeShape::Directed { from, to, payload } => (
            &from.ty,
            &to.ty,
            EdgeInfoShape::Directed {
                from_role: from.role.clone(),
                to_role: to.role.clone(),
                payload: payload.clone(),
            },
        ),
        EdgeShape::Undirected {
            first,
            second,
            payload,
        } => (
            first,
            second,
            EdgeInfoShape::Undirected {
                payload: payload.clone(),
            },
        ),
    };
    let from_node = node_infos
        .iter()
        .find(|n| n.type_ident == *from_type)
        .expect("validate() を通過していれば必ず見つかるはず");
    let to_node = node_infos
        .iter()
        .find(|n| n.type_ident == *to_type)
        .expect("validate() を通過していれば必ず見つかるはず");
    let kind = &decl.kind;
    let span = kind.span();
    let accessor_ident = Ident::new(&to_snake_case(&kind.to_string()), span);
    // 有向辺は始点側と終点側の2索引を持つため、位置を名前へ明示する。
    let index_field_ident = match &decl.shape {
        EdgeShape::Directed { .. } => format_ident!("{}_from_index", accessor_ident),
        EdgeShape::Undirected { .. } => format_ident!("{}_index", accessor_ident),
    };
    // 無向辺では使わないが、無条件に計算しておいて差し支えない (単なる
    // Ident の合成であり、無向辺では単に参照されないだけ)。
    let to_index_field_ident = format_ident!("{}_to_index", accessor_ident);
    let each = decl
        .constraints
        .each
        .iter()
        .map(|constraint| EdgeEach {
            role: constraint.role.clone(),
            spec: constraint.spec,
            side: schema_validate::resolve_each_side(decl, &constraint.role)
                .expect("validate_each_reference() を通過していれば必ず解決できるはず"),
        })
        .collect();
    EdgeInfo {
        kind,
        id_ty: PublicIdType {
            generated_ident: generated_id_ident(kind),
            explicit_path: decl.id_ty.clone(),
        },
        accessor_ident,
        index_field_ident,
        to_index_field_ident,
        from_node,
        to_node,
        shape,
        each,
        unique_pair: decl.constraints.unique_pair,
    }
}

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
fn gen_insertable_traits(
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
        /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
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
/// 端点対検索は位置0側 (有向辺は始点側、無向辺は唯一の端点型) の `NodeRef`
/// にだけ生やす。両端が同じノード型の辺でも、生成を位置0側の一致だけで
/// 判定するため1回しか生成されない。
fn gen_node_traversal_methods(node: &NodeInfo, edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    let node_type = &node.type_ident;
    edges
        .iter()
        .flat_map(|edge| {
            let mut methods = Vec::new();
            match &edge.shape {
                EdgeInfoShape::Directed {
                    from_role, to_role, ..
                } => {
                    if edge.from_node.type_ident == *node_type {
                        methods.push(gen_role_traversal_method(edge, from_role, EachSide::Source));
                    }
                    if edge.to_node.type_ident == *node_type {
                        methods.push(gen_role_traversal_method(edge, to_role, EachSide::Target));
                    }
                }
                EdgeInfoShape::Undirected { .. } => {
                    if edge.from_node.type_ident == *node_type {
                        methods.push(gen_incident_traversal_method(edge));
                    }
                }
            }
            if edge.from_node.type_ident == *node_type {
                methods.push(gen_between_traversal_methods(edge));
            }
            methods
        })
        .collect()
}

/// 有向辺の役割探索メソッド `{kind}_as_{役割}()` を1つ生成する。
///
/// 凍結時に構築済みの役割索引を内部位置で引くだけなので O(1)、追加確保なし。
/// 戻り型は問い合わせた役割そのものの `each` 制約で決まる
/// (`docs/schema_v4.md` §3.2)。
fn gen_role_traversal_method(edge: &EdgeInfo<'_>, role: &Ident, side: EachSide) -> TokenStream {
    let method = traversal_method_ident(edge.kind, role);
    let edge_reference = edge.reference_ident();
    let index = match side {
        EachSide::Source => &edge.index_field_ident,
        EachSide::Target => &edge.to_index_field_ident,
    };
    match RoleCardinality::classify(edge.each_for(side).map(|each| each.spec)) {
        RoleCardinality::Exact => quote! {
            /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
            pub fn #method(self) -> #edge_reference<'graph> {
                #edge_reference {
                    graph: self.graph,
                    internal_position: *self.graph.#index.get(self.internal_position.0),
                }
            }
        },
        RoleCardinality::Optional => quote! {
            /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
            pub fn #method(self) -> Option<#edge_reference<'graph>> {
                self.graph.#index.get(self.internal_position.0).copied()
                    .map(|internal_position| #edge_reference { graph: self.graph, internal_position })
            }
        },
        RoleCardinality::Multiple => quote! {
            /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
            /// 問い合わせ時に結果 `Vec` を確保しない。
            pub fn #method(self) -> impl Iterator<Item = #edge_reference<'graph>> + 'graph {
                let positions = self.graph.#index.get(self.internal_position.0);
                positions.iter().copied().map(move |internal_position| #edge_reference {
                    graph: self.graph,
                    internal_position,
                })
            }
        },
    }
}

/// 無向辺の接続探索メソッド `{kind}_incident()` を生成する。
fn gen_incident_traversal_method(edge: &EdgeInfo<'_>) -> TokenStream {
    let method = incident_method_ident(edge.kind);
    let edge_reference = edge.reference_ident();
    let index = &edge.index_field_ident;
    quote! {
        /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
        pub fn #method(self) -> impl Iterator<Item = #edge_reference<'graph>> + 'graph {
            let positions = self.graph.#index.get(self.internal_position.0);
            positions.iter().copied().map(move |internal_position| #edge_reference {
                graph: self.graph,
                internal_position,
            })
        }
    }
}

/// 端点対検索 (`{kind}_between` / `{kind}_try_between`) の生成で、有向辺と
/// 無向辺で異なる部分だけを束ねる。有向辺は端点対索引のキーが
/// `(位置, 位置)` のタプル、無向辺は `UnorderedPair::new(位置, 位置)` になる
/// (`gen_directed_edge_freeze_block`/`gen_undirected_edge_freeze_block` が
/// 積む索引のキー型に合わせる)。
struct EdgeQueryPairSpec {
    /// 相手側端点の参照型 (有向辺は終点側、無向辺は位置0側と同じ型)。
    other_reference: Ident,
    /// 端点対索引 (`{accessor}_by_pair`) を検索するキー式。
    pair_key: TokenStream,
    /// `try_between` の doc コメントに書く対の種類 (`順序付き`/`順序なし`)。
    pair_order_description: &'static str,
}

impl EdgeQueryPairSpec {
    fn from_edge(edge: &EdgeInfo<'_>) -> Self {
        match &edge.shape {
            EdgeInfoShape::Directed { .. } => EdgeQueryPairSpec {
                other_reference: edge.to_node.reference_ident(),
                pair_key: quote! { (self.internal_position, other.internal_position) },
                pair_order_description: "順序付き",
            },
            EdgeInfoShape::Undirected { .. } => EdgeQueryPairSpec {
                other_reference: edge.from_node.reference_ident(),
                pair_key: quote! {
                    graphite::UnorderedPair::new(self.internal_position, other.internal_position)
                },
                pair_order_description: "順序なし",
            },
        }
    }
}

/// 位置0側 `NodeRef` へ端点対検索 `{kind}_try_between` / `{kind}_between` を
/// 生成する。
///
/// `try_between` は2つの参照が同じ `Graph` から得られたかを構築印で照合し、
/// 異なれば [`graphite::GraphMismatch`] を返す。照合は受け手と相手の2者だけを
/// 突き合わせる (一方が有効なら他方も同じ `Graph` に属することが決まるため、
/// 3者目の照合は要らない)。
fn gen_between_traversal_methods(edge: &EdgeInfo<'_>) -> TokenStream {
    let EdgeQueryPairSpec {
        other_reference,
        pair_key,
        pair_order_description,
    } = EdgeQueryPairSpec::from_edge(edge);
    let accessor = &edge.accessor_ident;
    let try_between = kind_api_method_ident(accessor, "try_between");
    let between = kind_api_method_ident(accessor, "between");
    let node_reference = edge.from_node.reference_ident();
    let edge_reference = edge.reference_ident();
    let stamp = construction_stamp_field_ident(edge.kind.span());
    let pair_index = pair_index_field_ident(edge.kind);
    let between_result = if edge.unique_pair {
        quote! { Option<#edge_reference<'graph>> }
    } else {
        quote! { impl Iterator<Item = #edge_reference<'graph>> + 'graph }
    };
    let between_body = if edge.unique_pair {
        quote! {
            let found = self.graph.#pair_index.get(&#pair_key).copied();
            Ok(found.map(|internal_position| #edge_reference { graph: self.graph, internal_position }))
        }
    } else {
        quote! {
            let positions = self.graph.#pair_index.get(&#pair_key)
                .map(Vec::as_slice).unwrap_or(&[]);
            Ok(positions.iter().copied().map(move |internal_position| #edge_reference {
                graph: self.graph,
                internal_position,
            }))
        }
    };
    let try_between_doc =
        format!("{pair_order_description}端点対を平均 O(1)、追加確保なしで検索する。");
    let between_avoid_panic_doc =
        format!("パニックを避けたい場合は対の [`Self::{try_between}`] を使う。");
    quote! {
        #[doc = #try_between_doc]
        pub fn #try_between(self, other: #other_reference<'graph>)
            -> Result<#between_result, graphite::GraphMismatch>
        {
            if self.graph.#stamp != other.graph.#stamp { return Err(graphite::GraphMismatch); }
            #between_body
        }

        /// # Panics
        /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
        #[doc = #between_avoid_panic_doc]
        pub fn #between(self, other: #other_reference<'graph>) -> #between_result {
            self.#try_between(other).unwrap_or_else(|error| {
                panic!("{}::{}: {error}", stringify!(#node_reference), stringify!(#between))
            })
        }
    }
}

/// ノードと辺に共通する名前付き挿入・名前付き位置の束縛実装を生成する。
struct InsertableNamedSpec<'a> {
    insertable_trait_ident: &'a Ident,
    builder_ident: &'a Ident,
    graph_ident: &'a Ident,
    value_type: TokenStream,
    id_ty: &'a PublicIdType,
    named_position: &'a Ident,
    internal_position: &'a Ident,
    storage: &'a Ident,
    accessor: &'a Ident,
    reference: &'a Ident,
    stamp_field: &'a Ident,
    span: proc_macro2::Span,
}

fn gen_insertable_and_named_impl(spec: InsertableNamedSpec<'_>) -> TokenStream {
    let InsertableNamedSpec {
        insertable_trait_ident,
        builder_ident,
        graph_ident,
        value_type,
        id_ty,
        named_position,
        internal_position,
        storage,
        accessor,
        reference,
        stamp_field,
        span,
    } = spec;
    let insert_named_with_id = Ident::new("insert_named_with_id", span);
    let insert_with_id = Ident::new("insert_with_id", span);
    quote! {
        impl #insertable_trait_ident for #value_type {
            type Id = #id_ty;
            type NamedPosition = #named_position;

            fn #insert_named_with_id(
                self,
                b: &mut #builder_ident,
                id: Self::Id,
                _permit: &graphite::NamedInsertPermit,
            ) -> (Self::Id, Self::NamedPosition) {
                let named_position =
                    #named_position(#internal_position(b.#storage.len()), b.#stamp_field);
                let returned_id = id.clone();
                b.#accessor(id, self);
                (returned_id, named_position)
            }

            fn #insert_with_id(self, b: &mut #builder_ident, id: Self::Id) -> Self::Id {
                let returned_id = id.clone();
                b.#accessor(id, self);
                returned_id
            }
        }

        impl graphite::NamedGraphElement<#graph_ident> for #named_position {
            type Reference<'graph> = #reference<'graph>;

            fn bind<'graph>(&self, graph: &'graph #graph_ident) -> Self::Reference<'graph> {
                if graph.#stamp_field != self.1 {
                    panic!("名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です");
                }
                #reference { graph, internal_position: self.0 }
            }
        }
    }
}

/// v4 (`docs/schema_v4.md` §3.2) が要求する「ノード挿入用トレイト」
/// とその各ノード型への impl、およびノード種別ごとの `NodeRef` 型を生成する。
///
/// ## 背景: なぜこのトレイトが必要か
///
/// `graph!` はノード項を `key = 式` と書かせ、値の型をマクロが一切パース
/// しない (式の型は rustc の型推論に委ねる、という設計上の決定)。その結果
/// `graph!` はもはや「どのビルダーメソッドを呼ぶべきか」を型名から逆引き
/// できないため、値の型さえ分かれば正しい内部ストレージへ振り分けられる
/// **総称メソッド**が要る。この trait 境界を介した単相化がそれを実現する
/// (実行時のリフレクション・型判別・`dyn` ディスパッチは一切無い。
/// `docs/design_principles.md` 原則5: ゼロコスト志向)。
///
/// ## 読み取り側をここへ置かない理由
///
/// 公開IDからの検索と種別全体への操作 (`{node}_by_id`/`{node}_ids`/
/// `{node}_iter`/`{node}_len`/`{node}_value_mut`) は、個体と索引を所有する
/// `Graph` のメソッドとして `gen_schema_impl` が生成する。ノード型
/// (`Person` 等) はユーザーが `graph_schema!` の外で宣言する型であり複数
/// schema 間で共有されうるため、ユーザー struct への固有 impl は追加しない。
/// schema module 内にノード名の空 struct (読み取り用マーカー) も置かない
/// (issue #9: `Graph` を外から引数で渡す型名前空間を作らない)。
///
/// ## `{Schema}Insertable` と `{Schema}DefaultId`
///
/// 型付き挿入と関連型 `Id` は `{Schema}Insertable` に置く。文字列の束縛名から
/// IDを作る操作は自動生成IDだけが実装する `{Schema}DefaultId` に置く。
/// `{Schema}Node` はノード専用の型境界を保つマーカートレイトである。
///
/// ## 命名判断 (`docs/design_principles.md` 原則3: std 命名規約準拠)
///
/// **内部 trait 名は `{Schema}Node` とした**。生成 module に移した後も
/// `node Node;` や `edge Edge = ..;` と生成基盤名が衝突する可能性を増やさず、
/// コンパイラ診断から所属 schema を判別できる名前を維持する。
fn gen_node_trait_and_impls(
    node_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
    builder_ident: &Ident,
    graph_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_impls = nodes.iter().map(|n| {
        let ty = &n.type_ident;
        let id_ty = &n.id_ty;
        let accessor = &n.accessor_ident;
        let field = &n.field_ident;
        let reference = n.reference_ident();
        let internal_position = n.internal_position_ident();
        let named_position = n.named_position_ident();
        let stamp_field = construction_stamp_field_ident(ty.span());
        // IDE 支援 (`docs/ide_support_spec.md` §1.9, G3 ポリシー): このノード
        // 型への `{Schema}Node`/`{Schema}Insertable` impl が生やすメソッド名は
        // `n.type_ident` (ノード型そのもののトークン) のスパンを持たせる。
        // トレイト定義自体 (下の `pub trait #node_trait_ident { .. }`) は
        // 単一の由来トークンを持たない schema 全体のインフラなので call_site
        // のままでよい (指示どおり、impl 側だけに適用する)。
        let span = ty.span();
        let node_ref_id_ident = Ident::new("id", span);
        let node_ref_value_ident = Ident::new("value", span);
        let node_debug_impl = gen_reference_debug_impl(&reference, n.id_ty.is_generated());
        let traversal_methods = gen_node_traversal_methods(n, edges);
        let common_impl = gen_insertable_and_named_impl(InsertableNamedSpec {
            insertable_trait_ident,
            builder_ident,
            graph_ident,
            value_type: quote! { super::#ty },
            id_ty: &n.id_ty,
            named_position: &named_position,
            internal_position: &internal_position,
            storage: field,
            accessor,
            reference: &reference,
            stamp_field: &stamp_field,
            span,
        });
        let default_id_impl = if n.id_ty.is_generated() {
            let generated_id = &n.id_ty.generated_ident;
            quote! {
                impl #default_id_trait_ident for super::#ty {
                    fn insert_named_with_binding(
                        self,
                        b: &mut #builder_ident,
                        binding: String,
                        permit: &graphite::NamedInsertPermit,
                    ) -> (Self::Id, Self::NamedPosition) {
                        #insertable_trait_ident::insert_named_with_id(
                            self,
                            b,
                            #generated_id(binding),
                            permit,
                        )
                    }

                    fn insert_with_binding(self, b: &mut #builder_ident, binding: String) -> Self::Id {
                        #insertable_trait_ident::insert_with_id(self, b, #generated_id(binding))
                    }
                }
            }
        } else {
            quote! {}
        };
        let reference_doc = format!("完成済みグラフ上の `{ty}` ノード個体。");
        quote! {
            #common_impl

            #default_id_impl
            impl #node_trait_ident for super::#ty {}

            #[doc = #reference_doc]
            #[derive(Clone, Copy)]
            pub struct #reference<'graph> {
                graph: &'graph #graph_ident,
                internal_position: #internal_position,
            }

            impl<'graph> #reference<'graph> {
                pub fn #node_ref_id_ident(self) -> &'graph #id_ty {
                    self.graph.#field
                        .get_at(self.internal_position.0)
                        .expect("NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)")
                        .0
                }

                pub fn #node_ref_value_ident(self) -> &'graph super::#ty {
                    self.graph.#field
                        .get_at(self.internal_position.0)
                        .expect("NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)")
                        .1
                }

                #(#traversal_methods)*
            }

            impl<'graph> std::ops::Deref for #reference<'graph> {
                type Target = super::#ty;

                fn deref(&self) -> &Self::Target {
                    self.graph.#field
                        .get_at(self.internal_position.0)
                        .expect("NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)")
                        .1
                }
            }

            #node_debug_impl
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

/// エッジ挿入用トレイト (書き込み側専用)。`graph!` の辺行
/// `key = Kind(from -> to)` は名前付きフィールドの辺値型を関連コンストラクタで
/// 構築したあと、この trait 境界を介した総称 `{Builder}::add` に脱糖する
/// (`docs/schema_v4.md` §2/§3.2)。読み取り側は `Graph` の種別メソッド
/// (`{kind}_by_id`/`{kind}_iter`/`{kind}_ids`/`{kind}_len`、`gen_schema_impl`
/// 参照) と `NodeRef` のメソッド (`{kind}_as_{役割}`/`{kind}_incident`/
/// `{kind}_between`、`gen_node_traversal_methods` 参照) が提供するため、
/// このトレイトには含めない。
///
/// 型付き挿入と関連型 `Id` は `{Schema}Insertable` に集約する。このトレイトは
/// エッジ専用の型境界を保つマーカーになる。
fn gen_edge_trait_and_impls(
    edge_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
    builder_ident: &Ident,
    graph_ident: &Ident,
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let edge_impls = edges.iter().map(|e| {
        let kind = e.kind;
        let accessor = &e.accessor_ident;
        let reference = e.reference_ident();
        let internal_position = e.internal_position_ident();
        let named_position = e.named_position_ident();
        let stamp_field = construction_stamp_field_ident(kind.span());
        // 必須ではないが (このメソッドはユーザーが直接呼ぶ想定ではない)、
        // 他の生成メソッドとの一貫性のため `edge.kind` のスパンを付ける
        // (`docs/ide_support_spec.md` §1.9 の指示: 余裕があれば付けてよい)。
        let common_impl = gen_insertable_and_named_impl(InsertableNamedSpec {
            insertable_trait_ident,
            builder_ident,
            graph_ident,
            value_type: quote! { #kind },
            id_ty: &e.id_ty,
            named_position: &named_position,
            internal_position: &internal_position,
            storage: accessor,
            accessor,
            reference: &reference,
            stamp_field: &stamp_field,
            span: kind.span(),
        });
        let default_id_impl = if e.id_ty.is_generated() {
            let generated_id = &e.id_ty.generated_ident;
            quote! {
                impl #default_id_trait_ident for #kind {
                    fn insert_named_with_binding(
                        self,
                        b: &mut #builder_ident,
                        binding: String,
                        permit: &graphite::NamedInsertPermit,
                    ) -> (Self::Id, Self::NamedPosition) {
                        #insertable_trait_ident::insert_named_with_id(
                            self,
                            b,
                            #generated_id(binding),
                            permit,
                        )
                    }

                    fn insert_with_binding(self, b: &mut #builder_ident, binding: String) -> Self::Id {
                        #insertable_trait_ident::insert_with_id(self, b, #generated_id(binding))
                    }
                }
            }
        } else {
            quote! {}
        };
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

/// 明示ID型がないノード・エッジのスキーマ内限定の型付き文字列IDを生成する。
fn gen_default_id_types(nodes: &[NodeInfo], edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(|node| &node.id_ty)
        .chain(edges.iter().map(|edge| &edge.id_ty))
        .filter(|id_ty| id_ty.is_generated())
        .map(|id_ty| {
            let ident = &id_ty.generated_ident;
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #ident(pub String);
            }
        })
        .collect()
}

/// 公開IDとは別に、凍結済みグラフ内の内部位置を表す非公開型を生成する。
/// 種別ごとのnewtypeにすることで、別のノード表・辺表の位置を取り違えない。
fn gen_internal_position_types(nodes: &[NodeInfo], edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(NodeInfo::internal_position_ident)
        .chain(edges.iter().map(EdgeInfo::internal_position_ident))
        .map(|position| {
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                struct #position(usize);
            }
        })
        .collect()
}

/// `graph!` の名前付きラッパーへ凍結をまたいで内部位置を運ぶ型を生成する。
/// フィールドは非公開で、生成された挿入経路と `NamedGraphElement` 実装だけが
/// 構築・参照する。公開IDや `Graph` への参照は保持しない。
///
/// 第2要素は構築印 (`u64`)。挿入時にその場の `Builder` が持つ構築印を
/// そのまま埋め込み、`NamedGraphElement::bind` が `Graph` 側の構築印と
/// 照合する (`crates/graphite/src/lib.rs` の構築印発行関数を参照)。
fn gen_named_position_types(nodes: &[NodeInfo], edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(|node| {
            let named_position = node.named_position_ident();
            let internal_position = node.internal_position_ident();
            quote! {
                #[doc(hidden)]
                #[derive(Clone, Copy)]
                pub struct #named_position(#internal_position, u64);
            }
        })
        .chain(edges.iter().map(|edge| {
            let named_position = edge.named_position_ident();
            let internal_position = edge.internal_position_ident();
            quote! {
                #[doc(hidden)]
                #[derive(Clone, Copy)]
                pub struct #named_position(#internal_position, u64);
            }
        }))
        .collect()
}

/// 辺レコード構造体・辺参照値の積み荷フィールド `role: 型` を生成する
/// (積み荷が無ければ空)。有向/無向で生成コードが同一なため
/// `gen_edge_record_structs` から共有する純粋関数。
fn edge_record_payload_fields(payload: &Option<EdgePayload>) -> Vec<TokenStream> {
    payload
        .iter()
        .map(|payload| {
            let role = &payload.role;
            let ty = &payload.ty;
            quote! { #role: #ty }
        })
        .collect()
}

/// 辺参照値の積み荷アクセサ (役割名メソッドと `payload()` エイリアス) を
/// 生成する (積み荷が無ければ空)。有向/無向で生成コードが同一なため
/// `gen_edge_reference_types` から共有する純粋関数。`payload()` のスパンは
/// 辺種別トークンを継承する (`docs/ide_support_spec.md` §1.9)。
fn edge_reference_payload_methods(kind: &Ident, payload: &Option<EdgePayload>) -> TokenStream {
    let payload_ident = Ident::new("payload", kind.span());
    let methods = payload.iter().map(|payload| {
        let role = &payload.role;
        let ty = &payload.ty;
        quote! {
            pub fn #role(self) -> &'graph #ty {
                &self.record().#role
            }

            pub fn #payload_ident(self) -> &'graph #ty {
                &self.record().#role
            }
        }
    });
    quote! { #(#methods)* }
}

/// 辺参照値の共通メソッド (内部レコードの取得、`id()`) を生成する。
/// 有向/無向のどちらの `impl<'graph> #reference<'graph> { .. }` 本体からも
/// 同形で使うため共有する。`id()` のスパンは辺種別トークンを継承する。
fn edge_reference_core_methods(
    accessor: &Ident,
    record: &Ident,
    id_ty: &PublicIdType,
    kind_span: proc_macro2::Span,
) -> TokenStream {
    let id_ident = Ident::new("id", kind_span);
    quote! {
        fn record(self) -> &'graph #record {
            self.graph.#accessor
                .get_at(self.internal_position.0)
                .expect("EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)")
                .1
        }

        pub fn #id_ident(self) -> &'graph #id_ty {
            self.graph.#accessor
                .get_at(self.internal_position.0)
                .expect("EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)")
                .0
        }
    }
}

/// `NodeRef`/`EdgeRef` の `Debug` impl を生成する。`&Graph` は表示しない。
///
/// ID型・値型 (辺の場合は積み荷) に `Debug` を無条件要求しない契約
/// (`gen_edge_value_structs` の同種の契約と対) を守る必要がある。当初は
/// `where #id_ty: std::fmt::Debug` のような条件付き `impl` を試みたが、
/// `#reference<'graph>` はライフタイムのみが型引数でID型・値型はmacro展開時
/// に確定した具体型であるため、その `where` 節はジェネリック型引数を介した
/// 遅延検査にはならず **定義時に即座に充足性が検査される** ことを実測で
/// 確認した (2026-08-25、`cargo build --workspace --all-targets` で
/// 利用者定義の非Debug型を使う既存テストが軒並みコンパイルエラーになった)。
/// そのため `gen_edge_value_structs` の debug_impl と同じ方針
/// (macro展開時に安全と判定できる範囲だけを表示する無条件 `impl`) を採る。
/// 安全と判定できるのは自動生成ID型 (`gen_default_id_types` が常に
/// `#[derive(Debug, ..)]` を付ける) の場合のみで、値型・積み荷型は利用者
/// 定義でありmacroからは判定できないため表示対象に含めない。
fn gen_reference_debug_impl(reference: &Ident, id_is_generated: bool) -> TokenStream {
    let body = if id_is_generated {
        quote! {
            f.debug_struct(stringify!(#reference))
                .field("id", &self.id())
                .finish_non_exhaustive()
        }
    } else {
        quote! { f.write_str(stringify!(#reference)) }
    };
    quote! {
        impl<'graph> std::fmt::Debug for #reference<'graph> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #body
            }
        }
    }
}

/// 辺値は構築時の公開IDを保持するが、完成後のレコードは端点を内部位置で
/// 保持する。積み荷だけを辺値から移して保持し、探索時のID検索を不要にする。
fn gen_edge_record_structs(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|edge| {
            let record = edge.record_ident();
            let from_position = edge.from_node.internal_position_ident();
            let to_position = edge.to_node.internal_position_ident();
            match &edge.shape {
                EdgeInfoShape::Directed {
                    from_role,
                    to_role,
                    payload,
                } => {
                    let payload_field = edge_record_payload_fields(payload);
                    quote! {
                        #[allow(dead_code)]
                        struct #record {
                            #from_role: #from_position,
                            #to_role: #to_position,
                            #(#payload_field,)*
                        }
                    }
                }
                EdgeInfoShape::Undirected { payload } => {
                    let payload_field = edge_record_payload_fields(payload);
                    quote! {
                        #[allow(dead_code)]
                        struct #record {
                            endpoints: graphite::UnorderedPair<#from_position>,
                            #(#payload_field,)*
                        }
                    }
                }
            }
        })
        .collect()
}

/// 完成済みグラフ上の辺個体を表す薄い参照値を生成する。端点を返すメソッドは、
/// 保存レコード内の内部位置から NodeRef を直接作り、公開IDの索引を検索しない。
fn gen_edge_reference_types(graph_ident: &Ident, edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|edge| {
            let id_ty = &edge.id_ty;
            let accessor = &edge.accessor_ident;
            let reference = edge.reference_ident();
            let internal_position = edge.internal_position_ident();
            let record = edge.record_ident();
            let kind_span = edge.kind.span();
            let core_methods = edge_reference_core_methods(accessor, &record, id_ty, kind_span);
            match &edge.shape {
                EdgeInfoShape::Directed {
                    from_role,
                    to_role,
                    payload,
                } => {
                    let from_reference = edge.from_node.reference_ident();
                    let to_reference = edge.to_node.reference_ident();
                    let from_position = edge.from_node.internal_position_ident();
                    let to_position = edge.to_node.internal_position_ident();
                    let from_id = &edge.from_node.id_ty;
                    let to_id = &edge.to_node.id_ty;
                    let payload_methods = edge_reference_payload_methods(edge.kind, payload);
                    let from_ident = Ident::new("from", kind_span);
                    let to_ident = Ident::new("to", kind_span);
                    let from_id_ident = Ident::new("from_id", kind_span);
                    let to_id_ident = Ident::new("to_id", kind_span);
                    let debug_impl = gen_reference_debug_impl(&reference, edge.id_ty.is_generated());
                    quote! {
                        /// 完成済みグラフ上の有向辺個体。
                        #[derive(Clone, Copy)]
                        pub struct #reference<'graph> {
                            graph: &'graph #graph_ident,
                            internal_position: #internal_position,
                        }

                        impl<'graph> #reference<'graph> {
                            #core_methods

                            pub fn #from_role(self) -> #from_reference<'graph> {
                                #from_reference {
                                    graph: self.graph,
                                    internal_position: #from_position(self.record().#from_role.0),
                                }
                            }

                            pub fn #to_role(self) -> #to_reference<'graph> {
                                #to_reference {
                                    graph: self.graph,
                                    internal_position: #to_position(self.record().#to_role.0),
                                }
                            }

                            pub fn #from_ident(self) -> #from_reference<'graph> {
                                self.#from_role()
                            }

                            pub fn #to_ident(self) -> #to_reference<'graph> {
                                self.#to_role()
                            }

                            pub fn #from_id_ident(self) -> &'graph #from_id {
                                self.from().id()
                            }

                            pub fn #to_id_ident(self) -> &'graph #to_id {
                                self.to().id()
                            }

                            #payload_methods
                        }

                        #debug_impl
                    }
                }
                EdgeInfoShape::Undirected { payload } => {
                    let node_reference = edge.from_node.reference_ident();
                    let node_position = edge.from_node.internal_position_ident();
                    let payload_methods = edge_reference_payload_methods(edge.kind, payload);
                    let endpoints_ident = Ident::new("endpoints", kind_span);
                    let debug_impl = gen_reference_debug_impl(&reference, edge.id_ty.is_generated());
                    quote! {
                        /// 完成済みグラフ上の無向辺個体。
                        #[derive(Clone, Copy)]
                        pub struct #reference<'graph> {
                            graph: &'graph #graph_ident,
                            internal_position: #internal_position,
                        }

                        impl<'graph> #reference<'graph> {
                            #core_methods

                            pub fn #endpoints_ident(self) -> (#node_reference<'graph>, #node_reference<'graph>) {
                                let (first, second) = self.record().endpoints.endpoints();
                                (
                                    #node_reference {
                                        graph: self.graph,
                                        internal_position: #node_position(first.0),
                                    },
                                    #node_reference {
                                        graph: self.graph,
                                        internal_position: #node_position(second.0),
                                    },
                                )
                            }

                            #payload_methods
                        }

                        #debug_impl
                    }
                }
            }
        })
        .collect()
}

/// 辺種別ごとの公開名前付きフィールド値型を生成する。有向辺の端点と積み荷の
/// フィールド名はスキーマの役割名をそのまま使う。無向辺は順序なし対を
/// `endpoints` フィールドへ保持する。いずれもグラフを所有・借用しない普通のRust値。
fn gen_edge_value_structs(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|e| {
            let kind = e.kind;
            let p0_id = &e.from_node.id_ty;
            let p1_id = &e.to_node.id_ty;

            let (struct_def, constructor, literal_impl, debug_endpoints) = match &e.shape {
                EdgeInfoShape::Directed {
                    from_role,
                    to_role,
                    payload: None,
                } => (
                        quote! {
                            pub struct #kind {
                                pub #from_role: #p0_id,
                                pub #to_role: #p1_id,
                            }
                        },
                        quote! {
                            pub fn new(from: #p0_id, to: #p1_id) -> Self {
                                Self { #from_role: from, #to_role: to }
                            }
                        },
                        quote! {
                            impl graphite::DirectedEdgeLiteral<#p0_id, #p1_id, ()> for #kind {
                                fn from_graph_literal(from: #p0_id, to: #p1_id, (): ()) -> Self {
                                    Self::new(from, to)
                                }
                            }
                        },
                        (quote! { self.#from_role }, quote! { self.#to_role }),
                    ),
                EdgeInfoShape::Directed {
                    from_role,
                    to_role,
                    payload: Some(payload),
                } => {
                    let payload_role = &payload.role;
                    let attrs = &payload.ty;
                    (
                        quote! {
                            pub struct #kind {
                                pub #from_role: #p0_id,
                                pub #to_role: #p1_id,
                                pub #payload_role: #attrs,
                            }
                        },
                        quote! {
                            pub fn new(from: #p0_id, to: #p1_id, payload: #attrs) -> Self {
                                Self {
                                    #from_role: from,
                                    #to_role: to,
                                    #payload_role: payload,
                                }
                            }
                            pub fn payload(&self) -> &#attrs { &self.#payload_role }
                        },
                        quote! {
                            impl graphite::DirectedEdgeLiteral<#p0_id, #p1_id, #attrs> for #kind {
                                fn from_graph_literal(
                                    from: #p0_id,
                                    to: #p1_id,
                                    payload: #attrs,
                                ) -> Self {
                                    Self::new(from, to, payload)
                                }
                            }
                        },
                        (quote! { self.#from_role }, quote! { self.#to_role }),
                    )
                }
                EdgeInfoShape::Undirected { payload: None } => (
                        quote! { pub struct #kind { endpoints: graphite::UnorderedPair<#p0_id> } },
                        quote! {
                            pub fn new(a: #p0_id, b: #p1_id) -> Self {
                                Self { endpoints: graphite::UnorderedPair::new(a, b) }
                            }
                            pub fn endpoints(&self) -> (&#p0_id, &#p1_id) {
                                self.endpoints.endpoints()
                            }
                        },
                        quote! {
                            impl graphite::UndirectedEdgeLiteral<#p0_id, ()> for #kind {
                                fn from_graph_literal(a: #p0_id, b: #p0_id, (): ()) -> Self {
                                    Self::new(a, b)
                                }
                            }
                        },
                        (quote! { self.endpoints().0 }, quote! { self.endpoints().1 }),
                    ),
                EdgeInfoShape::Undirected {
                    payload: Some(payload),
                } => {
                    let payload_role = &payload.role;
                    let attrs = &payload.ty;
                    (
                        quote! {
                            pub struct #kind {
                                endpoints: graphite::UnorderedPair<#p0_id>,
                                pub #payload_role: #attrs,
                            }
                        },
                        quote! {
                            pub fn new(a: #p0_id, b: #p1_id, payload: #attrs) -> Self {
                                Self { endpoints: graphite::UnorderedPair::new(a, b), #payload_role: payload }
                            }
                            pub fn endpoints(&self) -> (&#p0_id, &#p1_id) {
                                self.endpoints.endpoints()
                            }
                            pub fn payload(&self) -> &#attrs { &self.#payload_role }
                        },
                        quote! {
                            impl graphite::UndirectedEdgeLiteral<#p0_id, #attrs> for #kind {
                                fn from_graph_literal(
                                    a: #p0_id,
                                    b: #p0_id,
                                    payload: #attrs,
                                ) -> Self {
                                    Self::new(a, b, payload)
                                }
                            }
                        },
                        (quote! { self.endpoints().0 }, quote! { self.endpoints().1 }),
                    )
                }
            };

            // 利用者定義IDと積み荷へDebugを要求しない契約を守るため、端点を
            // 表示できるのは両端が自動生成IDで積み荷がない場合に限る。
            let debug_impl = if e.payload().is_none()
                && e.from_node.id_ty.is_generated()
                && e.to_node.id_ty.is_generated()
            {
                let (first, second) = debug_endpoints;
                quote! {
                    impl std::fmt::Debug for #kind {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.debug_tuple(stringify!(#kind))
                                .field(&#first)
                                .field(&#second)
                                .finish()
                        }
                    }
                }
            } else {
                quote! {
                    impl std::fmt::Debug for #kind {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.write_str(stringify!(#kind))
                        }
                    }
                }
            };

            quote! {
                #[derive(Clone, PartialEq)]
                #struct_def

                impl #kind {
                    #constructor
                }

                #literal_impl
                #debug_impl
            }
        })
        .collect()
}

/// 違反 enum を生成する。
///
/// - ノード重複 (`Duplicate{Node}`) は v3 から維持。
/// - 辺キー重複 (`{Kind}DuplicateKey`) は v4 で新規追加 (辺も第一級キーを
///   持つため)。
/// - 未知の端点参照: 有向は `{Kind}UnknownSource`/`{Kind}UnknownTarget`
///   (どの辺がどちらの端点で未知キーを参照したかを型付きで持つ)、無向は
///   位置の区別が無いため `{Kind}UnknownEndpoint` 1種類。
/// - `each` 制約違反 (`{Kind}{Role}EachViolation`) は解決された側
///   (出次数/入次数) に応じて `source` または `target` を持つ。
/// - `unique pair` 違反 (`{Kind}UniquePairViolation`) は有向なら
///   `source`/`target`、無向なら順序の意味が無いため `a`/`b`。
fn gen_violation_enum(
    violation_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let dup_variants = nodes.iter().map(|n| {
        let v = n.dup_variant();
        let id = &n.id_ty;
        quote! { #v(#id) }
    });
    let dup_display_arms = nodes.iter().map(|n| {
        let v = n.dup_variant();
        let type_name_str = n.type_ident.to_string();
        if n.id_ty.is_generated() {
            quote! {
                #violation_ident::#v(id) => write!(f, "{}のキーが重複しています: {:?}", #type_name_str, id)
            }
        } else {
            quote! {
                #violation_ident::#v(_) => write!(f, "{}のキーが重複しています", #type_name_str)
            }
        }
    });

    let mut edge_variants: Vec<TokenStream> = Vec::new();
    let mut edge_display_arms: Vec<TokenStream> = Vec::new();

    for edge in edges {
        let kind_str = edge.kind.to_string();
        let edge_id = &edge.id_ty;

        let dup_key = edge.duplicate_key_variant();
        edge_variants.push(quote! {
            /// このエッジ種別のキーが重複している。
            #dup_key(#edge_id)
        });
        edge_display_arms.push(if edge.id_ty.is_generated() {
            quote! {
                #violation_ident::#dup_key(id) => write!(f, "{}のキーが重複しています: {:?}", #kind_str, id)
            }
        } else {
            quote! {
                #violation_ident::#dup_key(_) => write!(f, "{}のキーが重複しています", #kind_str)
            }
        });

        if matches!(&edge.shape, EdgeInfoShape::Directed { .. }) {
            let from_id = &edge.from_node.id_ty;
            let to_id = &edge.to_node.id_ty;
            let from_type_str = edge.from_node.type_ident.to_string();
            let to_type_str = edge.to_node.type_ident.to_string();

            let unk_src = edge.unknown_source_variant();
            edge_variants.push(quote! {
                /// このエッジが未知の始点キーを参照している。
                #unk_src { edge: #edge_id, source: #from_id }
            });
            edge_display_arms.push(
                if edge.id_ty.is_generated() && edge.from_node.id_ty.is_generated() {
                    quote! {
                        #violation_ident::#unk_src { edge, source } => write!(
                            f,
                            "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                            #kind_str, edge, #from_type_str, source
                        )
                    }
                } else {
                    quote! {
                        #violation_ident::#unk_src { .. } => write!(
                            f,
                            "未知のキーが参照されています (辺 `{}` の始点, {})",
                            #kind_str, #from_type_str
                        )
                    }
                },
            );

            let unk_dst = edge.unknown_target_variant();
            edge_variants.push(quote! {
                /// このエッジが未知の終点キーを参照している。
                #unk_dst { edge: #edge_id, target: #to_id }
            });
            edge_display_arms.push(
                if edge.id_ty.is_generated() && edge.to_node.id_ty.is_generated() {
                    quote! {
                        #violation_ident::#unk_dst { edge, target } => write!(
                            f,
                            "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                            #kind_str, edge, #to_type_str, target
                        )
                    }
                } else {
                    quote! {
                        #violation_ident::#unk_dst { .. } => write!(
                            f,
                            "未知のキーが参照されています (辺 `{}` の終点, {})",
                            #kind_str, #to_type_str
                        )
                    }
                },
            );

            for constraint in &edge.each {
                let spec = constraint.spec;
                let expected_str = match spec.max() {
                    Some(max) if spec.min() == max => format!("ちょうど{}", spec.min()),
                    Some(max) => format!("{}..{}", spec.min(), max),
                    None => format!("{}..*", spec.min()),
                };
                let v = each_violation_ident(edge.kind, &constraint.role);
                match constraint.side {
                    EachSide::Source => {
                        edge_variants.push(quote! {
                            /// このエッジ種別の `each` 制約違反 (出次数)。
                            #v { source: #from_id, count: usize }
                        });
                        edge_display_arms.push(if edge.from_node.id_ty.is_generated() {
                            quote! {
                                #violation_ident::#v { source, count } => write!(
                                    f,
                                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                                    #kind_str, #from_type_str, source, #expected_str, count
                                )
                            }
                        } else {
                            quote! {
                                #violation_ident::#v { count, .. } => write!(
                                    f,
                                    "多重度制約違反: 辺 `{}` は {} の出次数 {} を期待しますが実際は {} 本です",
                                    #kind_str, #from_type_str, #expected_str, count
                                )
                            }
                        });
                    }
                    EachSide::Target => {
                        edge_variants.push(quote! {
                            /// このエッジ種別の `each` 制約違反 (入次数)。
                            #v { target: #to_id, count: usize }
                        });
                        edge_display_arms.push(if edge.to_node.id_ty.is_generated() {
                            quote! {
                                #violation_ident::#v { target, count } => write!(
                                    f,
                                    "多重度制約違反: 辺 `{}` は {} {:?} について入次数 {} を期待しますが実際は {} 本です",
                                    #kind_str, #to_type_str, target, #expected_str, count
                                )
                            }
                        } else {
                            quote! {
                                #violation_ident::#v { count, .. } => write!(
                                    f,
                                    "多重度制約違反: 辺 `{}` は {} の入次数 {} を期待しますが実際は {} 本です",
                                    #kind_str, #to_type_str, #expected_str, count
                                )
                            }
                        });
                    }
                }
            }

            if edge.unique_pair {
                let v = edge.unique_pair_violation_variant();
                edge_variants.push(quote! {
                    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
                    /// 2本目の辺が張られた)。
                    #v { source: #from_id, target: #to_id }
                });
                edge_display_arms.push(
                    if edge.from_node.id_ty.is_generated() && edge.to_node.id_ty.is_generated() {
                        quote! {
                            #violation_ident::#v { source, target } => write!(
                                f,
                                "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                                #kind_str, source, target
                            )
                        }
                    } else {
                        quote! {
                            #violation_ident::#v { .. } => write!(
                                f,
                                "unique pair違反: 辺 `{}` の同じ始点・終点の対に既に辺が存在します",
                                #kind_str
                            )
                        }
                    },
                );
            }
        } else {
            // 無向辺: 両端は同じノード型 (validate 済み) なので from_node で代表する。
            let node_id = &edge.from_node.id_ty;
            let node_type_str = edge.from_node.type_ident.to_string();

            let unk = edge.unknown_endpoint_variant();
            edge_variants.push(quote! {
                /// このエッジが未知の端点キーを参照している (無向のため位置の
                /// 区別は無い)。
                #unk { edge: #edge_id, endpoint: #node_id }
            });
            edge_display_arms.push(
                if edge.id_ty.is_generated() && edge.from_node.id_ty.is_generated() {
                    quote! {
                        #violation_ident::#unk { edge, endpoint } => write!(
                            f,
                            "未知のキーが参照されています (辺 `{}` {:?} の端点, {}): {:?}",
                            #kind_str, edge, #node_type_str, endpoint
                        )
                    }
                } else {
                    quote! {
                        #violation_ident::#unk { .. } => write!(
                            f,
                            "未知のキーが参照されています (辺 `{}` の端点, {})",
                            #kind_str, #node_type_str
                        )
                    }
                },
            );

            if edge.unique_pair {
                let v = edge.unique_pair_violation_variant();
                edge_variants.push(quote! {
                    /// このエッジ種別の `unique pair` 違反 (無向のため
                    /// 順序を無視した対で判定)。
                    #v { a: #node_id, b: #node_id }
                });
                edge_display_arms.push(if edge.from_node.id_ty.is_generated() {
                    quote! {
                        #violation_ident::#v { a, b } => write!(
                            f,
                            "unique pair違反: 辺 `{}` は {{{:?}, {:?}}} の対に既に辺が存在します",
                            #kind_str, a, b
                        )
                    }
                } else {
                    quote! {
                        #violation_ident::#v { .. } => write!(
                            f,
                            "unique pair違反: 辺 `{}` の同じ端点対に既に辺が存在します",
                            #kind_str
                        )
                    }
                });
            }
        }
    }

    quote! {
        #[allow(clippy::enum_variant_names)]
        #[derive(Clone, PartialEq, Eq)]
        pub enum #violation_ident {
            #(#dup_variants,)*
            #(#edge_variants,)*
        }

        impl std::fmt::Display for #violation_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#dup_display_arms,)*
                    #(#edge_display_arms,)*
                }
            }
        }

        impl std::fmt::Debug for #violation_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }

        impl std::error::Error for #violation_ident {}
    }
}

fn gen_schema_struct(
    schema_name: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let stamp_field = construction_stamp_field_ident(schema_name.span());
    let node_fields = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let id = &n.id_ty;
        let ty = &n.type_ident;
        quote! { #field: graphite::KeyedTable<#id, super::#ty> }
    });
    let edge_fields = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let index_field = &e.index_field_ident;
        let id_ty = &e.id_ty;
        let record = e.record_ident();
        let edge_position = e.internal_position_ident();
        // 索引のキー型は位置0の内部位置型 (有向なら始点、無向なら両端同型)。
        let key_position = e.from_node.internal_position_ident();
        // 有向辺のみ終点索引を永続化する (`docs/reverse_query.md`)。
        // 終点役割クエリの索引であり、v4.1 で入次数 each 検証のためだけに
        // 一時構築していた索引をこれに統合した (無向辺は `index_field` が
        // 既に対称に両端を積むので不要)。
        let pair_index = pair_index_field_ident(e.kind);
        let pair_value = if e.unique_pair {
            quote! { #edge_position }
        } else {
            quote! { Vec<#edge_position> }
        };
        let to_index_decl = if matches!(&e.shape, EdgeInfoShape::Directed { .. }) {
            let to_index_field = &e.to_index_field_ident;
            let to_key_position = e.to_node.internal_position_ident();
            let to_index_ty = role_index_type(e, EachSide::Target, &edge_position);
            quote! {
                ,
                /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
                /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
                #to_index_field: #to_index_ty,
                #pair_index: std::collections::HashMap<(#key_position, #to_key_position), #pair_value>
            }
        } else {
            quote! {
                ,
                #pair_index: std::collections::HashMap<graphite::UnorderedPair<#key_position>, #pair_value>
            }
        };
        let index_ty = if matches!(&e.shape, EdgeInfoShape::Directed { .. }) {
            role_index_type(e, EachSide::Source, &edge_position)
        } else {
            quote! { graphite::MultipleRoleIndex<#edge_position> }
        };
        quote! {
            #accessor: graphite::KeyedTable<#id_ty, #record>,
            /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
            /// キーの一覧 (凍結時に構築)。
            #index_field: #index_ty
            #to_index_decl
        }
    });

    quote! {
        /// 凍結済み図式グラフ。構築後の構造は不変で、ノード値と辺の積み荷だけを
        /// `&mut Graph` を要求する種別APIから更新できる。
        pub struct #schema_name {
            #(#node_fields,)*
            #(#edge_fields,)*
            /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
            /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
            /// するかを `NamedGraphElement::bind` が照合するのに使う。
            #stamp_field: u64,
        }
    }
}

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
fn gen_schema_impl(
    schema_name: &Ident,
    violation_ident: &Ident,
    builder_ident: &Ident,
    nodes: &[NodeInfo],
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

/// ノード種別1つ分の種別API (`Graph` のメソッド) を生成する。
///
/// IDE 支援 (`docs/ide_support_spec.md` §1.9, G3 ポリシー) のため、生成する
/// メソッド名にはノード型そのもののトークンのスパンを持たせる
/// (`accessor_ident` がノード型トークンのスパンを引き継いでいる)。
fn gen_node_kind_api_methods(node: &NodeInfo) -> TokenStream {
    let ty = &node.type_ident;
    let id_ty = &node.id_ty;
    let field = &node.field_ident;
    let accessor = &node.accessor_ident;
    let reference = node.reference_ident();
    let internal_position = node.internal_position_ident();
    let by_id = kind_api_method_ident(accessor, "by_id");
    let value_mut = kind_api_method_ident(accessor, "value_mut");
    let ids = kind_api_method_ident(accessor, "ids");
    let iter = kind_api_method_ident(accessor, "iter");
    let len = kind_api_method_ident(accessor, "len");
    quote! {
        /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
        pub fn #by_id<'graph>(&'graph self, id: &#id_ty) -> Option<#reference<'graph>> {
            let internal_position = #internal_position(self.#field.position(id)?);
            Some(#reference { graph: self, internal_position })
        }

        /// グラフの構造を保ったままノード値だけを可変借用する。
        pub fn #value_mut(&mut self, id: &#id_ty) -> Option<&mut super::#ty> {
            self.#field.get_mut(id)
        }

        /// この種別のノードの公開IDを挿入順に走査する。
        pub fn #ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph #id_ty> {
            self.#field.ids()
        }

        /// この種別のノード個体を挿入順に走査する。追加確保はしない。
        pub fn #iter<'graph>(&'graph self) -> impl Iterator<Item = #reference<'graph>> + 'graph {
            self.#field.positions().map(move |position| #reference {
                graph: self,
                internal_position: #internal_position(position),
            })
        }

        /// この種別のノードの件数を返す。
        pub fn #len(&self) -> usize {
            self.#field.len()
        }
    }
}

/// 辺種別1つ分の種別API (`Graph` のメソッド) を生成する。
/// `{kind}_payload_mut` は積み荷を持つ辺種別にだけ生やす。
fn gen_edge_kind_api_methods(edge: &EdgeInfo<'_>) -> TokenStream {
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let reference = edge.reference_ident();
    let internal_position = edge.internal_position_ident();
    let by_id = kind_api_method_ident(accessor, "by_id");
    let ids = kind_api_method_ident(accessor, "ids");
    let iter = kind_api_method_ident(accessor, "iter");
    let len = kind_api_method_ident(accessor, "len");
    let payload_mut = gen_edge_payload_mut_method(edge);
    quote! {
        /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
        pub fn #by_id<'graph>(&'graph self, id: &#id_ty) -> Option<#reference<'graph>> {
            Some(#reference {
                graph: self,
                internal_position: #internal_position(self.#accessor.position(id)?),
            })
        }

        #payload_mut

        /// この種別の辺の公開IDを挿入順に走査する。
        pub fn #ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph #id_ty> {
            self.#accessor.ids()
        }

        /// この種別の辺個体を挿入順に走査する。追加確保はしない。
        pub fn #iter<'graph>(&'graph self) -> impl Iterator<Item = #reference<'graph>> + 'graph {
            self.#accessor.positions().map(move |position| #reference {
                graph: self,
                internal_position: #internal_position(position),
            })
        }

        /// この種別の辺の件数を返す。
        pub fn #len(&self) -> usize {
            self.#accessor.len()
        }
    }
}

fn gen_builder_struct(
    builder_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let stamp_field = construction_stamp_field_ident(builder_ident.span());
    let node_fields = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let id = &n.id_ty;
        let ty = &n.type_ident;
        quote! { #field: Vec<(#id, super::#ty)> }
    });
    let edge_fields = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let id_ty = &e.id_ty;
        let kind = e.kind;
        quote! { #accessor: Vec<(#id_ty, #kind)> }
    });

    quote! {
        /// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
        pub struct #builder_ident {
            #(#node_fields,)*
            #(#edge_fields,)*
            /// この構築を識別する構築印。`Builder::new()` が発行し、この
            /// builder から挿入する全ての名前付き位置と、凍結成功後の
            /// `Graph` へ同じ値を刻む。
            #stamp_field: u64,
        }
    }
}

// 生成する構築器の型名群とスキーマ情報を一か所で受け取るため引数が多い。
#[allow(clippy::too_many_arguments)]
fn gen_builder_impl(
    builder_ident: &Ident,
    violation_ident: &Ident,
    node_trait_ident: &Ident,
    edge_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
    schema_name: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_field_inits = nodes.iter().map(|n| {
        let field = &n.field_ident;
        quote! { #field: Vec::new() }
    });
    let edge_field_inits = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        quote! { #accessor: Vec::new() }
    });

    let node_methods = nodes.iter().map(|n| {
        let accessor = &n.accessor_ident;
        let field = &n.field_ident;
        let id_ty = &n.id_ty;
        let ty = &n.type_ident;
        quote! {
            pub fn #accessor(&mut self, id: #id_ty, value: super::#ty) -> &mut Self {
                self.#field.push((id, value));
                self
            }
        }
    });

    let edge_methods = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let id_ty = &e.id_ty;
        let kind = e.kind;
        quote! {
            pub fn #accessor(&mut self, id: #id_ty, value: #kind) -> &mut Self {
                self.#accessor.push((id, value));
                self
            }
        }
    });

    let freeze_body = gen_freeze_body(schema_name, violation_ident, nodes, edges);
    let stamp_field = construction_stamp_field_ident(builder_ident.span());

    quote! {
        impl #builder_ident {
            fn new() -> Self {
                Self {
                    #(#node_field_inits,)*
                    #(#edge_field_inits,)*
                    #stamp_field: graphite::次の構築印を発行する(),
                }
            }

            #(#node_methods)*
            #(#edge_methods)*

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
            /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
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
            /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
            #[doc(hidden)]
            pub fn insert_named_with_id<N: #node_trait_ident>(
                &mut self,
                id: N::Id,
                value: N,
                permit: &graphite::NamedInsertPermit,
            ) -> (N::Id, N::NamedPosition) {
                value.insert_named_with_id(self, id, permit)
            }

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

            #freeze_body
        }

        /// [`graphite::build_named_graph`] が `#schema_name`/`#violation_ident`
        /// の具体型を知らずに凍結を呼べるようにするための橋渡し。
        /// `freeze_into_graph` は既存の私有 `freeze()` (上記) へそのまま委譲する。
        impl graphite::FreezableBuilder for #builder_ident {
            type Graph = #schema_name;
            type Violation = #violation_ident;

            fn freeze_into_graph(self) -> Result<Self::Graph, Self::Violation> {
                self.freeze()
            }
        }
    }
}

/// `where each <参照名>: ..` の IDE 支援専用ゼロコスト検査文
/// (`docs/ide_support_spec.md` §1.9)。
///
/// `<参照名>` は名前付きフィールドの辺値型の役割名フィールドへ参照させる。
fn gen_each_type_check(edge: &EdgeInfo<'_>) -> TokenStream {
    let kind = edge.kind;
    let checks = edge.each.iter().map(|constraint| {
        let role = &constraint.role;
        quote! {
            let _: fn(&#kind) = |edge| {
                let _ = &edge.#role;
            };
        }
    });
    quote! { #(#checks)* }
}

/// 有向辺1種別分の凍結検査本体を生成する。
///
/// 手順:
/// 1. `Vec<(KindId, Kind)>` から `KeyedTable<KindId, Kind>` を構築 (重複キー
///    は `{Kind}DuplicateKey` 違反として記録し、その要素は捨てる)。
/// 2. 生き残った各辺について端点 (位置0/1) がそれぞれのノード表に実在するか
///    検査する (`{Kind}UnknownSource`/`{Kind}UnknownTarget`)。両端点とも
///    正当な辺だけを位置0索引 (`{accessor}_from_index`) と位置1索引
///    (`{accessor}_to_index`) の両方に積む。後者は `docs/reverse_query.md`
///    により構造体フィールドとして永続化する (終点役割クエリが使う。
///    v4.1 で入次数 each 検証のためだけに一時構築していた索引をこれに統合)。
///    `unique pair` 制約があれば、同じ (位置0, 位置1) の対が2回目に現れた
///    時点で `{Kind}UniquePairViolation` を記録する。
/// 3. `each` 制約があれば、`each_side` に応じて出次数 (位置0索引) または
///    入次数 (位置1索引、手順2で作った永続化済みのものをそのまま使う) を
///    検査する。
fn gen_directed_edge_freeze_block(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
    from_role: &Ident,
    to_role: &Ident,
) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let storage = edge_storage_ident(accessor);
    let record = edge.record_ident();
    let edge_position = edge.internal_position_ident();
    let from_position_type = edge.from_node.internal_position_ident();
    let to_position_type = edge.to_node.internal_position_ident();
    let from_index = &edge.index_field_ident;
    let to_index = &edge.to_index_field_ident;
    let from_field = &edge.from_node.field_ident;
    let to_field = &edge.to_node.field_ident;
    let dup_key = edge.duplicate_key_variant();
    let unk_src = edge.unknown_source_variant();
    let unk_dst = edge.unknown_target_variant();
    let kind = edge.kind;
    let pair_index = pair_index_field_ident(kind);
    let pair_index_type = if edge.unique_pair {
        quote! { std::collections::HashMap<(#from_position_type, #to_position_type), #edge_position> }
    } else {
        quote! { std::collections::HashMap<(#from_position_type, #to_position_type), Vec<#edge_position>> }
    };

    let (destructure_value, build_record) = match &edge.shape {
        EdgeInfoShape::Directed {
            payload: Some(payload),
            ..
        } => {
            let payload_role = &payload.role;
            (
                quote! {
                    let #kind { #from_role: from, #to_role: to, #payload_role } = value;
                },
                quote! {
                    #record { #from_role: from_position, #to_role: to_position, #payload_role }
                },
            )
        }
        EdgeInfoShape::Directed { payload: None, .. } => (
            quote! {
                let #kind { #from_role: from, #to_role: to } = value;
            },
            quote! {
                #record { #from_role: from_position, #to_role: to_position }
            },
        ),
        EdgeInfoShape::Undirected { .. } => unreachable!("有向辺の生成関数には有向辺だけを渡す"),
    };

    let (unique_pair_check, pair_insert) = if edge.unique_pair {
        let v = edge.unique_pair_violation_variant();
        (
            quote! {
                if #pair_index.contains_key(&(from_position, to_position)) {
                    __violations.push(#violation_ident::#v {
                        source: from.clone(),
                        target: to.clone(),
                    });
                }
            },
            quote! { #pair_index.insert((from_position, to_position), internal_edge_position); },
        )
    } else {
        (
            quote! {},
            quote! { #pair_index.entry((from_position, to_position)).or_default().push(internal_edge_position); },
        )
    };

    let each_type_check = gen_each_type_check(edge);
    let each_checks = edge.each.iter().map(|constraint| {
        let min = constraint.spec.min();
        let invalid_count = match constraint.spec.max() {
            Some(max) if min == max => quote! { count != #min },
            Some(max) => quote! { !(#min..=#max).contains(&count) },
            None => quote! { count < #min },
        };
        let v = each_violation_ident(edge.kind, &constraint.role);
        match constraint.side {
            EachSide::Source => quote! {
                for position in #from_field.positions() {
                    let internal_position = #from_position_type(position);
                    let key = #from_field.get_at(position).expect("列挙した内部位置はノード表に存在する").0;
                    let count = #from_index.get(&internal_position).map(Vec::len).unwrap_or(0);
                    if #invalid_count {
                        __violations.push(#violation_ident::#v { source: key.clone(), count });
                    }
                }
            },
            EachSide::Target => quote! {
                for position in #to_field.positions() {
                    let internal_position = #to_position_type(position);
                    let key = #to_field.get_at(position).expect("列挙した内部位置はノード表に存在する").0;
                    let count = #to_index.get(&internal_position).map(Vec::len).unwrap_or(0);
                    if #invalid_count {
                        __violations.push(#violation_ident::#v { target: key.clone(), count });
                    }
                }
            },
        }
    });

    quote! {
        let mut #storage: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut #from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #pair_index: #pair_index_type = std::collections::HashMap::new();
        for (id, value) in self.#accessor {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(#violation_ident::#dup_key(id));
                continue;
            }
            #destructure_value
            let from_position = #from_field.position(&from).map(#from_position_type);
            let to_position = #to_field.position(&to).map(#to_position_type);
            if from_position.is_none() {
                __violations.push(#violation_ident::#unk_src { edge: id.clone(), source: from.clone() });
            }
            if to_position.is_none() {
                __violations.push(#violation_ident::#unk_dst { edge: id.clone(), target: to.clone() });
            }
            if let (Some(from_position), Some(to_position)) = (from_position, to_position) {
                #unique_pair_check
                let internal_edge_position = #edge_position(#storage.len());
                #pair_insert
                #from_index.entry(from_position).or_default().push(internal_edge_position);
                #to_index.entry(to_position).or_default().push(internal_edge_position);
                let inserted = #storage.insert(id, #build_record);
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        #each_type_check
        #(#each_checks)*
    }
}

fn finalize_role_index(
    edge: &EdgeInfo<'_>,
    side: EachSide,
    index: &Ident,
    node_field: &Ident,
    node_position: &Ident,
) -> TokenStream {
    let constructor = match RoleCardinality::classify(edge.each_for(side).map(|each| each.spec)) {
        RoleCardinality::Exact => quote! { graphite::ExactlyOneRoleIndex::from_buckets },
        RoleCardinality::Optional => quote! { graphite::OptionalRoleIndex::from_buckets },
        RoleCardinality::Multiple => quote! { graphite::MultipleRoleIndex::from_buckets },
    };
    quote! {
        let #index = #constructor(
            (0..#node_field.len())
                .map(|position| #index.remove(&#node_position(position)).unwrap_or_default())
                .collect()
        );
    }
}

/// 無向辺1種別分の凍結検査本体を生成する
/// (`docs/edge_endpoints_v4_1.md` §2)。
///
/// 位置0/1索引 (`{accessor}_index`) は「その位置0キーに (有向の from_index
/// と同じ形で) 接続するエッジキーの一覧」だが、無向のため対称に構築する:
/// 位置0・位置1のどちらにも (自己ループなら1回だけ) 積む。これにより
/// - `{kind}_incident`/`{kind}_between` はどちらの位置に置かれてもこの索引から検索できる。
/// - 格納順 (挿入順) は `KeyedTable` の `iter()` の走査順そのままなので、索引の
///   `push` もその順で行われ、`docs/edge_endpoints_v4_1.md` §2 の
///   「挿入順保持」がそのまま満たされる。
fn gen_undirected_edge_freeze_block(violation_ident: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let storage = edge_storage_ident(accessor);
    let record = edge.record_ident();
    let edge_position = edge.internal_position_ident();
    let node_position_type = edge.from_node.internal_position_ident();
    let index = &edge.index_field_ident;
    let node_field = &edge.from_node.field_ident;
    let dup_key = edge.duplicate_key_variant();
    let unk = edge.unknown_endpoint_variant();
    let kind = edge.kind;
    let pair_index = pair_index_field_ident(kind);
    let pair_index_type = if edge.unique_pair {
        quote! { std::collections::HashMap<graphite::UnorderedPair<#node_position_type>, #edge_position> }
    } else {
        quote! { std::collections::HashMap<graphite::UnorderedPair<#node_position_type>, Vec<#edge_position>> }
    };

    let (destructure_value, build_record) = match &edge.shape {
        EdgeInfoShape::Undirected {
            payload: Some(payload),
        } => {
            let payload_role = &payload.role;
            (
                quote! {
                    let #kind { endpoints, #payload_role } = value;
                },
                quote! {
                    #record {
                        endpoints: graphite::UnorderedPair::new(first_position, second_position),
                        #payload_role,
                    }
                },
            )
        }
        EdgeInfoShape::Undirected { payload: None } => (
            quote! {
                let #kind { endpoints } = value;
            },
            quote! {
                #record {
                    endpoints: graphite::UnorderedPair::new(first_position, second_position),
                }
            },
        ),
        EdgeInfoShape::Directed { .. } => unreachable!("無向辺の生成関数には無向辺だけを渡す"),
    };

    // 無向辺の `unique pair` は `UnorderedPair` に同一性判定を委譲し、
    // ID型へ順序比較を要求せず (p0, p1) と (p1, p0) を同一視する。
    let (unique_pair_check, pair_insert) = if edge.unique_pair {
        let v = edge.unique_pair_violation_variant();
        (
            quote! {
                if #pair_index.contains_key(&graphite::UnorderedPair::new(first_position, second_position)) {
                    __violations.push(#violation_ident::#v {
                        a: p0.clone(),
                        b: p1.clone(),
                    });
                }
            },
            quote! { #pair_index.insert(graphite::UnorderedPair::new(first_position, second_position), internal_edge_position); },
        )
    } else {
        (
            quote! {},
            quote! { #pair_index.entry(graphite::UnorderedPair::new(first_position, second_position)).or_default().push(internal_edge_position); },
        )
    };

    quote! {
        let mut #storage: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut #index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #pair_index: #pair_index_type = std::collections::HashMap::new();
        for (id, value) in self.#accessor {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(#violation_ident::#dup_key(id));
                continue;
            }
            #destructure_value
            let (p0, p1) = endpoints.endpoints();
            let p0 = p0.clone();
            let p1 = p1.clone();
            let first_position = #node_field.position(&p0).map(#node_position_type);
            let second_position = #node_field.position(&p1).map(#node_position_type);
            if first_position.is_none() {
                __violations.push(#violation_ident::#unk { edge: id.clone(), endpoint: p0.clone() });
            }
            if p1 != p0 && second_position.is_none() {
                __violations.push(#violation_ident::#unk { edge: id.clone(), endpoint: p1.clone() });
            }
            if let (Some(first_position), Some(second_position)) = (first_position, second_position) {
                #unique_pair_check
                let internal_edge_position = #edge_position(#storage.len());
                #pair_insert
                #index.entry(first_position).or_default().push(internal_edge_position);
                if second_position != first_position {
                    #index.entry(second_position).or_default().push(internal_edge_position);
                }
                let inserted = #storage.insert(id, #build_record);
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
    }
}

fn gen_freeze_body(
    schema_name: &Ident,
    violation_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_table_builds = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let dup_variant = n.dup_variant();
        quote! {
            let mut #field: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
            for (id, value) in self.#field {
                if !#field.insert(id.clone(), value) {
                    __violations.push(#violation_ident::#dup_variant(id));
                }
            }
        }
    });

    let edge_blocks = edges.iter().map(|e| match &e.shape {
        EdgeInfoShape::Directed {
            from_role, to_role, ..
        } => gen_directed_edge_freeze_block(violation_ident, e, from_role, to_role),
        EdgeInfoShape::Undirected { .. } => gen_undirected_edge_freeze_block(violation_ident, e),
    });

    // 制約違反をすべて収集し終えてから、役割索引を公開クエリ向けの
    // 多重度別表現へ確定する。違反のある未完成索引を ExactlyOneRoleIndex/
    // OptionalRoleIndex 表現へ変換すると内部アサートになり、利用者向け
    // Violation を返せない。
    let edge_index_finalizers = edges.iter().flat_map(|edge| match &edge.shape {
        EdgeInfoShape::Directed { .. } => vec![
            finalize_role_index(
                edge,
                EachSide::Source,
                &edge.index_field_ident,
                &edge.from_node.field_ident,
                &edge.from_node.internal_position_ident(),
            ),
            finalize_role_index(
                edge,
                EachSide::Target,
                &edge.to_index_field_ident,
                &edge.to_node.field_ident,
                &edge.to_node.internal_position_ident(),
            ),
        ],
        EdgeInfoShape::Undirected { .. } => vec![finalize_role_index(
            edge,
            EachSide::Source,
            &edge.index_field_ident,
            &edge.from_node.field_ident,
            &edge.from_node.internal_position_ident(),
        )],
    });

    let node_field_names = nodes.iter().map(|n| &n.field_ident);
    let edge_field_inits = edges.iter().map(|e| {
        let field = &e.accessor_ident;
        let storage = edge_storage_ident(field);
        quote! { #field: #storage }
    });
    // 有向辺は位置0索引 (`{accessor}_from_index`) と位置1索引
    // (`{accessor}_to_index`) の両方をフィールドとして持つ。無向辺は
    // `index_field_ident` (対称な単一索引) のみ (`gen_schema_struct` 参照)。
    let edge_index_names: Vec<Ident> = edges
        .iter()
        .flat_map(|e| {
            let pair = pair_index_field_ident(e.kind);
            match &e.shape {
                EdgeInfoShape::Directed { .. } => {
                    vec![
                        e.index_field_ident.clone(),
                        e.to_index_field_ident.clone(),
                        pair,
                    ]
                }
                EdgeInfoShape::Undirected { .. } => vec![e.index_field_ident.clone(), pair],
            }
        })
        .collect();

    let stamp_field = construction_stamp_field_ident(schema_name.span());

    quote! {
        /// 検証ロジックの実体。最初の1件で打ち切らず全違反を `Vec` に
        /// 集めて返す。`freeze()` (単一エラー版) はこちらに委譲し先頭の1件を
        /// 取り出すだけの薄いラッパーにすることで、検証ロジックが二重実装に
        /// ならないようにしている。
        fn freeze_collecting(self) -> Result<#schema_name, Vec<#violation_ident>> {
            let mut __violations: Vec<#violation_ident> = Vec::new();
            let #stamp_field = self.#stamp_field;

            #(#node_table_builds)*
            #(#edge_blocks)*

            if !__violations.is_empty() {
                return Err(__violations);
            }

            #(#edge_index_finalizers)*

            Ok(#schema_name {
                #(#node_field_names,)*
                #(#edge_field_inits,)*
                #(#edge_index_names,)*
                #stamp_field,
            })
        }

        /// 最初の1件の違反で `Err` になる版。実装は
        /// `freeze_collecting` に委譲する。
        fn freeze(self) -> Result<#schema_name, #violation_ident> {
            self.freeze_collecting().map_err(|mut violations| violations.remove(0))
        }
    }
}

fn role_index_type(edge: &EdgeInfo<'_>, side: EachSide, edge_position: &Ident) -> TokenStream {
    match RoleCardinality::classify(edge.each_for(side).map(|each| each.spec)) {
        RoleCardinality::Exact => quote! { graphite::ExactlyOneRoleIndex<#edge_position> },
        RoleCardinality::Optional => quote! { graphite::OptionalRoleIndex<#edge_position> },
        RoleCardinality::Multiple => quote! { graphite::MultipleRoleIndex<#edge_position> },
    }
}

/// 辺の構造を保ったまま積み荷だけを可変借用する `{kind}_payload_mut` を
/// `Graph` のメソッドとして生成する (積み荷が無ければ空)。
///
/// 主語は `&mut Graph` である。`EdgeRef` は共有借用のハンドルなのでそこから
/// 可変借用は作れず、引数も公開IDのままにする。
fn gen_edge_payload_mut_method(edge: &EdgeInfo<'_>) -> TokenStream {
    let Some(payload) = edge.payload() else {
        return quote! {};
    };
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let record = edge.record_ident();
    let payload_role = &payload.role;
    let payload_ty = &payload.ty;
    let payload_mut = kind_api_method_ident(accessor, "payload_mut");
    quote! {
        /// 辺の構造を保ったまま積み荷だけを可変借用する。
        pub fn #payload_mut(&mut self, id: &#id_ty) -> Option<&mut #payload_ty> {
            self.#accessor.get_mut(id).map(|record: &mut #record| &mut record.#payload_role)
        }
    }
}
