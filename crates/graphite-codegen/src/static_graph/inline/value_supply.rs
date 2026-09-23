// instanceの値の式の供給関数 (issue #41 §3)。値ありの個体・積み荷を持つ
// 具体辺ごとに、値の式をそのまま返すだけの内部専用の素の関数 (C分類) を
// 用意する。`inline::assembly` がこの関数を組み立て関数の本体の中へ入れ子で
// 定義し (`inline::assembly` の冒頭コメント参照)、生成ファイル側
// (`file::instance_file`) は式を一切写さない。これにより式のトークンは
// 利用者のspanのまま利用者のスコープで型検査され、生成ファイルの指紋は
// 値の編集に左右されない。名前自体は `naming::internal_names` が作る
// (`naming/` の外で名前を作らない)。
//
// implではなく素の`fn`として組み立てる (呼び出し位置がユーザーの関数の中
// にあっても`non_local_definitions`の対象外になる。`impl`だけがこのlintの
// 対象であり、素の関数項目は対象にならない)。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

use crate::static_graph::naming::{個体供給関数名, 積み荷供給関数名};
use crate::static_graph::semantic::{具体辺, 個体};

// 値ありの個体1件分の供給関数定義。値なし宣言の個体は呼び出し元が対象外に
// する (`個体.値なし宣言か()` で事前に絞る)。
pub(crate) fn 個体供給関数を組み立てる(個体: &個体) -> TokenStream {
    let 関数名 = 個体供給関数名(個体.名前());
    let 実体型 = 個体.実体型();
    let 式 = 個体.値().expect("値ありの個体だけを渡す");
    値供給関数を組み立てる(関数名.ident(), 実体型, 式)
}

// 積み荷ありの具体辺1件分の供給関数定義。
pub(crate) fn 積み荷供給関数を組み立てる(具体辺: &具体辺) -> TokenStream {
    let 関数名 = 積み荷供給関数名(具体辺.名前());
    let 積み荷宣言 = 具体辺.種別().積み荷().expect("積み荷ありの具体辺だけを渡す");
    let 式 = 具体辺.積み荷式().expect("積み荷ありの具体辺だけを渡す");
    値供給関数を組み立てる(関数名.ident(), &積み荷宣言.型, 式)
}

fn 値供給関数を組み立てる(関数名: &proc_macro2::Ident, 戻り値型: &proc_macro2::Ident, 式: &Expr) -> TokenStream {
    // 個体名・辺名は利用者が自由に選ぶため大文字始まりもあり得る
    // (`__graphite_initial_value_{個体名}` の埋め込み部分)。関数名は
    // snake_caseの規約検査対象になるため `#[allow(non_snake_case)]` を
    // 添える (生成ファイル側のmodule全体に掛ける
    // `#[allow(non_snake_case, ..)]` と同じ理由、`docs/code_generation.md` 参照)。
    quote! {
        #[allow(non_snake_case)]
        fn #関数名() -> #戻り値型 {
            #式
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    use crate::static_graph::literal::input::静的グラフ入力;
    use crate::static_graph::schema::input::静的グラフ型入力;
    use crate::static_graph::semantic::意味モデル;

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
    fn 個体供給関数の名前は個体名を含みimplを使わない() {
        let 意味モデル = 意味モデルを作る();
        let 太郎 = 意味モデル.個体列().iter().find(|個体| 個体.名前() == "太郎").unwrap();
        assert_eq!(個体供給関数名(太郎.名前()).ident().to_string(), "__graphite_initial_value_太郎");

        let コード = 個体供給関数を組み立てる(太郎).to_string();
        assert!(コード.contains("__graphite_initial_value_太郎"));
        assert!(!コード.contains("impl"), "implブロックを使わないこと (non_local_definitions対策)");
    }

    #[test]
    fn 積み荷供給関数の名前は辺名を含みimplを使わない() {
        let 意味モデル = 意味モデルを作る();
        let 太郎の所属 = 意味モデル.具体辺列().iter().find(|辺| 辺.名前() == "太郎の所属").unwrap();
        assert_eq!(
            積み荷供給関数名(太郎の所属.名前()).ident().to_string(),
            "__graphite_payload_太郎の所属"
        );

        let コード = 積み荷供給関数を組み立てる(太郎の所属).to_string();
        assert!(コード.contains("__graphite_payload_太郎の所属"));
        assert!(コード.contains("任命記録"));
        assert!(!コード.contains("impl"), "implブロックを使わないこと (non_local_definitions対策)");
    }
}
