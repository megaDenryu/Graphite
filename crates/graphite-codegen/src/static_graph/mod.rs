//! 全個体がコンパイル時に確定する静的グラフの構文解析・検証・コード生成
//! (issue #24 段階2、`examples/graphitets-by-hand/macros/` からの移設)。
//! 公開する入口は `static_graph_schema!` (`graphite_macros::static_graph_schema`) 1個
//! だけ。schemaを構文解析・検証し、(1) schemaだけから決まる生成物
//! (node型アンカー、`schema::codegen`) と (2) schemaの生トークンを本体に
//! 焼き込んだ `macro_rules! {schema名}` を同じ展開の中で並べて出力する
//! (`schema_entry`)。生成されるmacro_rulesがschema名そのものをマクロ名に
//! する (利用側は `<schema名>! { graph <名前>; .. }` と書く) ので、利用側が
//! 別途schema名を書く必要はない。
//!
//! macro_rules!が実際に個体宣言を受け取ると、schemaの生トークンと個体宣言の
//! 生トークンを束ねて `#[doc(hidden)]` の内部proc macro `__static_graph_impl!`
//! (`instance_entry`、`graphite_macros::__static_graph_impl` から呼ばれる)
//! へ転送する。schemaとinstanceを1回の展開で同時に見ることで、多重度・
//! 対一意といった「両方が揃わないと検査できない」制約を迂回機構なしで、
//! 通常のcompile_error!として検出できる。
//!
//! 生成されたmacro_rulesは通常のmacro_rules!と同じテキスト順の制約を持つ:
//! `static_graph_schema! { schema <名前> { .. } }` より後ろの行でしか
//! `<名前>! { .. }` を呼べない (詳細は `docs/static_graph.md` を参照)。
//!
//! schema・instanceは共に `generated = "..."` を持ち、動的グラフと同じ
//! 生成ファイル・指紋照合の方式で公開APIを追跡する (issue #41 段階3)。
//! その場展開に残すのは、schema側が指紋照合・型アンカー・
//! `macro_rules! {schema名}`、instance側が相互検証の診断・指紋照合・値の
//! 供給関数・DSLトークンの型参照だけであり、公開生成物 (`Nodes`・`Edges`・
//! `{個体名}Ref`・`{辺名}Ref`・`Graph` 等) はすべて生成ファイル
//! (`file::schema_file`・`file::instance_file`) の中にある。ファイルの
//! 探索・読み書きは `graphite-cli`/`cargo xtask generate` が行う。

mod declaration_sites;
mod doc_render;
mod file;
mod inline;
mod instance_entry;
mod internal;
mod literal;
mod naming;
mod reserved_words;
mod schema;
mod schema_entry;
mod semantic;
mod tracked;
mod trace;

pub(crate) use tracked::instance展開用に解析する;
pub use tracked::{
    parse_tracked_static_instance, parse_tracked_static_schema, TrackedStaticInstance,
    TrackedStaticSchema,
};

pub use instance_entry::expand_static_graph_internal;
pub use schema_entry::parse_and_expand_static_graph_schema;
