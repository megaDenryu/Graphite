//! 再計算1件分の記録。

use crate::schema::CellId;

/// [`crate::engine::Engine::set_input`] が1回の更新で行った再計算1件分の記録。
///
/// `main.rs`/テストはこの列を「どのセルがどの順で再計算されたか」の
/// 証拠として読む。
#[derive(Debug, Clone, PartialEq)]
pub struct RecomputeStep {
    pub id: CellId,
    pub value: f64,
}
