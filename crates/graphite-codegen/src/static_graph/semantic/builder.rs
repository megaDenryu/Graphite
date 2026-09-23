// 意味モデルの組み立て。schemaとinstanceの相互検証 (`internal::validate`)
// が通った後にだけ呼ぶ。種別名→辺種別、端点名→個体の解決はここで1回だけ
// 行い、internal/codegen 側の再引き当て (`.expect(..)`) を無くす。

use crate::static_graph::literal::input::{
    ノード宣言 as instanceノード宣言, 辺中身, 辺形状 as instance辺形状, 辺宣言 as instance辺宣言,
    静的グラフ入力,
};
use crate::static_graph::schema::input::{辺形状 as 型形状, 静的グラフ型入力};

use super::{具体辺, 具体辺形状, 意味モデル, 個体, 辺種別};

pub(super) fn 意味モデルを組み立てる(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> 意味モデル {
    let 個体列: Vec<個体> = instance.ノード宣言達.iter().map(個体を作る).collect();
    let 辺種別列 = super::辺種別列をschemaから組み立てる(schema);
    let 具体辺列: Vec<具体辺> =
        instance.辺宣言達.iter().map(|辺| 具体辺を作る(辺, &辺種別列, &個体列)).collect();

    意味モデル {
        グラフ名: instance.グラフ名.clone(),
        schema名: schema.schema名.clone(),
        個体列,
        辺種別列,
        具体辺列,
    }
}

fn 個体を作る(宣言: &instanceノード宣言) -> 個体 {
    個体::new(宣言.名前.clone(), 宣言.実体型.clone(), 宣言.値.clone())
}

fn 具体辺を作る(辺: &instance辺宣言, 辺種別列: &[辺種別], 個体列: &[個体]) -> 具体辺 {
    let 種別 = 辺種別列
        .iter()
        .find(|種別| 種別.名前() == &辺.種別)
        .expect("相互検証済みなので種別は必ず実在する")
        .clone();
    let 個体を名前で探す = |名前: &proc_macro2::Ident| -> 個体 {
        個体列
            .iter()
            .find(|個体| 個体.名前() == 名前)
            .expect("相互検証済みなので端点は必ず宣言されている")
            .clone()
    };
    // 向きに応じた役割名は、この関数だけがschemaと突き合わせて解決する
    // (唯一の解決点)。以降 (file/・internal/codegen) は `具体辺形状` が
    // 既に持つ役割を読むだけで、schemaの辺形状へ再度突き合わせない
    // (issue #41 是正15)。
    let 形状 = match (&辺.形状, 種別.形状()) {
        (instance辺形状::有向 { 始点, 終点, 中身 }, 型形状::有向 { 始点役割, 終点役割, .. }) => {
            具体辺形状::有向 {
                始点役割: 始点役割.clone(),
                始点: 個体を名前で探す(始点),
                始点トークン: 始点.clone(),
                終点役割: 終点役割.clone(),
                終点: 個体を名前で探す(終点),
                終点トークン: 終点.clone(),
                積み荷式: 積み荷式を取り出す(中身),
            }
        }
        (instance辺形状::無向 { 端点1, 端点2, 中身 }, 型形状::無向 { 第1役割, 第2役割, .. }) => {
            具体辺形状::無向 {
                第1役割: 第1役割.clone(),
                端点1: 個体を名前で探す(端点1),
                端点1トークン: 端点1.clone(),
                第2役割: 第2役割.clone(),
                端点2: 個体を名前で探す(端点2),
                端点2トークン: 端点2.clone(),
                積み荷式: 積み荷式を取り出す(中身),
            }
        }
        _ => unreachable!("相互検証済みなので向きは一致している"),
    };
    具体辺::new(辺.名前.clone(), 種別, 辺.種別.clone(), 形状)
}

fn 積み荷式を取り出す(中身: &辺中身) -> Option<syn::Expr> {
    match 中身 {
        辺中身::無積み荷 => None,
        辺中身::積み荷あり(式) => Some(式.clone()),
    }
}
