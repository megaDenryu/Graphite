// instance の値マクロ・内部構築子・DSLトークンの型参照関数の内部生成名
// (issue #41 §3)。いずれもC分類 (`内部生成名`) であり
// 利用者トークンのspanを持たない。`naming/` の外で名前を作らない規則
// (mod.rs 冒頭) を `inline/` にも及ぼす。

use proc_macro2::Ident;

use super::tracked_name::内部生成名;

// instance展開が呼び出し位置に置く、値ありの個体の式をまとめて返すマクロの
// 名前。グラフ名を含むため、同じスコープに複数のinstanceを置いても
// 衝突しない (`docs/static_graph.md`「制約」節)。
pub(crate) fn 個体値マクロ名(グラフ名: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_values_{グラフ名}"))
}

// instance展開が呼び出し位置に置く、積み荷ありの具体辺の式をまとめて返す
// マクロの名前。
pub(crate) fn 積み荷値マクロ名(グラフ名: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_payloads_{グラフ名}"))
}

// `Nodes`/`Edges` の全個体・全積み荷を宣言順の位置引数にそのまま取る、
// 値の計算を持たない素の構築子。生成ファイルの `construct::nodes!`/
// `construct::edges!` (固定語彙、B分類) だけが呼ぶ内部専用の橋渡しであり、
// 利用者が直接呼べる公開契約には昇格しない。
pub(crate) fn 内部構築子名() -> 内部生成名 {
    内部生成名::new("__graphite_internal_new")
}

// DSLトークンの型参照 (`const _: () = { .. };` の中の、呼ばれない関数) の名前。
pub(crate) fn 型参照関数名() -> 内部生成名 {
    内部生成名::new("__graphite_dsl_token_type_reference")
}
