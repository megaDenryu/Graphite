// 意味モデルの組み立てが、種別名・端点名を解決済みの形で持つことを確かめる
// (issue #41 段階1)。

use quote::quote;

use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;

#[test]
fn schemaの辺種別列とinstanceの具体辺列を解決する() {
    let schema: 静的グラフ型入力 = syn::parse2(quote! {
        schema 組織 {
            node 社員;
            node 部署;
            edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
        }
    })
    .unwrap();
    let instance: 静的グラフ入力 = syn::parse2(quote! {
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部: 部署;
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    })
    .unwrap();

    let 意味モデル =
        crate::static_graph::internal::検証してから意味モデルを組み立てる(schema, instance);

    assert_eq!(意味モデル.グラフ名().to_string(), "開発チーム");
    assert_eq!(意味モデル.個体列().len(), 2);
    assert_eq!(意味モデル.辺種別列().len(), 1);
    assert_eq!(意味モデル.具体辺列().len(), 1);

    let 太郎 = 意味モデル.個体列().iter().find(|個体| 個体.名前().to_string() == "太郎").unwrap();
    let 太郎の所属 = &意味モデル.具体辺列()[0];
    assert_eq!(太郎の所属.種別().名前().to_string(), "所属");
    assert!(太郎の所属.端点に含むか(太郎.名前()));
    let 役割一覧: Vec<String> =
        太郎の所属.個体の役割一覧(太郎.名前()).iter().map(ToString::to_string).collect();
    assert_eq!(役割一覧, vec!["member".to_string()]);

    let 端点になっている辺: Vec<_> =
        意味モデル.この個体が端点になっている具体辺列(太郎.名前()).collect();
    assert_eq!(端点になっている辺.len(), 1);
}
