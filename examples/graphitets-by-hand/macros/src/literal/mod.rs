//! レイヤー2: `静的グラフ!` (graph! リテラル相当)。個体タグ・ノード達・辺達・
//! 参照の層・グラフ本体を生成し、レイヤー1 (`schema` module) が用意した骨組み
//! (種別ラベル・種別契約・役割アクセサ・多重度契約) へ具体的な個体を接続する
//! (issue #24 段階2)。

pub(crate) mod codegen;
pub(crate) mod input;
mod validate;
