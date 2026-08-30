//! schema宣言 (`schema <名前> { .. }`) の構文解析・検証・schemaだけから決まる
//! 生成物 (codegen module) を持つ。`static_schema!` はここでの構文解析・検証・
//! 生成の結果に加え、macro_rules! 転送用に生トークンのまま焼き込んだ内部
//! マクロ呼び出しを出力する。schemaとinstanceを両方見ないと決まらない生成物
//! (instance側の実体・参照structなど) は `__static_graph_impl!` (internal
//! module) が担う。

pub(crate) mod codegen;
pub(crate) mod input;
mod validate;
