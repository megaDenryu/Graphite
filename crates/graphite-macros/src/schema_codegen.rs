//! `graph_schema!` のコード生成本体。
//!
//! 生成物の形は `crates/graphite/tests/orgchart_handwritten.rs` (フェーズ2で
//! 手書きしたテンプレート) に準拠する。差異がある箇所はこのファイル末尾の
//! コメント、および README の「手書きテンプレートとの差異」節を参照。
//!
//! 多重度→コード片の変換表 (フェーズ2引き継ぎ知見):
//! - `(1)`    -> 参照直接返し (freeze で必須検査、アクセサは正当キー前提でパニック可)
//! - `(0..1)` -> `Option`
//! - `(0..*)` -> `Vec`

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Path};

use crate::naming::{plural_field_name, to_pascal_case, to_snake_case};
use crate::schema_dsl::{EdgeDecl, Multiplicity, NodeDecl, SchemaInput};

/// ノード宣言 1 つ分の、生成コードで使う識別子一式。
///
/// ノード値の型 (`Employee` 等) はユーザーが `graph_schema!` の外で宣言した
/// 普通の struct への参照であり、このマクロは生成しない
/// (`docs/edge_syntax_v3.md` 参照)。マクロが生成するのはグラフ機械
/// (newtype キー・ストレージ・builder・アクセサ・違反 enum) だけ。
struct NodeInfo {
    /// ノード値の型名 (`Employee`)。ユーザー宣言型への参照。
    type_ident: Ident,
    /// newtype キー型名 (`EmployeeId`)。
    id_ident: Ident,
    /// 内部ストレージの複数形フィールド名 (`employees`)。
    field_ident: Ident,
    /// アクセサ/builder メソッド名 = 単数形 snake_case (`employee`)。
    accessor_ident: Ident,
}

impl NodeInfo {
    fn new(decl: &NodeDecl) -> Self {
        let type_name = decl.name.to_string();
        let span = decl.name.span();
        // `node Type(plural);` で明示指定があればそれを内部ストレージの
        // フィールド名に使う。省略時は素朴な複数形化 (`+ "s"`) に
        // フォールバックする (README「手書きテンプレートとの差異」節参照)。
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
/// パラメータで表現する (`decl`/`NodeInfo` 自体は `schema` 由来のより長い
/// ライフタイムを持つが、共変性により短い方へ自動的に縮小される)。
struct EdgeInfo<'a> {
    decl: &'a EdgeDecl,
    label: Ident,
    from_node: &'a NodeInfo,
    to_node: &'a NodeInfo,
    /// エッジ属性型への参照 (`edge label: From -[Ty]-> To (mult);` の
    /// `Ty`)。ユーザーがマクロの外で宣言した型を指すだけで、このマクロは
    /// 属性型そのものを生成しない (`docs/edge_syntax_v3.md` 参照)。
    attrs_ty: Option<Path>,
}

impl<'a> EdgeInfo<'a> {
    /// エッジラベルの PascalCase 化 (違反 enum バリアント名の基底)。
    fn pascal(&self) -> String {
        to_pascal_case(&self.label.to_string())
    }

    /// 項目k (フェーズ5): 多重度違反のバリアント名。エッジ単位で型付けする
    /// (`{Label}Multiplicity { source: FromId, count: usize }`)。
    /// `(0..*)` には多重度違反という概念自体が無いので呼び出し元は
    /// `Multiplicity::One` / `ZeroOrOne` のときだけこれを使うこと。
    fn multiplicity_variant(&self) -> Ident {
        format_ident!("{}Multiplicity", self.pascal(), span = self.label.span())
    }

    /// 項目k (フェーズ5): 未知の始点キー参照のバリアント名
    /// (`{Label}UnknownSource { key: FromId }`)。
    fn unknown_source_variant(&self) -> Ident {
        format_ident!("{}UnknownSource", self.pascal(), span = self.label.span())
    }

    /// 項目k (フェーズ5): 未知の終点キー参照のバリアント名
    /// (`{Label}UnknownTarget { key: ToId }`)。
    fn unknown_target_variant(&self) -> Ident {
        format_ident!("{}UnknownTarget", self.pascal(), span = self.label.span())
    }
}

pub fn generate(schema: &SchemaInput) -> TokenStream {
    let schema_name = &schema.schema_name;
    let violation_ident = format_ident!("{}Violation", schema_name);
    let builder_ident = format_ident!("{}Builder", schema_name);
    // v3 (`docs/graph_literal_v3.md` §3): `graph!` が値の型名を一切知らずに
    // 済むようにするための、ノード挿入用トレイト。名前は schema ごとに
    // ユニークにする (`node_trait_ident`/`insert_into` の命名判断は
    // `gen_node_trait_and_impls` のドキュメントコメント参照)。
    let node_trait_ident = format_ident!("{}Node", schema_name);

    let node_infos: Vec<NodeInfo> = schema.nodes.iter().map(NodeInfo::new).collect();

    let edge_infos: Vec<EdgeInfo> = schema
        .edges
        .iter()
        .map(|edge| {
            let from_node = node_infos
                .iter()
                .find(|n| n.type_ident == edge.from)
                .expect("validate() を通過していれば必ず見つかるはず");
            let to_node = node_infos
                .iter()
                .find(|n| n.type_ident == edge.to)
                .expect("validate() を通過していれば必ず見つかるはず");
            EdgeInfo {
                decl: edge,
                label: edge.label.clone(),
                from_node,
                to_node,
                attrs_ty: edge.attrs_ty.clone(),
            }
        })
        .collect();

    let node_id_defs = gen_node_id_types(&node_infos);
    let violation_def = gen_violation_enum(&violation_ident, &node_infos, &edge_infos);
    let schema_struct_def = gen_schema_struct(schema_name, &node_infos, &edge_infos);
    let schema_impl = gen_schema_impl(
        schema_name,
        &violation_ident,
        &builder_ident,
        &node_infos,
        &edge_infos,
    );
    let builder_struct_def = gen_builder_struct(&builder_ident, &node_infos, &edge_infos);
    let builder_impl = gen_builder_impl(
        schema_name,
        &builder_ident,
        &violation_ident,
        &node_trait_ident,
        &node_infos,
        &edge_infos,
    );
    let node_trait_and_impls =
        gen_node_trait_and_impls(&node_trait_ident, &builder_ident, &node_infos);

    quote! {
        #(#node_id_defs)*
        #violation_def
        #schema_struct_def
        #schema_impl
        #builder_struct_def
        #node_trait_and_impls
        #builder_impl
    }
}

/// v3 (`docs/graph_literal_v3.md` §3, §4) が要求する「ノード挿入用トレイト」
/// とその各ノード型への impl を生成する。
///
/// ## 背景: なぜこのトレイトが必要か
///
/// v2 までの `graph!` はノード項を `key: Type { .. }` と書かせていたため、
/// `Type` という型名トークンから `to_snake_case` でビルダーメソッド名
/// (`b.person(..)`) を機械的に導出できていた。v3 (`docs/graph_literal_v3.md`)
/// はノード項を `key = 式` に変え、値の型をマクロが一切パースしなくなる
/// (式の型は rustc の型推論に委ねる、という設計上の決定)。その結果
/// `graph!` はもはや「どのビルダーメソッドを呼ぶべきか」を型名から逆引き
/// できないため、値の型さえ分かれば正しい内部ストレージへ振り分けられる
/// **総称メソッド**が要る。この trait 境界を介した単相化がそれを実現する
/// (実行時のリフレクション・型判別・`dyn` ディスパッチは一切無い。
/// 原則5: ゼロコスト志向)。
///
/// ## 命名判断 (原則3: std 命名規約準拠) — variance の理由を明記
///
/// `docs/graph_literal_v3.md` §3 のイメージでは trait 名 `OrgNode`・
/// メソッド名 `insert_into` が示されていた。ここでは:
///
/// - **trait 名は `{Schema}Node` とした** (イメージの `OrgNode` から
///   `OrgChartNode` のようにスキーマ名を冠する形に変更)。理由:
///   同一モジュール内に複数の schema が存在するとき、両方が同じ
///   `NodeTrait` という名前を使おうとして衝突する。README「同一モジュール
///   内で複数 schema がノード型を共有する場合の制約」で `{Node}Id` の
///   衝突は既知の制約として明記済みだが、trait 名はモジュール単位で必ず
///   1つしか存在できないただの識別子なので、schema 名を含めておけば
///   この衝突は最初から起きない (キー型と同じ理由でスキーマ名を
///   プレフィックスにする、というこのファイル全体の既存の命名方針
///   `{Schema}Violation`/`{Schema}Builder` と揃える)。
/// - **メソッド名は `insert_into` のままイメージを採用**。`Into<T>` の
///   `into(self) -> T` は純粋な変換だが、ここでは「`self` を builder に
///   格納し、発行されたキーを返す」という副作用を伴う操作であり、単純な
///   変換ではない。それでも「target.insert_into(dest)」という読み方
///   (「self をどこに insert するか」を引数で示す) は Rust の既存の
///   `*_into` 命名慣習 (`Write::write_all` 等ではなく、
///   むしろ `TryInto`/`Cow::into_owned` のような「変換先を明示する」系より
///   `HashMap::insert` の「格納先のメソッドとして呼ぶ」系に近い) からやや
///   外れるが、`{Builder}::insert` という総称メソッド名と対応が付く
///   (`insert` が最終的に呼ぶのが `insert_into`) 分かりやすさを優先し、
///   イメージ通りとした。
/// - **`{Builder}::insert` はイメージ通り採用**。`HashMap::insert`/
///   `Vec::insert` と同じ動詞であり、利用者が Rust の直感で予測できる。
fn gen_node_trait_and_impls(
    node_trait_ident: &Ident,
    builder_ident: &Ident,
    nodes: &[NodeInfo],
) -> TokenStream {
    let node_impls = nodes.iter().map(|n| {
        let ty = &n.type_ident;
        let id_ty = &n.id_ident;
        let accessor = &n.accessor_ident;
        quote! {
            impl #node_trait_ident for #ty {
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
        /// `graph!` の `insert` 経由のノード挿入で使うトレイト境界。
        /// 利用者がこの trait のメソッドを直接呼ぶことは想定しない
        /// (`{Builder}::insert` 経由で使う)。命名判断はこの関数
        /// (`gen_node_trait_and_impls`) のドキュメントコメント参照。
        pub trait #node_trait_ident: Sized {
            type Id;
            /// `self` を `b` の対応する内部ストレージへ格納し、発行された
            /// キーを返す。
            fn insert_into(self, b: &mut #builder_ident, key: String) -> Self::Id;
        }

        #(#node_impls)*
    }
}

/// ノード値の型 (`Employee` 等) はユーザー宣言への参照なので生成しない。
/// ここで生成するのは newtype キー型だけ (`EmployeeId(pub String)`)。
/// このキー型は内部で `HashMap` のキーとして使うため `Hash + Eq` を要求する
/// (ノード値の型自体には macro からの trait 要求は一切無い。README
/// 「エッジ属性型に対する trait 要求」節と対の説明を参照)。
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

/// 違反 enum を生成する。
///
/// 項目k (フェーズ5、破壊的変更): 以前は多重度違反のキーを `source: String`
/// (`Debug` 表現) に落とす妥協をしていたが、原則1 (型の strictness /
/// stringly-typed API 禁止) に反するため廃止し、エッジごとに型付きの
/// バリアントを生成する形へ置き換えた
/// (`docs/design_principles.md` 原則1 参照)。ノード重複 (`Duplicate{Node}`)
/// はエッジと無関係な純粋ノード単位の概念なのでノードごとのバリアントを
/// 維持するが、未知キー参照・多重度違反はどのエッジで起きたかが本質的な
/// 情報のため、エッジ単位の専用バリアント (`{Label}UnknownSource` /
/// `{Label}UnknownTarget` / `{Label}Multiplicity`) に置き換え、旧来の
/// ノード単位の汎用 `Unknown{Node}` / 文字列キー `MultiplicityViolation`
/// とは共存させない。
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
        let label_str = edge.label.to_string();
        let from_id = &edge.from_node.id_ident;
        let to_id = &edge.to_node.id_ident;
        let from_type_str = edge.from_node.type_ident.to_string();
        let to_type_str = edge.to_node.type_ident.to_string();

        let unk_src = edge.unknown_source_variant();
        edge_variants.push(quote! {
            /// このエッジ種別が未知の始点キーを参照している。
            #unk_src { key: #from_id }
        });
        edge_display_arms.push(quote! {
            #violation_ident::#unk_src { key } => write!(
                f,
                "未知のキーが参照されています (エッジ `{}` の始点, {}): {:?}",
                #label_str, #from_type_str, key
            )
        });

        let unk_dst = edge.unknown_target_variant();
        edge_variants.push(quote! {
            /// このエッジ種別が未知の終点キーを参照している。
            #unk_dst { key: #to_id }
        });
        edge_display_arms.push(quote! {
            #violation_ident::#unk_dst { key } => write!(
                f,
                "未知のキーが参照されています (エッジ `{}` の終点, {}): {:?}",
                #label_str, #to_type_str, key
            )
        });

        let expected_str = match edge.decl.mult {
            Multiplicity::One => Some("ちょうど1"),
            Multiplicity::ZeroOrOne => Some("0または1"),
            Multiplicity::ZeroOrMany => None,
        };
        if let Some(expected_str) = expected_str {
            let mult = edge.multiplicity_variant();
            edge_variants.push(quote! {
                /// このエッジ種別の多重度違反。
                #mult { source: #from_id, count: usize }
            });
            edge_display_arms.push(quote! {
                #violation_ident::#mult { source, count } => write!(
                    f,
                    "多重度違反: エッジ `{}` は {} {:?} について多重度 {} を期待しますが実際は {} 本です",
                    #label_str, #from_type_str, source, #expected_str, count
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

/// エッジ 1 本分の格納値の型 (属性の有無・多重度で分岐)。
fn edge_stored_value_type(edge: &EdgeInfo<'_>) -> TokenStream {
    let to_id = &edge.to_node.id_ident;
    match (&edge.decl.mult, &edge.attrs_ty) {
        (Multiplicity::One, None) => quote! { #to_id },
        (Multiplicity::One, Some(attrs)) => quote! { (#to_id, #attrs) },
        (Multiplicity::ZeroOrOne, None) => quote! { #to_id },
        (Multiplicity::ZeroOrOne, Some(attrs)) => quote! { (#to_id, #attrs) },
        (Multiplicity::ZeroOrMany, None) => quote! { Vec<#to_id> },
        (Multiplicity::ZeroOrMany, Some(attrs)) => quote! { Vec<(#to_id, #attrs)> },
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
        quote! { #field: std::collections::HashMap<#id, #ty> }
    });
    let edge_fields = edges.iter().map(|e| {
        let label = &e.label;
        let from_id = &e.from_node.id_ident;
        let value_ty = edge_stored_value_type(e);
        quote! { #label: std::collections::HashMap<#from_id, #value_ty> }
    });

    quote! {
        /// 凍結済み図式グラフ。構築後は不変 (可変 API は公開しない)。
        pub struct #schema_name {
            #(#node_fields,)*
            #(#edge_fields,)*
        }
    }
}

fn gen_schema_impl(
    schema_name: &Ident,
    violation_ident: &Ident,
    builder_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_accessors = nodes.iter().map(|n| {
        let accessor = &n.accessor_ident;
        let field = &n.field_ident;
        let id_ty = &n.id_ident;
        let ty = &n.type_ident;
        let ids_iter = gen_node_id_iter(n);
        quote! {
            pub fn #accessor(&self, id: &#id_ty) -> Option<&#ty> {
                self.#field.get(id)
            }
            #ids_iter
        }
    });

    let edge_accessors = edges.iter().map(|e| gen_edge_accessor(schema_name, e));

    quote! {
        impl #schema_name {
            /// builder をクロージャに貸し出し、戻ったら凍結して図式適合
            /// (端点種別・多重度) を一括検査する。最初の1件の違反で `Err` に
            /// なる (複数の違反を全件見たい場合は [`Self::create_collecting`]
            /// を使う)。
            pub fn create<F>(f: F) -> Result<Self, #violation_ident>
            where
                F: for<'b> FnOnce(&'b mut #builder_ident),
            {
                let mut builder = #builder_ident::new();
                f(&mut builder);
                builder.freeze()
            }

            /// 項目g (フェーズ5): `create` の複数違反収集版。builder を
            /// クロージャに貸し出し、戻ったら凍結して図式適合を検査する点は
            /// `create` と同じだが、最初の1件で打ち切らず全違反を
            /// `Vec` に集めて返す。組織図の全違反を一覧表示するような
            /// 検証系ユースケース向け。
            pub fn create_collecting<F>(f: F) -> Result<Self, Vec<#violation_ident>>
            where
                F: for<'b> FnOnce(&'b mut #builder_ident),
            {
                let mut builder = #builder_ident::new();
                f(&mut builder);
                builder.freeze_collecting()
            }

            #(#node_accessors)*
            #(#edge_accessors)*
        }
    }
}

/// ノード種別 1 つ分の、全キーを列挙するイテレータアクセサ (項目2: クエリAPI)。
fn gen_node_id_iter(node: &NodeInfo) -> TokenStream {
    let ids_ident = format_ident!("{}_ids", node.accessor_ident);
    let field = &node.field_ident;
    let id_ty = &node.id_ident;
    quote! {
        /// このノード種別の全キーを列挙する。
        pub fn #ids_ident(&self) -> impl Iterator<Item = &#id_ty> {
            self.#field.keys()
        }
    }
}

/// エッジ 1 種別につき、ビュー (`graphite::EdgeOne`/`EdgeOneWith`/
/// `EdgeOption`/`EdgeOptionWith`/`EdgeMany`/`EdgeManyWith`。多重度×属性有無
/// で 6 択) を返す薄いメソッド 1 つだけを生成する
/// (`docs/edge_view_api.md` §3.2)。旧版にあった `try_{label}`/`{label}_id`/
/// `try_{label}_id`/`{label}_ids`/`{label}_pairs` という導出名メソッド群は
/// 全廃した (痕跡なし)。操作の語彙 (`of`/`get`/`id_of`/`get_id`/`ids_of`/
/// `iter`/`len`/`is_empty`) とその rustdoc (`# Panics` 含む) はビュー型側
/// (`crates/graphite/src/edge_view.rs`) に集約されているため、ここで
/// 生成するアクセサ自体には多重度・属性の有無以上のドキュメントを書かない。
///
/// 多重度 (1) のビュー (`EdgeOne`/`EdgeOneWith`) だけがパニックする
/// `of`/`id_of` を持つため、旧アクセサと同等の情報量のパニック文言を組み立て
/// られるよう、コンストラクタにラベル名・スキーマ名を追加で渡す。他の 4 型
/// はパニックしないため、エッジ表・相手ノードストレージへの参照 2 つだけで
/// 構築する。
fn gen_edge_accessor(schema_name: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let label = &edge.label;
    let from_id = &edge.from_node.id_ident;
    let to_id = &edge.to_node.id_ident;
    let to_field = &edge.to_node.field_ident;
    let to_ty = &edge.to_node.type_ident;
    let label_str = label.to_string();
    let schema_name_str = schema_name.to_string();

    match (&edge.decl.mult, &edge.attrs_ty) {
        (Multiplicity::One, None) => quote! {
            /// 多重度 (1)・属性なしのエッジビューを返す。
            pub fn #label(&self) -> graphite::EdgeOne<'_, #from_id, #to_id, #to_ty> {
                graphite::EdgeOne::new(&self.#label, &self.#to_field, #label_str, #schema_name_str)
            }
        },
        (Multiplicity::One, Some(attrs_ty)) => quote! {
            /// 多重度 (1)・属性ありのエッジビューを返す。
            pub fn #label(&self) -> graphite::EdgeOneWith<'_, #from_id, #to_id, #to_ty, #attrs_ty> {
                graphite::EdgeOneWith::new(&self.#label, &self.#to_field, #label_str, #schema_name_str)
            }
        },
        (Multiplicity::ZeroOrOne, None) => quote! {
            /// 多重度 (0..1)・属性なしのエッジビューを返す。
            pub fn #label(&self) -> graphite::EdgeOption<'_, #from_id, #to_id, #to_ty> {
                graphite::EdgeOption::new(&self.#label, &self.#to_field)
            }
        },
        (Multiplicity::ZeroOrOne, Some(attrs_ty)) => quote! {
            /// 多重度 (0..1)・属性ありのエッジビューを返す。
            pub fn #label(&self) -> graphite::EdgeOptionWith<'_, #from_id, #to_id, #to_ty, #attrs_ty> {
                graphite::EdgeOptionWith::new(&self.#label, &self.#to_field)
            }
        },
        (Multiplicity::ZeroOrMany, None) => quote! {
            /// 多重度 (0..*)・属性なしのエッジビューを返す。
            pub fn #label(&self) -> graphite::EdgeMany<'_, #from_id, #to_id, #to_ty> {
                graphite::EdgeMany::new(&self.#label, &self.#to_field)
            }
        },
        (Multiplicity::ZeroOrMany, Some(attrs_ty)) => quote! {
            /// 多重度 (0..*)・属性ありのエッジビューを返す。
            pub fn #label(&self) -> graphite::EdgeManyWith<'_, #from_id, #to_id, #to_ty, #attrs_ty> {
                graphite::EdgeManyWith::new(&self.#label, &self.#to_field)
            }
        },
    }
}

fn edge_builder_value_tuple_type(edge: &EdgeInfo<'_>) -> TokenStream {
    let from_id = &edge.from_node.id_ident;
    let to_id = &edge.to_node.id_ident;
    match &edge.attrs_ty {
        None => quote! { (#from_id, #to_id) },
        Some(attrs) => quote! { (#from_id, #to_id, #attrs) },
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
        let label = &e.label;
        let value_ty = edge_builder_value_tuple_type(e);
        quote! { #label: Vec<#value_ty> }
    });

    quote! {
        /// 構築用 builder。凍結 (`freeze`) までは多重度検査を一切行わない。
        pub struct #builder_ident {
            #(#node_fields,)*
            #(#edge_fields,)*
        }
    }
}

fn gen_builder_impl(
    schema_name: &Ident,
    builder_ident: &Ident,
    violation_ident: &Ident,
    node_trait_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_field_inits = nodes.iter().map(|n| {
        let field = &n.field_ident;
        quote! { #field: Vec::new() }
    });
    let edge_field_inits = edges.iter().map(|e| {
        let label = &e.label;
        quote! { #label: Vec::new() }
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
        let label = &e.label;
        let from_id = &e.from_node.id_ident;
        let to_id = &e.to_node.id_ident;
        match &e.attrs_ty {
            None => quote! {
                pub fn #label(&mut self, from: #from_id, to: #to_id) -> &mut Self {
                    self.#label.push((from, to));
                    self
                }
            },
            Some(attrs_ty) => quote! {
                pub fn #label(&mut self, from: #from_id, to: #to_id, attrs: #attrs_ty) -> &mut Self {
                    self.#label.push((from, to, attrs));
                    self
                }
            },
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

            /// v3 (`docs/graph_literal_v3.md` §3): 型名付きメソッド
            /// (`b.#accessor(id, value)` 群、上記 `#node_methods`) の総称版。
            /// `graph!` はノード項の値の型を一切パースしないため
            /// (`key = 式` の「式」でしかない)、このメソッドで値の型
            /// (`N: #node_trait_ident`) から正しい内部ストレージへの
            /// 振り分けを rustc の型推論任せにする。命名判断・trait の形は
            /// `gen_node_trait_and_impls` のドキュメントコメント参照。
            pub fn insert<N: #node_trait_ident>(&mut self, key: impl Into<String>, value: N) -> N::Id {
                value.insert_into(self, key.into())
            }

            #freeze_body
        }
    }
}

fn gen_freeze_body(
    schema_name: &Ident,
    violation_ident: &Ident,
    nodes: &[NodeInfo],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_map_builds = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let id_ty = &n.id_ident;
        let ty = &n.type_ident;
        let dup_variant = n.dup_variant();
        quote! {
            let mut #field: std::collections::HashMap<#id_ty, #ty> = std::collections::HashMap::new();
            for (id, value) in self.#field {
                if #field.contains_key(&id) {
                    __violations.push(#violation_ident::#dup_variant(id));
                    continue;
                }
                #field.insert(id, value);
            }
        }
    });

    let edge_blocks = edges.iter().map(|e| gen_edge_freeze_block(violation_ident, e));

    let node_field_names = nodes.iter().map(|n| &n.field_ident);
    let edge_field_names = edges.iter().map(|e| &e.label);

    quote! {
        /// 項目g (フェーズ5): 検証ロジックの実体。最初の1件で打ち切らず
        /// 全違反を `Vec` に集めて返す。`freeze` (単一エラー版) はこちらに
        /// 委譲し先頭の1件を取り出すだけの薄いラッパーにすることで、
        /// 検証ロジックが二重実装にならないようにしている。
        fn freeze_collecting(self) -> Result<#schema_name, Vec<#violation_ident>> {
            let mut __violations: Vec<#violation_ident> = Vec::new();

            #(#node_map_builds)*
            #(#edge_blocks)*

            if !__violations.is_empty() {
                return Err(__violations);
            }

            Ok(#schema_name {
                #(#node_field_names,)*
                #(#edge_field_names,)*
            })
        }

        /// 最初の1件の違反で `Err` になる版。実装は
        /// `freeze_collecting` に委譲する。
        fn freeze(self) -> Result<#schema_name, #violation_ident> {
            self.freeze_collecting().map_err(|mut violations| violations.remove(0))
        }
    }
}

/// 項目g (フェーズ5): エッジ1本分の freeze 検査を「継続収集」スタイルで
/// 生成する。以前の早期 `return Err(..)` は最初の1件で打ち切ってしまい
/// `create_collecting` の全件収集ができないため、`__violations.push(..); continue;`
/// (異常な行はスキップして次へ) に置き換えた。
///
/// 終点キーが未知の行を multiplicity のカウント対象から単純に除外すると、
/// 「終点が壊れているだけ」の1つの根本原因から `UnknownTarget` と
/// `Multiplicity` の2件の違反が二重に生えてしまう (始点は正当なのに
/// 見かけ上0本になるため)。これを避けるため、始点が正当な行は終点の
/// 正否に関わらず「試行された1本」としてカウントする
/// (`#label` マップへの実際の格納は終点が正当な場合のみ)。
fn gen_edge_freeze_block(violation_ident: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let label = &edge.label;
    let count_ident = format_ident!("{}_count", label);
    let from_field = &edge.from_node.field_ident;
    let to_field = &edge.to_node.field_ident;
    let unk_src = edge.unknown_source_variant();
    let unk_dst = edge.unknown_target_variant();
    let value_ty = edge_stored_value_type(edge);

    let (bind_pattern, push_value) = match &edge.attrs_ty {
        None => (quote! { (from, to) }, quote! { to }),
        Some(_) => (quote! { (from, to, attrs) }, quote! { (to, attrs) }),
    };

    match edge.decl.mult {
        Multiplicity::One | Multiplicity::ZeroOrOne => {
            let mult = edge.multiplicity_variant();
            let post_check = match edge.decl.mult {
                Multiplicity::One => quote! {
                    for key in #from_field.keys() {
                        let count = #count_ident.get(key).copied().unwrap_or(0);
                        if count != 1 {
                            __violations.push(#violation_ident::#mult {
                                source: key.clone(),
                                count,
                            });
                        }
                    }
                },
                Multiplicity::ZeroOrOne => quote! {
                    for (key, count) in &#count_ident {
                        if *count > 1 {
                            __violations.push(#violation_ident::#mult {
                                source: key.clone(),
                                count: *count,
                            });
                        }
                    }
                },
                Multiplicity::ZeroOrMany => unreachable!("One/ZeroOrOneの分岐内なので到達しない"),
            };
            quote! {
                let mut #label: std::collections::HashMap<_, #value_ty> = std::collections::HashMap::new();
                let mut #count_ident: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
                for #bind_pattern in self.#label {
                    if !#from_field.contains_key(&from) {
                        __violations.push(#violation_ident::#unk_src { key: from });
                        continue;
                    }
                    if !#to_field.contains_key(&to) {
                        __violations.push(#violation_ident::#unk_dst { key: to });
                        *#count_ident.entry(from.clone()).or_insert(0) += 1;
                        continue;
                    }
                    *#count_ident.entry(from.clone()).or_insert(0) += 1;
                    #label.insert(from, #push_value);
                }
                #post_check
            }
        }
        Multiplicity::ZeroOrMany => quote! {
            let mut #label: std::collections::HashMap<_, #value_ty> = std::collections::HashMap::new();
            for #bind_pattern in self.#label {
                if !#from_field.contains_key(&from) {
                    __violations.push(#violation_ident::#unk_src { key: from });
                    continue;
                }
                if !#to_field.contains_key(&to) {
                    __violations.push(#violation_ident::#unk_dst { key: to });
                    continue;
                }
                #label.entry(from).or_default().push(#push_value);
            }
        },
    }
}
