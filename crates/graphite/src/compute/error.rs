//! 計算グラフの凍結時に返しうる構築エラーを所有する。

use std::error::Error as StdError;
use std::fmt;

use crate::{CycleError, GraphError};

/// [`crate::ComputeGraphBuilder::freeze`] が返しうる構築エラー。
///
/// - [`Self::Graph`] — キー重複・未宣言依存 ([`crate::GraphError`] をそのまま
///   運ぶ。stringly-typed な理由メッセージへ潰さず型付きのまま公開する、
///   `docs/design_principles.md` 原則1)。
/// - [`Self::Cycle`] — 循環依存 ([`crate::CycleError`] をそのまま運ぶ。
///   `cycle` フィールドに循環を構成するキー列がそのまま入っている)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeGraphError {
    Graph(GraphError<String>),
    Cycle(CycleError<String>),
}

impl fmt::Display for ComputeGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputeGraphError::Graph(e) => write!(f, "{e}"),
            ComputeGraphError::Cycle(e) => write!(f, "{e}"),
        }
    }
}

impl StdError for ComputeGraphError {}
