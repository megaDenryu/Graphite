// 多重度制約 (`where each <役割>: 下限..上限`) の検証。役割の実体型に属する
// 全個体 (その種別の辺を1本も持たない個体も含む、instanceのnode宣言達を全
// 走査) について、その役割位置での本数を数えて範囲を検査する。

use crate::literal::input::{辺形状 as 具体形状, 静的グラフ入力};
use crate::schema::input::{制約, 辺形状 as 型形状, 静的グラフ型入力};

pub(super) fn 検証する(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> syn::Result<()> {
    for 型宣言 in &schema.辺宣言達 {
        let (始点役割, 始点型, 終点型) = match &型宣言.形状 {
            型形状::有向 { 始点役割, 始点型, 終点型, .. } => (始点役割, 始点型, 終点型),
            // 無向辺には役割名が無く、schema検証 (schema::validate) が
            // `each` 制約を無向辺へ付けること自体を既に拒んでいる。
            型形状::無向 { .. } => continue,
        };

        for 制約 in &型宣言.制約達 {
            let (役割, 下限, 上限) = match 制約 {
                制約::多重度 { 役割, 下限, 上限 } => (役割, *下限, *上限),
                制約::対一意 => continue,
            };
            let 始点側か = 役割 == 始点役割;
            let 対象型 = if 始点側か { 始点型 } else { 終点型 };

            for 個体 in instance.ノード宣言達.iter().filter(|n| &n.実体型 == 対象型) {
                let 本数 = instance
                    .辺宣言達
                    .iter()
                    .filter(|e| e.種別 == 型宣言.名前)
                    .filter(|e| match &e.形状 {
                        具体形状::有向 { 始点, 終点, .. } => {
                            if 始点側か { 始点 == &個体.名前 } else { 終点 == &個体.名前 }
                        }
                        具体形状::無向 { .. } => false,
                    })
                    .count();
                if 本数 < 下限 || 本数 > 上限 {
                    return Err(syn::Error::new_spanned(
                        &個体.名前,
                        format!(
                            "多重度制約違反: `{}` の `{}` (役割 `{}`) の本数が{}件で、範囲 {}..{} の外です",
                            個体.名前,
                            型宣言.名前,
                            役割,
                            本数,
                            下限,
                            上限表示(上限),
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn 上限表示(上限: usize) -> String {
    if 上限 == usize::MAX { "*".to_string() } else { 上限.to_string() }
}
