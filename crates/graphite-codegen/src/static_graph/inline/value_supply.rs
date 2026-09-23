// instanceの値の式をまとめて返すマクロ (issue #41 §3)。値ありの個体・
// 積み荷それぞれについて、宣言順の式をタプルにして返す `macro_rules!` を
// 1個ずつ (グラフあたり2個) 呼び出し位置に置く。生成ファイル側の
// `construct::nodes!`/`construct::edges!` (`file::instance_file::construct`)
// が無修飾の名前でこのマクロを呼び、内部構築子
// (`Nodes::__graphite_internal_new`等) へ渡す。式は生成ファイルへ一切
// 写さない。
//
// `macro_rules!` として展開されるため、実際に呼ばれる位置 (`construct::nodes!`
// 等の本体、同じ呼び出し位置へ展開される) のスコープで式が評価される。
// これにより、instanceを置いた関数のローカル変数・引数・ジェネリックの
// 型引数を、通常のRust式と同じように参照できる。
// `macro_rules!`は項目であり`impl`ではないため、呼び出し位置がユーザーの
// 関数の中にあっても`non_local_definitions`の対象にならない。
//
// `pub(crate) use`を意図的に付けない。macro_rules!の既定のテキスト順
// スコープだけに閉じることで、`construct::nodes!`/`construct::edges!`を
// 呼んでよい位置を「instance宣言と同じテキスト順スコープ (同じmodule、
// またはinstanceを置いた同じ関数の中)」だけに機械的に限定する。値の式の
// 中の関数名・型名 (ローカル変数以外の名前) はmacro_rules!の衛生規則により
// この値マクロが実際に展開される位置 (`construct::nodes!`等の呼び出し位置)
// を起点に解決されるため、instance宣言と無関係な別module・別ファイルから
// 呼べてしまうと、たまたま同名の別の関数・型へ意味がすり替わる恐れがある
// (`docs/static_graph.md`「制約」節)。`pub(crate) use`で公開してしまうと
// この呼び出し位置の制約が失われるため、公開しないことが対策そのものである。

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Expr;

use crate::static_graph::naming::{個体値マクロ名, 積み荷値マクロ名};
use crate::static_graph::semantic::意味モデル;

// 値ありの個体すべての式を宣言順のタプルで返すマクロ定義。値ありの個体が
// 1件も無ければ空タプルを返す。
pub(crate) fn 個体値マクロを組み立てる(意味モデル: &意味モデル) -> TokenStream {
    let 式列: Vec<&Expr> =
        意味モデル.個体列().iter().filter_map(|個体| 個体.値()).collect();
    値マクロを組み立てる(個体値マクロ名(意味モデル.グラフ名()).ident(), &式列)
}

// 積み荷ありの具体辺すべての式を宣言順のタプルで返すマクロ定義。
pub(crate) fn 積み荷値マクロを組み立てる(意味モデル: &意味モデル) -> TokenStream {
    let 式列: Vec<&Expr> =
        意味モデル.具体辺列().iter().filter_map(|辺| 辺.積み荷式()).collect();
    値マクロを組み立てる(積み荷値マクロ名(意味モデル.グラフ名()).ident(), &式列)
}

fn 値マクロを組み立てる(マクロ名: &Ident, 式列: &[&Expr]) -> TokenStream {
    // グラフ名は利用者が自由に選ぶため大文字始まりもあり得る
    // (`__graphite_values_{グラフ名}` の埋め込み部分)。マクロ名自体は
    // snake_caseの規約検査対象にならないため `#[allow(non_snake_case)]` は
    // 不要 (`macro_rules!` はitemの命名規約lintの対象外)。`pub(crate) use`
    // を付けないのは意図的である (このファイル冒頭のコメント参照)。
    quote! {
        macro_rules! #マクロ名 {
            () => {
                (#(#式列,)*)
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    use crate::static_graph::literal::input::静的グラフ入力;
    use crate::static_graph::schema::input::静的グラフ型入力;

    fn 意味モデルを作る() -> 意味モデル {
        let schema: 静的グラフ型入力 = syn::parse2(quote! {
            schema 組織 {
                node 社員;
                node 部署;
                edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署);
            }
        })
        .unwrap();
        let instance: 静的グラフ入力 = syn::parse2(quote! {
            graph 開発チーム;
            node 太郎 = 社員 { 名前: "太郎".into() };
            node 開発部 = 部署 { 名前: "開発部".into() };
            edge 太郎の所属 = 所属(太郎 -[任命記録 { 任命日: 2020 }]-> 開発部);
        })
        .unwrap();
        crate::static_graph::internal::検証してから意味モデルを組み立てる(schema, instance)
    }

    #[test]
    fn 個体値マクロの名前はグラフ名を含みimplを使わない() {
        let 意味モデル = 意味モデルを作る();
        assert_eq!(個体値マクロ名(意味モデル.グラフ名()).ident().to_string(), "__graphite_values_開発チーム");

        let コード = 個体値マクロを組み立てる(&意味モデル).to_string();
        assert!(コード.contains("__graphite_values_開発チーム"));
        assert!(!コード.contains("impl"), "implブロックを使わないこと (non_local_definitions対策)");
    }

    #[test]
    fn 積み荷値マクロの名前はグラフ名を含みimplを使わない() {
        let 意味モデル = 意味モデルを作る();
        assert_eq!(積み荷値マクロ名(意味モデル.グラフ名()).ident().to_string(), "__graphite_payloads_開発チーム");

        let コード = 積み荷値マクロを組み立てる(&意味モデル).to_string();
        assert!(コード.contains("__graphite_payloads_開発チーム"));
        assert!(コード.contains("任命記録"));
        assert!(!コード.contains("impl"), "implブロックを使わないこと (non_local_definitions対策)");
    }
}
