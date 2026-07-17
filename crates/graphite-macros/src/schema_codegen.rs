//! `graph_schema!` のコード生成本体 (v4、`docs/schema_v4.md` §3 参照。
//! v4.1 の役割名・無向辺は `docs/edge_endpoints_v4_1.md` 参照)。
//!
//! ## 生成物の全体像 (1エッジ種別分)
//!
//! `edge Boss = Person -[BossEdge]-> Person where each Person: 0..1;` から:
//!
//! ```text
//! pub struct BossId(pub String);
//! pub struct Boss(pub PersonId, pub PersonId, pub BossEdge);
//! impl Boss {
//!     pub fn from(&self) -> &PersonId { &self.0 }
//!     pub fn to(&self) -> &PersonId { &self.1 }
//!     pub fn payload(&self) -> &BossEdge { &self.2 }
//!
//!     pub fn of(g: &Org, from: &PersonId) -> Option<(&Person, &BossEdge)> { .. }
//!     pub fn get(g: &Org, id: &BossId) -> Option<&Boss> { .. }
//!     pub fn between(g: &Org, from: &PersonId, to: &PersonId) -> Vec<&Boss> { .. }
//!     pub fn iter(g: &Org) -> impl Iterator<Item = (&BossId, &Boss)> { .. }
//!     pub fn ids(g: &Org) -> impl Iterator<Item = &BossId> { .. }
//!     pub fn len(g: &Org) -> usize { .. }
//! }
//! impl OrgEdge for Boss { .. } // 書き込み側 (graph! の総称 add 用)
//! ```
//!
//! v4.1 で役割名つき有向辺 (`(subordinate: Employee) -> (superior: Employee)`)
//! は `.from()`/`.to()` の代わりに `.subordinate()`/`.superior()` を生やす。
//! 無向辺 (`Person -- Person`) は `.from()`/`.to()` の代わりに
//! `.endpoints() -> (&PersonId, &PersonId)` を生やし、`of`/`between` は
//! どちらの位置に置かれても対称に検索できる。内部の freeze/query 実装は
//! いずれもタプル位置 (`.0`/`.1`) を直接使い、公開アクセサ名 (from/to,
//! 役割名, endpoints) とは独立させている。
//!
//! 辺は「マクロが生成する型」なのでノードと異なり**固有 impl (inherent impl)
//! で読み取り API を生やせる** (`docs/schema_v4.md` §3.2「辺 — 種別型
//! (マクロ生成) への固有 impl」)。ノード型はユーザーが `graph_schema!` の外に
//! 宣言する型で複数 schema 間の共有もありうるため、代わりに `{Schema}Node`
//! トレイトの関連関数として生やす (README/`gen_node_trait_and_impls` 参照)。
//!
//! where 制約 → 戻り型の対応表 (`docs/schema_v4.md` §3.2、有向・始点側のみ):
//! - `each X: 1`    -> `of` は直接参照 (未知キーはパニック、非パニック版 `get_of`)
//! - `each X: 0..1` -> `of` は `Option`
//! - 制約なし        -> `of` は `Vec`
//! - `unique pair`  -> `between` は `Option`、それ以外は `Vec`
//!
//! 役割名つきの辺で `each` が終点側 (入次数) を指定した場合、`of` の戻り型は
//! 上記表に従わず常に `Vec` になる (`of` は常に始点側キーで検索するため、
//! 始点側が無制約なら平行辺を許すのが自然)。無向辺の `each` は次数制約であり、
//! `of`/`between` の戻り型は有向の表と同じ規則 (次数制約が Option/直接参照を、
//! 無制約が Vec を、`unique pair` が `between` の戻り型を決める) で決まる。
//!
//! ## 終点側クエリ `{Kind}::sources_of` (`docs/reverse_query.md`)
//!
//! 有向辺には `of` の対称として `sources_of`/`get_sources_of` を生成する
//! (無向辺には生成しない — `of` が既に対称なので同じものになるため)。
//! `sources_of(g, to)` は `to` を終点とする辺の**始点側**(相手ノード値+積み荷)
//! を返す。戻り型は上記表と同じ規則だが、判定に使う制約は **終点側
//! (入次数、`each_side == Target`)** の `each` のみ (役割名つきの辺でしか
//! 起こらない。役割名なしの辺・無向辺は必ず `Vec`)。相手はノード値で返す
//! (キー版は生やさない — `docs/reverse_query.md` の最小方針)。
//!
//! 実装は freeze 時に構築・永続化する終点索引 `{accessor}_to_index`
//! (`ToId -> Vec<KindId>`、`gen_schema_struct`/`gen_directed_edge_freeze_block`
//! 参照) を検索するだけなので O(1) 償却。この索引は v4.1 で入次数 each 検証
//! のためだけに一時構築していたものを構造体フィールドとして格上げ・統合した
//! もの (`docs/reverse_query.md` 実装ノート)。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Path};

use crate::naming::{plural_field_name, to_snake_case};
use crate::schema_dsl::{EachSpec, EdgeDecl, NodeDecl, SchemaInput};
use crate::schema_validate::{self, EachSide};

/// ノード宣言 1 つ分の、生成コードで使う識別子一式。
///
/// ノード値の型 (`Person` 等) はユーザーが `graph_schema!` の外で宣言した
/// 普通の struct への参照であり、このマクロは生成しない。マクロが生成するのは
/// グラフ機械 (newtype キー・ストレージ・builder・アクセサ・違反 enum) だけ。
struct NodeInfo {
    /// ノード値の型名 (`Person`)。ユーザー宣言型への参照。
    type_ident: Ident,
    /// newtype キー型名 (`PersonId`)。
    id_ident: Ident,
    /// 内部ストレージの複数形フィールド名 (`persons`)。
    field_ident: Ident,
    /// builder のノード追加メソッド名 = 単数形 snake_case (`person`)。
    accessor_ident: Ident,
}

impl NodeInfo {
    fn new(decl: &NodeDecl) -> Self {
        let type_name = decl.name.to_string();
        let span = decl.name.span();
        // 内部ストレージのフィールド名は常に素朴な複数形化 (`+ "s"`)。
        // 明示指定構文 (`node Type(plural);`) は v4 で廃止した
        // (`docs/graph_splice.md` §3): このフィールドは利用者から不可視
        // (非公開) なので、不規則複数形 (`Category` → `Categorys`) でも
        // 機能上の問題はない。
        let field_ident = Ident::new(&plural_field_name(&type_name), span);
        NodeInfo {
            type_ident: decl.name.clone(),
            id_ident: format_ident!("{}Id", decl.name, span = span),
            field_ident,
            accessor_ident: Ident::new(&to_snake_case(&type_name), span),
        }
    }

    fn dup_variant(&self) -> Ident {
        format_ident!("Duplicate{}", self.type_ident)
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
    id_ident: Ident,
    /// 内部ストレージのフィールド名 = builder 追加メソッド名 = 単数形
    /// snake_case (`boss`)。`Kind` は既に PascalCase (型名) なので
    /// ノードと同じ `to_snake_case` 変換で導出できる。
    accessor_ident: Ident,
    /// 位置0キー -> その位置0からの (有向: 出る / 無向: 接続する) エッジキー
    /// 一覧の内部フィールド名。freeze 時に構築する (`docs/schema_v4.md`
    /// §3.2)。有向辺は従来どおり `{accessor}_from_index` (既存の手書きコード
    /// `orgchart_macro.rs` がこのフィールド名を直接参照しているため後方互換で
    /// 固定)、無向辺は方向の意味を持たないため `{accessor}_index`。
    index_field_ident: Ident,
    /// 位置1キー (終点) -> そこへ入るエッジキー一覧の内部フィールド名
    /// (`{accessor}_to_index`)。**有向辺のみ**構造体フィールドとして持つ
    /// (無向辺は `index_field_ident` が既に対称なので不要)。freeze 時に
    /// 構築・永続化する (`docs/reverse_query.md`)。`{Kind}::sources_of` の
    /// 索引であり、v4.1 で入次数 each 検証のためだけに一時構築していた索引を
    /// これに統合した。
    to_index_field_ident: Ident,
    from_node: &'a NodeInfo,
    to_node: &'a NodeInfo,
    /// エッジ属性型への参照。ユーザーがマクロの外で宣言した型を指すだけで、
    /// このマクロは属性型そのものを生成しない。
    attrs_ty: Option<Path>,
    /// 有向 (`->`/`-[Attrs]->`) か無向 (`--`/`-[Attrs]-`) か。
    directed: bool,
    /// 役割名 (役割名つき有向辺のみ `Some`)。
    from_role: Option<Ident>,
    to_role: Option<Ident>,
    each: Option<EachSpec>,
    /// `each` 制約がどちら側 (出次数/入次数/次数) を指すか。
    each_side: Option<EachSide>,
    /// `where each <参照名>: ..` の `<参照名>` トークンそのもの
    /// (検証意味論には使わず、IDE 支援専用: `gen_each_type_check` が
    /// このトークンのスパンを使ったゼロコスト検査文を生成することで、
    /// rust-analyzer の定義ジャンプ (F12) がこのトークンから対応するノード型/
    /// アクセサメソッドの宣言へ着地できるようにする。
    /// `docs/ide_support_spec.md` §1.9)。
    each_token: Option<Ident>,
    unique_pair: bool,
}

impl<'a> EdgeInfo<'a> {
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
    fn each_violation_variant(&self) -> Ident {
        format_ident!("{}EachViolation", self.kind, span = self.kind.span())
    }
    fn unique_pair_violation_variant(&self) -> Ident {
        format_ident!("{}UniquePairViolation", self.kind, span = self.kind.span())
    }
}

pub fn generate(schema: &SchemaInput) -> TokenStream {
    let schema_name = &schema.schema_name;
    let violation_ident = format_ident!("{}Violation", schema_name);
    let builder_ident = format_ident!("{}Builder", schema_name);
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

    let node_infos: Vec<NodeInfo> = schema.nodes.iter().map(NodeInfo::new).collect();

    let edge_infos: Vec<EdgeInfo> = schema
        .edges
        .iter()
        .map(|edge| build_edge_info(edge, &node_infos))
        .collect();

    let node_id_defs = gen_node_id_types(&node_infos);
    let edge_id_defs = gen_edge_id_types(&edge_infos);
    let edge_tuple_struct_defs = gen_edge_tuple_structs(&edge_infos);
    let violation_def = gen_violation_enum(&violation_ident, &node_infos, &edge_infos);
    let schema_struct_def = gen_schema_struct(schema_name, &node_infos, &edge_infos);
    let schema_impl = gen_schema_impl(schema_name, &violation_ident, &builder_ident);
    let builder_struct_def = gen_builder_struct(&builder_ident, &node_infos, &edge_infos);
    let builder_impl = gen_builder_impl(
        &builder_ident,
        &violation_ident,
        &node_trait_ident,
        &edge_trait_ident,
        &insertable_trait_ident,
        schema_name,
        &node_infos,
        &edge_infos,
    );
    let insertable_trait_def = gen_insertable_trait(&insertable_trait_ident, &builder_ident);
    let node_trait_and_impls = gen_node_trait_and_impls(
        &node_trait_ident,
        &insertable_trait_ident,
        &builder_ident,
        schema_name,
        &node_infos,
    );
    let edge_trait_and_impls = gen_edge_trait_and_impls(
        &edge_trait_ident,
        &insertable_trait_ident,
        &builder_ident,
        &edge_infos,
    );
    let edge_query_impls = edge_infos.iter().map(|e| gen_edge_query_impl(schema_name, e));

    quote! {
        #(#node_id_defs)*
        #(#edge_id_defs)*
        #(#edge_tuple_struct_defs)*
        #violation_def
        #schema_struct_def
        #schema_impl
        #builder_struct_def
        #insertable_trait_def
        #node_trait_and_impls
        #edge_trait_and_impls
        #builder_impl
        #(#edge_query_impls)*
    }
}

fn build_edge_info<'a>(decl: &'a EdgeDecl, node_infos: &'a [NodeInfo]) -> EdgeInfo<'a> {
    let from_node = node_infos
        .iter()
        .find(|n| n.type_ident == decl.from)
        .expect("validate() を通過していれば必ず見つかるはず");
    let to_node = node_infos
        .iter()
        .find(|n| n.type_ident == decl.to)
        .expect("validate() を通過していれば必ず見つかるはず");
    let kind = &decl.kind;
    let span = kind.span();
    let accessor_ident = Ident::new(&to_snake_case(&kind.to_string()), span);
    // 有向辺の内部索引フィールド名は既存の手書きコード
    // (`crates/graphite/tests/orgchart_macro.rs` の `colleagues()`) が
    // `self.belongs_to_from_index` を直接参照しているため後方互換で固定する。
    let index_field_ident = if decl.directed {
        format_ident!("{}_from_index", accessor_ident)
    } else {
        format_ident!("{}_index", accessor_ident)
    };
    // 無向辺では使わないが、無条件に計算しておいて差し支えない (単なる
    // Ident の合成であり、無向辺では単に参照されないだけ)。
    let to_index_field_ident = format_ident!("{}_to_index", accessor_ident);
    let each_side = decl.constraints.each.as_ref().map(|(ident, _)| {
        schema_validate::resolve_each_side(decl, ident)
            .expect("validate_each_reference() を通過していれば必ず解決できるはず")
    });
    EdgeInfo {
        kind,
        id_ident: format_ident!("{}Id", kind, span = span),
        accessor_ident,
        index_field_ident,
        to_index_field_ident,
        from_node,
        to_node,
        attrs_ty: decl.attrs_ty.clone(),
        directed: decl.directed,
        from_role: decl.from_role.clone(),
        to_role: decl.to_role.clone(),
        each: decl.constraints.each.as_ref().map(|(_, spec)| *spec),
        each_side,
        each_token: decl.constraints.each.as_ref().map(|(ident, _)| ident.clone()),
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
/// 統一できるか? しない」)。この2つの要求を両立させるため、`insert_into`/
/// `Id` を本トレイトに集約し、`{Schema}Node`/`{Schema}Edge` をこの supertrait
/// として再定義する。
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
fn gen_insertable_trait(insertable_trait_ident: &Ident, builder_ident: &Ident) -> TokenStream {
    quote! {
        /// ノード・エッジ共通の挿入トレイト。統一 `extend` (下記
        /// `{Builder}::extend`) の型境界として使う。利用者がこのトレイトの
        /// メソッドを直接呼ぶことは想定しない。
        pub trait #insertable_trait_ident: Sized {
            type Id;
            /// `self` を `b` の対応する内部ストレージへ格納し、発行された
            /// キーを返す。
            fn insert_into(self, b: &mut #builder_ident, key: String) -> Self::Id;
        }
    }
}

/// v4 (`docs/schema_v4.md` §3.2) が要求する「ノード挿入用トレイト」
/// とその各ノード型への impl を生成する。
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
/// ## v4 での拡張: 読み取り側 (`get`/`ids`/`iter`)
///
/// v4 は「ノードは `{Schema}Node` トレイトの関連関数」(`docs/schema_v4.md`
/// §3.2) と明記しており、`Person::get(&g, &id)` のように呼べる形を要求する。
/// ノード型 (`Person` 等) はユーザーが `graph_schema!` の外で宣言する型で
/// あり、複数 schema 間で共有されうるため、ユーザー struct への固有 impl
/// (`impl Person { .. }`) は複数 schema が同名メソッドを生やそうとして
/// 衝突する可能性がある。トレイトの関連関数なら `use {Schema}Node` で
/// スコープに persist させた上で `Type::method` 呼び出しができ、かつ
/// 複数 schema が同じ `Person` に対しそれぞれ別のトレイトを impl できる
/// (トレイトは同名でも schema ごとに別型なので衝突しない)。
///
/// ## v4 での拡張: `{Schema}Insertable` supertrait (`extend` 統一)
///
/// `insert_into`/`Id` は `{Schema}Insertable` (`gen_insertable_trait` 参照) に
/// 移動し、このトレイトはその supertrait として `get`/`ids`/`iter` だけを
/// 追加する。`insert` の型境界は変わらず `N: #node_trait_ident` のまま
/// (ノード専用の型境界を保つ)。ノード型1つにつき impl ブロックが2つ
/// (`{Schema}Insertable` + `{Schema}Node`) になるが、`insert`/`add` の型の
/// 厳密さ (ノード専用/エッジ専用) を保ったまま `extend` の共通境界を得る
/// トレードオフとして採用する。
///
/// ## 命名判断 (`docs/design_principles.md` 原則3: std 命名規約準拠)
///
/// - **trait 名は `{Schema}Node` とした**。理由は README「同一モジュール内で
///   複数 schema がノード型を共有する場合の制約」と同様: schema 名を
///   プレフィックスにすることで、モジュール内に複数 schema があっても
///   トレイト名が衝突しない。
/// - **メソッド名は `insert_into`/`get`/`ids`/`iter`**。`get`/`ids`/`iter` は
///   `docs/schema_v4.md` §3.2 のイメージ通り (std の `HashMap::get`/
///   `HashMap::keys`/`HashMap::iter` に倣った命名)。
fn gen_node_trait_and_impls(
    node_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
    builder_ident: &Ident,
    schema_name: &Ident,
    nodes: &[NodeInfo],
) -> TokenStream {
    let node_impls = nodes.iter().map(|n| {
        let ty = &n.type_ident;
        let id_ty = &n.id_ident;
        let accessor = &n.accessor_ident;
        let field = &n.field_ident;
        // IDE 支援 (`docs/ide_support_spec.md` §1.9, G3 ポリシー): このノード
        // 型への `{Schema}Node`/`{Schema}Insertable` impl が生やすメソッド名は
        // `n.type_ident` (ノード型そのもののトークン) のスパンを持たせる。
        // トレイト定義自体 (下の `pub trait #node_trait_ident { .. }`) は
        // 単一の由来トークンを持たない schema 全体のインフラなので call_site
        // のままでよい (指示どおり、impl 側だけに適用する)。
        let span = ty.span();
        let insert_into_ident = Ident::new("insert_into", span);
        let get_ident = Ident::new("get", span);
        let ids_ident = Ident::new("ids", span);
        let iter_ident = Ident::new("iter", span);
        quote! {
            impl #insertable_trait_ident for #ty {
                type Id = #id_ty;

                fn #insert_into_ident(self, b: &mut #builder_ident, key: String) -> Self::Id {
                    let id = #id_ty(key);
                    b.#accessor(id.clone(), self);
                    id
                }
            }

            impl #node_trait_ident for #ty {
                fn #get_ident<'g>(g: &'g #schema_name, id: &Self::Id) -> Option<&'g Self>
                where
                    Self: 'g,
                {
                    g.#field.get(id)
                }

                fn #ids_ident<'g>(g: &'g #schema_name) -> impl Iterator<Item = &'g Self::Id>
                where
                    Self: 'g,
                {
                    g.#field.ids()
                }

                fn #iter_ident<'g>(g: &'g #schema_name) -> impl Iterator<Item = (&'g Self::Id, &'g Self)>
                where
                    Self: 'g,
                {
                    g.#field.iter()
                }
            }
        }
    });

    quote! {
        /// ノードの読み書きで使うトレイト境界 (`docs/schema_v4.md` §3.2)。
        /// 書き込み (`insert_into`、`{Schema}Insertable` supertrait 経由) は
        /// `{Builder}::insert` 経由、読み取り (`get`/`ids`/`iter`) は
        /// `Type::method(&g, ..)` の形で使う想定 (このトレイトを `use` で
        /// スコープに入れておく必要がある)。利用者が `insert_into` を直接
        /// 呼ぶことは想定しない。
        pub trait #node_trait_ident: #insertable_trait_ident {
            /// キーからノード値を引く。
            fn get<'g>(g: &'g #schema_name, id: &Self::Id) -> Option<&'g Self>
            where
                Self: 'g;
            /// このノード種別の全キーを列挙する。挿入順を保持する
            /// (`KeyedTable` の仕様、`crates/graphite/src/keyed_table.rs`)。
            fn ids<'g>(g: &'g #schema_name) -> impl Iterator<Item = &'g Self::Id>
            where
                Self: 'g;
            /// このノード種別の全要素を `(キー, 値)` で走査する。挿入順を
            /// 保持する (`KeyedTable` の仕様)。
            fn iter<'g>(g: &'g #schema_name) -> impl Iterator<Item = (&'g Self::Id, &'g Self)>
            where
                Self: 'g;
        }

        #(#node_impls)*
    }
}

/// エッジ挿入用トレイト (書き込み側専用)。`graph!` の辺行
/// `key = Kind(from -> to)` はタプル struct `Kind(from_id, to_id, ..)` を
/// 構築したあと、この trait 境界を介した総称 `{Builder}::add` に脱糖する
/// (`docs/schema_v4.md` §2/§3.2)。読み取り側 (`of`/`get`/`between`/`iter`/
/// `ids`/`len`) は各エッジ種別型 (`Kind`) への固有 impl で提供するため、
/// このトレイトには含めない (`gen_edge_query_impl` 参照)。
///
/// `insert_into`/`Id` は `{Schema}Insertable` (`gen_insertable_trait` 参照) に
/// 集約したため、このトレイト自体は supertrait 境界のみのマーカーになる
/// (`extend` 統一、`docs/graph_splice.md` §2)。`add` の型境界は変わらず
/// `E: #edge_trait_ident` のまま (エッジ専用の型境界を保つ)。
fn gen_edge_trait_and_impls(
    edge_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
    builder_ident: &Ident,
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let edge_impls = edges.iter().map(|e| {
        let kind = e.kind;
        let id_ty = &e.id_ident;
        let accessor = &e.accessor_ident;
        // 必須ではないが (このメソッドはユーザーが直接呼ぶ想定ではない)、
        // 他の生成メソッドとの一貫性のため `edge.kind` のスパンを付ける
        // (`docs/ide_support_spec.md` §1.9 の指示: 余裕があれば付けてよい)。
        let insert_into_ident = Ident::new("insert_into", kind.span());
        quote! {
            impl #insertable_trait_ident for #kind {
                type Id = #id_ty;

                fn #insert_into_ident(self, b: &mut #builder_ident, key: String) -> Self::Id {
                    let id = #id_ty(key);
                    b.#accessor(id.clone(), self);
                    id
                }
            }

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

/// ノード値の型 (`Person` 等) はユーザー宣言への参照なので生成しない。
/// ここで生成するのは newtype キー型だけ (`PersonId(pub String)`)。
fn gen_node_id_types(nodes: &[NodeInfo]) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(|n| {
            let id_ty = &n.id_ident;
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
                pub struct #id_ty(pub String);
            }
        })
        .collect()
}

/// エッジ種別ごとの newtype キー型 (`BossId(pub String)`)。ノードキーと
/// 同じ規約 (`docs/schema_v4.md` §3.1)。
fn gen_edge_id_types(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|e| {
            let id_ty = &e.id_ident;
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
                pub struct #id_ty(pub String);
            }
        })
        .collect()
}

/// エッジ種別ごとのタプル struct とその位置アクセサ・`payload` メソッド。
///
/// `docs/schema_v4.md` §3.1: 「タプル struct として実在し、マクロ外でも
/// `Boss(from_id, to_id, payload)` で普通に構築できる」(原則6: 消去可能な
/// 拡張のみ)。読み取りは位置 (`.0`/`.1`/`.2`) を人間に晒さず、固定語彙の
/// メソッドを生成する。
///
/// v4.1 (`docs/edge_endpoints_v4_1.md`) での分岐:
/// - 役割名なしの有向辺: `.from()`/`.to()` (従来どおり)。
/// - 役割名つきの有向辺: `.from()`/`.to()` の**代わりに**役割名そのままの
///   メソッド (`.subordinate()`/`.superior()`)。生成する ident は役割名
///   トークンのスパンを持つ (F12 で宣言の役割名に着地、G3 スパン規約)。
/// - 無向辺: `.from()`/`.to()` という嘘の語彙を避け、`.endpoints()` を
///   1つだけ生やす。
fn gen_edge_tuple_structs(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|e| {
            let kind = e.kind;
            let p0_id = &e.from_node.id_ident;
            let p1_id = &e.to_node.id_ident;

            let (struct_def, payload_method) = match &e.attrs_ty {
                None => (
                    quote! { pub struct #kind(pub #p0_id, pub #p1_id); },
                    quote! {},
                ),
                Some(attrs) => (
                    quote! { pub struct #kind(pub #p0_id, pub #p1_id, pub #attrs); },
                    quote! {
                        /// この辺の積み荷 (属性値) を返す。
                        pub fn payload(&self) -> &#attrs {
                            &self.2
                        }
                    },
                ),
            };

            let position_accessors = if !e.directed {
                quote! {
                    /// この辺の両端点を返す (無向辺には from/to という向きの
                    /// 語彙が無いため `endpoints` を使う)。
                    pub fn endpoints(&self) -> (&#p0_id, &#p1_id) {
                        (&self.0, &self.1)
                    }
                }
            } else {
                match (&e.from_role, &e.to_role) {
                    (Some(from_role), Some(to_role)) => {
                        let m0 = Ident::new(&from_role.to_string(), from_role.span());
                        let m1 = Ident::new(&to_role.to_string(), to_role.span());
                        quote! {
                            /// この辺の始点キーを返す (役割名: 宣言側の役割名
                            /// アクセサ、`.from()` は生成しない)。
                            pub fn #m0(&self) -> &#p0_id {
                                &self.0
                            }
                            /// この辺の終点キーを返す (役割名アクセサ)。
                            pub fn #m1(&self) -> &#p1_id {
                                &self.1
                            }
                        }
                    }
                    _ => quote! {
                        /// この辺の始点キーを返す。
                        pub fn from(&self) -> &#p0_id {
                            &self.0
                        }
                        /// この辺の終点キーを返す。
                        pub fn to(&self) -> &#p1_id {
                            &self.1
                        }
                    },
                }
            };

            quote! {
                #[derive(Debug, Clone, PartialEq)]
                #struct_def

                impl #kind {
                    #position_accessors
                    #payload_method
                }
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
/// - `each` 制約違反 (`{Kind}EachViolation`) は解決された側
///   (出次数/入次数/次数) に応じてフィールド名 (`source`/`target`/`node`) が
///   変わる。役割名は変数命名 (フィールド名) には混ぜない
///   (`docs/edge_endpoints_v4_1.md` §1「違反バリアントの命名は従来規約の
///   まま」)。
/// - `unique pair` 違反 (`{Kind}UniquePairViolation`) は有向なら
///   `source`/`target`、無向なら順序の意味が無いため `a`/`b`。
fn gen_violation_enum(
    violation_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let dup_variants = nodes.iter().map(|n| {
        let v = n.dup_variant();
        let id = &n.id_ident;
        quote! { #v(#id) }
    });
    let dup_display_arms = nodes.iter().map(|n| {
        let v = n.dup_variant();
        let type_name_str = n.type_ident.to_string();
        quote! {
            #violation_ident::#v(id) => write!(f, "{}のキーが重複しています: {:?}", #type_name_str, id)
        }
    });

    let mut edge_variants: Vec<TokenStream> = Vec::new();
    let mut edge_display_arms: Vec<TokenStream> = Vec::new();

    for edge in edges {
        let kind_str = edge.kind.to_string();
        let edge_id = &edge.id_ident;

        let dup_key = edge.duplicate_key_variant();
        edge_variants.push(quote! {
            /// このエッジ種別のキーが重複している。
            #dup_key(#edge_id)
        });
        edge_display_arms.push(quote! {
            #violation_ident::#dup_key(id) => write!(
                f, "{}のキーが重複しています: {:?}", #kind_str, id
            )
        });

        if edge.directed {
            let from_id = &edge.from_node.id_ident;
            let to_id = &edge.to_node.id_ident;
            let from_type_str = edge.from_node.type_ident.to_string();
            let to_type_str = edge.to_node.type_ident.to_string();

            let unk_src = edge.unknown_source_variant();
            edge_variants.push(quote! {
                /// このエッジが未知の始点キーを参照している。
                #unk_src { edge: #edge_id, source: #from_id }
            });
            edge_display_arms.push(quote! {
                #violation_ident::#unk_src { edge, source } => write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    #kind_str, edge, #from_type_str, source
                )
            });

            let unk_dst = edge.unknown_target_variant();
            edge_variants.push(quote! {
                /// このエッジが未知の終点キーを参照している。
                #unk_dst { edge: #edge_id, target: #to_id }
            });
            edge_display_arms.push(quote! {
                #violation_ident::#unk_dst { edge, target } => write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    #kind_str, edge, #to_type_str, target
                )
            });

            if let (Some(spec), Some(side)) = (edge.each, edge.each_side) {
                let expected_str = match spec {
                    EachSpec::One => "ちょうど1",
                    EachSpec::ZeroOrOne => "0または1",
                };
                let v = edge.each_violation_variant();
                match side {
                    EachSide::Source => {
                        edge_variants.push(quote! {
                            /// このエッジ種別の `each` 制約違反 (出次数)。
                            #v { source: #from_id, count: usize }
                        });
                        edge_display_arms.push(quote! {
                            #violation_ident::#v { source, count } => write!(
                                f,
                                "each制約違反: エッジ `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                                #kind_str, #from_type_str, source, #expected_str, count
                            )
                        });
                    }
                    EachSide::Target => {
                        edge_variants.push(quote! {
                            /// このエッジ種別の `each` 制約違反 (入次数)。
                            #v { target: #to_id, count: usize }
                        });
                        edge_display_arms.push(quote! {
                            #violation_ident::#v { target, count } => write!(
                                f,
                                "each制約違反: エッジ `{}` は {} {:?} について入次数 {} を期待しますが実際は {} 本です",
                                #kind_str, #to_type_str, target, #expected_str, count
                            )
                        });
                    }
                    EachSide::Degree => unreachable!("有向辺のeachはDegreeにはならない"),
                }
            }

            if edge.unique_pair {
                let v = edge.unique_pair_violation_variant();
                edge_variants.push(quote! {
                    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
                    /// 2本目の辺が張られた)。
                    #v { source: #from_id, target: #to_id }
                });
                edge_display_arms.push(quote! {
                    #violation_ident::#v { source, target } => write!(
                        f,
                        "unique pair違反: エッジ `{}` は {:?} -> {:?} の対に既に辺が存在します",
                        #kind_str, source, target
                    )
                });
            }
        } else {
            // 無向辺: 両端は同じノード型 (validate 済み) なので from_node で代表する。
            let node_id = &edge.from_node.id_ident;
            let node_type_str = edge.from_node.type_ident.to_string();

            let unk = edge.unknown_endpoint_variant();
            edge_variants.push(quote! {
                /// このエッジが未知の端点キーを参照している (無向のため位置の
                /// 区別は無い)。
                #unk { edge: #edge_id, endpoint: #node_id }
            });
            edge_display_arms.push(quote! {
                #violation_ident::#unk { edge, endpoint } => write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の端点, {}): {:?}",
                    #kind_str, edge, #node_type_str, endpoint
                )
            });

            if let (Some(spec), Some(EachSide::Degree)) = (edge.each, edge.each_side) {
                let expected_str = match spec {
                    EachSpec::One => "ちょうど1",
                    EachSpec::ZeroOrOne => "0または1",
                };
                let v = edge.each_violation_variant();
                edge_variants.push(quote! {
                    /// このエッジ種別の `each` 制約違反 (次数、無向)。
                    #v { node: #node_id, count: usize }
                });
                edge_display_arms.push(quote! {
                    #violation_ident::#v { node, count } => write!(
                        f,
                        "each制約違反: エッジ `{}` は {} {:?} について次数 {} を期待しますが実際は {} 本です",
                        #kind_str, #node_type_str, node, #expected_str, count
                    )
                });
            }

            if edge.unique_pair {
                let v = edge.unique_pair_violation_variant();
                edge_variants.push(quote! {
                    /// このエッジ種別の `unique pair` 違反 (無向のため
                    /// 順序を無視した対で判定)。
                    #v { a: #node_id, b: #node_id }
                });
                edge_display_arms.push(quote! {
                    #violation_ident::#v { a, b } => write!(
                        f,
                        "unique pair違反: エッジ `{}` は {{{:?}, {:?}}} の対に既に辺が存在します",
                        #kind_str, a, b
                    )
                });
            }
        }
    }

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
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

        impl std::error::Error for #violation_ident {}
    }
}

fn gen_schema_struct(
    schema_name: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_fields = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let id = &n.id_ident;
        let ty = &n.type_ident;
        quote! { #field: graphite::KeyedTable<#id, #ty> }
    });
    let edge_fields = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let index_field = &e.index_field_ident;
        let id_ty = &e.id_ident;
        let kind = e.kind;
        // 索引のキー型は位置0の型 (有向なら始点、無向なら両端同型なので
        // どちらでも同じ)。
        let key_id = &e.from_node.id_ident;
        // 有向辺のみ終点索引を永続化する (`docs/reverse_query.md`)。
        // `{Kind}::sources_of` の索引であり、v4.1 で入次数 each 検証のためだけに
        // 一時構築していた索引をこれに統合した (無向辺は `index_field` が
        // 既に対称に両端を積むので不要)。
        let to_index_decl = if e.directed {
            let to_index_field = &e.to_index_field_ident;
            let to_key_id = &e.to_node.id_ident;
            quote! {
                ,
                /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (freeze 時に
                /// 構築。`{Kind}::sources_of` の索引、`docs/reverse_query.md`)。
                #to_index_field: std::collections::HashMap<#to_key_id, Vec<#id_ty>>
            }
        } else {
            quote! {}
        };
        quote! {
            #accessor: graphite::KeyedTable<#id_ty, #kind>,
            /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
            /// キーの一覧 (freeze 時に構築)。
            #index_field: std::collections::HashMap<#key_id, Vec<#id_ty>>
            #to_index_decl
        }
    });

    quote! {
        /// 凍結済み図式グラフ。構築後は不変 (可変 API は公開しない)。
        pub struct #schema_name {
            #(#node_fields,)*
            #(#edge_fields,)*
        }
    }
}

/// スキーマ struct 本体の impl。v4 (`docs/schema_v4.md` §3.2「g.メソッドは
/// 廃止」) によりノード・エッジの個別アクセサは一切ここに生やさない。
/// 残るのは構築用の `create`/`create_collecting` だけ (読み取りは型名前空間
/// の関連関数 = `{Schema}Node`/`Kind` の固有 impl 経由)。
fn gen_schema_impl(schema_name: &Ident, violation_ident: &Ident, builder_ident: &Ident) -> TokenStream {
    quote! {
        impl #schema_name {
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

fn gen_builder_struct(
    builder_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_fields = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let id = &n.id_ident;
        let ty = &n.type_ident;
        quote! { #field: Vec<(#id, #ty)> }
    });
    let edge_fields = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let id_ty = &e.id_ident;
        let kind = e.kind;
        quote! { #accessor: Vec<(#id_ty, #kind)> }
    });

    quote! {
        /// 構築用 builder。凍結 (`freeze`) までは where 制約検査を一切行わない。
        pub struct #builder_ident {
            #(#node_fields,)*
            #(#edge_fields,)*
        }
    }
}

fn gen_builder_impl(
    builder_ident: &Ident,
    violation_ident: &Ident,
    node_trait_ident: &Ident,
    edge_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
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
        let id_ty = &n.id_ident;
        let ty = &n.type_ident;
        quote! {
            pub fn #accessor(&mut self, id: #id_ty, value: #ty) -> &mut Self {
                self.#field.push((id, value));
                self
            }
        }
    });

    let edge_methods = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let id_ty = &e.id_ident;
        let kind = e.kind;
        quote! {
            pub fn #accessor(&mut self, id: #id_ty, value: #kind) -> &mut Self {
                self.#accessor.push((id, value));
                self
            }
        }
    });

    let freeze_body = gen_freeze_body(schema_name, violation_ident, nodes, edges);

    quote! {
        impl #builder_ident {
            fn new() -> Self {
                Self {
                    #(#node_field_inits,)*
                    #(#edge_field_inits,)*
                }
            }

            #(#node_methods)*
            #(#edge_methods)*

            /// 型名付きメソッド (`b.#accessor(id, value)` 群、上記
            /// `#node_methods`) の総称版。`graph!` はノード項の値の型を
            /// 一切パースしないため (`key = 式` の「式」でしかない)、この
            /// メソッドで値の型 (`N: #node_trait_ident`) から正しい内部
            /// ストレージへの振り分けを rustc の型推論任せにする。
            /// 命名判断・trait の形は `gen_node_trait_and_impls` の
            /// ドキュメントコメント参照。
            pub fn insert<N: #node_trait_ident>(&mut self, key: impl Into<String>, value: N) -> N::Id {
                value.insert_into(self, key.into())
            }

            /// `insert` のエッジ版。`graph!` の辺行 `key = Kind(from -> to)`
            /// はタプル struct `Kind(from_id, to_id, ..)` を構築したあと、
            /// この総称メソッドへ脱糖する (`docs/schema_v4.md` §2/§3.2)。
            pub fn add<E: #edge_trait_ident>(&mut self, key: impl Into<String>, value: E) -> E::Id {
                value.insert_into(self, key.into())
            }

            /// `insert`/`add` のイテレータ版 (`docs/bulk_construction.md`、
            /// `docs/graph_splice.md` §2)。実行時データからの構築で for
            /// ループが構築コードに残るのを避けるため、要素単位 API の反復に
            /// 完全に一致する意味論 (挿入順保持・検証は freeze 時) をまとめて
            /// 提供する。ノード用・エッジ用の呼び分けが要らない単一の総称
            /// メソッドに統一している (v4 破壊的変更、旧 `extend_nodes`/
            /// `extend_edges` は廃止): 値の型が `T: #insertable_trait_ident`
            /// を満たせばノードでもエッジでもよい (どちらになるかは rustc の
            /// 型推論任せ)。`graph!` のスプライス項 (`..式`) もこのメソッドへ
            /// 脱糖する。`insert`/`add` と同じ理由 (トレイトが schema ごとに
            /// 名前が異なる) で、graphite ランタイム側の共通機構ではなく
            /// ここに生成する。
            pub fn extend<K, T>(&mut self, items: impl IntoIterator<Item = (K, T)>) -> Vec<T::Id>
            where
                K: Into<String>,
                T: #insertable_trait_ident,
            {
                items.into_iter().map(|(k, v)| v.insert_into(self, k.into())).collect()
            }

            #freeze_body
        }
    }
}

/// `where each <参照名>: ..` の IDE 支援専用ゼロコスト検査文
/// (`docs/ide_support_spec.md` §1.9)。
///
/// - 役割名なし (`each_from_role` が `None`): `<参照名>` はノード型名を指す
///   ので、従来どおり型検査文 `let _: fn(&<参照名>) = |_| {};` を生成する
///   (無向辺の次数制約もこちらに含まれる — 役割名を持たないため)。
/// - 役割名あり: `<参照名>` はもはや型名ではなく役割名 (アクセサメソッド名)
///   なので、型検査ではなく `Kind::<参照名>` というメソッド項参照に変える。
///   これにより F12 は生成された `fn <役割名>(..)` (スパンは endpoint 宣言の
///   役割名トークン) に着地する。
fn gen_each_type_check(edge: &EdgeInfo<'_>) -> TokenStream {
    let Some(tok) = &edge.each_token else {
        return quote! {};
    };
    if edge.from_role.is_some() {
        let kind = edge.kind;
        quote! {
            #[allow(unused)]
            let _ = #kind::#tok;
        }
    } else {
        quote! {
            let _: fn(&#tok) = |_| {};
        }
    }
}

/// 有向辺1種別分の freeze 検査本体を生成する。
///
/// 手順:
/// 1. `Vec<(KindId, Kind)>` から `KeyedTable<KindId, Kind>` を構築 (重複キー
///    は `{Kind}DuplicateKey` 違反として記録し、その要素は捨てる)。
/// 2. 生き残った各辺について端点 (位置0/1) がそれぞれのノード表に実在するか
///    検査する (`{Kind}UnknownSource`/`{Kind}UnknownTarget`)。両端点とも
///    正当な辺だけを位置0索引 (`{accessor}_from_index`) と位置1索引
///    (`{accessor}_to_index`) の両方に積む。後者は `docs/reverse_query.md`
///    により構造体フィールドとして永続化する (`{Kind}::sources_of` が使う。
///    v4.1 で入次数 each 検証のためだけに一時構築していた索引をこれに統合)。
///    `unique pair` 制約があれば、同じ (位置0, 位置1) の対が2回目に現れた
///    時点で `{Kind}UniquePairViolation` を記録する。
/// 3. `each` 制約があれば、`each_side` に応じて出次数 (位置0索引) または
///    入次数 (位置1索引、手順2で作った永続化済みのものをそのまま使う) を
///    検査する。
fn gen_directed_edge_freeze_block(violation_ident: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let from_index = &edge.index_field_ident;
    let to_index = &edge.to_index_field_ident;
    let from_field = &edge.from_node.field_ident;
    let to_field = &edge.to_node.field_ident;
    let dup_key = edge.duplicate_key_variant();
    let unk_src = edge.unknown_source_variant();
    let unk_dst = edge.unknown_target_variant();

    // `__seen_pairs` は `unique pair` 制約がある場合のみ宣言する。常に
    // 宣言すると、制約が無いエッジ種別では一度も使われない
    // `HashSet<_>` になり、型が確定できず E0282 (type annotations needed)
    // になってしまう (要素型が使用箇所から逆算されるため、未使用だと
    // 逆算できる場所が無い)。
    let (seen_pairs_decl, unique_pair_check) = if edge.unique_pair {
        let v = edge.unique_pair_violation_variant();
        (
            quote! {
                let mut __seen_pairs: std::collections::HashSet<_> = std::collections::HashSet::new();
            },
            quote! {
                if !__seen_pairs.insert((from.clone(), to.clone())) {
                    __violations.push(#violation_ident::#v {
                        source: from.clone(),
                        target: to.clone(),
                    });
                }
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    let each_type_check = gen_each_type_check(edge);

    let each_check = match (edge.each, edge.each_side) {
        (Some(EachSpec::One), Some(EachSide::Source)) => {
            let v = edge.each_violation_variant();
            quote! {
                for key in #from_field.ids() {
                    let count = #from_index.get(key).map(Vec::len).unwrap_or(0);
                    if count != 1 {
                        __violations.push(#violation_ident::#v {
                            source: key.clone(),
                            count,
                        });
                    }
                }
            }
        }
        (Some(EachSpec::ZeroOrOne), Some(EachSide::Source)) => {
            let v = edge.each_violation_variant();
            quote! {
                for (key, ids) in &#from_index {
                    if ids.len() > 1 {
                        __violations.push(#violation_ident::#v {
                            source: key.clone(),
                            count: ids.len(),
                        });
                    }
                }
            }
        }
        (Some(EachSpec::One), Some(EachSide::Target)) => {
            let v = edge.each_violation_variant();
            quote! {
                for key in #to_field.ids() {
                    let count = #to_index.get(key).map(Vec::len).unwrap_or(0);
                    if count != 1 {
                        __violations.push(#violation_ident::#v {
                            target: key.clone(),
                            count,
                        });
                    }
                }
            }
        }
        (Some(EachSpec::ZeroOrOne), Some(EachSide::Target)) => {
            let v = edge.each_violation_variant();
            quote! {
                for (key, ids) in &#to_index {
                    if ids.len() > 1 {
                        __violations.push(#violation_ident::#v {
                            target: key.clone(),
                            count: ids.len(),
                        });
                    }
                }
            }
        }
        _ => quote! {},
    };

    quote! {
        let mut #accessor: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.#accessor {
            if !#accessor.insert(id.clone(), value) {
                __violations.push(#violation_ident::#dup_key(id));
            }
        }

        let mut #from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        #seen_pairs_decl
        for (id, edge) in #accessor.iter() {
            let from = &edge.0;
            let to = &edge.1;
            let mut __ok = true;
            if !#from_field.contains_key(from) {
                __violations.push(#violation_ident::#unk_src { edge: id.clone(), source: from.clone() });
                __ok = false;
            }
            if !#to_field.contains_key(to) {
                __violations.push(#violation_ident::#unk_dst { edge: id.clone(), target: to.clone() });
                __ok = false;
            }
            if __ok {
                #unique_pair_check
                #from_index.entry(from.clone()).or_default().push(id.clone());
                #to_index.entry(to.clone()).or_default().push(id.clone());
            }
        }
        #each_type_check
        #each_check
    }
}

/// 無向辺1種別分の freeze 検査本体を生成する
/// (`docs/edge_endpoints_v4_1.md` §2)。
///
/// 位置0/1索引 (`{accessor}_index`) は「その位置0キーに (有向の from_index
/// と同じ形で) 接続するエッジキーの一覧」だが、無向のため対称に構築する:
/// 位置0・位置1のどちらにも (自己ループなら1回だけ) 積む。これにより
/// - 次数 (each) は `index.get(x).len()` で求まる (自己ループは1本と数える)。
/// - `of`/`between` はどちらの位置に置かれてもこの索引から検索できる。
/// - 格納順 (挿入順) は `KeyedTable::iter()` の走査順そのままなので、索引の
///   `push` もその順で行われ、`docs/edge_endpoints_v4_1.md` §2 の
///   「挿入順保持」がそのまま満たされる。
fn gen_undirected_edge_freeze_block(violation_ident: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let index = &edge.index_field_ident;
    let node_field = &edge.from_node.field_ident;
    let dup_key = edge.duplicate_key_variant();
    let unk = edge.unknown_endpoint_variant();

    let (seen_pairs_decl, unique_pair_check) = if edge.unique_pair {
        let v = edge.unique_pair_violation_variant();
        (
            quote! {
                let mut __seen_pairs: std::collections::HashSet<_> = std::collections::HashSet::new();
            },
            quote! {
                let __pair = if p0 <= p1 { (p0.clone(), p1.clone()) } else { (p1.clone(), p0.clone()) };
                if !__seen_pairs.insert(__pair) {
                    __violations.push(#violation_ident::#v {
                        a: p0.clone(),
                        b: p1.clone(),
                    });
                }
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    let each_type_check = gen_each_type_check(edge);

    let each_check = match edge.each {
        Some(EachSpec::One) => {
            let v = edge.each_violation_variant();
            quote! {
                for key in #node_field.ids() {
                    let count = #index.get(key).map(Vec::len).unwrap_or(0);
                    if count != 1 {
                        __violations.push(#violation_ident::#v {
                            node: key.clone(),
                            count,
                        });
                    }
                }
            }
        }
        Some(EachSpec::ZeroOrOne) => {
            let v = edge.each_violation_variant();
            quote! {
                for (key, ids) in &#index {
                    if ids.len() > 1 {
                        __violations.push(#violation_ident::#v {
                            node: key.clone(),
                            count: ids.len(),
                        });
                    }
                }
            }
        }
        None => quote! {},
    };

    quote! {
        let mut #accessor: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.#accessor {
            if !#accessor.insert(id.clone(), value) {
                __violations.push(#violation_ident::#dup_key(id));
            }
        }

        let mut #index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        #seen_pairs_decl
        for (id, edge) in #accessor.iter() {
            let p0 = &edge.0;
            let p1 = &edge.1;
            let mut __ok = true;
            if !#node_field.contains_key(p0) {
                __violations.push(#violation_ident::#unk { edge: id.clone(), endpoint: p0.clone() });
                __ok = false;
            }
            if p1 != p0 && !#node_field.contains_key(p1) {
                __violations.push(#violation_ident::#unk { edge: id.clone(), endpoint: p1.clone() });
                __ok = false;
            }
            if __ok {
                #unique_pair_check
                #index.entry(p0.clone()).or_default().push(id.clone());
                if p1 != p0 {
                    #index.entry(p1.clone()).or_default().push(id.clone());
                }
            }
        }
        #each_type_check
        #each_check
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

    let edge_blocks = edges.iter().map(|e| {
        if e.directed {
            gen_directed_edge_freeze_block(violation_ident, e)
        } else {
            gen_undirected_edge_freeze_block(violation_ident, e)
        }
    });

    let node_field_names = nodes.iter().map(|n| &n.field_ident);
    let edge_field_names = edges.iter().map(|e| &e.accessor_ident);
    // 有向辺は位置0索引 (`{accessor}_from_index`) と位置1索引
    // (`{accessor}_to_index`) の両方をフィールドとして持つ。無向辺は
    // `index_field_ident` (対称な単一索引) のみ (`gen_schema_struct` 参照)。
    let edge_index_names: Vec<&Ident> = edges
        .iter()
        .flat_map(|e| {
            if e.directed {
                vec![&e.index_field_ident, &e.to_index_field_ident]
            } else {
                vec![&e.index_field_ident]
            }
        })
        .collect();

    quote! {
        /// 検証ロジックの実体。最初の1件で打ち切らず全違反を `Vec` に
        /// 集めて返す。`freeze` (単一エラー版) はこちらに委譲し先頭の1件を
        /// 取り出すだけの薄いラッパーにすることで、検証ロジックが二重実装に
        /// ならないようにしている。
        fn freeze_collecting(self) -> Result<#schema_name, Vec<#violation_ident>> {
            let mut __violations: Vec<#violation_ident> = Vec::new();

            #(#node_table_builds)*
            #(#edge_blocks)*

            if !__violations.is_empty() {
                return Err(__violations);
            }

            Ok(#schema_name {
                #(#node_field_names,)*
                #(#edge_field_names,)*
                #(#edge_index_names,)*
            })
        }

        /// 最初の1件の違反で `Err` になる版。実装は
        /// `freeze_collecting` に委譲する。
        fn freeze(self) -> Result<#schema_name, #violation_ident> {
            self.freeze_collecting().map_err(|mut violations| violations.remove(0))
        }
    }
}

/// エッジ種別1つ分の読み取りAPI (`Kind` への固有 impl) を生成する。
/// 有向/無向で実装が大きく異なるためここで分岐する。
fn gen_edge_query_impl(schema_name: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    if edge.directed {
        gen_directed_edge_query_impl(schema_name, edge)
    } else {
        gen_undirected_edge_query_impl(schema_name, edge)
    }
}

/// 有向辺の読み取り API。`docs/schema_v4.md` §3.2 の where 制約 → 戻り型
/// 対応表をそのまま実装する。`of`/`get_of` の戻り型は常に「出次数
/// (`each_side == Source`)」の制約のみを見る (`docs/edge_endpoints_v4_1.md`
/// §1: 入次数制約は freeze 検証のみに使われ、`of` の戻り型には影響しない —
/// `of` は常に始点側キーで検索するため)。
fn gen_directed_edge_query_impl(schema_name: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let kind = edge.kind;
    let id_ty = &edge.id_ident;
    let accessor = &edge.accessor_ident;
    let from_index = &edge.index_field_ident;
    let to_index = &edge.to_index_field_ident;
    let from_id = &edge.from_node.id_ident;
    let to_id = &edge.to_node.id_ident;
    let from_field = &edge.from_node.field_ident;
    let from_ty = &edge.from_node.type_ident;
    let to_field = &edge.to_node.field_ident;
    let to_ty = &edge.to_node.type_ident;

    // IDE 支援 (`docs/ide_support_spec.md` §1.9, G3 ポリシー): このエッジ
    // 種別への固有 impl が生やすメソッド名は、全て `edge.kind` (schema の
    // `edge Kind = ..` の `Kind` トークン) のスパンを持たせる。これにより
    // `Boss::of(..)` の `of` から F12 すると schema の `edge Boss` 宣言へ
    // 着地するようになる (call_site のままだと macro 定義側に着地してしまう)。
    let kind_span = kind.span();
    let of_ident = Ident::new("of", kind_span);
    let get_of_ident = Ident::new("get_of", kind_span);
    let sources_of_ident = Ident::new("sources_of", kind_span);
    let get_sources_of_ident = Ident::new("get_sources_of", kind_span);
    let get_ident = Ident::new("get", kind_span);
    let between_ident = Ident::new("between", kind_span);
    let iter_ident = Ident::new("iter", kind_span);
    let ids_ident = Ident::new("ids", kind_span);
    let len_ident = Ident::new("len", kind_span);

    // `of`/`get_of` の戻り値の型・実装は「積み荷の有無」「出次数 each 制約」
    // の組み合わせで分岐する。これらの関数はいずれも `&self` を取らず
    // `g: &'g Schema` を第一引数に取る associated function なので、
    // 標準の省略規則 (`&self` があれば自動で結び付く規則) が使えない —
    // 参照引数が複数ある (`g` と `from`/`to`/`id`) ため、返り値に含まれる
    // 参照が `g` 由来であることを示す明示的なライフタイム `'g` が必須
    // (省略すると E0106)。
    let target_ref_ty = quote! { &'g #to_ty };
    let of_item_ty = match &edge.attrs_ty {
        None => quote! { #target_ref_ty },
        Some(attrs) => quote! { (#target_ref_ty, &'g #attrs) },
    };
    let resolve_one = |edge_id_expr: TokenStream| -> TokenStream {
        match &edge.attrs_ty {
            None => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("from_indexに載っている辺はstorageに必ず存在する");
                    g.#to_field.get(&e.1).expect("freezeで端点存在を検証済みのはず")
                }
            },
            Some(_) => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("from_indexに載っている辺はstorageに必ず存在する");
                    let target = g.#to_field.get(&e.1).expect("freezeで端点存在を検証済みのはず");
                    (target, e.payload())
                }
            },
        }
    };

    // `of` の戻り型を決めるのは常に出次数側 (Source) の each のみ
    // (`docs/edge_endpoints_v4_1.md` §1)。
    let source_each = match edge.each_side {
        Some(EachSide::Source) => edge.each,
        _ => None,
    };

    let of_and_get_of = match source_each {
        Some(EachSpec::One) => {
            let resolved = resolve_one(quote! { &ids[0] });
            quote! {
                /// この辺種別の自然な戻り値 (`each 1` → 直接参照)。
                ///
                /// # Panics
                /// `from` がこのグラフに存在しない (このグラフが発行した
                /// ものではない) キーの場合パニックする。これは入力検証の
                /// 欠如ではなく呼び出し規約の違反であり
                /// (`docs/design_principles.md` 原則2)、非パニック版
                /// [`Self::get_of`] も併せて提供する。
                pub fn #of_ident<'g>(g: &'g #schema_name, from: &#from_id) -> #of_item_ty {
                    Self::#get_of_ident(g, from).unwrap_or_else(|| {
                        panic!(
                            "{}::of: 未知のキーです (このグラフが発行したキーではありません): {:?}",
                            stringify!(#kind), from
                        )
                    })
                }

                /// [`Self::of`] の非パニック版。未知キーは `None` を返す。
                pub fn #get_of_ident<'g>(g: &'g #schema_name, from: &#from_id) -> Option<#of_item_ty> {
                    let ids = g.#from_index.get(from)?;
                    Some(#resolved)
                }
            }
        }
        Some(EachSpec::ZeroOrOne) => {
            let resolved = resolve_one(quote! { &ids[0] });
            quote! {
                /// この辺種別の自然な戻り値 (`each 0..1` → `Option`)。
                /// 無い/未知キーはどちらも `None` に落ちる (「無い」ことが
                /// 正常なドメイン状態なのでパニックしない)。
                pub fn #of_ident<'g>(g: &'g #schema_name, from: &#from_id) -> Option<#of_item_ty> {
                    let ids = g.#from_index.get(from)?;
                    Some(#resolved)
                }
            }
        }
        None => {
            let resolved = resolve_one(quote! { id });
            quote! {
                /// この辺種別の自然な戻り値 (出次数に制約なし → `Vec`)。
                /// 無い/未知キーはどちらも空 `Vec` に落ちる。格納順 (構築時の
                /// 追加順) を保持する。
                pub fn #of_ident<'g>(g: &'g #schema_name, from: &#from_id) -> Vec<#of_item_ty> {
                    match g.#from_index.get(from) {
                        Some(ids) => ids.iter().map(|id| #resolved).collect(),
                        None => Vec::new(),
                    }
                }
            }
        }
    };

    // `sources_of`/`get_sources_of` (`docs/reverse_query.md`): `of` の対称、
    // 終点で引いて始点側を返す。`of` が「積み荷の有無」×「出次数 each」で
    // 分岐するのと同じ形で、「積み荷の有無」×「入次数 each
    // (`each_side == Target`)」で分岐する。
    let source_ref_ty = quote! { &'g #from_ty };
    let sources_of_item_ty = match &edge.attrs_ty {
        None => quote! { #source_ref_ty },
        Some(attrs) => quote! { (#source_ref_ty, &'g #attrs) },
    };
    let resolve_source = |edge_id_expr: TokenStream| -> TokenStream {
        match &edge.attrs_ty {
            None => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("to_indexに載っている辺はstorageに必ず存在する");
                    g.#from_field.get(&e.0).expect("freezeで端点存在を検証済みのはず")
                }
            },
            Some(_) => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("to_indexに載っている辺はstorageに必ず存在する");
                    let source = g.#from_field.get(&e.0).expect("freezeで端点存在を検証済みのはず");
                    (source, e.payload())
                }
            },
        }
    };

    // `sources_of` の戻り型を決めるのは常に入次数側 (Target) の each のみ
    // (`of` の出次数版と対称、`docs/reverse_query.md`)。役割名なしの辺は
    // 入次数 each を書けないので常に `None` になり `Vec` を返す。
    let target_each = match edge.each_side {
        Some(EachSide::Target) => edge.each,
        _ => None,
    };

    let sources_of_and_get = match target_each {
        Some(EachSpec::One) => {
            let resolved = resolve_source(quote! { &ids[0] });
            quote! {
                /// `of` の対称 (`docs/reverse_query.md`): 終点で引き、始点側
                /// (相手ノード値+積み荷) を返す。`each 1` (入次数) → 直接参照。
                ///
                /// # Panics
                /// `to` がこのグラフに存在しない (このグラフが発行したもの
                /// ではない) キーの場合パニックする
                /// (`docs/design_principles.md` 原則2)。非パニック版
                /// [`Self::get_sources_of`] も併せて提供する。
                pub fn #sources_of_ident<'g>(g: &'g #schema_name, to: &#to_id) -> #sources_of_item_ty {
                    Self::#get_sources_of_ident(g, to).unwrap_or_else(|| {
                        panic!(
                            "{}::sources_of: 未知のキーです (このグラフが発行したキーではありません): {:?}",
                            stringify!(#kind), to
                        )
                    })
                }

                /// [`Self::sources_of`] の非パニック版。未知キーは `None` を返す。
                pub fn #get_sources_of_ident<'g>(g: &'g #schema_name, to: &#to_id) -> Option<#sources_of_item_ty> {
                    let ids = g.#to_index.get(to)?;
                    Some(#resolved)
                }
            }
        }
        Some(EachSpec::ZeroOrOne) => {
            let resolved = resolve_source(quote! { &ids[0] });
            quote! {
                /// `of` の対称 (`docs/reverse_query.md`): 終点で引き、始点側
                /// (相手ノード値+積み荷) を返す。`each 0..1` (入次数) →
                /// `Option`。無い/未知キーはどちらも `None` に落ちる。
                pub fn #sources_of_ident<'g>(g: &'g #schema_name, to: &#to_id) -> Option<#sources_of_item_ty> {
                    let ids = g.#to_index.get(to)?;
                    Some(#resolved)
                }
            }
        }
        None => {
            let resolved = resolve_source(quote! { id });
            quote! {
                /// `of` の対称 (`docs/reverse_query.md`): 終点で引き、始点側
                /// (相手ノード値+積み荷) を返す。入次数に制約なし → `Vec`。
                /// 無い/未知キーはどちらも空 `Vec` に落ちる。格納順 (構築時の
                /// 追加順) を保持する。
                pub fn #sources_of_ident<'g>(g: &'g #schema_name, to: &#to_id) -> Vec<#sources_of_item_ty> {
                    match g.#to_index.get(to) {
                        Some(ids) => ids.iter().map(|id| #resolved).collect(),
                        None => Vec::new(),
                    }
                }
            }
        }
    };

    let between = if edge.unique_pair {
        quote! {
            /// 対 (始点, 終点) で辺を検索する (`unique pair` → 高々1本)。
            pub fn #between_ident<'g>(g: &'g #schema_name, from: &#from_id, to: &#to_id) -> Option<&'g #kind> {
                g.#from_index
                    .get(from)?
                    .iter()
                    .filter_map(|id| g.#accessor.get(id))
                    .find(|e| &e.1 == to)
            }
        }
    } else {
        quote! {
            /// 対 (始点, 終点) で辺を検索する (制約なしなら平行辺を許すため
            /// `Vec`)。格納順 (構築時の追加順) を保持する。
            pub fn #between_ident<'g>(g: &'g #schema_name, from: &#from_id, to: &#to_id) -> Vec<&'g #kind> {
                match g.#from_index.get(from) {
                    Some(ids) => ids
                        .iter()
                        .filter_map(|id| g.#accessor.get(id))
                        .filter(|e| &e.1 == to)
                        .collect(),
                    None => Vec::new(),
                }
            }
        }
    };

    quote! {
        impl #kind {
            #of_and_get_of

            #sources_of_and_get

            /// キーで辺1本を検索する。
            pub fn #get_ident<'g>(g: &'g #schema_name, id: &#id_ty) -> Option<&'g #kind> {
                g.#accessor.get(id)
            }

            #between

            /// 表全体を `(キー, 値)` で走査する。挿入順 (構築時の追加順) を
            /// 保持する (`KeyedTable` の仕様)。
            pub fn #iter_ident(g: &#schema_name) -> impl Iterator<Item = (&#id_ty, &#kind)> {
                g.#accessor.iter()
            }

            /// この辺種別の全キーを列挙する。挿入順 (構築時の追加順) を
            /// 保持する (`KeyedTable` の仕様)。
            pub fn #ids_ident(g: &#schema_name) -> impl Iterator<Item = &#id_ty> {
                g.#accessor.ids()
            }

            /// この辺種別に含まれる辺の本数。
            pub fn #len_ident(g: &#schema_name) -> usize {
                g.#accessor.len()
            }
        }
    }
}

/// 無向辺の読み取り API (`docs/edge_endpoints_v4_1.md` §2)。
///
/// `of(&g, &x)` は `x` が位置0/1のどちらに置かれていても、もう一方の端点を
/// 返す (自己ループなら `x` 自身を返す)。戻り型は次数 (`each`) 制約が決める
/// 規則で有向の表と同じ。`between(&g, &a, &b)` は対称 (順序を無視) に検索する。
fn gen_undirected_edge_query_impl(schema_name: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let kind = edge.kind;
    let id_ty = &edge.id_ident;
    let accessor = &edge.accessor_ident;
    let index = &edge.index_field_ident;
    let node_id = &edge.from_node.id_ident;
    let node_field = &edge.from_node.field_ident;
    let node_ty = &edge.from_node.type_ident;

    let kind_span = kind.span();
    let of_ident = Ident::new("of", kind_span);
    let get_of_ident = Ident::new("get_of", kind_span);
    let get_ident = Ident::new("get", kind_span);
    let between_ident = Ident::new("between", kind_span);
    let iter_ident = Ident::new("iter", kind_span);
    let ids_ident = Ident::new("ids", kind_span);
    let len_ident = Ident::new("len", kind_span);

    let other_ref_ty = quote! { &'g #node_ty };
    let of_item_ty = match &edge.attrs_ty {
        None => quote! { #other_ref_ty },
        Some(attrs) => quote! { (#other_ref_ty, &'g #attrs) },
    };
    let resolve_one = |edge_id_expr: TokenStream| -> TokenStream {
        match &edge.attrs_ty {
            None => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("indexに載っている辺はstorageに必ず存在する");
                    let other = if &e.0 == x { &e.1 } else { &e.0 };
                    g.#node_field.get(other).expect("freezeで端点存在を検証済みのはず")
                }
            },
            Some(_) => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("indexに載っている辺はstorageに必ず存在する");
                    let other = if &e.0 == x { &e.1 } else { &e.0 };
                    let node = g.#node_field.get(other).expect("freezeで端点存在を検証済みのはず");
                    (node, e.payload())
                }
            },
        }
    };

    let of_and_get_of = if let Some(EachSide::Degree) = edge.each_side {
        match edge.each.expect("each_sideがDegreeならeachも必ずSome") {
            EachSpec::One => {
                let resolved = resolve_one(quote! { &ids[0] });
                quote! {
                    /// この辺種別の自然な戻り値 (`each 1` → 直接参照)。
                    ///
                    /// # Panics
                    /// `x` がこのグラフに存在しない (このグラフが発行した
                    /// ものではない) キーの場合パニックする
                    /// (`docs/design_principles.md` 原則2)。非パニック版
                    /// [`Self::get_of`] も併せて提供する。
                    pub fn #of_ident<'g>(g: &'g #schema_name, x: &#node_id) -> #of_item_ty {
                        Self::#get_of_ident(g, x).unwrap_or_else(|| {
                            panic!(
                                "{}::of: 未知のキーです (このグラフが発行したキーではありません): {:?}",
                                stringify!(#kind), x
                            )
                        })
                    }

                    /// [`Self::of`] の非パニック版。未知キーは `None` を返す。
                    pub fn #get_of_ident<'g>(g: &'g #schema_name, x: &#node_id) -> Option<#of_item_ty> {
                        let ids = g.#index.get(x)?;
                        Some(#resolved)
                    }
                }
            }
            EachSpec::ZeroOrOne => {
                let resolved = resolve_one(quote! { &ids[0] });
                quote! {
                    /// この辺種別の自然な戻り値 (`each 0..1` → `Option`)。
                    pub fn #of_ident<'g>(g: &'g #schema_name, x: &#node_id) -> Option<#of_item_ty> {
                        let ids = g.#index.get(x)?;
                        Some(#resolved)
                    }
                }
            }
        }
    } else {
        let resolved = resolve_one(quote! { id });
        quote! {
            /// この辺種別の自然な戻り値 (次数に制約なし → `Vec`)。無い/未知
            /// キーはどちらも空 `Vec` に落ちる。格納順 (構築時の追加順) を
            /// 保持する。
            pub fn #of_ident<'g>(g: &'g #schema_name, x: &#node_id) -> Vec<#of_item_ty> {
                match g.#index.get(x) {
                    Some(ids) => ids.iter().map(|id| #resolved).collect(),
                    None => Vec::new(),
                }
            }
        }
    };

    let between = if edge.unique_pair {
        quote! {
            /// 対 (a, b) で辺を検索する (`unique pair` → 高々1本、順序は無視)。
            pub fn #between_ident<'g>(g: &'g #schema_name, a: &#node_id, b: &#node_id) -> Option<&'g #kind> {
                g.#index
                    .get(a)?
                    .iter()
                    .filter_map(|id| g.#accessor.get(id))
                    .find(|e| {
                        let other = if &e.0 == a { &e.1 } else { &e.0 };
                        other == b
                    })
            }
        }
    } else {
        quote! {
            /// 対 (a, b) で辺を検索する (制約なしなら平行辺を許すため `Vec`、
            /// 順序は無視)。格納順 (構築時の追加順) を保持する。
            pub fn #between_ident<'g>(g: &'g #schema_name, a: &#node_id, b: &#node_id) -> Vec<&'g #kind> {
                match g.#index.get(a) {
                    Some(ids) => ids
                        .iter()
                        .filter_map(|id| g.#accessor.get(id))
                        .filter(|e| {
                            let other = if &e.0 == a { &e.1 } else { &e.0 };
                            other == b
                        })
                        .collect(),
                    None => Vec::new(),
                }
            }
        }
    };

    quote! {
        impl #kind {
            #of_and_get_of

            /// キーで辺1本を検索する。
            pub fn #get_ident<'g>(g: &'g #schema_name, id: &#id_ty) -> Option<&'g #kind> {
                g.#accessor.get(id)
            }

            #between

            /// 表全体を `(キー, 値)` で走査する。挿入順 (構築時の追加順) を
            /// 保持する (`KeyedTable` の仕様)。
            pub fn #iter_ident(g: &#schema_name) -> impl Iterator<Item = (&#id_ty, &#kind)> {
                g.#accessor.iter()
            }

            /// この辺種別の全キーを列挙する。挿入順 (構築時の追加順) を
            /// 保持する (`KeyedTable` の仕様)。
            pub fn #ids_ident(g: &#schema_name) -> impl Iterator<Item = &#id_ty> {
                g.#accessor.ids()
            }

            /// この辺種別に含まれる辺の本数。
            pub fn #len_ident(g: &#schema_name) -> usize {
                g.#accessor.len()
            }
        }
    }
}
