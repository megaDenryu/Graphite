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

mod instance_codegen;
mod instance_dsl;
mod naming;
mod schema_codegen;
mod schema_dsl;
mod schema_validate;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// ノード種別・エッジ種別 (多重度付き) から図式グラフのスキーマ一式
/// (ノード struct・newtype キー・エッジ属性 struct・スキーマ struct・
/// builder・違反 enum) を生成する。
///
/// ```text
/// graphite::graph_schema! {
///     schema OrgChart {
///         node Employee { name: String, id: u32 }
///         node Department { name: String }
///
///         edge belongs_to: Employee -> Department (1);
///         edge boss:       Employee -> Employee   (0..1) { since: i32 };
///         edge reports:    Employee -> Employee   (0..*);
///     }
/// }
/// ```
#[proc_macro]
pub fn graph_schema(input: TokenStream) -> TokenStream {
    let schema = parse_macro_input!(input as schema_dsl::SchemaInput);
    if let Err(err) = schema_validate::validate(&schema) {
        return err.to_compile_error().into();
    }
    schema_codegen::generate(&schema).into()
}

/// `graph_schema!` で宣言したスキーマのインスタンスをリテラルに近い記法で
/// 組み立てる。`SchemaName::create(|b| { ... })` へ脱糖する。
///
/// ```text
/// let g = graphite::graph!(OrgChart {
///     tanaka: Employee { name: "田中".into(), id: 1 },
///     sales:  Department { name: "営業".into() },
///
///     tanaka -[belongs_to]-> sales,
/// });
/// ```
#[proc_macro]
pub fn graph(input: TokenStream) -> TokenStream {
    let graph_input = parse_macro_input!(input as instance_dsl::GraphInput);
    match instance_codegen::generate(&graph_input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
