// 意味カード5: 積み荷アクセサ (`.任命()`)。積み荷アクセサ
// (file/instance_file/edge_ref.rs の `積み荷アクセサを組み立てる`) は元々
// doc を持たなかった。`名前の由来::SchemaPayloadRole` を新設し、意味カードを
// 作れるようにする。

use quote::quote;

use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::名前の由来;

use super::super::card_names::積み荷アクセサの追跡情報を作る;
use super::src_main;

#[test]
fn 意味カード5_積み荷アクセサ() {
    let schema: 静的グラフ型入力 = syn::parse2(quote! {
        schema 組織 {
            node 社員;
            edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員);
        }
    })
    .unwrap();
    let instance: 静的グラフ入力 = syn::parse2(quote! {
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 次郎 = 社員 { 名前: "次郎".into() };
        edge 太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 2020 }]-> 次郎);
    })
    .unwrap();
    let 意味モデル: 意味モデル =
        crate::static_graph::internal::検証してから意味モデルを組み立てる(schema, instance);
    let 太郎の上司 = 意味モデル.具体辺列().iter().find(|辺| 辺.名前() == "太郎の上司").unwrap();

    let 追跡 = 積み荷アクセサの追跡情報を作る(太郎の上司, &src_main());
    assert!(matches!(
        追跡.由来(),
        名前の由来::SchemaPayloadRole { 積み荷役割, .. } if 積み荷役割 == "任命"
    ));
    assert_eq!(
        追跡.意味カード(),
        "Graphite 静的グラフの積み荷アクセサ。\n\
         \n\
         - 辺種別: `上司`\n\
         - 積み荷: `任命: 任命記録`\n\
         - 具体辺: `太郎の上司`\n\
         \n\
         宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員)`\n\
         \n\
         関係する instance 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`"
    );
}
