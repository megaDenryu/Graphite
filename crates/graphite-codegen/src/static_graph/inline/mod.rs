//! このmoduleは、静的グラフの「その場展開」に残す部分 (issue #41 §3・
//! §5.4) を持つ。schema/instanceの指紋照合・macro_rules!転送は
//! `static_graph::mod`/`internal` (既存) が担い、ここは生成ファイルへ
//! 写さない2つだけを持つ:
//! (1) instanceの値の式の供給関数 (`value_supply`、値をそのまま返すだけの
//! 内部専用関数。生成ファイルはこれを呼ぶだけで式を写さない)
//! (2) DSLトークンの型参照 (`token_type_reference`、読むだけの型参照で
//! F12を助ける)。

mod token_type_reference;
mod value_supply;

// `file::instance_file` が生成ファイル側の呼び出しに使う供給関数の名前は
// `naming::internal_names` (`個体供給関数名`・`積み荷供給関数名`) から直接
// 読む (`inline/` は名前を作らない)。
pub(super) use token_type_reference::dslトークンの型参照を組み立てる;
pub(super) use value_supply::{個体供給関数を組み立てる, 積み荷供給関数を組み立てる};
