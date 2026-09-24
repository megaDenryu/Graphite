use super::*;
use quote::quote;

use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;

const 生成パス: &str = "generated/開発チーム.rs";

fn 意味モデルを作る(instance本文: TokenStream) -> 意味モデル {
    let schema: 静的グラフ型入力 = syn::parse2(quote! {
        schema 組織 {
            node 社員;
            node 部署;
            edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署);
        }
    })
    .unwrap();
    let instance: 静的グラフ入力 = syn::parse2(instance本文).unwrap();
    crate::static_graph::internal::検証してから意味モデルを組み立てる(schema, instance)
}

#[test]
fn 項目位置の個体値マクロは捕捉しない関数を経由し名前はグラフ名を含む() {
    let 意味モデル = 意味モデルを作る(quote! {
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -[任命記録 { 任命日: 2020 }]-> 開発部);
    });
    let マクロ名 = 個体値マクロ名(意味モデル.グラフ名(), 生成パス).ident().to_string();
    assert!(マクロ名.starts_with("__graphite_values_開発チーム_"), "グラフ名の後ろにinstance印が付くこと: {マクロ名}");

    let コード = 個体値マクロを組み立てる(&意味モデル, 生成パス).to_string();
    assert!(コード.contains(&マクロ名));
    assert!(コード.contains("fn __graphite_value_太郎_開発チーム"));
    assert!(!コード.contains("impl"), "implブロックを使わないこと (non_local_definitions対策)");
    assert!(!コード.contains("let __graphite_captured"), "項目位置ではクロージャ束縛を使わないこと");
}

#[test]
fn 関数内の個体値マクロはlet束縛したクロージャを経由する() {
    let 意味モデル = 意味モデルを作る(quote! {
        graph 開発チーム in fn;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部 = 部署 { 名前: "開発部".into() };
    });

    let コード = 個体値マクロを組み立てる(&意味モデル, 生成パス).to_string();
    assert!(コード.contains("let __graphite_captured_太郎_開発チーム"));
    assert!(!コード.contains("move ||"), "宣言の後でもローカルを借りたまま使えるようmoveを付けないこと");
    assert!(!コード.contains("fn __graphite_value_"), "関数内位置ではfnを使わないこと");
}

#[test]
fn 同じグラフ名でも生成パスが違えば個体値マクロ名が衝突しない() {
    let 意味モデル = 意味モデルを作る(quote! {
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部 = 部署 { 名前: "開発部".into() };
    });
    let 一つ目 = 個体値マクロ名(意味モデル.グラフ名(), "generated/m1.rs").ident().to_string();
    let 二つ目 = 個体値マクロ名(意味モデル.グラフ名(), "generated/m2.rs").ident().to_string();
    assert_ne!(一つ目, 二つ目, "生成パスが違うinstance同士は同じグラフ名でも別の名前になること");
}

#[test]
fn 積み荷値マクロの名前はグラフ名を含みimplを使わない() {
    let 意味モデル = 意味モデルを作る(quote! {
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -[任命記録 { 任命日: 2020 }]-> 開発部);
    });
    let マクロ名 = 積み荷値マクロ名(意味モデル.グラフ名(), 生成パス).ident().to_string();
    assert!(マクロ名.starts_with("__graphite_payloads_開発チーム_"), "グラフ名の後ろにinstance印が付くこと: {マクロ名}");

    let コード = 積み荷値マクロを組み立てる(&意味モデル, 生成パス).to_string();
    assert!(コード.contains(&マクロ名));
    assert!(コード.contains("任命記録"));
    assert!(!コード.contains("impl"), "implブロックを使わないこと (non_local_definitions対策)");
}
