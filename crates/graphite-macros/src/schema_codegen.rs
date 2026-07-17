//! `graph_schema!` のコード生成本体 (v4、`docs/schema_v4.md` §3 参照)。
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
//! 辺は「マクロが生成する型」なのでノードと異なり**固有 impl (inherent impl)
//! で読み取り API を生やせる** (`docs/schema_v4.md` §3.2「辺 — 種別型
//! (マクロ生成) への固有 impl」)。ノード型はユーザーが `graph_schema!` の外で
//! 宣言する型で複数 schema 間の共有もありうるため、代わりに `{Schema}Node`
//! トレイトの関連関数として生やす (README/`gen_node_trait_and_impls` 参照)。
//!
//! where 制約 → 戻り型の対応表 (`docs/schema_v4.md` §3.2):
//! - `each X: 1`    -> `of` は直接参照 (未知キーはパニック、非パニック版 `get_of`)
//! - `each X: 0..1` -> `of` は `Option`
//! - 制約なし        -> `of` は `Vec`
//! - `unique pair`  -> `between` は `Option`、それ以外は `Vec`

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Path};

use crate::naming::{plural_field_name, to_snake_case};
use crate::schema_dsl::{EachSpec, EdgeDecl, NodeDecl, SchemaInput};

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
        // `node Type(plural);` で明示指定があればそれを内部ストレージの
        // フィールド名に使う。省略時は素朴な複数形化 (`+ "s"`) に
        // フォールバックする。
        let field_ident = match &decl.plural {
            Some(plural) => Ident::new(&plural.to_string(), plural.span()),
            None => Ident::new(&plural_field_name(&type_name), span),
        };
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
/// パラメータで表現する。
struct EdgeInfo<'a> {
    kind: &'a Ident,
    /// エッジ種別の newtype キー型名 (`BossId`)。
    id_ident: Ident,
    /// 内部ストレージのフィールド名 = builder 追加メソッド名 = 単数形
    /// snake_case (`boss`)。`Kind` は既に PascalCase (型名) なので
    /// ノードと同じ `to_snake_case` 変換で導出できる。
    accessor_ident: Ident,
    /// from 索引の内部フィールド名 (`boss_from_index`)。freeze 時に構築する
    /// (`docs/schema_v4.md` §3.2)。
    from_index_ident: Ident,
    from_node: &'a NodeInfo,
    to_node: &'a NodeInfo,
    /// エッジ属性型への参照。ユーザーがマクロの外で宣言した型を指すだけで、
    /// このマクロは属性型そのものを生成しない。
    attrs_ty: Option<Path>,
    each: Option<EachSpec>,
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
        schema_name,
        &node_infos,
        &edge_infos,
    );
    let node_trait_and_impls =
        gen_node_trait_and_impls(&node_trait_ident, &builder_ident, schema_name, &node_infos);
    let edge_trait_and_impls =
        gen_edge_trait_and_impls(&edge_trait_ident, &builder_ident, &edge_infos);
    let edge_query_impls = edge_infos
        .iter()
        .map(|e| gen_edge_query_impl(schema_name, e));

    quote! {
        #(#node_id_defs)*
        #(#edge_id_defs)*
        #(#edge_tuple_struct_defs)*
        #violation_def
        #schema_struct_def
        #schema_impl
        #builder_struct_def
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
    let from_index_ident = format_ident!("{}_from_index", accessor_ident);
    EdgeInfo {
        kind,
        id_ident: format_ident!("{}Id", kind, span = span),
        accessor_ident,
        from_index_ident,
        from_node,
        to_node,
        attrs_ty: decl.attrs_ty.clone(),
        each: decl.constraints.each.as_ref().map(|(_, spec)| *spec),
        unique_pair: decl.constraints.unique_pair,
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
    builder_ident: &Ident,
    schema_name: &Ident,
    nodes: &[NodeInfo],
) -> TokenStream {
    let node_impls = nodes.iter().map(|n| {
        let ty = &n.type_ident;
        let id_ty = &n.id_ident;
        let accessor = &n.accessor_ident;
        let field = &n.field_ident;
        quote! {
            impl #node_trait_ident for #ty {
                type Id = #id_ty;

                fn insert_into(self, b: &mut #builder_ident, key: String) -> Self::Id {
                    let id = #id_ty(key);
                    b.#accessor(id.clone(), self);
                    id
                }

                fn get<'g>(g: &'g #schema_name, id: &Self::Id) -> Option<&'g Self>
                where
                    Self: 'g,
                {
                    g.#field.get(id)
                }

                fn ids<'g>(g: &'g #schema_name) -> impl Iterator<Item = &'g Self::Id>
                where
                    Self: 'g,
                {
                    g.#field.ids()
                }

                fn iter<'g>(g: &'g #schema_name) -> impl Iterator<Item = (&'g Self::Id, &'g Self)>
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
        /// 書き込み ( `insert_into` ) は `{Builder}::insert` 経由、読み取り
        /// (`get`/`ids`/`iter`) は `Type::method(&g, ..)` の形で使う想定
        /// (このトレイトを `use` でスコープに入れておく必要がある)。
        /// 利用者が `insert_into` を直接呼ぶことは想定しない。
        pub trait #node_trait_ident: Sized {
            type Id;
            /// `self` を `b` の対応する内部ストレージへ格納し、発行された
            /// キーを返す。
            fn insert_into(self, b: &mut #builder_ident, key: String) -> Self::Id;
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
fn gen_edge_trait_and_impls(
    edge_trait_ident: &Ident,
    builder_ident: &Ident,
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let edge_impls = edges.iter().map(|e| {
        let kind = e.kind;
        let id_ty = &e.id_ident;
        let accessor = &e.accessor_ident;
        quote! {
            impl #edge_trait_ident for #kind {
                type Id = #id_ty;

                fn insert_into(self, b: &mut #builder_ident, key: String) -> Self::Id {
                    let id = #id_ty(key);
                    b.#accessor(id.clone(), self);
                    id
                }
            }
        }
    });

    quote! {
        /// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
        /// この trait のメソッドを直接呼ぶことは想定しない
        /// (`{Builder}::add` 経由で使う)。
        pub trait #edge_trait_ident: Sized {
            type Id;
            fn insert_into(self, b: &mut #builder_ident, key: String) -> Self::Id;
        }

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

/// エッジ種別ごとのタプル struct とその `from`/`to`/`payload` メソッド。
///
/// `docs/schema_v4.md` §3.1: 「タプル struct として実在し、マクロ外でも
/// `Boss(from_id, to_id, payload)` で普通に構築できる」(原則6: 消去可能な
/// 拡張のみ)。読み取りは位置 (`.0`/`.1`/`.2`) を人間に晒さず、固定語彙の
/// メソッドを生成する。
fn gen_edge_tuple_structs(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|e| {
            let kind = e.kind;
            let from_id = &e.from_node.id_ident;
            let to_id = &e.to_node.id_ident;

            let (struct_def, payload_method) = match &e.attrs_ty {
                None => (
                    quote! { pub struct #kind(pub #from_id, pub #to_id); },
                    quote! {},
                ),
                Some(attrs) => (
                    quote! { pub struct #kind(pub #from_id, pub #to_id, pub #attrs); },
                    quote! {
                        /// この辺の積み荷 (属性値) を返す。
                        pub fn payload(&self) -> &#attrs {
                            &self.2
                        }
                    },
                ),
            };

            quote! {
                #[derive(Debug, Clone, PartialEq)]
                #struct_def

                impl #kind {
                    /// この辺の始点キーを返す。
                    pub fn from(&self) -> &#from_id {
                        &self.0
                    }
                    /// この辺の終点キーを返す。
                    pub fn to(&self) -> &#to_id {
                        &self.1
                    }
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
/// - 未知の端点参照 (`{Kind}UnknownSource`/`{Kind}UnknownTarget`) はどの辺
///   (`edge: {Kind}Id`) がどの端点キー (`source`/`target`) を参照している
///   かを両方型付きで持つ (`docs/design_principles.md` 原則1: stringly-typed
///   API 禁止)。
/// - `each` 制約違反 (`{Kind}EachViolation`) と `unique pair` 違反
///   (`{Kind}UniquePairViolation`) は宣言されている場合のみ生成する。
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
        let from_id = &edge.from_node.id_ident;
        let to_id = &edge.to_node.id_ident;
        let from_type_str = edge.from_node.type_ident.to_string();
        let to_type_str = edge.to_node.type_ident.to_string();

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

        if let Some(spec) = edge.each {
            let expected_str = match spec {
                EachSpec::One => "ちょうど1",
                EachSpec::ZeroOrOne => "0または1",
            };
            let v = edge.each_violation_variant();
            edge_variants.push(quote! {
                /// このエッジ種別の `each` 制約違反。
                #v { source: #from_id, count: usize }
            });
            edge_display_arms.push(quote! {
                #violation_ident::#v { source, count } => write!(
                    f,
                    "each制約違反: エッジ `{}` は {} {:?} について本数 {} を期待しますが実際は {} 本です",
                    #kind_str, #from_type_str, source, #expected_str, count
                )
            });
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
        let from_index = &e.from_index_ident;
        let id_ty = &e.id_ident;
        let kind = e.kind;
        let from_id = &e.from_node.id_ident;
        quote! {
            #accessor: graphite::KeyedTable<#id_ty, #kind>,
            /// 始点キー -> この始点から出るエッジキーの一覧 (freeze 時に構築)。
            #from_index: std::collections::HashMap<#from_id, Vec<#id_ty>>
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

            #freeze_body
        }
    }
}

/// 辺1種別分の freeze 検査本体を生成する。
///
/// 手順:
/// 1. `Vec<(KindId, Kind)>` から `KeyedTable<KindId, Kind>` を構築 (重複キー
///    は `{Kind}DuplicateKey` 違反として記録し、その要素は捨てる)。
/// 2. 生き残った各辺について端点 (from/to) がそれぞれのノード表に実在するか
///    検査する (`{Kind}UnknownSource`/`{Kind}UnknownTarget`)。両端点とも
///    正当な辺だけを from 索引 (`{accessor}_from_index`) に積む。
///    `unique pair` 制約があれば、同じ (from, to) の対が2回目に現れた時点で
///    `{Kind}UniquePairViolation` を記録する。
/// 3. `each` 制約があれば、from 索引の本数を検査する
///    (`each 1` は生存する全始点ノードについて、`each 0..1` は索引に現れた
///    始点についてのみ検査すればよい — 現れない始点は本数0で自動的に合法)。
fn gen_edge_freeze_block(violation_ident: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let accessor = &edge.accessor_ident;
    let from_index = &edge.from_index_ident;
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

    let each_check = match edge.each {
        Some(EachSpec::One) => {
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
        Some(EachSpec::ZeroOrOne) => {
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
        None => quote! {},
    };

    quote! {
        let mut #accessor: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.#accessor {
            if !#accessor.insert(id.clone(), value) {
                __violations.push(#violation_ident::#dup_key(id));
            }
        }

        let mut #from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        #seen_pairs_decl
        for (id, edge) in #accessor.iter() {
            let from = edge.from();
            let to = edge.to();
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
            }
        }
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

    let edge_blocks = edges.iter().map(|e| gen_edge_freeze_block(violation_ident, e));

    let node_field_names = nodes.iter().map(|n| &n.field_ident);
    let edge_field_names = edges.iter().map(|e| &e.accessor_ident);
    let edge_from_index_names = edges.iter().map(|e| &e.from_index_ident);

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
                #(#edge_from_index_names,)*
            })
        }

        /// 最初の1件の違反で `Err` になる版。実装は
        /// `freeze_collecting` に委譲する。
        fn freeze(self) -> Result<#schema_name, #violation_ident> {
            self.freeze_collecting().map_err(|mut violations| violations.remove(0))
        }
    }
}

/// エッジ種別1つ分の読み取りAPI (`Kind` への固有 impl)。
/// `docs/schema_v4.md` §3.2 の where 制約 → 戻り型対応表をそのまま実装する。
fn gen_edge_query_impl(schema_name: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let kind = edge.kind;
    let id_ty = &edge.id_ident;
    let accessor = &edge.accessor_ident;
    let from_index = &edge.from_index_ident;
    let from_id = &edge.from_node.id_ident;
    let to_id = &edge.to_node.id_ident;
    let to_field = &edge.to_node.field_ident;
    let to_ty = &edge.to_node.type_ident;

    // `of`/`get_of` の戻り値の型・実装は「積み荷の有無」「each 制約」の
    // 組み合わせで分岐する。これらの関数はいずれも `&self` を取らず
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
                    g.#to_field.get(e.to()).expect("freezeで端点存在を検証済みのはず")
                }
            },
            Some(_) => quote! {
                {
                    let e = g.#accessor.get(#edge_id_expr).expect("from_indexに載っている辺はstorageに必ず存在する");
                    let target = g.#to_field.get(e.to()).expect("freezeで端点存在を検証済みのはず");
                    (target, e.payload())
                }
            },
        }
    };

    let of_and_get_of = match edge.each {
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
                pub fn of<'g>(g: &'g #schema_name, from: &#from_id) -> #of_item_ty {
                    Self::get_of(g, from).unwrap_or_else(|| {
                        panic!(
                            "{}::of: 未知のキーです (このグラフが発行したキーではありません): {:?}",
                            stringify!(#kind), from
                        )
                    })
                }

                /// [`Self::of`] の非パニック版。未知キーは `None` を返す。
                pub fn get_of<'g>(g: &'g #schema_name, from: &#from_id) -> Option<#of_item_ty> {
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
                pub fn of<'g>(g: &'g #schema_name, from: &#from_id) -> Option<#of_item_ty> {
                    let ids = g.#from_index.get(from)?;
                    Some(#resolved)
                }
            }
        }
        None => {
            let resolved = resolve_one(quote! { id });
            quote! {
                /// この辺種別の自然な戻り値 (制約なし → `Vec`)。無い/未知
                /// キーはどちらも空 `Vec` に落ちる。格納順 (構築時の追加順)
                /// を保持する。
                pub fn of<'g>(g: &'g #schema_name, from: &#from_id) -> Vec<#of_item_ty> {
                    match g.#from_index.get(from) {
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
            pub fn between<'g>(g: &'g #schema_name, from: &#from_id, to: &#to_id) -> Option<&'g #kind> {
                g.#from_index
                    .get(from)?
                    .iter()
                    .filter_map(|id| g.#accessor.get(id))
                    .find(|e| e.to() == to)
            }
        }
    } else {
        quote! {
            /// 対 (始点, 終点) で辺を検索する (制約なしなら平行辺を許すため
            /// `Vec`)。格納順 (構築時の追加順) を保持する。
            pub fn between<'g>(g: &'g #schema_name, from: &#from_id, to: &#to_id) -> Vec<&'g #kind> {
                match g.#from_index.get(from) {
                    Some(ids) => ids
                        .iter()
                        .filter_map(|id| g.#accessor.get(id))
                        .filter(|e| e.to() == to)
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
            pub fn get<'g>(g: &'g #schema_name, id: &#id_ty) -> Option<&'g #kind> {
                g.#accessor.get(id)
            }

            #between

            /// 表全体を `(キー, 値)` で走査する。挿入順 (構築時の追加順) を
            /// 保持する (`KeyedTable` の仕様)。
            pub fn iter(g: &#schema_name) -> impl Iterator<Item = (&#id_ty, &#kind)> {
                g.#accessor.iter()
            }

            /// この辺種別の全キーを列挙する。挿入順 (構築時の追加順) を
            /// 保持する (`KeyedTable` の仕様)。
            pub fn ids(g: &#schema_name) -> impl Iterator<Item = &#id_ty> {
                g.#accessor.ids()
            }

            /// この辺種別に含まれる辺の本数。
            pub fn len(g: &#schema_name) -> usize {
                g.#accessor.len()
            }
        }
    }
}
