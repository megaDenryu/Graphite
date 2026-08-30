//! instance宣言 (`graph <名前>; (<個体宣言>,)*`) の構文解析と、schemaを
//! 参照しない構造検証 (個体名・辺名の重複、端点の宣言漏れ) を持つ。
//! schemaとの相互検証・コード生成は `__static_graph_impl!` (`internal`
//! module) が両方のASTを受け取ってから行う。

pub(crate) mod input;
mod validate;
