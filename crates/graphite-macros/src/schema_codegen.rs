//! `graph_schema!` のコード生成本体 (v4、`docs/schema_v4.md` §3 参照。
//! v4.1 の役割名・無向辺は `docs/edge_endpoints_v4_1.md`、ID 型の既定生成と
//! 明示指定は `docs/node_id_v4_2.md` 参照)。
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
//! impl Boss {
//!     pub fn payload(&self) -> &BossEdge { &self.appointment }
//!
//!     pub fn of(g: &Graph, from: &PersonId) -> Option<(&super::Person, &super::BossEdge)> { .. }
//!     pub fn get(g: &Graph, id: &BossId) -> Option<&Boss> { .. }
//! }
//! pub struct Person; // ノード読み取り用マーカー
//! pub struct Graph { .. }
//! pub struct Builder { .. }
//! pub enum Violation { .. }
//! }
//! ```
//!
//! 有向辺 (`(subordinate: Employee) -> (superior: Employee)`) は役割名を
//! 公開フィールド名として保持し、利用側は役割名を通して端点を読む。
//! 無向辺 (`Person -- Person`) は
//! `.endpoints() -> (&PersonId, &PersonId)` を生やし、`of`/`between` は
//! どちらの位置に置かれても対称に検索できる。内部の freeze/query 実装は
//! named fieldを直接使う。
//!
//! 辺はスキーマ module 内の生成型なので固有 impl で読み取り API を生やす。
//! ノード値型はユーザーが module 外に宣言し、複数 schema 間で共有できる。
//! ユーザー型への固有 impl は追加せず、`Org::Person::get` のようにスキーマ
//! module 内のノードマーカーへ読み取り API を生成する。
//! ID 型を省略した `node Person;` は schema module 内に `PersonId(String)` を
//! 生成する。既存型を使う場合は `node Person(id: ExistingId);` と明示する。
//!
//! where 制約 → 戻り型の対応表 (`docs/schema_v4.md` §3.2、有向・始点側のみ):
//! - `each X: 1`    -> `of` は直接参照 (未知キーはパニック、非パニック版 `get_of`)
//! - `each X: 0..1` -> `of` は `Option`
//! - その他の範囲または制約なし -> `of` は `Vec`
//! - `unique pair`  -> `between` は `Option`、それ以外は `Vec`
//!
//! 役割名つきの辺で `each` が終点側 (入次数) を指定した場合、`of` の戻り型は
//! 上記表に従わず常に `Vec` になる (`of` は常に始点側キーで検索するため、
//! 始点側が無制約なら平行辺を許すのが自然)。無向辺は端点の役割名を持たないため
//! `each` を受け付けず、`unique pair` だけを指定できる。
//!
//! ## 終点側クエリ `{Kind}::sources_of` (`docs/reverse_query.md`)
//!
//! 有向辺には `of` の対称として `sources_of`/`get_sources_of` を生成する
//! (無向辺には生成しない — `of` が既に対称なので同じものになるため)。
//! `sources_of(g, to)` は `to` を終点とする辺の**始点側**(相手ノード値+積み荷)
//! を返す。戻り型は上記表と同じ規則だが、判定に使う制約は **終点側
//! (入次数、`each_side == Target`)** の `each` のみ。無向辺は必ず `Vec` を返す。
//! 相手はノード値で返す
//! (キー版は生やさない — `docs/reverse_query.md` の最小方針)。
//!
//! 実装は freeze 時に構築・永続化する終点索引 `{accessor}_to_index`
//! (`ToId -> Vec<KindId>`、`gen_schema_struct`/`gen_directed_edge_freeze_block`
//! 参照) を検索するだけなので O(1) 償却。この索引は v4.1 で入次数 each 検証
//! のためだけに一時構築していたものを構造体フィールドとして格上げ・統合した
//! もの (`docs/reverse_query.md` 実装ノート)。

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Path};

use crate::naming::{
    each_violation_ident, edge_storage_ident, generated_id_ident, graph_type_ident,
    plural_field_name, to_snake_case,
};
use crate::schema_dsl::{EachSpec, EdgeDecl, EdgePayload, EdgeShape, NodeDecl, SchemaInput};
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
    /// 一覧の内部フィールド名。freeze 時に構築する (`docs/schema_v4.md`
    /// §3.2)。有向辺は始点を表す `{accessor}_from_index`、無向辺は方向の
    /// 意味を持たないため `{accessor}_index` とする。
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
            EdgeInfoShape::Directed { payload, .. }
            | EdgeInfoShape::Undirected { payload } => payload.as_ref(),
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
}

pub fn generate(schema: &SchemaInput) -> TokenStream {
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
    let edge_value_struct_defs = gen_edge_value_structs(&edge_infos);
    let violation_def = gen_violation_enum(&violation_ident, &node_infos, &edge_infos);
    let schema_struct_def = gen_schema_struct(&graph_ident, &node_infos, &edge_infos);
    let schema_impl = gen_schema_impl(&graph_ident, &violation_ident, &builder_ident);
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
    );
    let edge_trait_and_impls = gen_edge_trait_and_impls(
        &edge_trait_ident,
        &insertable_trait_ident,
        &default_id_trait_ident,
        &builder_ident,
        &edge_infos,
    );
    let edge_query_impls = edge_infos
        .iter()
        .map(|e| gen_edge_query_impl(&graph_ident, e));

    quote! {
        #[allow(non_snake_case)]
        pub mod #schema_name {
            use super::*;

            #(#default_id_defs)*
            #(#edge_value_struct_defs)*
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
        pub trait #insertable_trait_ident: Sized {
            type Id;
            fn insert_with_id(self, b: &mut #builder_ident, id: Self::Id) -> Self::Id;
        }

        /// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
        /// 実装する。明示ID型には実装せず、文字列変換を要求しない。
        pub trait #default_id_trait_ident: #insertable_trait_ident {
            fn insert_with_binding(self, b: &mut #builder_ident, binding: String) -> Self::Id;
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
/// ## 読み取り側 (`get`/`ids`/`iter`)
///
/// ノード型 (`Person` 等) はユーザーが `graph_schema!` の外で宣言する型で
/// あり、複数 schema 間で共有されうる。ユーザー struct へ固有 impl を
/// 追加すると schema 間で同名メソッドが衝突するため、読み取り API は
/// `Org::Person::get(&g, &id)` のように生成したノードマーカーへ置く。
///
/// ## `{Schema}Insertable` と `{Schema}DefaultId`
///
/// 型付き挿入と関連型 `Id` は `{Schema}Insertable` に置く。文字列の束縛名から
/// IDを作る操作は自動生成IDだけが実装する `{Schema}DefaultId` に置く。
/// `{Schema}Node` はノード専用の型境界を保つマーカートレイトである。
///
/// ## 命名判断 (`docs/design_principles.md` 原則3: std 命名規約準拠)
///
/// - **内部 trait 名は `{Schema}Node` とした**。生成 module に移した後も
///   `node Node;` や `edge Edge = ..;` と生成基盤名が衝突する可能性を増やさず、
///   コンパイラ診断から所属 schema を判別できる名前を維持する。
/// - **メソッド名は `insert_with_id`/`get`/`ids`/`iter`**。`get`/`ids`/`iter` は
///   `docs/schema_v4.md` §3.2 のイメージ通り (std の `HashMap::get`/
///   `HashMap::keys`/`HashMap::iter` に倣った命名)。
fn gen_node_trait_and_impls(
    node_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
    builder_ident: &Ident,
    graph_ident: &Ident,
    nodes: &[NodeInfo],
) -> TokenStream {
    let node_impls = nodes.iter().map(|n| {
        let ty = &n.type_ident;
        let id_ty = &n.id_ty;
        let accessor = &n.accessor_ident;
        let field = &n.field_ident;
        // IDE 支援 (`docs/ide_support_spec.md` §1.9, G3 ポリシー): このノード
        // 型への `{Schema}Node`/`{Schema}Insertable` impl が生やすメソッド名は
        // `n.type_ident` (ノード型そのもののトークン) のスパンを持たせる。
        // トレイト定義自体 (下の `pub trait #node_trait_ident { .. }`) は
        // 単一の由来トークンを持たない schema 全体のインフラなので call_site
        // のままでよい (指示どおり、impl 側だけに適用する)。
        let span = ty.span();
        let insert_with_id_ident = Ident::new("insert_with_id", span);
        let get_ident = Ident::new("get", span);
        let ids_ident = Ident::new("ids", span);
        let iter_ident = Ident::new("iter", span);
        let default_id_impl = if n.id_ty.is_generated() {
            let generated_id = &n.id_ty.generated_ident;
            quote! {
                impl #default_id_trait_ident for super::#ty {
                    fn insert_with_binding(self, b: &mut #builder_ident, binding: String) -> Self::Id {
                        #insertable_trait_ident::insert_with_id(self, b, #generated_id(binding))
                    }
                }
            }
        } else {
            quote! {}
        };
        quote! {
            impl #insertable_trait_ident for super::#ty {
                type Id = #id_ty;

                fn #insert_with_id_ident(self, b: &mut #builder_ident, id: Self::Id) -> Self::Id {
                    let returned_id = id.clone();
                    b.#accessor(id, self);
                    returned_id
                }
            }

            #default_id_impl
            impl #node_trait_ident for super::#ty {}

            /// このスキーマにおける `#ty` ノード種別の問い合わせ名前空間。
            pub struct #ty;

            impl #ty {
                pub fn #get_ident<'g>(g: &'g #graph_ident, id: &#id_ty) -> Option<&'g super::#ty> {
                    g.#field.get(id)
                }

                pub fn #ids_ident<'g>(g: &'g #graph_ident) -> impl Iterator<Item = &'g #id_ty> {
                    g.#field.ids()
                }

                pub fn #iter_ident<'g>(g: &'g #graph_ident) -> impl Iterator<Item = (&'g #id_ty, &'g super::#ty)> {
                    g.#field.iter()
                }
            }
        }
    });

    quote! {
        /// ノード挿入で使うトレイト境界。読み取りは同じ module 内の
        /// ノードマーカー型が提供する。利用者がこのトレイトのメソッドを
        /// 直接呼ぶことは想定しない。
        pub trait #node_trait_ident: #insertable_trait_ident {}

        #(#node_impls)*
    }
}

/// エッジ挿入用トレイト (書き込み側専用)。`graph!` の辺行
/// `key = Kind(from -> to)` は名前付きフィールドの辺値型を関連コンストラクタで
/// 構築したあと、この trait 境界を介した総称 `{Builder}::add` に脱糖する
/// (`docs/schema_v4.md` §2/§3.2)。読み取り側 (`of`/`get`/`between`/`iter`/
/// `ids`/`len`) は各エッジ種別型 (`Kind`) への固有 impl で提供するため、
/// このトレイトには含めない (`gen_edge_query_impl` 参照)。
///
/// 型付き挿入と関連型 `Id` は `{Schema}Insertable` に集約する。このトレイトは
/// エッジ専用の型境界を保つマーカーになる。
fn gen_edge_trait_and_impls(
    edge_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
    builder_ident: &Ident,
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let edge_impls = edges.iter().map(|e| {
        let kind = e.kind;
        let id_ty = &e.id_ty;
        let accessor = &e.accessor_ident;
        // 必須ではないが (このメソッドはユーザーが直接呼ぶ想定ではない)、
        // 他の生成メソッドとの一貫性のため `edge.kind` のスパンを付ける
        // (`docs/ide_support_spec.md` §1.9 の指示: 余裕があれば付けてよい)。
        let insert_with_id_ident = Ident::new("insert_with_id", kind.span());
        let default_id_impl = if e.id_ty.is_generated() {
            let generated_id = &e.id_ty.generated_ident;
            quote! {
                impl #default_id_trait_ident for #kind {
                    fn insert_with_binding(self, b: &mut #builder_ident, binding: String) -> Self::Id {
                        #insertable_trait_ident::insert_with_id(self, b, #generated_id(binding))
                    }
                }
            }
        } else {
            quote! {}
        };
        quote! {
            impl #insertable_trait_ident for #kind {
                type Id = #id_ty;

                fn #insert_with_id_ident(self, b: &mut #builder_ident, id: Self::Id) -> Self::Id {
                    let returned_id = id.clone();
                    b.#accessor(id, self);
                    returned_id
                }
            }

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

            let (struct_def, constructor, debug_endpoints) = match &e.shape {
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
                            #[doc(hidden)]
                            pub fn __graphite_directed(from: #p0_id, to: #p1_id) -> Self {
                                Self::new(from, to)
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
                            #[doc(hidden)]
                            pub fn __graphite_directed(
                                from: #p0_id,
                                to: #p1_id,
                                payload: #attrs,
                            ) -> Self {
                                Self::new(from, to, payload)
                            }
                            pub fn payload(&self) -> &#attrs { &self.#payload_role }
                        },
                        (quote! { self.#from_role }, quote! { self.#to_role }),
                    )
                }
                EdgeInfoShape::Undirected { payload: None } => (
                        quote! { pub struct #kind { pub endpoints: graphite::UnorderedPair<#p0_id> } },
                        quote! {
                            pub fn new(a: #p0_id, b: #p1_id) -> Self {
                                Self { endpoints: graphite::UnorderedPair::new(a, b) }
                            }
                            #[doc(hidden)]
                            pub fn __graphite_undirected(a: #p0_id, b: #p1_id) -> Self {
                                Self::new(a, b)
                            }
                            pub fn endpoints(&self) -> (&#p0_id, &#p1_id) {
                                self.endpoints.endpoints()
                            }
                        },
                        (quote! { self.endpoints.endpoints().0 }, quote! { self.endpoints.endpoints().1 }),
                    ),
                EdgeInfoShape::Undirected {
                    payload: Some(payload),
                } => {
                    let payload_role = &payload.role;
                    let attrs = &payload.ty;
                    (
                        quote! {
                            pub struct #kind {
                                pub endpoints: graphite::UnorderedPair<#p0_id>,
                                pub #payload_role: #attrs,
                            }
                        },
                        quote! {
                            pub fn new(a: #p0_id, b: #p1_id, payload: #attrs) -> Self {
                                Self { endpoints: graphite::UnorderedPair::new(a, b), #payload_role: payload }
                            }
                            #[doc(hidden)]
                            pub fn __graphite_undirected(
                                a: #p0_id,
                                b: #p1_id,
                                payload: #attrs,
                            ) -> Self {
                                Self::new(a, b, payload)
                            }
                            pub fn endpoints(&self) -> (&#p0_id, &#p1_id) {
                                self.endpoints.endpoints()
                            }
                            pub fn payload(&self) -> &#attrs { &self.#payload_role }
                        },
                        (quote! { self.endpoints.endpoints().0 }, quote! { self.endpoints.endpoints().1 }),
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
        let kind = e.kind;
        // 索引のキー型は位置0の型 (有向なら始点、無向なら両端同型なので
        // どちらでも同じ)。
        let key_id = &e.from_node.id_ty;
        // 有向辺のみ終点索引を永続化する (`docs/reverse_query.md`)。
        // `{Kind}::sources_of` の索引であり、v4.1 で入次数 each 検証のためだけに
        // 一時構築していた索引をこれに統合した (無向辺は `index_field` が
        // 既に対称に両端を積むので不要)。
        let to_index_decl = if matches!(&e.shape, EdgeInfoShape::Directed { .. }) {
            let to_index_field = &e.to_index_field_ident;
            let to_key_id = &e.to_node.id_ty;
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
/// の関連関数 = schema module 内のノードマーカー/辺種別の固有 impl 経由)。
fn gen_schema_impl(
    schema_name: &Ident,
    violation_ident: &Ident,
    builder_ident: &Ident,
) -> TokenStream {
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
        /// 構築用 builder。凍結 (`freeze`) までは where 制約検査を一切行わない。
        pub struct #builder_ident {
            #(#node_fields,)*
            #(#edge_fields,)*
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
            pub fn insert<N>(&mut self, key: impl Into<String>, value: N) -> N::Id
            where
                N: #node_trait_ident + #default_id_trait_ident,
            {
                value.insert_with_binding(self, key.into())
            }

            /// `@ ID式` を書いたノード項の脱糖先。明示ID型と既定ID型の
            /// どちらにも使える。
            /// 参照: #6の名前付き静的アクセサを実装するときは脱糖先を再検討する。
            pub fn insert_with_id<N: #node_trait_ident>(&mut self, id: N::Id, value: N) -> N::Id {
                value.insert_with_id(self, id)
            }

            /// `insert` のエッジ版。`graph!` の辺行 `key = Kind(from -> to)`
            /// は名前付きフィールドの辺値型を関連コンストラクタで構築したあと、
            /// この総称メソッドへ脱糖する (`docs/schema_v4.md` §2/§3.2)。
            pub fn add<E>(&mut self, key: impl Into<String>, value: E) -> E::Id
            where
                E: #edge_trait_ident + #default_id_trait_ident,
            {
                value.insert_with_binding(self, key.into())
            }

            /// `@ ID式` を書いたエッジ項の脱糖先。明示ID型と既定ID型の
            /// どちらにも使える。
            /// 参照: #6の名前付き静的アクセサを実装するときは脱糖先を再検討する。
            pub fn add_with_id<E: #edge_trait_ident>(&mut self, id: E::Id, value: E) -> E::Id {
                value.insert_with_id(self, id)
            }

            /// `insert`/`add` のイテレータ版 (`docs/bulk_construction.md`、
            /// `docs/graph_splice.md` §2)。実行時データからの構築で for
            /// ループが構築コードに残るのを避けるため、要素単位 API の反復に
            /// 完全に一致する意味論 (挿入順保持・検証は freeze 時) をまとめて
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
                // スプライスの完全な名前・ID意味論は #6/#2 で確定する。
                items.into_iter().map(|(k, v)| v.insert_with_binding(self, k.into())).collect()
            }

            #freeze_body
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
fn gen_directed_edge_freeze_block(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
    from_role: &Ident,
    to_role: &Ident,
) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let storage = edge_storage_ident(accessor);
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
                for key in #from_field.ids() {
                    let count = #from_index.get(key).map(Vec::len).unwrap_or(0);
                    if #invalid_count {
                        __violations.push(#violation_ident::#v { source: key.clone(), count });
                    }
                }
            },
            EachSide::Target => quote! {
                for key in #to_field.ids() {
                    let count = #to_index.get(key).map(Vec::len).unwrap_or(0);
                    if #invalid_count {
                        __violations.push(#violation_ident::#v { target: key.clone(), count });
                    }
                }
            },
        }
    });

    quote! {
        let mut #storage: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.#accessor {
            if !#storage.insert(id.clone(), value) {
                __violations.push(#violation_ident::#dup_key(id));
            }
        }

        let mut #from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut #to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        #seen_pairs_decl
        for (id, edge) in #storage.iter() {
            let from = &edge.#from_role;
            let to = &edge.#to_role;
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
        #(#each_checks)*
    }
}

/// 無向辺1種別分の freeze 検査本体を生成する
/// (`docs/edge_endpoints_v4_1.md` §2)。
///
/// 位置0/1索引 (`{accessor}_index`) は「その位置0キーに (有向の from_index
/// と同じ形で) 接続するエッジキーの一覧」だが、無向のため対称に構築する:
/// 位置0・位置1のどちらにも (自己ループなら1回だけ) 積む。これにより
/// - `of`/`between` はどちらの位置に置かれてもこの索引から検索できる。
/// - 格納順 (挿入順) は `KeyedTable::iter()` の走査順そのままなので、索引の
///   `push` もその順で行われ、`docs/edge_endpoints_v4_1.md` §2 の
///   「挿入順保持」がそのまま満たされる。
fn gen_undirected_edge_freeze_block(violation_ident: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let storage = edge_storage_ident(accessor);
    let index = &edge.index_field_ident;
    let node_field = &edge.from_node.field_ident;
    let dup_key = edge.duplicate_key_variant();
    let unk = edge.unknown_endpoint_variant();

    // 無向辺の `unique pair` は (p0, p1) と (p1, p0) を同一視する。ID型へ
    // 順序比較を要求しないため、両方の順序で `__seen_pairs` を検索する。
    let (seen_pairs_decl, unique_pair_check) = if edge.unique_pair {
        let v = edge.unique_pair_violation_variant();
        (
            quote! {
                let mut __seen_pairs: std::collections::HashSet<_> = std::collections::HashSet::new();
            },
            quote! {
                if __seen_pairs.contains(&(p0.clone(), p1.clone())) || __seen_pairs.contains(&(p1.clone(), p0.clone())) {
                    __violations.push(#violation_ident::#v {
                        a: p0.clone(),
                        b: p1.clone(),
                    });
                } else {
                    __seen_pairs.insert((p0.clone(), p1.clone()));
                }
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    quote! {
        let mut #storage: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.#accessor {
            if !#storage.insert(id.clone(), value) {
                __violations.push(#violation_ident::#dup_key(id));
            }
        }

        let mut #index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        #seen_pairs_decl
        for (id, edge) in #storage.iter() {
            let (p0, p1) = edge.endpoints.endpoints();
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
            from_role,
            to_role,
            ..
        } => gen_directed_edge_freeze_block(violation_ident, e, from_role, to_role),
        EdgeInfoShape::Undirected { .. } => {
            gen_undirected_edge_freeze_block(violation_ident, e)
        }
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
    let edge_index_names: Vec<&Ident> = edges
        .iter()
        .flat_map(|e| match &e.shape {
            EdgeInfoShape::Directed { .. } => {
                vec![&e.index_field_ident, &e.to_index_field_ident]
            }
            EdgeInfoShape::Undirected { .. } => vec![&e.index_field_ident],
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
                #(#edge_field_inits,)*
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
    match &edge.shape {
        EdgeInfoShape::Directed {
            from_role,
            to_role,
            payload,
        } => gen_directed_edge_query_impl(
            schema_name,
            edge,
            from_role,
            to_role,
            payload.as_ref(),
        ),
        EdgeInfoShape::Undirected { payload } => {
            gen_undirected_edge_query_impl(schema_name, edge, payload.as_ref())
        }
    }
}

/// 有向辺の読み取り API。`docs/schema_v4.md` §3.2 の where 制約 → 戻り型
/// 対応表をそのまま実装する。`of`/`get_of` の戻り型は常に「出次数
/// (`each_side == Source`)」の制約のみを見る (`docs/edge_endpoints_v4_1.md`
/// §1: 入次数制約は freeze 検証のみに使われ、`of` の戻り型には影響しない —
/// `of` は常に始点側キーで検索するため)。
fn gen_directed_edge_query_impl(
    schema_name: &Ident,
    edge: &EdgeInfo<'_>,
    from_role: &Ident,
    to_role: &Ident,
    payload: Option<&EdgePayload>,
) -> TokenStream {
    let kind = edge.kind;
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let from_index = &edge.index_field_ident;
    let to_index = &edge.to_index_field_ident;
    let from_id = &edge.from_node.id_ty;
    let to_id = &edge.to_node.id_ty;
    let from_field = &edge.from_node.field_ident;
    let from_ty = &edge.from_node.type_ident;
    let to_field = &edge.to_node.field_ident;
    let to_ty = &edge.to_node.type_ident;
    let payload_role = payload.map(|value| &value.role);

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
    let target_ref_ty = quote! { &'g super::#to_ty };
    let of_item_ty = match payload {
        None => quote! { #target_ref_ty },
        Some(value) => {
            let attrs = &value.ty;
            quote! { (#target_ref_ty, &'g #attrs) }
        }
    };
    let resolve_one = |edge_id_expr: TokenStream| -> TokenStream {
        match payload {
            None => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("from_indexに載っている辺はstorageに必ず存在する");
                    g.#to_field.get(&e.#to_role).expect("freezeで端点存在を検証済みのはず")
                }
            },
            Some(_) => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("from_indexに載っている辺はstorageに必ず存在する");
                    let target = g.#to_field.get(&e.#to_role).expect("freezeで端点存在を検証済みのはず");
                    (target, &e.#payload_role)
                }
            },
        }
    };

    // `of` の戻り型を決めるのは常に出次数側 (Source) の each のみ
    // (`docs/edge_endpoints_v4_1.md` §1)。
    let source_each = edge
        .each_for(EachSide::Source)
        .map(|constraint| constraint.spec);

    let of_and_get_of = match source_each {
        Some(spec) if spec.is_exactly_one() => {
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
                        panic!("{}::of: 未知のキーです (このグラフが発行したキーではありません)", stringify!(#kind))
                    })
                }

                /// [`Self::of`] の非パニック版。未知キーは `None` を返す。
                pub fn #get_of_ident<'g>(g: &'g #schema_name, from: &#from_id) -> Option<#of_item_ty> {
                    let ids = g.#from_index.get(from)?;
                    Some(#resolved)
                }
            }
        }
        Some(spec) if spec.is_zero_or_one() => {
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
        _ => {
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
    let source_ref_ty = quote! { &'g super::#from_ty };
    let sources_of_item_ty = match payload {
        None => quote! { #source_ref_ty },
        Some(value) => {
            let attrs = &value.ty;
            quote! { (#source_ref_ty, &'g #attrs) }
        }
    };
    let resolve_source = |edge_id_expr: TokenStream| -> TokenStream {
        match payload {
            None => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("to_indexに載っている辺はstorageに必ず存在する");
                    g.#from_field.get(&e.#from_role).expect("freezeで端点存在を検証済みのはず")
                }
            },
            Some(_) => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("to_indexに載っている辺はstorageに必ず存在する");
                    let source = g.#from_field.get(&e.#from_role).expect("freezeで端点存在を検証済みのはず");
                    (source, &e.#payload_role)
                }
            },
        }
    };

    // `sources_of` の戻り型を決めるのは常に入次数側 (Target) の each のみ
    // (`of` の出次数版と対称、`docs/reverse_query.md`)。
    let target_each = edge
        .each_for(EachSide::Target)
        .map(|constraint| constraint.spec);

    let sources_of_and_get = match target_each {
        Some(spec) if spec.is_exactly_one() => {
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
                        panic!("{}::sources_of: 未知のキーです (このグラフが発行したキーではありません)", stringify!(#kind))
                    })
                }

                /// [`Self::sources_of`] の非パニック版。未知キーは `None` を返す。
                pub fn #get_sources_of_ident<'g>(g: &'g #schema_name, to: &#to_id) -> Option<#sources_of_item_ty> {
                    let ids = g.#to_index.get(to)?;
                    Some(#resolved)
                }
            }
        }
        Some(spec) if spec.is_zero_or_one() => {
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
        _ => {
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
                    .find(|e| &e.#to_role == to)
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
                        .filter(|e| &e.#to_role == to)
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
fn gen_undirected_edge_query_impl(
    schema_name: &Ident,
    edge: &EdgeInfo<'_>,
    payload: Option<&EdgePayload>,
) -> TokenStream {
    let kind = edge.kind;
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let index = &edge.index_field_ident;
    let node_id = &edge.from_node.id_ty;
    let node_field = &edge.from_node.field_ident;
    let node_ty = &edge.from_node.type_ident;
    let payload_role = payload.map(|value| &value.role);

    let kind_span = kind.span();
    let of_ident = Ident::new("of", kind_span);
    let get_ident = Ident::new("get", kind_span);
    let between_ident = Ident::new("between", kind_span);
    let iter_ident = Ident::new("iter", kind_span);
    let ids_ident = Ident::new("ids", kind_span);
    let len_ident = Ident::new("len", kind_span);

    let other_ref_ty = quote! { &'g super::#node_ty };
    let of_item_ty = match payload {
        None => quote! { #other_ref_ty },
        Some(value) => {
            let attrs = &value.ty;
            quote! { (#other_ref_ty, &'g #attrs) }
        }
    };
    let resolve_one = |edge_id_expr: TokenStream| -> TokenStream {
        match payload {
            None => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("indexに載っている辺はstorageに必ず存在する");
                    let (first, second) = e.endpoints.endpoints();
                    let other = if first == x { second } else { first };
                    g.#node_field.get(other).expect("freezeで端点存在を検証済みのはず")
                }
            },
            Some(_) => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("indexに載っている辺はstorageに必ず存在する");
                    let (first, second) = e.endpoints.endpoints();
                    let other = if first == x { second } else { first };
                    let node = g.#node_field.get(other).expect("freezeで端点存在を検証済みのはず");
                    (node, &e.#payload_role)
                }
            },
        }
    };

    let resolved = resolve_one(quote! { id });
    let of_and_get_of = quote! {
        /// 無向辺には端点の役割名がないため多重度制約を持たず、
        /// 接続先を挿入順の `Vec` で返す。
        pub fn #of_ident<'g>(g: &'g #schema_name, x: &#node_id) -> Vec<#of_item_ty> {
            match g.#index.get(x) {
                Some(ids) => ids.iter().map(|id| #resolved).collect(),
                None => Vec::new(),
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
                        let (first, second) = e.endpoints.endpoints();
                        let other = if first == a { second } else { first };
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
                            let (first, second) = e.endpoints.endpoints();
                            let other = if first == a { second } else { first };
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
