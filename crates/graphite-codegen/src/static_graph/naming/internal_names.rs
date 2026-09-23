// instance の値供給関数・DSLトークンの型参照関数の内部生成名 (§3・§5.4)。
// いずれもC分類 (`内部生成名`) であり利用者トークンのspanを持たない。
// `naming/` の外で名前を作らない規則 (mod.rs 冒頭) を `inline/` にも及ぼす。

use proc_macro2::Ident;

use super::tracked_name::内部生成名;

// `Nodes` の関連関数として定義する、個体の初期値を返す関数の名前。
pub(crate) fn 個体供給関数名(個体名: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_initial_value_{個体名}"))
}

// `Edges` の関連関数として定義する、積み荷の値を返す関数の名前。
pub(crate) fn 積み荷供給関数名(辺名: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_payload_{辺名}"))
}

// DSLトークンの型参照 (`const _: () = { .. };` の中の、呼ばれない関数) の名前。
pub(crate) fn 型参照関数名() -> 内部生成名 {
    内部生成名::new("__graphite_dsl_token_type_reference")
}
