//! `graph!` のコード生成本体 (v4、`docs/schema_v4.md` §2 参照。スプライス項は
//! v4.2、`docs/graph_splice.md` §1)。名前付きラッパー・名前付き位置型・
//! 呼び出し箇所・凍結の用語定義は `docs/schema_v4.md` §3.1.1 参照。
//!
//! `SchemaName::Graph::create_named(|__graphite_b, __graphite_permit| { ... })`
//! と、呼び出し箇所ごとの名前付きラッパー型へ脱糖する。名前付きラッパーは
//! 素の `Graph` と静的項ごとの型付き名前付き位置を所有し、左辺名のメソッドから
//! `Graph` の借用に束縛された参照値 (`NodeRef`/`EdgeRef`) を返す。
//!
//! ノード項・エッジの積み荷の値はいずれもユーザーの式トークンをそのまま
//! 埋め込むだけで、値の型はマクロが一切パースしない。ノード項は
//! `graph_schema!` が生成した総称 `insert_named` メソッド (`schema_codegen.rs::
//! gen_node_trait_and_impls` 参照) にキー文字列と値の式をそのまま渡し、
//! `N::Id` の型推論を rustc に委ねる (許可証付き経路の詳細は
//! `crates/graphite/src/lib.rs` の `NamedInsertPermit` 参照)。
//!
//! エッジ項 (`key = Kind(from -> to)` / `key = Kind(from -[式]-> to)`) は
//! 名前付きフィールドの辺値型を、柄に対応する内部コンストラクタで構築したあと、
//! 同じ形の総称 `add_named` メソッド (`schema_codegen.rs::gen_edge_trait_and_impls`
//! 参照) へ渡す。**辺の名前も (ノードと同様) 常にキーの束縛**
//! (`docs/schema_v4.md` §0 規則1) なので、エッジ項も `let key = ..;` を生成する。
//!
//! スプライス項 (`..式`) は統一 `extend` (`schema_codegen.rs::
//! gen_builder_impl` 参照) への呼び出し `__graphite_b.extend(式);` に脱糖する。
//! 静的な項と異なり名前を持たないため `let` 束縛は作らず、戻り値の `Id` 列も
//! 捨てる (`docs/graph_splice.md` §1)。
//!
//! ## 展開形 (項目G1、`docs/ide_support_spec.md` 参照)
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
//!前に定義されている必要がある。よって展開は「全ノード → (全エッジ+全
//! スプライスを記述順)」の2段に並べ替える (builder の検証は凍結時なので
//! 意味論は変わらない。スプライスの (0..*) 系の挿入順保証には第2段内の記述順
//! がそのまま現れる、`docs/graph_splice.md` §1)。
//!
//! builder のクロージャ引数名は `b` ではなく `__graphite_b` にする。ユーザーが
//! `b` というノードキーを書いたときに生成する `let b = ..;` が builder を
//! 隠してしまう衝突を避けるため。
//!
//! ## エラー回復との関係 (項目G4b、`docs/ide_support_spec.md` 参照)
//!
//! `lib.rs` は `instance_dsl::GraphInput::parse_recovering` で項目単位の
//! 回復パースを行い、パースに失敗した項目を除いた残りをここに渡す。
//! `generate` の `has_parse_errors` 引数はそのとき1件以上パースエラーが
//! あったかどうかを伝える。パースエラーがある状態では「エッジ端点が
//! 未宣言」という検証エラーを出さず、その辺を生成対象から除外する (二次エラー
//! 抑制)。一方 `collect_declared_keys` の重複キー診断はパースエラーの有無に
//! 関わらず常にハード失敗のまま (現行維持) — これは意図的な設計判断で、
//! 「同じキーの二重宣言」は回復パース由来の巻き添えとは考えにくく、
//! 単純に握りつぶすとむしろ紛らわしいと判断したため。

use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};

use crate::instance_dsl::{EdgeDirection, GraphInput, GraphItem};
use crate::naming::{
    graph_type_ident, named_binding_position_ident, named_graph_wrapper_ident,
    named_wrapper_parameter_ident,
};

/// v4 (`docs/schema_v4.md` §0 規則1): `graph!` 内の識別子はノード・エッジを
/// 問わず単一の平坦な名前空間 (全行が `名前 = 値` であり、名前は常にキーの
/// 束縛であるため)。この制約下では「同じ識別子を2回宣言する」ミスが起きやすい
/// ため、`HashSet` で重複を無視せず、2回目の宣言をその場で
/// `syn::Error` として報告する。最初の宣言の span も添えて「どこが最初か」を
/// 示す (`schema_validate.rs::validate_unique_node_names` と同じパターン)。
///
/// 戻り値は `(全キーの集合, ノードキーだけの集合)`。エッジの端点検証は
/// 「ノードとして宣言されているか」を見る必要があるため、ノードキーだけの
/// 集合を別途返す (エッジキーを終点/始点に指定するのは意味論的に無効であり、
/// 混同を避けるため両者を区別する)。
fn collect_declared_keys(items: &[GraphItem]) -> syn::Result<(HashSet<String>, HashSet<String>)> {
    let mut all_keys: HashSet<String> = HashSet::new();
    let mut node_keys: HashSet<String> = HashSet::new();
    let mut key_spans: HashMap<String, proc_macro2::Span> = HashMap::new();

    for item in items {
        let key = match item {
            GraphItem::Node(node) => &node.key,
            GraphItem::Edge(edge) => &edge.key,
            // スプライス項は名前を持たない (名前は静的な項だけの概念、
            // `docs/graph_splice.md` §1) ので、キーの重複検査の対象外。
            GraphItem::Spread(_) => continue,
        };
        let key_str = key.to_string();
        if key_str == "into_graph" {
            return Err(syn::Error::new(
                key.span(),
                "識別子 `into_graph` は名前付きグラフを素の `Graph` へ戻す予約メソッド名です。別の名前を付けてください",
            ));
        }
        if let Some(&prev_span) = key_spans.get(&key_str) {
            let mut err = syn::Error::new(
                key.span(),
                format!("識別子 `{key_str}` は既に宣言されています"),
            );
            err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
            return Err(err);
        }
        key_spans.insert(key_str.clone(), key.span());
        all_keys.insert(key_str.clone());
        if matches!(item, GraphItem::Node(_)) {
            node_keys.insert(key_str);
        }
    }

    Ok((all_keys, node_keys))
}

struct GeneratedItems {
    node_calls: Vec<TokenStream>,
    rest_calls: Vec<TokenStream>,
    named_keys: Vec<Ident>,
    named_positions: Vec<Ident>,
}

fn generate_items(
    input: &GraphInput,
    schema_name: &Ident,
    node_keys: &HashSet<String>,
    has_parse_errors: bool,
) -> syn::Result<GeneratedItems> {
    // 項目G1 (`docs/graph_splice.md` §1 で拡張): 「全ノード → (全エッジ +
    // 全スプライスを記述順)」の2段に並べ替えるため、生成するトークン列を
    // 別々の Vec に集めておき、最後に結合する。`rest_calls` はエッジと
    // スプライスの両方を、元の記述順のまま (この1つのループで出現順に push
    // するだけなので自然に順序が保たれる) 保持する。
    let mut node_calls: Vec<TokenStream> = Vec::new();
    let mut rest_calls: Vec<TokenStream> = Vec::new();
    let mut named_keys: Vec<Ident> = Vec::new();
    let mut named_positions: Vec<Ident> = Vec::new();

    for item in &input.items {
        match item {
            GraphItem::Node(node) => {
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
            GraphItem::Edge(edge) => {
                // 端点キーがノードとして宣言されているかどうかの検証。
                let from_known = node_keys.contains(&edge.from.to_string());
                let to_known = node_keys.contains(&edge.to.to_string());
                if !from_known || !to_known {
                    if has_parse_errors {
                        // G4b: 二次エラー抑制。他の項目が既にパース失敗して
                        // いる状態では、この「未宣言キー参照」は壊れた項目の
                        // 巻き添えの可能性が高い。エラーにはせず、このエッジ
                        // を生成対象から除外して次の項目へ進む。
                        continue;
                    }
                    // 現行維持: パースエラーが無ければ通常通りエラーにする。
                    let bad = if !from_known { &edge.from } else { &edge.to };
                    return Err(syn::Error::new_spanned(
                        bad,
                        format!(
                            "`{}` はこの graph! 呼び出し内でノードとして宣言されていません",
                            bad
                        ),
                    ));
                }

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
            GraphItem::Spread(spread) => {
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

    Ok(GeneratedItems {
        node_calls,
        rest_calls,
        named_keys,
        named_positions,
    })
}

/// `has_parse_errors`: G4b (`docs/ide_support_spec.md` 参照)。呼び出し元
/// (`lib.rs`) が項目単位の回復パースで1件以上のパースエラーを蓄積していた
/// 場合に `true` を渡す。このとき「エッジ端点が未宣言」という検証エラーは
/// 出さず、その辺を生成対象から除外する (壊れた項目由来の二次
/// 噴出を避けるため)。`false` (パースエラー0件) のときは現行通り `Err` で
/// 全体を中断する。なお `collect_declared_keys` の重複キー診断は
/// `has_parse_errors` に関わらず常にハード失敗のまま (現行維持)。
pub fn generate(input: &GraphInput, has_parse_errors: bool) -> syn::Result<TokenStream> {
    let schema_name = &input.schema_name;
    let graph_ident = graph_type_ident(schema_name);

    let (_all_keys, node_keys) = collect_declared_keys(&input.items)?;

    let GeneratedItems {
        node_calls,
        rest_calls,
        named_keys,
        named_positions,
    } = generate_items(input, schema_name, &node_keys, has_parse_errors)?;

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

    Ok(quote! {{
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
    }})
}
