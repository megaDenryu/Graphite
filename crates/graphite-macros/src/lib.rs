//! graphite-macros — Graphite の proc-macro クレート。
//!
//! proc-macro クレートはランタイム型を直接持てない (手続き型マクロは
//! コンパイラプラグインの一種で、生成する側と生成されたコードが依存する側の
//! 型を同じクレートに置けない) ため、ランタイムクレート `graphite` とは
//! 分離されている。利用者はこのクレートに直接依存せず、`graphite` 経由で
//! re-export されたマクロを使う。
//!
//! フェーズ3で `graph_schema!` (図式グラフのスキーマ宣言) と `graph!`
//! (インスタンスリテラル) を実装した。生成コードの形は
//! `crates/graphite/tests/orgchart_handwritten.rs` (フェーズ2の手書き
//! テンプレート) に準拠する。
//!
//! 設計の一次資料:
//! - `../../../Bullet/docs/rust_graph_extension_sketch.md`
//! - `../../../Bullet/docs/graph_design_sketches.md`
//!
//! ## 宣言単位のエラー回復展開 (項目G4、`docs/ide_support_spec.md` 参照)
//!
//! `graph_schema!`/`graph!` は共に「宣言 (schema 側) / 項目 (graph! 側)」
//! 単位の回復パーサ (`schema_dsl::SchemaInput::parse_recovering` /
//! `instance_dsl::GraphInput::parse_recovering`) でボディを読む。ヘッダ
//! (`schema Name {` / `SchemaName {`) 自体が壊れている場合のみ、従来通り
//! 全体を諦めて `Err` の `compile_error!` を返す。ボディ内で壊れた宣言/項目
//! が見つかった場合は、その `syn::Error` を蓄積しつつ次の宣言/項目境界まで
//! 読み飛ばし、パースできた残りだけで通常通り validate + codegen を行う。
//! 蓄積したエラーは `compile_error!` として生成物の前に併記する。
//!
//! これにより、DSL 入力の一部が編集途中で構文的に壊れていても、それ以外の
//! 宣言由来の型・アクセサは生成され続け、利用側コードが一斉に赤くならない
//! (rust-analyzer の speculative expansion にも効く可能性がある)。

mod instance_codegen;
mod instance_dsl;
mod naming;
mod schema_codegen;
mod schema_dsl;
mod schema_validate;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Parser;

/// ノード種別・エッジ種別 (where 制約付き) から図式グラフのスキーマ一式
/// (ノード struct・newtype キー・エッジ種別ごとのタプル struct・エッジ
/// newtype キー・スキーマ struct・builder・違反 enum) を生成する。
///
/// ```text
/// pub struct Employee { pub name: String, pub id: u32 }
/// pub struct Department { pub name: String }
/// pub struct BossEdge { pub since: i32 }
///
/// graphite::graph_schema! {
///     schema OrgChart {
///         node Employee;
///         node Department;
///
///         edge BelongsTo = Employee -> Department              where each Employee: 1;
///         edge Boss      = Employee -[BossEdge]-> Employee     where each Employee: 0..1;
///         edge Reports   = Employee -> Employee;
///     }
/// }
/// ```
///
/// `Employee`/`Department`/`BossEdge` はいずれもこのマクロの外でユーザーが
/// 宣言した普通の struct への参照であり、このマクロは値の型そのものを一切
/// 生成しない (`docs/schema_v4.md` 参照)。生成するのはグラフ機械
/// (newtype キー・エッジタプル struct・ストレージ・builder・アクセサ・
/// 違反 enum) だけ。
#[proc_macro]
pub fn graph_schema(input: TokenStream) -> TokenStream {
    // G4a: ヘッダ (`schema Name {`) 自体が壊れている場合はここで Err になり、
    // 従来通り全体を諦めて compile_error! だけを返す (回復しない)。
    let schema_dsl::SchemaParse {
        schema,
        errors: parse_errors,
    } = match schema_dsl::SchemaInput::parse_recovering.parse(input) {
        Ok(parsed) => parsed,
        Err(header_err) => return header_err.to_compile_error().into(),
    };
    let has_parse_errors = !parse_errors.is_empty();

    // G4a 二次エラー抑制: パースエラーが1件以上あるときは「エッジ端点が
    // 未知のノード型」というエラーを出さず、そのエッジを黙って生成対象から
    // 除外する (壊れたノード宣言をたまたま参照しているだけの可能性が高い)。
    // パースエラーが0件のときは現行通り validate_edge_endpoints で検査する。
    let edges = if has_parse_errors {
        schema_validate::filter_edges_with_known_endpoints(&schema.nodes, schema.edges)
    } else {
        schema.edges
    };

    // 重複ノード名・重複エッジ種別名診断は現行維持: パース回復の有無に
    // 関わらず常に検査し、見つかった場合はコード生成を行わない
    // (従来から「validate 失敗時はコード生成なし」という挙動だった)。
    let mut validate_errors: Vec<syn::Error> = Vec::new();
    if let Err(e) = schema_validate::validate_unique_node_names(&schema.nodes) {
        validate_errors.push(e);
    }
    if !has_parse_errors {
        if let Err(e) = schema_validate::validate_edge_endpoints(&schema.nodes, &edges) {
            validate_errors.push(e);
        }
    }
    if let Err(e) = schema_validate::validate_unique_edge_kinds(&edges) {
        validate_errors.push(e);
    }
    if let Err(e) = schema_validate::validate_undirected_same_type(&edges) {
        validate_errors.push(e);
    }
    if let Err(e) = schema_validate::validate_each_reference(&edges) {
        validate_errors.push(e);
    }

    let error_tokens: TokenStream2 = parse_errors
        .iter()
        .chain(validate_errors.iter())
        .map(syn::Error::to_compile_error)
        .collect();

    if !validate_errors.is_empty() {
        // validate 失敗時はコード生成しない (現行維持)。パース回復で蓄積した
        // エラーがあればそれも併記する。
        return error_tokens.into();
    }

    let schema_for_codegen = schema_dsl::SchemaInput {
        schema_name: schema.schema_name,
        nodes: schema.nodes,
        edges,
    };
    let generated = schema_codegen::generate(&schema_for_codegen);

    let combined = quote! {
        #error_tokens
        #generated
    };
    combined.into()
}

/// `graph_schema!` で宣言したスキーマのインスタンスをリテラルに近い記法で
/// 組み立てる。`SchemaName::create(|b| { ... })` へ脱糖する。
///
/// ```text
/// let g = graphite::graph!(OrgChart {
///     tanaka = Employee { name: "田中".into(), id: 1 },
///     sales  = Department { name: "営業".into() },
///
///     belongs = BelongsTo(tanaka -> sales),
/// });
/// ```
#[proc_macro]
pub fn graph(input: TokenStream) -> TokenStream {
    // G4b: ヘッダ (`SchemaName {`) 自体が壊れている場合はここで Err になり、
    // 従来通り全体を諦めて compile_error! だけを返す (回復しない)。
    let instance_dsl::GraphParse {
        graph,
        errors: parse_errors,
    } = match instance_dsl::GraphInput::parse_recovering.parse(input) {
        Ok(parsed) => parsed,
        Err(header_err) => return header_err.to_compile_error().into(),
    };
    let has_parse_errors = !parse_errors.is_empty();

    let error_tokens: TokenStream2 = parse_errors.iter().map(syn::Error::to_compile_error).collect();

    match instance_codegen::generate(&graph, has_parse_errors) {
        Ok(tokens) => {
            if has_parse_errors {
                // G4b: `graph!` は式位置で使われる (`SchemaName::create(..)`
                // という式に脱糖する) ため、蓄積した compile_error! を単純に
                // 前置すると式として不正になる。ブロック式
                // `{ compile_error!(..); ...; SchemaName::create(..) }`
                // の形にして式として妥当な形を保つ。
                quote! {
                    {
                        #error_tokens
                        #tokens
                    }
                }
                .into()
            } else {
                // 正常系 (パースエラー0件): 従来通りブロックで包まず
                // そのまま返す (挙動を一切変えないため)。
                tokens.into()
            }
        }
        Err(err) => {
            // 重複キー等の意味検査エラー: 現行維持 (コード生成なしで
            // compile_error! のみ)。パース回復で蓄積していたエラーが
            // あれば併記する。この形は式位置では不正になり得るため、
            // 既存テスト (`graph_duplicate_node_key.rs`) と同様に
            // `graph!` の呼び出しは文 (statement) 位置で使うこと。
            let mut all = error_tokens;
            all.extend(err.to_compile_error());
            all.into()
        }
    }
}
