//! このmoduleは、静的グラフの「その場展開」に残す部分 (issue #41 §3・
//! §5.4) を持つ。schema/instanceの指紋照合・macro_rules!
//! 転送は `static_graph::mod`/`internal` (既存) が担い、ここは生成
//! ファイルへ写さない2つだけを持つ:
//! (1) instanceの値の式をまとめて返すマクロ (`value_supply`。値ありの
//! 個体・積み荷それぞれの式を宣言順のタプルで返す `macro_rules!`。
//! instanceの宣言位置に置いた捕捉しない`fn`の本体として値の式が固定される
//! ため、instanceを関数の中に置いても、値の式はその関数のローカル変数・
//! 引数を参照できず (参照すると通常のRustのE0434になる)、外側の
//! ジェネリックの型引数も参照できない (参照すると通常のRustのE0401になる)。
//! 生成ファイル側の`construct!` (`file::instance_file::construct`) が
//! このマクロを呼ぶ)
//! (2) DSLトークンの型参照 (`token_type_reference`、読むだけの型参照で
//! F12を助ける)。

mod token_type_reference;
mod value_binding;
mod value_supply;
#[cfg(test)]
mod tests;

pub(super) use token_type_reference::dslトークンの型参照を組み立てる;
pub(super) use value_supply::{個体値マクロを組み立てる, 積み荷値マクロを組み立てる};
