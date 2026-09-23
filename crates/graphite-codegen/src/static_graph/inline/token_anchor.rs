// DSLトークンの錨 (issue #41 §5.4)。instanceの展開へ、読むだけの型参照を
// 置く。種別トークン・辺名トークン・個体名トークン・端点トークンの4種類
// それぞれに、instanceに書かれたその出現箇所のspanを持つ型参照を置き、
// schema module越し・グラフmodule越しの修飾パス (`組織::所属Edge`・
// `開発チーム::{名前}Ref`) でF12の着地先を実際の型定義へ向ける。錨は実行時
// のコードを生まない (`const _: () = { .. };` の中の、呼ばれない関数の
// シグネチャとしてだけ存在する)。
//
// 種別トークン・端点トークンは `具体辺` が持つ「instanceに書かれたトークン
// そのもの」を使う (解決済みの辺種別・個体の宣言位置を代用しない。
// issue #41 是正1)。名前の組み立ては `naming::reference_paths`・
// `naming::internal_names` だけが行う (`format_ident!` はここに書かない、
// issue #41 是正13)。
//
// 段階2ではこの構造だけを用意する。実際のマクロ展開への配線は段階3で行う。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::naming::{個体参照パス, 錨関数名, 辺値参照パス, 辺参照パス};
use crate::static_graph::semantic::意味モデル;

pub(crate) fn 錨を組み立てる(意味モデル: &意味モデル) -> TokenStream {
    let 種別錨列 = 意味モデル.具体辺列().iter().map(|辺| {
        let 型参照 = 辺値参照パス(意味モデル, 辺);
        quote! { _: &#型参照<'_> }
    });
    let 辺名錨列 = 意味モデル.具体辺列().iter().map(|辺| {
        let 参照 = 辺参照パス(意味モデル, 辺.名前());
        quote! { _: &#参照<'_> }
    });
    let 端点錨列 = 意味モデル.具体辺列().iter().flat_map(|辺| {
        辺.端点トークン列().into_iter().map(|トークン| {
            let 参照 = 個体参照パス(意味モデル, トークン);
            quote! { _: &#参照<'_> }
        })
    });
    let 個体錨列 = 意味モデル.個体列().iter().map(|個体| {
        let 参照 = 個体参照パス(意味モデル, 個体.名前());
        quote! { _: &#参照<'_> }
    });
    let 引数列: Vec<TokenStream> = 種別錨列.chain(辺名錨列).chain(端点錨列).chain(個体錨列).collect();
    let 関数名 = 錨関数名();
    quote! {
        const _: () = {
            #[allow(dead_code)]
            fn #関数名(#(#引数列),*) {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    use crate::static_graph::literal::input::静的グラフ入力;
    use crate::static_graph::schema::input::静的グラフ型入力;
    use crate::static_graph::semantic::意味モデル;

    #[test]
    fn 錨は種別と個体それぞれの修飾済み型参照を持ち実行コードを生まない() {
        let schema: 静的グラフ型入力 = syn::parse2(quote! {
            schema 組織 {
                node 社員;
                node 部署;
                edge 所属 = (member: 社員) -> (team: 部署);
            }
        })
        .unwrap();
        let instance: 静的グラフ入力 = syn::parse2(quote! {
            graph 開発チーム;
            node 太郎 = 社員 { 名前: "太郎".into() };
            node 開発部 = 部署 { 名前: "開発部".into() };
            edge 太郎の所属 = 所属(太郎 -> 開発部);
        })
        .unwrap();
        let 意味モデル = 意味モデル::組み立てる(&schema, &instance);

        let トークン列 = 錨を組み立てる(&意味モデル);
        let コード = トークン列.to_string();
        assert!(コード.contains("fn __graphite_dsl_token_anchor"));
        // 種別トークン: schema module越しに修飾する (issue #41 是正2)。
        assert!(コード.contains("組織 :: 所属Edge"));
        // 辺名・個体名・端点のトークン: グラフmodule越しに修飾する。
        assert!(コード.contains("開発チーム :: 太郎の所属Ref"));
        assert!(コード.contains("開発チーム :: 太郎Ref"));
        assert!(コード.contains("開発チーム :: 開発部Ref"));

        // 呼ばれない関数シグネチャの型参照だけであり、実行コードを生まない
        // (関数本体に文が無い)。
        let file: syn::File = syn::parse2(quote! { #トークン列 }).unwrap();
        let syn::Item::Const(定数) = &file.items[0] else { panic!("const _ ブロックのはず") };
        let syn::Expr::Block(ブロック式) = 定数.expr.as_ref() else { panic!("ブロック式のはず") };
        let syn::Stmt::Item(syn::Item::Fn(関数)) = &ブロック式.block.stmts[0] else {
            panic!("関数定義のはず")
        };
        assert!(関数.block.stmts.is_empty(), "錨関数の本体は空でなければならない");
    }

    #[test]
    fn 自己ループ辺は端点トークンを2つ持つ() {
        let schema: 静的グラフ型入力 = syn::parse2(quote! {
            schema 組織 {
                node 社員;
                edge 上司 = (subordinate: 社員) -> (superior: 社員);
            }
        })
        .unwrap();
        let instance: 静的グラフ入力 = syn::parse2(quote! {
            graph 開発チーム;
            node 太郎 = 社員 { 名前: "太郎".into() };
            edge 自己 = 上司(太郎 -> 太郎);
        })
        .unwrap();
        let 意味モデル = 意味モデル::組み立てる(&schema, &instance);
        let 太郎の所属 = &意味モデル.具体辺列()[0];
        let [始点, 終点] = 太郎の所属.端点トークン列();
        assert_eq!(始点.to_string(), "太郎");
        assert_eq!(終点.to_string(), "太郎");

        let コード = 錨を組み立てる(&意味モデル).to_string();
        // 端点2つ分 + 個体宣言1つ分で、`開発チーム :: 太郎Ref` の出現が3回。
        assert_eq!(コード.matches("開発チーム :: 太郎Ref").count(), 3);
    }
}
