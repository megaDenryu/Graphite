//! このmoduleは、静的グラフの「その場展開」に残す部分 (issue #41 §3・
//! §5.4) を持つ。schema/instanceの指紋照合・macro_rules!転送は
//! `static_graph::mod`/`internal` (既存) が担い、ここは生成ファイルへ
//! 写さない3つだけを持つ:
//! (1) instanceの値の式の供給関数 (`value_supply`、値をそのまま返すだけの
//! 内部専用の素の関数。implを使わない。生成ファイルはこれを呼ばず、
//! `assembly` の組み立て関数が本体の中へ入れ子で呼ぶ)
//! (2) 個体・辺の組み立て関数 (`assembly`。`value_supply`の値と、値なし
//! 宣言の引数を、生成ファイル側の素の構築子 `Nodes::new`/`Edges::new` へ渡す)
//! (3) DSLトークンの型参照 (`token_type_reference`、読むだけの型参照で
//! F12を助ける)。

mod assembly;
mod token_type_reference;
mod value_supply;
#[cfg(test)]
mod tests;

pub(super) use assembly::{個体組み立て関数を組み立てる, 辺組み立て関数を組み立てる};
pub(super) use token_type_reference::dslトークンの型参照を組み立てる;
