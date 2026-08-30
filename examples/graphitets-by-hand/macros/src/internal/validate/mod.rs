//! schemaとinstanceの相互検証。schema単体・instance単体の構造検証 (名前の
//! 重複・端点の宣言漏れ) は既に通っている前提で、両者を突き合わせないと
//! 検出できない誤りだけをここで見る: 種別の存在・向きの一致・積み荷有無の
//! 一致・端点の実体型の一致 (species)、多重度制約 (multiplicity)、対一意
//! 制約 (unique_pair)。

mod multiplicity;
mod species;
mod unique_pair;

use crate::literal::input::静的グラフ入力;
use crate::schema::input::静的グラフ型入力;

pub(super) fn 相互検証する(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> syn::Result<()> {
    for 辺 in &instance.辺宣言達 {
        species::辺を検証する(schema, instance, 辺)?;
    }
    multiplicity::検証する(schema, instance)?;
    unique_pair::検証する(schema, instance)?;
    Ok(())
}
