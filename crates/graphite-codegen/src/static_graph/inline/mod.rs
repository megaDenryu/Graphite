//! このmoduleは、静的グラフの「その場展開」に残す部分 (issue #41 §3・
//! §5.4) を持つ。schema/instanceの指紋照合・macro_rules!
//! 転送は `static_graph::mod`/`internal` (既存) が担い、ここは生成
//! ファイルへ写さない2つだけを持つ:
//! (1) instanceの値の式をまとめて返すマクロ (`value_supply`。値ありの
//! 個体・積み荷それぞれの式を宣言順のタプルで返す `macro_rules!`。
//! `macro_rules!`として呼び出し位置に展開されるため、instanceを置いた
//! 関数のローカル変数・引数・型引数を通常のRust式と同じように参照できる。
//! 生成ファイル側の `construct::nodes!`/`construct::edges!`
//! (`file::instance_file::construct`) がこのマクロを呼ぶ)
//! (2) DSLトークンの型参照 (`token_type_reference`、読むだけの型参照で
//! F12を助ける)。

mod token_type_reference;
mod value_supply;
#[cfg(test)]
mod tests;

pub(super) use token_type_reference::dslトークンの型参照を組み立てる;
pub(super) use value_supply::{個体値マクロを組み立てる, 積み荷値マクロを組み立てる};
