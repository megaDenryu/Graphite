// instanceの値をまとめて返すマクロ (issue #41 §3)。値ありの個体・積み荷
// それぞれについて、宣言順の式をタプルにして返す `macro_rules!` を1個ずつ
// (グラフあたり2個) instanceの宣言位置に置く。生成ファイル側の `construct!`
// (`file::instance_file::construct`) が無修飾の名前でこのマクロを呼び、
// 内部構築子 (`{グラフ名}::Graph::__graphite_internal_new`) へ渡す。式は
// 生成ファイルへ一切写さない。マクロ名は`グラフ名`と`generated`文字列から
// 計算する印を含むため、別moduleが同じグラフ名を選んでも`generated`文字列が
// 違えば衝突しない (`naming::internal_names::個体値マクロ名`参照)。同じ
// グラフ名で同じ`generated`文字列を持つinstanceが存在しないことは、
// この展開自体では検査できず、生成器 (`static_resolution::
// instance_duplication`) が保証する (`docs/static_graph.md`「制約」節)。
//
// 本体は個々の値を1件ずつ計算する束縛マクロ (`value_binding::値束縛を組み立てる`)
// を呼ぶだけであり、式そのものはここには現れない。名前解決を宣言位置へ
// 固定する仕組みは束縛マクロ側 (`value_binding.rs`) が持つ
// (`docs/static_graph.md`「値の式の名前解決」節)。

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::value_binding::値束縛を組み立てる;
use crate::generated_path::生成先パス;
use crate::static_graph::naming::{個体値マクロ名, 積み荷値マクロ名, 値束縛マクロ名};
use crate::static_graph::semantic::{個体, 具体辺, 意味モデル};

// 値ありの個体すべての式を宣言順のタプルで返すマクロ定義。値ありの個体が
// 1件も無ければ空タプルを返す。`generated_path`はinstanceの
// `generated = "..."`文字列そのもの (呼び出し側 `instance_entry.rs`が持つ)。
pub(crate) fn 個体値マクロを組み立てる(意味モデル: &意味モデル, generated_path: 生成先パス<'_>) -> TokenStream {
    let 値あり個体列: Vec<&個体> =
        意味モデル.個体列().iter().filter(|個体| !個体.値なし宣言か()).collect();
    let 束縛定義列: Vec<TokenStream> = 値あり個体列
        .iter()
        .map(|個体| {
            値束縛を組み立てる(
                意味モデル.グラフ名(),
                個体.名前(),
                個体.実体型(),
                個体.値().expect("値ありの個体のみを対象にしている"),
            )
        })
        .collect();
    let 呼び出し列: Vec<TokenStream> = 値あり個体列
        .iter()
        .map(|個体| 束縛呼び出しを組み立てる(意味モデル.グラフ名(), 個体.名前()))
        .collect();

    束縛定義列と呼び出し列をまとめるマクロを組み立てる(
        個体値マクロ名(意味モデル.グラフ名(), generated_path).ident(),
        &束縛定義列,
        &呼び出し列,
    )
}

// 積み荷ありの具体辺すべての式を宣言順のタプルで返すマクロ定義。
pub(crate) fn 積み荷値マクロを組み立てる(意味モデル: &意味モデル, generated_path: 生成先パス<'_>) -> TokenStream {
    let 積み荷あり辺列: Vec<&具体辺> =
        意味モデル.具体辺列().iter().filter(|辺| 辺.積み荷式().is_some()).collect();
    let 束縛定義列: Vec<TokenStream> = 積み荷あり辺列
        .iter()
        .map(|辺| {
            let 積み荷型 = &辺.種別().積み荷().expect("積み荷ありの具体辺は種別も積み荷を持つ").型;
            値束縛を組み立てる(
                意味モデル.グラフ名(),
                辺.名前(),
                積み荷型,
                辺.積み荷式().expect("積み荷ありの具体辺のみを対象にしている"),
            )
        })
        .collect();
    let 呼び出し列: Vec<TokenStream> =
        積み荷あり辺列.iter().map(|辺| 束縛呼び出しを組み立てる(意味モデル.グラフ名(), 辺.名前())).collect();

    束縛定義列と呼び出し列をまとめるマクロを組み立てる(
        積み荷値マクロ名(意味モデル.グラフ名(), generated_path).ident(),
        &束縛定義列,
        &呼び出し列,
    )
}

fn 束縛呼び出しを組み立てる(グラフ名: &Ident, 名前: &Ident) -> TokenStream {
    let 束縛マクロ名 = 値束縛マクロ名(グラフ名, 名前);
    quote! { #束縛マクロ名!() }
}

fn 束縛定義列と呼び出し列をまとめるマクロを組み立てる(
    マクロ名: &Ident,
    束縛定義列: &[TokenStream],
    呼び出し列: &[TokenStream],
) -> TokenStream {
    // グラフ名は利用者が自由に選ぶため大文字始まりもあり得る
    // (`__graphite_values_{グラフ名}` の埋め込み部分)。マクロ名自体は
    // snake_caseの規約検査対象にならないため `#[allow(non_snake_case)]` は
    // 不要 (`macro_rules!` はitemの命名規約lintの対象外)。`pub(crate) use`
    // を付けないのは意図的である (`docs/static_graph.md`「値の式の名前解決」
    // 節参照)。
    quote! {
        #(#束縛定義列)*
        macro_rules! #マクロ名 {
            () => {
                (#(#呼び出し列,)*)
            };
        }
    }
}

#[cfg(test)]
mod tests;
