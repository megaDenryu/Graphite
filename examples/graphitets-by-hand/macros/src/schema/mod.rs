//! レイヤー1: `静的グラフ型!` (graph_schema! 相当)。型の骨組み (種別ラベル・
//! 種別契約・役割アクセサ・多重度の契約) をグラフ名に依存しない形で1回生成
//! する (issue #24 段階2)。

pub(crate) mod codegen;
pub(crate) mod input;
mod validate;
