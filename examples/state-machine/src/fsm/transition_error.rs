//! 未定義の遷移を要求したことを表す誤り。

use std::fmt;

use super::event::Event;
use crate::schema::OrderStateId;

/// `state` の状態で `event` に対応する遷移が定義されていないことを表す。
///
/// bool フラグ持ち・enum+match 散在の設計では「その状態でそのイベントは
/// 無効」ということが実行時まで (最悪本番まで) 分からない。ここでは
/// `OrderFsm` が持つ遷移表を引いた結果として型で返るので、呼び出し側は
/// 必ず `Result` を処理しなければならない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionError {
    pub state: OrderStateId,
    pub event: Event,
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "状態 {:?} でイベント `{}` は定義されていません",
            self.state, self.event
        )
    }
}

impl std::error::Error for TransitionError {}
