// instance の値マクロ・内部構築子・DSLトークンの型参照関数の内部生成名
// (issue #41 §3)。いずれもC分類 (`内部生成名`) であり
// 利用者トークンのspanを持たない。`naming/` の外で名前を作らない規則
// (mod.rs 冒頭) を `inline/` にも及ぼす。

use proc_macro2::Ident;

use super::tracked_name::内部生成名;

// instanceの宣言位置に置く、値ありの個体・積み荷すべての式を宣言順の
// タプルで返すマクロの名前。グラフ名を含むため、同じスコープに複数の
// instanceを置いても衝突しない (`docs/static_graph.md`「制約」節)。本体は
// 個々の値を1件ずつ計算する束縛マクロ (`値束縛マクロ名`) を呼ぶだけであり、
// 式そのものはここには無い (`inline/value_supply.rs`)。
pub(crate) fn 個体値マクロ名(グラフ名: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_values_{グラフ名}"))
}

pub(crate) fn 積み荷値マクロ名(グラフ名: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_payloads_{グラフ名}"))
}

// 個体1件・積み荷1件ごとの値を計算する束縛マクロの名前。instanceの宣言位置
// に、値の式そのもの (項目位置なら関数の本体、関数内なら束縛したクロージャ
// の本体) と対で置く。個体名・辺名は同じinstance内で横断的に一意
// (PR #45レビューE、`crates/graphite-codegen/src/static_graph/literal/validate.rs`
// が検証する) なので、個体・積み荷のどちらであっても同じ命名で衝突しない。
pub(crate) fn 値束縛マクロ名(グラフ名: &Ident, 名前: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_bind_{名前}_{グラフ名}"))
}

// 項目の位置 (`graph <名前>;`) の値を計算する、捕捉しない関数の名前。
// 関数の本体は宣言位置の通常のスコープで名前解決される (`inline/value_supply.rs`)。
pub(crate) fn 値関数名(グラフ名: &Ident, 名前: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_value_{名前}_{グラフ名}"))
}

// 関数の中の位置 (`graph <名前> in fn;`) の値を保持する、let束縛した
// クロージャの変数名。クロージャは宣言位置のローカル変数・引数・型引数を
// 通常のRustの式と同じように捕捉する (`inline/value_supply.rs`)。
pub(crate) fn 値キャプチャ変数名(グラフ名: &Ident, 名前: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_captured_{名前}_{グラフ名}"))
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
