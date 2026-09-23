// 意味カード3: 端点の役割アクセサ (`team()`、issue #41 §5.2)。

use quote::quote;

use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::名前の由来;

use super::super::card_names::役割アクセサの追跡情報を作る;
use super::{src_main, 開発チームの意味モデルを作る};

#[test]
fn 意味カード3_役割アクセサ() {
    let 意味モデル = 開発チームの意味モデルを作る();
    let 開発部 = 意味モデル.個体列().iter().find(|個体| 個体.名前() == "開発部").unwrap();
    let 太郎の所属 = 意味モデル.具体辺列().iter().find(|辺| 辺.名前() == "太郎の所属").unwrap();
    let 役割 = 太郎の所属.個体の役割(開発部.名前()).unwrap();
    let 追跡 = 役割アクセサの追跡情報を作る(太郎の所属, 役割, 開発部, &src_main());
    assert!(matches!(
        追跡.由来(),
        名前の由来::SchemaRole { 種別名, 役割 } if 種別名 == "所属" && 役割 == "team"
    ));
    assert_eq!(
        追跡.意味カード(),
        "Graphite 静的グラフの端点の役割アクセサ。\n\
         \n\
         - 辺種別: `所属`\n\
         - 役割: `team: 部署`\n\
         - 具体辺: `太郎の所属`\n\
         - 具体端点: `開発部`\n\
         - 戻り値: `開発部Ref`\n\
         - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)\n\
         \n\
         宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`\n\
         \n\
         関係する instance 宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`"
    );
}

// 自己ループ辺 (始点と終点が同じ個体) の第一・第二役割アクセサが、
// それぞれ渡された役割で正しく区別されることを確かめる。呼び出し元が
// 役割を明示的に渡さず個体名から引き直すと、両方が「最初に一致した役割」
// (第一役割) に化けてしまう (issue #41 是正5)。
#[test]
fn 意味カード3_自己ループ辺の役割アクセサは第一役割と第二役割で異なる() {
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
    let 意味モデル: 意味モデル = 意味モデル::組み立てる(&schema, &instance);
    let 太郎 = 意味モデル.個体列().iter().find(|個体| 個体.名前() == "太郎").unwrap();
    let 自己 = 意味モデル.具体辺列().iter().find(|辺| 辺.名前() == "自己").unwrap();

    let 第一役割 = proc_macro2::Ident::new("subordinate", proc_macro2::Span::call_site());
    let 第二役割 = proc_macro2::Ident::new("superior", proc_macro2::Span::call_site());
    let 第一追跡 = 役割アクセサの追跡情報を作る(自己, &第一役割, 太郎, &src_main());
    let 第二追跡 = 役割アクセサの追跡情報を作る(自己, &第二役割, 太郎, &src_main());

    assert!(matches!(
        第一追跡.由来(),
        名前の由来::SchemaRole { 役割, .. } if 役割 == "subordinate"
    ));
    assert!(matches!(
        第二追跡.由来(),
        名前の由来::SchemaRole { 役割, .. } if 役割 == "superior"
    ));
    assert!(第一追跡.意味カード().contains("役割: `subordinate: 社員`"));
    assert!(第二追跡.意味カード().contains("役割: `superior: 社員`"));
}
