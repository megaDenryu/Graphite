//! 静的グラフの追跡情報 (issue #41)。生成名がどのDSL宣言に由来するか
//! (`名前の由来`)・Graphiteが定義する固定語彙かどうか (`固定語彙`)・
//! それらを利用者語彙で説明する意味カード (`追跡情報::意味カード`) を持つ。
//!
//! ここは型と意味カードの組み立て規則だけを持ち、`意味モデル` を読んで
//! 具体的な追跡情報を組み立てる仕事は `static_graph/naming` が担う
//! (このmoduleは `意味モデル` を知らない)。

mod declaration_ref;
mod fixed_vocabulary_kind;
mod origin;
mod semantic_item;
mod tracked_info;

pub(crate) use fixed_vocabulary_kind::固定語彙;
pub(crate) use origin::名前の由来;
pub(crate) use semantic_item::意味項目;
pub(crate) use tracked_info::{追跡情報, 追跡情報構築器};
