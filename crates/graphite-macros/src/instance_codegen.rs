//! `graph!` のコード生成本体 (v4、`docs/schema_v4.md` §2 参照。スプライス項は
//! v4.2、`docs/graph_splice.md` §1)。名前付きラッパー・名前付き位置型・
//! 呼び出し箇所・凍結の用語定義は `docs/schema_v4.md` §3.1.1 参照。
//!
//! `SchemaName::Graph::create_named(|__graphite_b, __graphite_permit| { ... })`
//! と、呼び出し箇所ごとの名前付きラッパー型へ脱糖する。名前付きラッパーは
//! 素の `Graph` と静的項ごとの型付き名前付き位置を所有し、左辺名のメソッドから
//! `Graph` の借用に束縛された参照値 (`NodeRef`/`EdgeRef`) を返す。
//!
//! この生成層は `instance_semantic::構文を検証してグラフリテラルを組み立てる`
//! が確定させた [`instance_semantic::検証済みグラフリテラル`] を受け取って
//! 機械的にトークン化するだけで、意味の判断 (重複キー検査・端点解決・並べ替え
//! の決定) は一切行わない。
//!
//! ノード項・エッジの積み荷の値はいずれもユーザーの式トークンをそのまま
//! 埋め込むだけで、値の型はマクロが一切パースしない。ノード項は
//! `graph_schema!` が生成した総称 `insert_named` メソッド (`graphite_codegen::
//! schema::codegen::insertable_trait::marker_traits::gen_node_trait_and_impls`
//! 参照) にキー文字列と値の式をそのまま渡し、
//! `N::Id` の型推論を rustc に委ねる (許可証付き経路の詳細は
//! `crates/graphite/src/lib.rs` の `NamedInsertPermit` 参照)。
//!
//! エッジ項 (`key = Kind(from -> to)` / `key = Kind(from -[式]-> to)`) は
//! 名前付きフィールドの辺値型を、柄に対応する内部コンストラクタで構築したあと、
//! 同じ形の総称 `add_named` メソッド (`graphite_codegen::schema::codegen::
//! insertable_trait::marker_traits::gen_edge_trait_and_impls` 参照) へ渡す。
//! **辺の名前も (ノードと同様) 常にキーの束縛**
//! (`docs/schema_v4.md` §0 規則1) なので、エッジ項も `let key = ..;` を生成する。
//!
//! スプライス項 (`..式`) は統一 `extend` (`graphite_codegen::schema::codegen::
//! builder::gen_builder_impl` 参照) への呼び出し `__graphite_b.extend(式);` に脱糖する。
//! 静的な項と異なり名前を持たないため `let` 束縛は作らず、戻り値の `Id` 列も
//! 捨てる (`docs/graph_splice.md` §1)。
//!
//! ## 展開形 (項目G1、`docs/development/ide_support_spec.md` 参照)
//!
//! ノードキー・エッジキーはその場で文字列化せず、キーごとに 1 つの `let`
//! 束縛を作り、以後は識別子参照で運ぶ。これにより rust-analyzer 上でキーの
//! 定義ジャンプ・rename・参照検索・hover が「普通のローカル変数」として
//! 機能する:
//!
//! ```text
//! Org::Graph::create_named(|__graphite_b, __graphite_permit| {
//!     // (1) 全ノード宣言 (記述順)
//!     let (alice, alice_position) =
//!         __graphite_b.insert_named("alice", Person { .. }, __graphite_permit);
//!     let (eng, eng_position) =
//!         __graphite_b.insert_named("eng", Team { .. }, __graphite_permit);
//!     // (2) 全エッジとスプライスを記述順に (`docs/graph_splice.md` §1)
//!     let (a_team, a_team_position) = __graphite_b.add_named(
//!         "a_team",
//!         BelongsTo(alice.clone(), eng.clone()),
//!         __graphite_permit,
//!     );
//!     __graphite_b.extend(staff);
//!     (alice_position, eng_position, a_team_position)
//! })
//! ```
//!
//! 凍結成功後、この `(Graph, positions)` をローカルの名前付きラッパーへ移す。
//! 静的アクセサは名前付き位置から参照値を直接構築し、公開IDのハッシュ表での
//! 検索を行わない。スプライスは名前付き位置を返さないため、元の名前を
//! 暗黙再公開しない。
//!
//! エッジはノードキー (`from`/`to`) を参照するため、`let` 束縛は使用より
//! 前に定義されている必要がある。よって展開は「全ノード → (全エッジ+全
//! スプライスを記述順)」の2段に並べ替える (この並べ替えの決定は
//! `instance_semantic::検証済みグラフリテラル` が既に確定させている。builder
//! の検証は凍結時なので意味論は変わらない。スプライスの (0..*) 系の挿入順
//! 保証には第2段内の記述順がそのまま現れる、`docs/graph_splice.md` §1)。
//!
//! builder のクロージャ引数名は `b` ではなく `__graphite_b` にする。ユーザーが
//! `b` というノードキーを書いたときに生成する `let b = ..;` が builder を
//! 隠してしまう衝突を避けるため。
//!
//! 名前付きラッパーの型引数 (`wrapper_parameters`) と位置フィールド
//! (`named_positions`) の並びも、`named_keys` を経由してこの「全ノード → 残り」
//! の2段をそのまま引き継ぐ。よって利用者の記述順 (ノードと辺の混在順) とは
//! 異なる。この並びは生成物の内部だけで整合しており、アクセサは名前で引くため
//! (§14)、利用者から順序が見えることはない。

use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};

use crate::instance_dsl::EdgeDirection;
use crate::instance_semantic::{検証済みグラフリテラル, 検証済み残り項};
use graphite_codegen::naming::{
    graph_type_ident, named_binding_position_ident, named_graph_wrapper_ident,
    named_wrapper_parameter_ident,
};

struct GeneratedItems {
    node_calls: Vec<TokenStream>,
    rest_calls: Vec<TokenStream>,
    named_keys: Vec<Ident>,
    named_positions: Vec<Ident>,
}

fn generate_items(model: &検証済みグラフリテラル, schema_name: &Ident) -> GeneratedItems {
    // 項目G1 (`docs/graph_splice.md` §1 で拡張): 「全ノード → (全エッジ +
    // 全スプライスを記述順)」の2段への並べ替えは `instance_semantic` が既に
    // 確定させている (`model.ノード項の列()` → `model.残り項の列()` の順に
    // 回すだけでよい)。`rest_calls` はエッジとスプライスの両方を、元の記述順
    // のまま (`検証済み残り項の列` が保持する順) 保持する。
    let mut node_calls: Vec<TokenStream> = Vec::new();
    let mut rest_calls: Vec<TokenStream> = Vec::new();
    let mut named_keys: Vec<Ident> = Vec::new();
    let mut named_positions: Vec<Ident> = Vec::new();

    for node in model.ノード項の列() {
        // スパン規約: let の束縛識別子はノード宣言に書かれた出現の
        // Ident をそのまま使う (文字列から作り直さない)。
        let key_ident = node.key.clone();
        let named_position = named_binding_position_ident(&node.key);
        let key_str = node.key.to_string();
        let explicit_id = &node.id;
        let value = &node.value;
        // 孤立ノード (どのエッジにも参照されないノード) は正当な
        // グラフであり、この let 束縛はマクロの実装詳細 (G1) に
        // 過ぎない。エッジで使われない場合 rustc は
        // `unused variable` を出すが、これはユーザーのグラフ設計
        // の問題ではなくノイズなので抑制する。
        let call = match explicit_id {
            Some(id) => {
                quote! { __graphite_b.insert_named_with_id(#id, #value, __graphite_permit) }
            }
            None => {
                quote! { __graphite_b.insert_named(#key_str, #value, __graphite_permit) }
            }
        };
        node_calls.push(quote! {
            #[allow(unused_variables, non_snake_case)]
            let (#key_ident, #named_position) = #call;
        });
        named_keys.push(key_ident);
        named_positions.push(named_position);
    }

    for item in model.残り項の列() {
        match item {
            検証済み残り項::辺(edge) => {
                // スパン規約: エッジ関連の識別子・キーはすべて書かれた出現の
                // トークンをそのまま使う。
                let key_ident = edge.key.clone();
                let named_position = named_binding_position_ident(&edge.key);
                let key_str = edge.key.to_string();
                let explicit_id = &edge.id;
                let kind = &edge.kind;
                let from_ident = edge.from.clone();
                let to_ident = edge.to.clone();
                let literal_trait = match edge.direction {
                    EdgeDirection::Directed => quote! { graphite::DirectedEdgeLiteral<_, _, _> },
                    EdgeDirection::Undirected => quote! { graphite::UndirectedEdgeLiteral<_, _> },
                };

                // 辺値の関連コンストラクタ + 総称 add への脱糖
                // (`docs/schema_v4.md` §2/§3.2)。柄の向きに対応する関連関数だけを
                // 生成するため、未知の辺種別と向きの不一致はいずれも rustc が検出する。
                let ctor = match &edge.attrs {
                    None => {
                        quote! {
                            <#schema_name::#kind as #literal_trait>::from_graph_literal(
                                #from_ident.clone(),
                                #to_ident.clone(),
                                (),
                            )
                        }
                    }
                    Some(attrs_expr) => quote! {
                        <#schema_name::#kind as #literal_trait>::from_graph_literal(
                            #from_ident.clone(),
                            #to_ident.clone(),
                            #attrs_expr,
                        )
                    },
                };

                // 辺の名前もキーの束縛 (`docs/schema_v4.md` §0 規則1)。
                // ノード同様、どこからも参照されない辺キーは
                // `unused variable` 警告のノイズになるため抑制する。
                let call = match explicit_id {
                    Some(id) => {
                        quote! { __graphite_b.add_named_with_id(#id, #ctor, __graphite_permit) }
                    }
                    // 既定IDを生成できない場合のtrait boundエラーは、macro呼び出し
                    // 全体ではなく、利用者が修正すべきエッジ種別名へ結び付ける。
                    None => {
                        quote_spanned! { kind.span()=>
                            __graphite_b.add_named(#key_str, #ctor, __graphite_permit)
                        }
                    }
                };
                rest_calls.push(quote! {
                    #[allow(unused_variables, non_snake_case)]
                    let (#key_ident, #named_position) = #call;
                });
                named_keys.push(key_ident);
                named_positions.push(named_position);
            }
            検証済み残り項::スプライス(spread) => {
                // 統一 `extend` への脱糖 (`docs/graph_splice.md` §1/§2)。
                // スプライスの要素は名前を持たないため `let` 束縛は作らず、
                // 戻り値のキー列もその場で捨てる (式文として実行するのみ)。
                let expr = &spread.expr;
                rest_calls.push(quote! {
                    __graphite_b.extend(#expr);
                });
            }
        }
    }

    GeneratedItems {
        node_calls,
        rest_calls,
        named_keys,
        named_positions,
    }
}

/// 検証済みグラフリテラルから `graph!` の展開結果を生成する。意味の判断
/// (重複キー検査・端点解決・G4bの二次エラー抑制の決定) は呼び出し元
/// (`lib.rs`) が `instance_semantic::構文を検証してグラフリテラルを組み立てる`
/// で既に完了させているため、ここでは失敗しない。
pub fn generate(model: &検証済みグラフリテラル) -> TokenStream {
    let schema_name = model.スキーマ名();
    let graph_ident = graph_type_ident(schema_name);

    let GeneratedItems {
        node_calls,
        rest_calls,
        named_keys,
        named_positions,
    } = generate_items(model, schema_name);

    let wrapper_ident = named_graph_wrapper_ident(schema_name);
    let wrapper_parameters: Vec<Ident> = named_keys
        .iter()
        .enumerate()
        .map(|(index, key)| named_wrapper_parameter_ident(index, key))
        .collect();
    let accessors = named_keys
        .iter()
        .zip(named_positions.iter())
        .zip(wrapper_parameters.iter())
        .map(|((key, position), parameter)| {
            quote! {
                pub fn #key(
                    &self,
                ) -> <#parameter as graphite::NamedGraphElement<#schema_name::#graph_ident>>::Reference<'_> {
                    <#parameter as graphite::NamedGraphElement<#schema_name::#graph_ident>>::bind(
                        &self.#position,
                        &self.__graphite_graph,
                    )
                }
            }
        });
    let accessor_impl = if wrapper_parameters.is_empty() {
        quote! {}
    } else {
        quote! {
            #[allow(non_snake_case)]
            impl<#(#wrapper_parameters),*> #wrapper_ident<#schema_name::#graph_ident #(, #wrapper_parameters)*>
            where
                #(#wrapper_parameters: graphite::NamedGraphElement<#schema_name::#graph_ident>),*
            {
                #(#accessors)*
            }
        }
    };

    quote! {{
        #[allow(non_snake_case)]
        struct #wrapper_ident<__GraphiteGraph #(, #wrapper_parameters)*> {
            __graphite_graph: __GraphiteGraph,
            #(#named_positions: #wrapper_parameters,)*
        }

        impl<__GraphiteGraph #(, #wrapper_parameters)*>
            #wrapper_ident<__GraphiteGraph #(, #wrapper_parameters)*>
        {
            pub fn into_graph(self) -> __GraphiteGraph {
                self.__graphite_graph
            }
        }

        impl<__GraphiteGraph #(, #wrapper_parameters)*> std::ops::Deref
            for #wrapper_ident<__GraphiteGraph #(, #wrapper_parameters)*>
        {
            type Target = __GraphiteGraph;

            fn deref(&self) -> &Self::Target {
                &self.__graphite_graph
            }
        }

        impl<__GraphiteGraph #(, #wrapper_parameters)*> std::ops::DerefMut
            for #wrapper_ident<__GraphiteGraph #(, #wrapper_parameters)*>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.__graphite_graph
            }
        }

        #accessor_impl

        #schema_name::#graph_ident::create_named(|__graphite_b, __graphite_permit| {
            #(#node_calls)*
            #(#rest_calls)*
            (#(#named_positions,)*)
        })
        .map(|(__graphite_graph, __graphite_named_positions)| {
            // クロージャの引数パターンには文単位の属性を付けられないため、
            // 一度この let へ受けてから #[allow(non_snake_case)] を付ける
            // (左辺名を再利用する位置束縛は大文字始まりでもありうる、A2)。
            #[allow(non_snake_case)]
            let (#(#named_positions,)*) = __graphite_named_positions;
            #wrapper_ident {
                __graphite_graph,
                #(#named_positions,)*
            }
        })
    }}
}
