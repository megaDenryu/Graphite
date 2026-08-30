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
//! schema の公開APIは通常の Rust ファイルとして生成する。`graph_schema!`
//! は宣言を検証して生成ファイルの指紋と照合するだけで、型と実装を展開しない
//! (規約は `docs/code_generation.md`)。構文解析・検証・生成は
//! `graphite-codegen` にあり、ファイルの読み書きは `graphite-cli` と、それを
//! 呼ぶ `xtask` が行う。
//!
//! 設計の一次資料:
//! - `../../../Bullet/docs/rust_graph_extension_sketch.md`
//! - `../../../Bullet/docs/graph_design_sketches.md`
//!
//! ## 宣言単位のエラー回復展開 (項目G4、`docs/development/ide_support_spec.md` 参照)
//!
//! schema 側と `graph!` 側は共に「宣言 / 項目」
//! 単位の回復パーサ (`graphite_codegen` の `SchemaInput::parse_recovering` /
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
//!
//! schema 側の回復展開は、公開APIをファイル生成へ移した後も
//! `graphite_codegen::expand_inline_for_test` に残っている。`graph_schema!`
//! は回復展開せず診断を全件返し、回復の挙動は `#[doc(hidden)]` の
//! `__graph_schema_inline_for_test!` を通じて compile-fail テストが検査する。

mod flow_codegen;
mod flow_dsl;
mod instance_codegen;
mod instance_dsl;
mod instance_semantic;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Parser;

/// ノード種別・エッジ種別を宣言し、通常の Rust 生成ファイルが宣言と一致する
/// ことをコンパイル時に検査する。
///
/// ```text
/// pub struct Employee { pub name: String, pub id: u32 }
/// pub struct Department { pub name: String }
/// pub struct BossEdge { pub since: i32 }
///
/// graphite::graph_schema! {
///     generated = "generated/org_chart.rs";
///     schema OrgChart {
///         node Employee;
///         node Department(id: ExistingDepartmentId);
///
///         edge BelongsTo = (employee: Employee) -> (department: Department) where each employee: 1;
///         edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;
///         edge Reports = (reporter: Employee) -> (recipient: Employee);
///     }
/// }
/// ```
///
/// 宣言と同じファイルには
/// `pub mod OrgChart { include!("generated/org_chart.rs"); }` を置く
/// (moduleへ付ける属性は `docs/code_generation.md` が定める2行で固定する)。
/// 生成ファイルはパッケージのディレクトリで `cargo graphite generate` を実行して
/// 更新する (Graphite リポジトリ自身の開発では `cargo xtask generate`)。
/// 公開APIの実装はこの通常の Rust ファイルだけに存在する。
#[proc_macro]
pub fn graph_schema(input: TokenStream) -> TokenStream {
    let schema = match graphite_codegen::parse_tracked_schema(input.into()) {
        Ok(schema) => schema,
        Err(errors) => {
            return errors
                .iter()
                .map(syn::Error::to_compile_error)
                .collect::<proc_macro2::TokenStream>()
                .into();
        }
    };
    let schema_name = schema.schema_name();
    let [first, second, third, fourth] = schema.fingerprint();
    quote! {
        const _: () = {
            let actual = #schema_name::__GRAPHITE_SCHEMA_FINGERPRINT;
            if !(actual[0] == #first
                && actual[1] == #second
                && actual[2] == #third
                && actual[3] == #fourth)
            {
                panic!("Graphite schema の生成ファイルが古いため、パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)");
            }
        };
    }
    .into()
}

/// 回復診断のテスト専用であり、利用者向けではないインラインschema展開。
#[doc(hidden)]
#[proc_macro]
pub fn __graph_schema_inline_for_test(input: TokenStream) -> TokenStream {
    graphite_codegen::expand_inline_for_test(input.into()).into()
}

/// `graph_schema!` で宣言したスキーマのインスタンスをリテラルに近い記法で
/// 組み立てる。`SchemaName::Graph::create_named(|b| { ... })` と、左辺名の
/// 静的アクセサを持つ呼び出し箇所ローカルラッパーへ脱糖する。
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

    let error_tokens: TokenStream2 = parse_errors
        .iter()
        .map(syn::Error::to_compile_error)
        .collect();

    match instance_semantic::構文を検証してグラフリテラルを組み立てる(graph, has_parse_errors) {
        Ok(検証済みグラフリテラル) => {
            let tokens = instance_codegen::generate(&検証済みグラフリテラル);
            if has_parse_errors {
                // G4b: `graph!` は式位置で使われる (`SchemaName::create_named(..)`
                // という式に脱糖する) ため、蓄積した compile_error! を単純に
                // 前置すると式として不正になる。ブロック式
                // `{ compile_error!(..); ...; SchemaName::create_named(..) }`
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

/// 全個体がコンパイル時に確定する静的グラフの schema を宣言する
/// (issue #24)。`graph_schema!`/`graph!` は実行時に個体を追加できる
/// グラフ向けだが、`static_schema!` は個体・辺の集合そのものが
/// コンパイル時に固定されているグラフ向けであり、多重度・対一意といった
/// `where` 制約を実行時検証ではなくコンパイルエラーとして検出する。
///
/// schema を検証し、schema 名そのものを名前にした `macro_rules!` を生成する
/// (macro_rules!転送)。利用側はこの生成された `macro_rules!` へ個体宣言を
/// 渡して具体グラフを組み立てる。
///
/// ```text
/// graphite::static_schema! {
///     schema Organization {
///         node Employee;
///         node Department;
///         edge BelongsTo = (member: Employee) -> (team: Department) where each member: 1;
///         edge Boss = (subordinate: Employee) -[appointment: Appointment]-> (superior: Employee) where each subordinate: 0..1;
///     }
/// }
///
/// Organization! {
///     graph DevTeam;
///     node alice = Employee { name: "alice".into() };
///     node dev: Department = Department { name: "dev".into() };
///     edge alice_belongs = BelongsTo(alice -> dev);
/// }
/// ```
///
/// `node` は3形態を受理する: `node 名前 = 型 { .. };` (型はリテラルのパスから
/// 読む)、`node 名前: 型 = 式;` (任意の式)、`node 名前: 型;` (実体値は
/// `Nodes::new(..)` へ実行時に渡す)。
///
/// 生成される `macro_rules!` はschema宣言と同じテキスト順の制約を持つ:
/// `static_schema! { schema <名前> { .. } }` より後ろの行でしか
/// `<名前>! { .. }` を呼べない。構文・生成される名前の公開契約・
/// コンパイル時検査の一覧は `docs/static_graph.md` を参照。
#[proc_macro]
pub fn static_schema(input: TokenStream) -> TokenStream {
    graphite_codegen::parse_and_expand_static_schema(input.into()).into()
}

/// `static_schema!` が生成する `macro_rules!` からだけ呼ばれる内部マクロ。
/// 利用者が直接書くことは想定しない。
#[doc(hidden)]
#[proc_macro]
pub fn __static_graph_impl(input: TokenStream) -> TokenStream {
    graphite_codegen::expand_static_graph_internal(input.into()).into()
}

/// データフロー矢印 `始点 -[関数式]-> 束縛名` を並べる文位置マクロ
/// (`docs/flow_macro.md` 参照)。
///
/// ```text
/// graphite::flow! {
///     input -[parse]-> parsed,
///     parsed -[validate]-> valid,          // fan-out: parsed を
///     parsed -[stats]-> report,            //   2本の矢印に流す
///     (valid, report) -[merge]-> out,      // fan-in: タプル始点
/// };
/// println!("{}", out.summary);             // 束縛は flow! の後で普通に見える
/// ```
///
/// **即時実行の純粋な脱糖** (消去可能な拡張)。項の記述順に
/// `let 束縛名 = (関数式)(始点..);` を並べるだけで、graph!/graph_schema!
/// のようなスキーマ・builder は一切関与しない。`x -[f]-> y -[g]-> z`
/// (チェーン形) は `x -[f]-> y, y -[g]-> z` の糖衣。束縛名は普通の `let`
/// 束縛としてこのマクロ呼び出しの後に見える (`graph!` の項目キーが builder
/// クロージャの中に閉じるのとは異なり、`flow!` は文位置マクロなので
/// call-site スパンの識別子がそのまま呼び出し元のスコープに現れる)。
#[proc_macro]
pub fn flow(input: TokenStream) -> TokenStream {
    // flow! には graph!/graph_schema! のような「壊れていたら全体を諦める」
    // ヘッダが無いため、parse_recovering は実質常に Ok を返す
    // (`flow_dsl.rs` 参照)。他の2マクロと呼び出し規約を揃えるため、同じ
    // match の形は残す。
    let flow_dsl::FlowParse {
        flow,
        errors: parse_errors,
    } = match flow_dsl::FlowInput::parse_recovering.parse(input) {
        Ok(parsed) => parsed,
        Err(header_err) => return header_err.to_compile_error().into(),
    };

    let error_tokens: TokenStream2 = parse_errors
        .iter()
        .map(syn::Error::to_compile_error)
        .collect();

    match flow_codegen::generate(&flow) {
        Ok(tokens) => {
            // flow! は文位置マクロなので、graph! の式位置ラップ (ブロック式
            // で包む) は不要。compile_error! はどの位置でも単体で正しく
            // コンパイルエラーを発生させるため、そのまま並べて返す。
            quote! {
                #error_tokens
                #tokens
            }
            .into()
        }
        Err(err) => {
            // 束縛名重複などの意味検査エラー: コード生成なしで
            // compile_error! のみ (現行の graph!/graph_schema! と同じ方針)。
            let mut all = error_tokens;
            all.extend(err.to_compile_error());
            all.into()
        }
    }
}
