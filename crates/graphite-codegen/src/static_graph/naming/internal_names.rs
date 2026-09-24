// instance の値マクロ・内部構築子・DSLトークンの型参照関数の内部生成名
// (issue #41 §3)。いずれもC分類 (`内部生成名`) であり
// 利用者トークンのspanを持たない。`naming/` の外で名前を作らない規則
// (mod.rs 冒頭) を `inline/` にも及ぼす。

use proc_macro2::Ident;

use crate::generated_path::生成先パス;

use super::tracked_name::内部生成名;

// instanceの宣言位置に置く、値ありの個体・積み荷すべての式を宣言順の
// タプルで返すマクロの名前。グラフ名と`instance印`の両方を含むため、
// 同じスコープに複数のinstanceを置いても、別モジュールが同じグラフ名を
// 選んでも衝突しない (`docs/static_graph.md`「制約」節)。この印がinstance
// ごとに実際に一意であることは、生成器が同じCargo target内の重複を
// 拒否して保証する (`生成先パス`冒頭コメント参照)。本体は個々の値を1件ずつ
// 計算する束縛マクロ (`値束縛マクロ名`) を呼ぶだけであり、式そのものは
// ここには無い (`inline/value_supply.rs`)。
pub(crate) fn 個体値マクロ名(グラフ名: &Ident, generated_path: 生成先パス<'_>) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_values_{グラフ名}_{}", generated_path.instance印を計算する()))
}

pub(crate) fn 積み荷値マクロ名(グラフ名: &Ident, generated_path: 生成先パス<'_>) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_payloads_{グラフ名}_{}", generated_path.instance印を計算する()))
}

// 個体1件・積み荷1件ごとの値を計算する束縛マクロの名前。instanceの宣言位置
// に、値の式そのもの (捕捉しない関数の本体) と対で置く。個体名・辺名は
// 同じinstance内で横断的に一意 (`crates/graphite-codegen/src/static_graph/
// literal/validate.rs` が検証する) なので、個体・積み荷のどちらであっても
// 同じ命名で衝突しない。この束縛マクロは、名前が一意な`個体値マクロ名`/
// `積み荷値マクロ名`の本体からしか呼ばれないため、`instance印`を混ぜる
// 必要が無い。
pub(crate) fn 値束縛マクロ名(グラフ名: &Ident, 名前: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_bind_{名前}_{グラフ名}"))
}

// instanceの宣言位置に置く、値を計算する捕捉しない関数の名前。関数の本体は
// 宣言位置の通常のスコープで名前解決される (`inline/value_supply.rs`)。
// instanceがモジュール直下にあっても関数の中にあっても同じ形で置ける
// (関数の中に置いた場合、入れ子の`fn`は外側のローカル変数・引数を捕捉
// できないため、値の式がそれらを参照すると通常のRustのE0434になり、
// 外側のジェネリックの型引数を参照すると通常のRustのE0401になる)。
pub(crate) fn 値関数名(グラフ名: &Ident, 名前: &Ident) -> 内部生成名 {
    内部生成名::new(&format!("__graphite_value_{名前}_{グラフ名}"))
}

// `Graph`が持つ全個体・全積み荷を宣言順の位置引数にそのまま取る、値の
// 計算を持たない素の構築子。生成ファイルの `construct!` (固定語彙、B分類)
// だけが呼ぶ内部専用の橋渡しであり、利用者が直接呼べる公開契約には
// 昇格しない。
pub(crate) fn 内部構築子名() -> 内部生成名 {
    内部生成名::new("__graphite_internal_new")
}

// DSLトークンの型参照 (`const _: () = { .. };` の中の、呼ばれない関数) の名前。
pub(crate) fn 型参照関数名() -> 内部生成名 {
    内部生成名::new("__graphite_dsl_token_type_reference")
}
