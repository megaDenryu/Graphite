//! `graph!` の構文 (`instance_dsl::GraphItem` の列) を検査し、意味として
//! 確定したグラフリテラルへ変換する。
//!
//! ## この層が持ってよいもの・持ってはならないもの
//!
//! (`graphite_codegen::schema::semantic` の規律に揃える。) ここで判断するのは
//! Graphite の意味論だけである: 予約名 `into_graph` の使用禁止、キーの重複
//! (`docs/schema_v4.md` §0 規則1「名前は常にキーの束縛」の帰結)、エッジ端点が
//! ノードとして宣言されているかの解決、「全ノード → 残り (エッジ+スプライスを
//! 記述順)」への並べ替え。`quote::` は use せず、`TokenStream` をフィールドに
//! 持たず、意味モデル型へ `ToTokens` を実装しない。生成コードの形をこの層が
//! 決めると、コード生成層 (`instance_codegen`) が意味を判断する場所へ
//! 戻ってしまうためである。
//!
//! `instance_dsl::{NodeInstance, EdgeInstance, SpreadInstance}` は利用者が
//! 書いた式・識別子をそのまま保持する型 (`syn::Expr`/`proc_macro2::Ident` の
//! 保持は許される範囲、`quote::` に依存しない) なので、この層は新しく
//! 意味モデル型を作り直さず、それらを検査し並べ替えて包むだけでよい。
//!
//! ## G4b (二次エラー抑制) との関係
//!
//! `has_parse_errors` は `lib.rs` の項目単位の回復パース
//! (`instance_dsl::GraphInput::parse_recovering`) が1件以上のパースエラーを
//! 蓄積していたかどうかを伝える。このとき「エッジ端点が未宣言」という
//! 検証エラーは出さず、その辺を除外して次の項目へ進む
//! (`instance_codegen.rs` ファイル冒頭ドキュメント「展開形」も参照)。
//! 重複キー・予約名の診断は `has_parse_errors` に関わらず常にハード失敗の
//! まま (現行維持) — これは意図的な設計判断で、「同じキーの二重宣言」は
//! 回復パース由来の巻き添えとは考えにくく、単純に握りつぶすとむしろ紛らわしい
//! と判断したため。
//!
//! ## 検査の順序 (変えない)
//!
//! trybuild の `.stderr` がバイト一致で検査するため、以下の順序を変えない:
//! 1. 予約名 `into_graph` の使用・キーの重複 (項目の記述順で最初に見つかった
//!    もの。全項目を1回走査し、見つかり次第即座に返す)
//! 2. 1を通過した後にだけ、エッジ端点がノードとして宣言されているかの検証
//!    (項目の記述順で最初に見つかったもの)

use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span};

use crate::instance_dsl::{EdgeInstance, GraphInput, GraphItem, NodeInstance, SpreadInstance};

/// エッジ項とスプライス項は、ノード宣言より後という並びの意味を除けば対等な
/// 残り項である。生成層はこの列を記述順のまま回すだけでよい。
pub enum 辺かスプライスの項 {
    辺(EdgeInstance),
    スプライス(SpreadInstance),
}

/// `graph!` 1呼び出し分の、検査を通過し並び替え済みの意味。コード生成層は
/// この値だけを読む。
///
/// 項の並びは「全ノード → 残り (エッジ+スプライスを記述順)」の2段
/// (`instance_codegen.rs` ファイル冒頭ドキュメント「展開形」参照。エッジは
/// ノードキーを参照するため、生成する `let` 束縛は使用より前に必要)。
pub struct 検証済みグラフリテラル {
    スキーマ名: Ident,
    ノード項の列: Vec<NodeInstance>,
    辺とスプライスの項の列: Vec<辺かスプライスの項>,
}

impl 検証済みグラフリテラル {
    pub fn スキーマ名(&self) -> &Ident {
        &self.スキーマ名
    }

    /// ノード項を記述順で返す。
    pub fn ノード項の列(&self) -> &[NodeInstance] {
        &self.ノード項の列
    }

    /// エッジ項とスプライス項を記述順で返す。
    pub fn 辺とスプライスの項の列(&self) -> &[辺かスプライスの項] {
        &self.辺とスプライスの項の列
    }
}

/// 構文を検査し、並べ替え済みのグラフリテラルを組み立てる。
pub fn 構文を検証してグラフリテラルを組み立てる(
    input: GraphInput,
    has_parse_errors: bool,
) -> syn::Result<検証済みグラフリテラル> {
    let ノードキー集合 = 予約名と重複キーを検査してノードキー集合を得る(&input.items)?;
    let 全ノードと残りの2段へ並べ替えた項の列 =
        端点を検証して全ノードと残りの2段へ並べ替える(input.items, &ノードキー集合, has_parse_errors)?;
    Ok(検証済みグラフリテラル {
        スキーマ名: input.schema_name,
        ノード項の列: 全ノードと残りの2段へ並べ替えた項の列.ノード項の列,
        辺とスプライスの項の列: 全ノードと残りの2段へ並べ替えた項の列.辺とスプライスの項の列,
    })
}

/// [`端点を検証して全ノードと残りの2段へ並べ替える`] の戻り値。ノード項の列と
/// 辺とスプライスの項の列のどちらがどちらかを型で区別するため、無名タプルではなく
/// フィールド名を持つ構造体で返す (`instance_codegen::GeneratedItems` と同じ
/// パターン)。
struct 全ノードと残りの2段へ並べ替えた項の列 {
    ノード項の列: Vec<NodeInstance>,
    辺とスプライスの項の列: Vec<辺かスプライスの項>,
}

/// v4 (`docs/schema_v4.md` §0 規則1): `graph!` 内の識別子はノード・エッジを
/// 問わず単一の平坦な名前空間 (全行が `名前 = 値` であり、名前は常にキーの
/// 束縛であるため)。この制約下では「同じ識別子を2回宣言する」ミスが起きやすい
/// ため、`HashSet` で重複を無視せず、2回目の宣言をその場で
/// `syn::Error` として報告する。最初の宣言の span も添えて「どこが最初か」を
/// 示す (`graphite_codegen::schema::validate::unique_declaration_names::
/// validate_unique_node_names` と同じパターン)。
///
/// 戻り値はノードキーだけの集合。エッジの端点検証は「ノードとして宣言
/// されているか」を見る必要があるため、ノードキーだけの集合を返す
/// (エッジキーを終点/始点に指定するのは意味論的に無効であり、混同を避ける
/// ため両者を区別する)。
fn 予約名と重複キーを検査してノードキー集合を得る(
    items: &[GraphItem],
) -> syn::Result<HashSet<Ident>> {
    let mut ノードキー集合: HashSet<Ident> = HashSet::new();
    let mut キーの初出位置: HashMap<Ident, Span> = HashMap::new();

    for item in items {
        let key = match item {
            GraphItem::Node(node) => &node.key,
            GraphItem::Edge(edge) => &edge.key,
            // スプライス項は名前を持たない (名前は静的な項だけの概念、
            // `docs/graph_splice.md` §1) ので、キーの重複検査の対象外。
            GraphItem::Spread(_) => continue,
        };
        if key == "into_graph" {
            return Err(syn::Error::new(
                key.span(),
                "識別子 `into_graph` は名前付きグラフを素の `Graph` へ戻す予約メソッド名です。別の名前を付けてください",
            ));
        }
        if let Some(prev_span) = キーの初出位置.get(key) {
            let mut err = syn::Error::new(
                key.span(),
                format!("識別子 `{key}` は既に宣言されています"),
            );
            err.combine(syn::Error::new(*prev_span, "最初の宣言はこちら"));
            return Err(err);
        }
        キーの初出位置.insert(key.clone(), key.span());
        if matches!(item, GraphItem::Node(_)) {
            ノードキー集合.insert(key.clone());
        }
    }

    Ok(ノードキー集合)
}

/// エッジ端点の検証と、「全ノード → 残り (エッジ+スプライスを記述順)」の
/// 2段への並べ替えを1回の走査で行う。
fn 端点を検証して全ノードと残りの2段へ並べ替える(
    items: Vec<GraphItem>,
    ノードキー集合: &HashSet<Ident>,
    has_parse_errors: bool,
) -> syn::Result<全ノードと残りの2段へ並べ替えた項の列> {
    let mut ノード項の列: Vec<NodeInstance> = Vec::new();
    let mut 辺とスプライスの項の列: Vec<辺かスプライスの項> = Vec::new();

    for item in items {
        match item {
            GraphItem::Node(node) => ノード項の列.push(node),
            GraphItem::Edge(edge) => {
                // 端点キーがノードとして宣言されているかどうかの検証。
                let from_known = ノードキー集合.contains(&edge.from);
                let to_known = ノードキー集合.contains(&edge.to);
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
                辺とスプライスの項の列.push(辺かスプライスの項::辺(edge));
            }
            GraphItem::Spread(spread) => {
                辺とスプライスの項の列.push(辺かスプライスの項::スプライス(spread));
            }
        }
    }

    Ok(全ノードと残りの2段へ並べ替えた項の列 {
        ノード項の列,
        辺とスプライスの項の列,
    })
}
