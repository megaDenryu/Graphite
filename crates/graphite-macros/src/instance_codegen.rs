//! `graph!` のコード生成本体。
//!
//! `SchemaName::create(|__graphite_b| { ... })` の呼び出し列へ脱糖する。
//!
//! v3 (`docs/graph_literal_v3.md`): ノード項・エッジ属性の値はいずれも
//! ユーザーの式トークンをそのまま埋め込むだけで、値の型はマクロが一切
//! パースしない (旧版はノード項の型名から `to_snake_case` でビルダー
//! メソッド名を機械的に導出していたが、v3 ではノード項の型名自体が
//! 構文から消えたためこの導出が出来なくなった)。代わりに `graph_schema!`
//! が生成した総称 `insert` メソッド (`schema_codegen.rs::gen_node_trait_and_impls`
//! 参照) にキー文字列と値の式をそのまま渡し、`N::Id` の型推論を rustc に
//! 委ねる。エッジも同様にビルダーの型名付きメソッド (`b.label(from, to, 式)`)
//! へ式を素通しするだけになる。
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
//!     let tanaka = __graphite_b.insert("tanaka", Employee { .. }); // ← ノード宣言の出現スパン
//!     let sales = __graphite_b.insert("sales", Department { .. });
//!     // (2) 全エッジ (記述順)
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
//! ## v3 でのハンドシェイクマクロ全廃 (`docs/graph_literal_v3.md` §4)
//!
//! v2 まではエッジ行ごとに `__graphite_edge_{Schema}!(check label)` を
//! 埋め込み、未知ラベルを親切な `compile_error!` で検出していた。v3 は
//! 属性ペイロードが式渡しになったため、この二段マクロ展開 (proc-macro →
//! macro_rules) 自体が不要になった。未知ラベルは
//! `__graphite_b.#label(..)` の呼び出しがそのまま rustc の method-not-found
//! (E0599) に落ちることで検出される (診断の「利用可能一覧」は失うが、
//! 健全性には関与しないためユーザー決定により許容: `docs/graph_literal_v3.md`
//! §4)。この全廃により、`graph_schema!`/`graph!` の同一ファイル制約 (G5、
//! `docs/ide_support_spec.md`) も構造的に消滅した (`graph!` が参照するのは
//! 通常の型・メソッドだけになったため、別モジュールから `use` すれば足りる)。
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
use quote::quote;

use crate::instance_dsl::{GraphInput, GraphItem};

/// 項目h (フェーズ5): `graph!` 内のノード識別子はノード型を跨いで単一の
/// 平坦な名前空間 (README「名前空間に関する制約」節参照。型ごとに分ける
/// 再設計はフェーズ5では見送った)。この制約下では「同じ識別子を2回ノード
/// 宣言する」ミスが起きやすいため、`HashSet` で黙って無視するのではなく、
/// 2回目の宣言をその場で `syn::Error` として報告する。最初の宣言の span も
/// 添えて「どこが最初か」を示す
/// (`schema_validate.rs::validate_unique_node_names` と同じパターン)。
///
/// v3 では値の型をここで追跡する必要が無くなったため (`insert` が rustc の
/// 型推論に委ねる)、戻り値は「宣言済みキー文字列の集合」だけで足りる
/// (v2 までは `HashMap<String, Ident>` でキー→型名を持っていた)。
fn collect_declared_keys(items: &[GraphItem]) -> syn::Result<std::collections::HashSet<String>> {
    let mut declared: std::collections::HashSet<String> = std::collections::HashSet::new();
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
            declared.insert(key_str);
        }
    }

    Ok(declared)
}

/// `has_parse_errors`: G4b (`docs/ide_support_spec.md` 参照)。呼び出し元
/// (`lib.rs`) が項目単位の回復パースで1件以上のパースエラーを蓄積していた
/// 場合に `true` を渡す。このとき「エッジ端点が未宣言」という検証エラーは
/// 出さず、そのエッジを黙って生成対象から除外する (壊れた項目由来の二次
/// 噴出を避けるため)。`false` (パースエラー0件) のときは現行通り `Err` で
/// 全体を中断する。なお `collect_declared_keys` の重複キー診断は
/// `has_parse_errors` に関わらず常にハード失敗のまま (現行維持)。
pub fn generate(input: &GraphInput, has_parse_errors: bool) -> syn::Result<TokenStream> {
    let schema_name = &input.schema_name;

    // 宣言済みノードキー文字列の集合。edge が端点を検証するための表。
    let declared_keys = collect_declared_keys(&input.items)?;

    // 項目G1: 「全ノード → 全エッジ」の2段に並べ替えるため、生成する
    // トークン列を別々の Vec に集めておき、最後に結合する。
    let mut node_calls: Vec<TokenStream> = Vec::new();
    let mut edge_calls: Vec<TokenStream> = Vec::new();

    for item in &input.items {
        match item {
            GraphItem::Node(node) => {
                // スパン規約: let の束縛識別子はノード宣言に書かれた出現の
                // Ident をそのまま使う (文字列から作り直さない)。
                let key_ident = node.key.clone();
                let key_str = node.key.to_string();
                let value = &node.value;
                node_calls.push(quote! {
                    let #key_ident = __graphite_b.insert(#key_str, #value);
                });
            }
            GraphItem::Edge(edge) => {
                // 端点キーがノードとして宣言されているかどうかの検証。
                let from_known = declared_keys.contains(&edge.from.to_string());
                let to_known = declared_keys.contains(&edge.to.to_string());
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

                // v3 (`docs/graph_literal_v3.md` §4): ハンドシェイクマクロは
                // 全廃した。未知ラベルは `__graphite_b.#label(..)` がそのまま
                // rustc の method-not-found (E0599) に落ちることで検出される。
                match &edge.attrs {
                    None => {
                        edge_calls.push(quote! {
                            __graphite_b.#label(#from_ident.clone(), #to_ident.clone());
                        });
                    }
                    Some(attrs_expr) => {
                        edge_calls.push(quote! {
                            __graphite_b.#label(
                                #from_ident.clone(),
                                #to_ident.clone(),
                                #attrs_expr
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
