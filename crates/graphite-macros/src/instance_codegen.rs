//! `graph!` のコード生成本体。
//!
//! `SchemaName::create(|__graphite_b| { ... })` の呼び出し列へ脱糖する。
//! `graph!` はスキーマの中身を知らないので、ここで使う名前 (builder メソッド名・
//! newtype キー型名・属性型名) は `graph_schema!` (`schema_codegen.rs`) と
//! 全く同じ命名規則 (`crate::naming`) から機械的に導出する。両者がずれると
//! ここで生成した呼び出しがコンパイルエラーになる (メソッドが見つからない等)。
//!
//! ## 展開形 (項目G1、`docs/ide_support_spec.md` 参照)
//!
//! ノードキーはその場で文字列化せず、キーごとに 1 つの `let` 束縛を作り、
//! 以後は識別子参照で運ぶ。これにより rust-analyzer 上でノードキーの
//! 定義ジャンプ・rename・参照検索・hover が「普通のローカル変数」として
//! 機能する:
//!
//! ```text
//! OrgChart::create(|__graphite_b| {
//!     // (1) 全ノード宣言 (記述順)
//!     let tanaka = EmployeeId("tanaka".to_string()); // ← ノード宣言の出現スパン
//!     __graphite_b.employee(tanaka.clone(), Employee { .. });
//!     let sales = DepartmentId("sales".to_string());
//!     __graphite_b.department(sales.clone(), Department { .. });
//!     // (2) 全エッジ (記述順)
//!     __graphite_check_edge_OrgChart!(belongs_to);
//!     __graphite_b.belongs_to(tanaka.clone(), sales.clone()); // ← 各エッジでの出現スパン
//! })
//! ```
//!
//! `graph!` は従来エッジをノード宣言より先に書けたが (キー→型の逆引き表は
//! 全項目を先に走査して作るため)、`let` 束縛は使用より前に定義されている
//! 必要があるので、展開は「全ノード → 全エッジ」の2段に並べ替える
//! (builder の検証は freeze 時なので意味論は変わらない)。`(0..*)` エッジ
//! 同士の記述順保持 (README「`(0..*)` エッジの順序保証」節) はエッジ列内の
//! 相対順序なので、この並べ替えの影響を受けない。
//!
//! builder のクロージャ引数名は `b` ではなく `__graphite_b` にする。ユーザーが
//! `b` というノードキーを書いたときに生成する `let b = ..;` が builder を
//! 隠してしまう衝突を避けるため (proc macro の入力トークンは call site
//! ハイジーンなので、名前が同じなら実際に衝突する)。
//!
//! ## エラー回復との関係 (項目G4b、`docs/ide_support_spec.md` 参照)
//!
//! `lib.rs` は `instance_dsl::GraphInput::parse_recovering` で項目単位の
//! 回復パースを行い、パースに失敗した項目を除いた残りをここに渡す。
//! `generate` の `has_parse_errors` 引数はそのとき1件以上パースエラーが
//! あったかどうかを伝える。パースエラーがある状態では「エッジ端点が
//! 未宣言」という検証エラーを出さずそのエッジを黙って落とす (二次エラー
//! 抑制)。一方 `build_key_types` の重複キー診断はパースエラーの有無に
//! 関わらず常にハード失敗のまま (現行維持) — これは意図的な設計判断で、
//! 「同じキーの二重宣言」は回復パース由来の巻き添えとは考えにくく、
//! 単純に握りつぶすとむしろ紛らわしいと判断したため。

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::instance_dsl::{FieldValue, GraphInput, GraphItem};
use crate::naming::{to_pascal_case, to_snake_case};

/// 項目h (フェーズ5): `graph!` 内のノード識別子はノード型を跨いで単一の
/// 平坦な名前空間 (README「名前空間に関する制約」節参照。型ごとに分ける
/// 再設計はフェーズ5では見送った)。この制約下では「同じ識別子を2回ノード
/// 宣言する」ミスが起きやすいため、`HashMap::insert` で黙って上書きする
/// のではなく、2回目の宣言をその場で `syn::Error` として報告する。
/// 最初の宣言の span も添えて「どこが最初か」を示す
/// (`schema_validate.rs::validate_unique_node_names` と同じパターン)。
fn build_key_types(items: &[GraphItem]) -> syn::Result<HashMap<String, Ident>> {
    let mut key_types: HashMap<String, Ident> = HashMap::new();
    let mut key_spans: HashMap<String, proc_macro2::Span> = HashMap::new();

    for item in items {
        if let GraphItem::Node(node) = item {
            let key_str = node.key.to_string();
            if let Some(&prev_span) = key_spans.get(&key_str) {
                let mut err = syn::Error::new(
                    node.key.span(),
                    format!("識別子 `{key_str}` は既に宣言されています"),
                );
                err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
                return Err(err);
            }
            key_spans.insert(key_str.clone(), node.key.span());
            key_types.insert(key_str, node.type_name.clone());
        }
    }

    Ok(key_types)
}

/// `has_parse_errors`: G4b (`docs/ide_support_spec.md` 参照)。呼び出し元
/// (`lib.rs`) が項目単位の回復パースで1件以上のパースエラーを蓄積していた
/// 場合に `true` を渡す。このとき「エッジ端点が未宣言」という検証エラーは
/// 出さず、そのエッジを黙って生成対象から除外する (壊れた項目由来の二次
/// 噴出を避けるため)。`false` (パースエラー0件) のときは現行通り `Err` で
/// 全体を中断する。なお `build_key_types` の重複キー診断は
/// `has_parse_errors` に関わらず常にハード失敗のまま (現行維持)。
pub fn generate(input: &GraphInput, has_parse_errors: bool) -> syn::Result<TokenStream> {
    let schema_name = &input.schema_name;

    // key (識別子の文字列) -> 宣言時のノード型名。edge が端点の型を逆引きするための表。
    let key_types = build_key_types(&input.items)?;

    // 項目G1: 「全ノード → 全エッジ」の2段に並べ替えるため、生成する
    // トークン列を別々の Vec に集めておき、最後に結合する。
    let mut node_calls: Vec<TokenStream> = Vec::new();
    let mut edge_calls: Vec<TokenStream> = Vec::new();

    for item in &input.items {
        match item {
            GraphItem::Node(node) => {
                // G3 スパンポリシー: String 補間はスパン継承が働かないため明示
                let builder_method = format_ident!(
                    "{}",
                    to_snake_case(&node.type_name.to_string()),
                    span = node.type_name.span()
                );
                let id_type = format_ident!("{}Id", node.type_name);
                // スパン規約: let の束縛識別子はノード宣言に書かれた出現の
                // Ident をそのまま使う (文字列から作り直さない)。
                let key_ident = node.key.clone();
                let key_str = node.key.to_string();
                let type_name = &node.type_name;
                let field_tokens = fields_to_tokens(&node.fields);
                node_calls.push(quote! {
                    let #key_ident = #id_type(#key_str.to_string());
                    __graphite_b.#builder_method(
                        #key_ident.clone(),
                        #type_name { #(#field_tokens),* }
                    );
                });
            }
            GraphItem::Edge(edge) => {
                // 端点キーがノードとして宣言されているかどうかの検証。
                // 検証にのみ使い、コード生成自体はノード側で作った let
                // 束縛への識別子参照 (edge.from / edge.to) で足りるので、
                // 逆引きした型名そのものはここでは使わない。
                let from_known = key_types.contains_key(&edge.from.to_string());
                let to_known = key_types.contains_key(&edge.to.to_string());
                if !from_known || !to_known {
                    if has_parse_errors {
                        // G4b: 二次エラー抑制。他の項目が既にパース失敗して
                        // いる状態では、この「未宣言キー参照」は壊れた項目の
                        // 巻き添えの可能性が高い。エラーにはせず、このエッジ
                        // を黙って生成対象から除外して次の項目へ進む。
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

                // スパン規約: エッジ呼び出しの引数はエッジに書かれた出現の
                // Ident をそのまま使う (from/to それぞれの出現スパン)。
                let from_ident = edge.from.clone();
                let to_ident = edge.to.clone();
                let label = &edge.label;

                // 項目5 (フェーズ4): `graph_schema!` が生成したハンドシェイク
                // 用マクロを呼び、未知のエッジラベルを親切なメッセージで検出
                // する。`graph!` はスキーマの中身を知らないので、スキーマ名
                // から名前を機械的に導出して呼ぶだけで済む
                // (`schema_codegen.rs::gen_edge_check_macro` 参照)。
                let check_macro = format_ident!("__graphite_check_edge_{}", schema_name);

                match &edge.attrs {
                    None => {
                        edge_calls.push(quote! {
                            #check_macro!(#label);
                            __graphite_b.#label(#from_ident.clone(), #to_ident.clone());
                        });
                    }
                    Some(attr_fields) => {
                        // G3 スパンポリシー: String 補間はスパン継承が働かないため明示
                        let attrs_type = format_ident!(
                            "{}Attrs",
                            to_pascal_case(&label.to_string()),
                            span = label.span()
                        );
                        let attr_tokens = fields_to_tokens(attr_fields);
                        edge_calls.push(quote! {
                            #check_macro!(#label);
                            __graphite_b.#label(
                                #from_ident.clone(),
                                #to_ident.clone(),
                                #attrs_type { #(#attr_tokens),* }
                            );
                        });
                    }
                }
            }
        }
    }

    Ok(quote! {
        #schema_name::create(|__graphite_b| {
            #(#node_calls)*
            #(#edge_calls)*
        })
    })
}

fn fields_to_tokens(fields: &[FieldValue]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let name = &f.name;
            let value = &f.value;
            quote! { #name: #value }
        })
        .collect()
}
