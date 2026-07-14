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

    fn unknown_variant(&self) -> Ident {
        format_ident!("Unknown{}", self.type_ident)
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
    let violation_def = gen_violation_enum(&violation_ident, &node_infos);
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

    quote! {
        #(#node_struct_defs)*
        #(#attrs_struct_defs)*
        #violation_def
        #schema_struct_def
        #schema_impl
        #builder_struct_def
        #builder_impl
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

fn gen_violation_enum(violation_ident: &Ident, nodes: &[NodeInfo<'_>]) -> TokenStream {
    let dup_variants = nodes.iter().map(|n| {
        let v = n.dup_variant();
        let id = &n.id_ident;
        quote! { #v(#id) }
    });
    let unknown_variants = nodes.iter().map(|n| {
        let v = n.unknown_variant();
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
    let unknown_display_arms = nodes.iter().map(|n| {
        let v = n.unknown_variant();
        let type_name_str = n.type_ident.to_string();
        quote! {
            #violation_ident::#v(id) => write!(f, "未知の{}キーが参照されています: {:?}", #type_name_str, id)
        }
    });

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum #violation_ident {
            #(#dup_variants,)*
            #(#unknown_variants,)*
            /// 多重度違反。`source` は違反した側キーの `Debug` 表現
            /// (エッジによって始点ノード型が異なりうるため、型を固定できず
            /// 文字列に落としている。手書きテンプレートとの差異の一つ)。
            MultiplicityViolation {
                edge: &'static str,
                source: String,
                expected: &'static str,
                actual: usize,
            },
        }

        impl std::fmt::Display for #violation_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#dup_display_arms,)*
                    #(#unknown_display_arms,)*
                    #violation_ident::MultiplicityViolation { edge, source, expected, actual } => write!(
                        f,
                        "多重度違反: エッジ種別 `{}` はキー {} について多重度 {} を期待しますが実際は {} 本です",
                        edge, source, expected, actual
                    ),
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
            /// (端点種別・多重度) を一括検査する。
            pub fn create<F>(f: F) -> Result<Self, #violation_ident>
            where
                F: for<'b> FnOnce(&'b mut #builder_ident),
            {
                let mut builder = #builder_ident::new();
                f(&mut builder);
                builder.freeze()
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
                    return Err(#violation_ident::#dup_variant(id));
                }
                #field.insert(id, value);
            }
        }
    });

    let edge_blocks = edges.iter().map(|e| gen_edge_freeze_block(violation_ident, e));

    let node_field_names = nodes.iter().map(|n| &n.field_ident);
    let edge_field_names = edges.iter().map(|e| &e.label);

    quote! {
        fn freeze(self) -> Result<#schema_name, #violation_ident> {
            #(#node_map_builds)*
            #(#edge_blocks)*

            Ok(#schema_name {
                #(#node_field_names,)*
                #(#edge_field_names,)*
            })
        }
    }
}

fn gen_edge_freeze_block(violation_ident: &Ident, edge: &EdgeInfo<'_>) -> TokenStream {
    let label = &edge.label;
    let label_str = label.to_string();
    let count_ident = format_ident!("{}_count", label);
    let from_field = &edge.from_node.field_ident;
    let to_field = &edge.to_node.field_ident;
    let from_unknown = edge.from_node.unknown_variant();
    let to_unknown = edge.to_node.unknown_variant();
    let value_ty = edge_stored_value_type(edge);

    let (bind_pattern, push_value) = match &edge.attrs_type_ident {
        None => (quote! { (from, to) }, quote! { to }),
        Some(_) => (quote! { (from, to, attrs) }, quote! { (to, attrs) }),
    };

    match edge.decl.mult {
        Multiplicity::One => quote! {
            let mut #label: std::collections::HashMap<_, #value_ty> = std::collections::HashMap::new();
            let mut #count_ident: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
            for #bind_pattern in self.#label {
                if !#from_field.contains_key(&from) {
                    return Err(#violation_ident::#from_unknown(from));
                }
                if !#to_field.contains_key(&to) {
                    return Err(#violation_ident::#to_unknown(to));
                }
                *#count_ident.entry(from.clone()).or_insert(0) += 1;
                #label.insert(from, #push_value);
            }
            for key in #from_field.keys() {
                let count = #count_ident.get(key).copied().unwrap_or(0);
                if count != 1 {
                    return Err(#violation_ident::MultiplicityViolation {
                        edge: #label_str,
                        source: format!("{:?}", key),
                        expected: "ちょうど1",
                        actual: count,
                    });
                }
            }
        },
        Multiplicity::ZeroOrOne => quote! {
            let mut #label: std::collections::HashMap<_, #value_ty> = std::collections::HashMap::new();
            let mut #count_ident: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
            for #bind_pattern in self.#label {
                if !#from_field.contains_key(&from) {
                    return Err(#violation_ident::#from_unknown(from));
                }
                if !#to_field.contains_key(&to) {
                    return Err(#violation_ident::#to_unknown(to));
                }
                *#count_ident.entry(from.clone()).or_insert(0) += 1;
                #label.insert(from, #push_value);
            }
            for (key, count) in &#count_ident {
                if *count > 1 {
                    return Err(#violation_ident::MultiplicityViolation {
                        edge: #label_str,
                        source: format!("{:?}", key),
                        expected: "0または1",
                        actual: *count,
                    });
                }
            }
        },
        Multiplicity::ZeroOrMany => quote! {
            let mut #label: std::collections::HashMap<_, #value_ty> = std::collections::HashMap::new();
            for #bind_pattern in self.#label {
                if !#from_field.contains_key(&from) {
                    return Err(#violation_ident::#from_unknown(from));
                }
                if !#to_field.contains_key(&to) {
                    return Err(#violation_ident::#to_unknown(to));
                }
                #label.entry(from).or_default().push(#push_value);
            }
        },
    }
}
