//! 行番号付きのパースエラー。どのパイプライン定義行が壊れているかを
//! 利用者が即座に特定できるように、必ず行番号を伴わせる。

use std::fmt;

// 行番号付きパースエラー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}行目: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

pub(super) fn err(line: usize, message: impl Into<String>) -> ParseError {
    ParseError {
        line,
        message: message.into(),
    }
}
