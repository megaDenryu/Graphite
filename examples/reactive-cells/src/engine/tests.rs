//! 再計算エンジンの単体テスト。確かめる内容ごとにサブモジュールへ分ける。

mod construction;
mod contract;
mod propagation;
mod recompute_scope;

use crate::schema::CellId;

/// セルキーの綴りから `CellId` を作る、テスト内での短縮形。
fn id(spelling: &str) -> CellId {
    CellId(spelling.to_string())
}
