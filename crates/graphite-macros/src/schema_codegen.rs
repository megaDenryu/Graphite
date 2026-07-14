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
use syn::Ident;

use crate::naming::{plural_field_name, to_pascal_case, to_snake_case};
use crate::schema_dsl::{EdgeDecl, Multiplicity, NodeDecl, SchemaInput};

/// ノード宣言 1 つ分の、生成コードで使う識別子一式。
struct NodeInfo<'a> {
    decl: &'a NodeDecl,
    /// ノード値の型名 (`Employee`)。
    type_ident: Ident,
    /// newtype キー型名 (`EmployeeId`)。
    id_ident: Ident,
    /// 内部ストレージの複数形フィールド名 (`employees`)。
    field_ident: Ident,
    /// アクセサ/builder メソッド名 = 単数形 snake_case (`employee`)。
    accessor_ident: Ident,
}

impl<'a> NodeInfo<'a> {
    fn new(decl: &'a NodeDecl) -> Self {
        let type_name = decl.name.to_string();
        let span = decl.name.span();
        // 項目4 (フェーズ4): `node Type(plural) { .. }` で明示指定があれば
        // それを内部ストレージのフィールド名に使う。省略時は素朴な複数形化
        // (`+ "s"`) にフォールバックする (README「手書きテンプレートとの
        // 差異」節参照)。
        let field_ident = match &decl.plural {
            Some(plural) => Ident::new(&plural.to_string(), plural.span()),
            None => Ident::new(&plural_field_name(&type_name), span),
        };
        NodeInfo {
            decl,
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
    from_node: &'a NodeInfo<'a>,
    to_node: &'a NodeInfo<'a>,
    attrs_type_ident: Option<Ident>,
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
                attrs_type_ident: edge
                    .attrs
                    .as_ref()
                    .map(|_| format_ident!("{}Attrs", to_pascal_case(&edge.label.to_string()))),
            }
        })
        .collect();

    let node_struct_defs = gen_node_structs(&node_infos);
    let attrs_struct_defs = gen_attrs_structs(&edge_infos);
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
        &node_infos,
        &edge_infos,
    );
    let edge_check_macro = gen_edge_check_macro(schema_name, &edge_infos);

    quote! {
        #(#node_struct_defs)*
        #(#attrs_struct_defs)*
        #violation_def
        #schema_struct_def
        #schema_impl
        #builder_struct_def
        #builder_impl
        #edge_check_macro
    }
}

/// 項目5 (フェーズ4): `graph!` が未知のエッジラベルを親切なエラーで検出する
/// ためのハンドシェイク用宣言的マクロを生成する。`graph_schema!` はスキーマの
/// エッジ一覧を知っているのでここで列挙し、`graph!` はスキーマの中身を
/// 知らないまま「スキーマ名からマクロ名を機械的に導出して呼ぶ」だけで済む。
///
/// `macro_rules!` は既定でテキストスコープ (定義箇所より後、同一クレート内
/// でのみ利用可能。モジュール境界は無視されるが、`mod foo;` で外部ファイルを
/// 読み込む場合や別クレートからは `#[macro_export]` や `pub(crate) use` が
/// 必要) のため、同一モジュール (同一ファイル) 内での利用が主ケースとなる
/// (README「未決事項」節に制約を明記)。
fn gen_edge_check_macro(schema_name: &Ident, edges: &[EdgeInfo<'_>]) -> TokenStream {
    let macro_ident = format_ident!("__graphite_check_edge_{}", schema_name);
    let labels: Vec<&Ident> = edges.iter().map(|e| &e.label).collect();
    let label_strs: Vec<String> = labels.iter().map(|l| l.to_string()).collect();
    let available = label_strs.join(", ");
    let schema_name_str = schema_name.to_string();

    quote! {
        /// `graph!` マクロが各エッジ行の脱糖時に呼び出す検査用マクロ。
        /// 存在するエッジラベルなら何もせず、存在しなければ
        /// `compile_error!` で親切なメッセージを出す。利用者が直接呼ぶことは
        /// 想定しない。
        #[doc(hidden)]
        #[allow(unused_macros)]
        macro_rules! #macro_ident {
            #( (#labels) => {}; )*
            ($other:ident) => {
                compile_error!(concat!(
                    "スキーマ ", #schema_name_str, " にエッジ `",
                    stringify!($other),
                    "` は存在しません。利用可能: ", #available
                ));
            };
        }
    }
}

fn gen_node_structs(nodes: &[NodeInfo<'_>]) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(|n| {
            let ty = &n.type_ident;
            let id_ty = &n.id_ident;
            let field_defs = n.decl.fields.iter().map(|f| {
                let name = &f.name;
                let field_ty = &f.ty;
                quote! { pub #name: #field_ty }
            });
            quote! {
                // 項目3 (フェーズ4): `Eq` は付けない。ノードのフィールド型に
                // `f64` のような `Eq` を実装できない型を使えるようにするため
                // (newtype キー `#id_ty` は内部で HashMap キーとして使うため
                // `Hash + Eq` を維持する)。
                #[derive(Debug, Clone, PartialEq)]
                pub struct #ty {
                    #(#field_defs),*
                }

                #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
                pub struct #id_ty(pub String);
            }
        })
        .collect()
}

fn gen_attrs_structs(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .filter_map(|e| {
            let attrs_ty = e.attrs_type_ident.as_ref()?;
            let attr_fields = e.decl.attrs.as_ref().unwrap();
            let field_defs = attr_fields.iter().map(|f| {
                let name = &f.name;
                let field_ty = &f.ty;
                quote! { pub #name: #field_ty }
            });
            Some(quote! {
                // 項目3 (フェーズ4): ノード struct と同様に `Eq` は付けない
                // (`f64` 等の属性フィールドを許容するため)。
                #[derive(Debug, Clone, PartialEq)]
                pub struct #attrs_ty {
                    #(#field_defs),*
                }
            })
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
    nodes: &[NodeInfo<'_>],
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
    match (&edge.decl.mult, &edge.attrs_type_ident) {
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
    nodes: &[NodeInfo<'_>],
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
    nodes: &[NodeInfo<'_>],
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
fn gen_node_id_iter(node: &NodeInfo<'_>) -> TokenStream {
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

/// 多重度 (1) アクセサの非パニック版 (項目1)。`Vec` における `v[i]` (パニック
/// する) と `v.get(i)` (`Option` を返す) の対の関係に相当する。
fn gen_try_edge_accessor(edge: &EdgeInfo<'_>) -> TokenStream {
    let label = &edge.label;
    let try_ident = format_ident!("try_{}", label);
    let from_id = &edge.from_node.id_ident;
    let to_ty = &edge.to_node.type_ident;
    let to_field = &edge.to_node.field_ident;

    match &edge.attrs_type_ident {
        None => quote! {
            /// 多重度 (1) の非パニック版。未知キーは (パニックせず) `None` を
            /// 返す。
            pub fn #try_ident(&self, id: &#from_id) -> Option<&#to_ty> {
                let to_id = self.#label.get(id)?;
                Some(&self.#to_field[to_id])
            }
        },
        Some(attrs_ty) => quote! {
            /// 多重度 (1) + 属性ありの非パニック版。未知キーは (パニックせず)
            /// `None` を返す。
            pub fn #try_ident(&self, id: &#from_id) -> Option<(&#to_ty, &#attrs_ty)> {
                let (to_id, attrs) = self.#label.get(id)?;
                Some((&self.#to_field[to_id], attrs))
            }
        },
    }
}

/// エッジ種別 1 つ分の (始点キー, 終点キー[, 属性]) ペアイテレータ (項目2:
/// クエリAPI)。`match` パターンによるクエリ DSL の代替として、メソッド
/// チェーンで検索・フィルタができるようにする。多重度 (0..*) は全ペアへ
/// 展開する。
fn gen_edge_pairs_iter(edge: &EdgeInfo<'_>) -> TokenStream {
    let label = &edge.label;
    let pairs_ident = format_ident!("{}_pairs", label);
    let from_id = &edge.from_node.id_ident;
    let to_id = &edge.to_node.id_ident;

    match (&edge.decl.mult, &edge.attrs_type_ident) {
        (Multiplicity::One, None) | (Multiplicity::ZeroOrOne, None) => quote! {
            /// 全ての (始点キー, 終点キー) ペアを列挙する。
            pub fn #pairs_ident(&self) -> impl Iterator<Item = (&#from_id, &#to_id)> {
                self.#label.iter().map(|(from, to)| (from, to))
            }
        },
        (Multiplicity::One, Some(attrs_ty)) | (Multiplicity::ZeroOrOne, Some(attrs_ty)) => quote! {
            /// 全ての (始点キー, 終点キー, 属性) タプルを列挙する。
            pub fn #pairs_ident(&self) -> impl Iterator<Item = (&#from_id, &#to_id, &#attrs_ty)> {
                self.#label.iter().map(|(from, (to, attrs))| (from, to, attrs))
            }
        },
        (Multiplicity::ZeroOrMany, None) => quote! {
            /// 全ての (始点キー, 終点キー) ペアを列挙する
            /// (多重度 0..* は始点ごとの複数終点へ展開する)。
            pub fn #pairs_ident(&self) -> impl Iterator<Item = (&#from_id, &#to_id)> {
                self.#label
                    .iter()
                    .flat_map(|(from, tos)| tos.iter().map(move |to| (from, to)))
            }
        },
        (Multiplicity::ZeroOrMany, Some(attrs_ty)) => quote! {
            /// 全ての (始点キー, 終点キー, 属性) タプルを列挙する
            /// (多重度 0..* は始点ごとの複数終点へ展開する)。
            pub fn #pairs_ident(&self) -> impl Iterator<Item = (&#from_id, &#to_id, &#attrs_ty)> {
                self.#label
                    .iter()
                    .flat_map(|(from, items)| items.iter().map(move |(to, attrs)| (from, to, attrs)))
            }
        },
    }
}

/// 項目d (フェーズ5): ID 版アクセサ。相手ノードの値ではなくキーを返す。
/// 指揮系統チェーンのように「次のノードのキーへ辿ってまたそこから辿る」
/// 処理をしたい場合に、値からキーを逆引きする追加コードを不要にする。
/// 属性は既存の値アクセサ (`{label}`) で取得できるため ID 版には含めない
/// (`docs/design_principles.md` 原則1: 型のstrictness — キーは newtype で
/// 運び、値アクセサと役割を混在させない)。
fn gen_edge_id_accessor(edge: &EdgeInfo<'_>) -> TokenStream {
    let label = &edge.label;
    let from_id = &edge.from_node.id_ident;
    let to_id = &edge.to_node.id_ident;
    let label_str = label.to_string();

    let project_one = |expr: TokenStream| -> TokenStream {
        match &edge.attrs_type_ident {
            None => expr,
            Some(_) => quote! { (#expr).map(|(to_id, _attrs)| to_id) },
        }
    };

    match edge.decl.mult {
        Multiplicity::One => {
            let id_fn = format_ident!("{}_id", label);
            let try_id_fn = format_ident!("try_{}_id", label);
            let get_expr = project_one(quote! { self.#label.get(id) });
            quote! {
                /// 多重度 (1) の ID 版アクセサ。相手ノードの値ではなく
                /// キーを返す。
                ///
                /// # Panics
                /// `id` がこのグラフに存在しないキーの場合パニックする
                /// (呼び出し規約違反。このグラフが発行したキーだけを渡すこと)。
                pub fn #id_fn(&self, id: &#from_id) -> &#to_id {
                    #get_expr.unwrap_or_else(|| {
                        panic!(
                            "{}_id: 未知のキーです (このグラフが発行したキーではありません): {:?}",
                            #label_str, id
                        )
                    })
                }

                /// 上記の非パニック版。未知キーは (パニックせず) `None` を
                /// 返す。
                pub fn #try_id_fn(&self, id: &#from_id) -> Option<&#to_id> {
                    #get_expr
                }
            }
        }
        Multiplicity::ZeroOrOne => {
            let id_fn = format_ident!("{}_id", label);
            let get_expr = project_one(quote! { self.#label.get(id) });
            quote! {
                /// 多重度 (0..1) の ID 版アクセサ。相手ノードの値ではなく
                /// キーを `Option` で返す。未知キーも `None` に落ちる。
                pub fn #id_fn(&self, id: &#from_id) -> Option<&#to_id> {
                    #get_expr
                }
            }
        }
        Multiplicity::ZeroOrMany => {
            let ids_fn = format_ident!("{}_ids", label);
            let map_expr = match &edge.attrs_type_ident {
                None => quote! { items.iter().collect() },
                Some(_) => quote! { items.iter().map(|(to_id, _attrs)| to_id).collect() },
            };
            quote! {
                /// 多重度 (0..*) の ID 版アクセサ。相手ノードの値ではなく
                /// キーの列を返す。無い/未知キーはどちらも空。格納順
                /// (構築時の追加順、`graph!` の場合はソース記述順) を保持する
                /// (README「`(0..*)` エッジの順序保証」節参照)。
                pub fn #ids_fn(&self, id: &#from_id) -> Vec<&#to_id> {
                    match self.#label.get(id) {
                        Some(items) => #map_expr,
                        None => Vec::new(),
                    }
                }
            }
        }
    }
}

fn gen_edge_accessor(schema_name: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let label = &edge.label;
    let from_id = &edge.from_node.id_ident;
    let to_field = &edge.to_node.field_ident;
    let to_ty = &edge.to_node.type_ident;
    let label_str = label.to_string();
    let schema_name_str = schema_name.to_string();

    let try_accessor = match edge.decl.mult {
        Multiplicity::One => gen_try_edge_accessor(edge),
        Multiplicity::ZeroOrOne | Multiplicity::ZeroOrMany => quote! {},
    };
    let pairs_iter = gen_edge_pairs_iter(edge);
    let id_accessor = gen_edge_id_accessor(edge);

    let main_accessor = match (&edge.decl.mult, &edge.attrs_type_ident) {
        (Multiplicity::One, None) => quote! {
            /// 多重度 (1) -> 参照そのものを返す。
            ///
            /// # Panics
            /// `id` がこのグラフに存在しないキーの場合パニックする
            /// (呼び出し規約違反。このグラフが発行したキーだけを渡すこと)。
            pub fn #label(&self, id: &#from_id) -> &#to_ty {
                let to_id = self.#label.get(id).unwrap_or_else(|| {
                    panic!(
                        "{}: 未知のキーです (この{}が発行したキーではありません): {:?}",
                        #label_str, #schema_name_str, id
                    )
                });
                &self.#to_field[to_id]
            }
        },
        (Multiplicity::One, Some(attrs_ty)) => quote! {
            /// 多重度 (1) + 属性あり -> `(参照, 属性参照)` を返す。
            ///
            /// # Panics
            /// `id` がこのグラフに存在しないキーの場合パニックする。
            pub fn #label(&self, id: &#from_id) -> (&#to_ty, &#attrs_ty) {
                let (to_id, attrs) = self.#label.get(id).unwrap_or_else(|| {
                    panic!(
                        "{}: 未知のキーです (この{}が発行したキーではありません): {:?}",
                        #label_str, #schema_name_str, id
                    )
                });
                (&self.#to_field[to_id], attrs)
            }
        },
        (Multiplicity::ZeroOrOne, None) => quote! {
            /// 多重度 (0..1) -> `Option<&T>`。未知キーも `None` に落ちる。
            pub fn #label(&self, id: &#from_id) -> Option<&#to_ty> {
                self.#label.get(id).map(|to_id| &self.#to_field[to_id])
            }
        },
        (Multiplicity::ZeroOrOne, Some(attrs_ty)) => quote! {
            /// 多重度 (0..1) + 属性あり -> `Option<(&T, &Attrs)>`。
            pub fn #label(&self, id: &#from_id) -> Option<(&#to_ty, &#attrs_ty)> {
                let (to_id, attrs) = self.#label.get(id)?;
                Some((&self.#to_field[to_id], attrs))
            }
        },
        (Multiplicity::ZeroOrMany, None) => quote! {
            /// 多重度 (0..*) -> `Vec<&T>`。無い/未知キーはどちらも空。
            pub fn #label(&self, id: &#from_id) -> Vec<&#to_ty> {
                match self.#label.get(id) {
                    Some(ids) => ids.iter().map(|to_id| &self.#to_field[to_id]).collect(),
                    None => Vec::new(),
                }
            }
        },
        (Multiplicity::ZeroOrMany, Some(attrs_ty)) => quote! {
            /// 多重度 (0..*) + 属性あり -> `Vec<(&T, &Attrs)>`。
            pub fn #label(&self, id: &#from_id) -> Vec<(&#to_ty, &#attrs_ty)> {
                match self.#label.get(id) {
                    Some(items) => items
                        .iter()
                        .map(|(to_id, attrs)| (&self.#to_field[to_id], attrs))
                        .collect(),
                    None => Vec::new(),
                }
            }
        },
    };

    quote! {
        #main_accessor
        #try_accessor
        #id_accessor
        #pairs_iter
    }
}

fn edge_builder_value_tuple_type(edge: &EdgeInfo<'_>) -> TokenStream {
    let from_id = &edge.from_node.id_ident;
    let to_id = &edge.to_node.id_ident;
    match &edge.attrs_type_ident {
        None => quote! { (#from_id, #to_id) },
        Some(attrs) => quote! { (#from_id, #to_id, #attrs) },
    }
}

fn gen_builder_struct(
    builder_ident: &Ident,
    nodes: &[NodeInfo<'_>],
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
    nodes: &[NodeInfo<'_>],
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
        match &e.attrs_type_ident {
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

            #freeze_body
        }
    }
}

fn gen_freeze_body(
    schema_name: &Ident,
    violation_ident: &Ident,
    nodes: &[NodeInfo<'_>],
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

    let (bind_pattern, push_value) = match &edge.attrs_type_ident {
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
