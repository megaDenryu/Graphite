//! `graph!` のコード生成本体 (v4、`docs/schema_v4.md` §2 参照。スプライス項は
//! v4.2、`docs/graph_splice.md` §1)。
//!
//! `SchemaName::create(|__graphite_b| { ... })` の呼び出し列へ脱糖する。
//!
//! ノード項・エッジの積み荷の値はいずれもユーザーの式トークンをそのまま
//! 埋め込むだけで、値の型はマクロが一切パースしない。ノード項は
//! `graph_schema!` が生成した総称 `insert` メソッド (`schema_codegen.rs::
//! gen_node_trait_and_impls` 参照) にキー文字列と値の式をそのまま渡し、
//! `N::Id` の型推論を rustc に委ねる。
//!
//! エッジ項 (`key = Kind(from -> to)` / `key = Kind(from -[式]-> to)`) は
//! タプル struct `Kind(from_id.clone(), to_id.clone(), ..)` を構築したあと、
//! 同じ形の総称 `add` メソッド (`schema_codegen.rs::gen_edge_trait_and_impls`
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
//! Org::create(|__graphite_b| {
//!     // (1) 全ノード宣言 (記述順)
//!     let alice = __graphite_b.insert("alice", Person { .. });
//!     let eng = __graphite_b.insert("eng", Team { .. });
//!     // (2) 全エッジとスプライスを記述順に (`docs/graph_splice.md` §1)
//!     let a_team = __graphite_b.add("a_team", BelongsTo(alice.clone(), eng.clone()));
//!     __graphite_b.extend(staff);
//! })
//! ```
//!
//! エッジはノードキー (`from`/`to`) を参照するため、`let` 束縛は使用より
//!前に定義されている必要がある。よって展開は「全ノード → (全エッジ+全
//! スプライスを記述順)」の2段に並べ替える (builder の検証は freeze 時なので
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
//! 未宣言」という検証エラーを出さずそのエッジを黙って落とす (二次エラー
//! 抑制)。一方 `collect_declared_keys` の重複キー診断はパースエラーの有無に
//! 関わらず常にハード失敗のまま (現行維持) — これは意図的な設計判断で、
//! 「同じキーの二重宣言」は回復パース由来の巻き添えとは考えにくく、
//! 単純に握りつぶすとむしろ紛らわしいと判断したため。

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;

use crate::instance_dsl::{GraphInput, GraphItem};

/// v4 (`docs/schema_v4.md` §0 規則1): `graph!` 内の識別子はノード・エッジを
/// 問わず単一の平坦な名前空間 (全行が `名前 = 値` であり、名前は常にキーの
/// 束縛であるため)。この制約下では「同じ識別子を2回宣言する」ミスが起きやすい
/// ため、`HashSet` で黙って無視するのではなく、2回目の宣言をその場で
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

/// `has_parse_errors`: G4b (`docs/ide_support_spec.md` 参照)。呼び出し元
/// (`lib.rs`) が項目単位の回復パースで1件以上のパースエラーを蓄積していた
/// 場合に `true` を渡す。このとき「エッジ端点が未宣言」という検証エラーは
/// 出さず、そのエッジを黙って生成対象から除外する (壊れた項目由来の二次
/// 噴出を避けるため)。`false` (パースエラー0件) のときは現行通り `Err` で
/// 全体を中断する。なお `collect_declared_keys` の重複キー診断は
/// `has_parse_errors` に関わらず常にハード失敗のまま (現行維持)。
pub fn generate(input: &GraphInput, has_parse_errors: bool) -> syn::Result<TokenStream> {
    let schema_name = &input.schema_name;

    let (_all_keys, node_keys) = collect_declared_keys(&input.items)?;

    // 項目G1 (`docs/graph_splice.md` §1 で拡張): 「全ノード → (全エッジ +
    // 全スプライスを記述順)」の2段に並べ替えるため、生成するトークン列を
    // 別々の Vec に集めておき、最後に結合する。`rest_calls` はエッジと
    // スプライスの両方を、元の記述順のまま (この1つのループで出現順に push
    // するだけなので自然に順序が保たれる) 保持する。
    let mut node_calls: Vec<TokenStream> = Vec::new();
    let mut rest_calls: Vec<TokenStream> = Vec::new();

    for item in &input.items {
        match item {
            GraphItem::Node(node) => {
                // スパン規約: let の束縛識別子はノード宣言に書かれた出現の
                // Ident をそのまま使う (文字列から作り直さない)。
                let key_ident = node.key.clone();
                let key_str = node.key.to_string();
                let value = &node.value;
                // 孤立ノード (どのエッジにも参照されないノード) は正当な
                // グラフであり、この let 束縛はマクロの実装詳細 (G1) に
                // 過ぎない。エッジで使われない場合 rustc は
                // `unused variable` を出すが、これはユーザーのグラフ設計
                // の問題ではなくノイズなので抑制する。
                node_calls.push(quote! {
                    #[allow(unused_variables)]
                    let #key_ident = __graphite_b.insert(#key_str, #value);
                });
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

                // スパン規約: エッジ関連の識別子・キーはすべて書かれた出現の
                // トークンをそのまま使う。
                let key_ident = edge.key.clone();
                let key_str = edge.key.to_string();
                let kind = &edge.kind;
                let from_ident = edge.from.clone();
                let to_ident = edge.to.clone();

                // タプル struct 構築 + 総称 add への脱糖
                // (`docs/schema_v4.md` §2/§3.2)。未知の Kind 名は
                // `#kind(..)` がそのまま rustc の cannot-find-type/
                // no-such-function に落ちることで検出される。
                let ctor = match &edge.attrs {
                    None => quote! { #kind(#from_ident.clone(), #to_ident.clone()) },
                    Some(attrs_expr) => quote! {
                        #kind(#from_ident.clone(), #to_ident.clone(), #attrs_expr)
                    },
                };

                // 辺の名前もキーの束縛 (`docs/schema_v4.md` §0 規則1)。
                // ノード同様、どこからも参照されない辺キーは
                // `unused variable` 警告のノイズになるため抑制する。
                rest_calls.push(quote! {
                    #[allow(unused_variables)]
                    let #key_ident = __graphite_b.add(#key_str, #ctor);
                });
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

    Ok(quote! {
        #schema_name::create(|__graphite_b| {
            #(#node_calls)*
            #(#rest_calls)*
        })
    })
}
