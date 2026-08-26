use std::fmt;

/// リポジトリ内 Rust ソース参照が指す行番号の指定。
///
/// 行番号なしの参照 (ファイル全体を指す)・単一行・範囲の3種を判別共用体で
/// 表す。文字列のまま保持すると、範囲の開始・終了や「指定なし」の判定を
/// 使用箇所ごとにパースし直すことになる。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceLineSpan {
    Unspecified,
    Single(usize),
    Range(usize, usize),
}

impl SourceLineSpan {
    /// 実ファイルの行数と比較すべき最終行番号。指定が無ければ比較不要。
    pub fn last_line(&self) -> Option<usize> {
        match *self {
            Self::Unspecified => None,
            Self::Single(line) => Some(line),
            Self::Range(_, end) => Some(end),
        }
    }
}

impl fmt::Display for SourceLineSpan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => Ok(()),
            Self::Single(line) => write!(formatter, ":{line}"),
            Self::Range(start, end) => write!(formatter, ":{start}-{end}"),
        }
    }
}

/// このリポジトリ内 Rust ソースを指す1個の参照。リポジトリルート相対の
/// 綴りと行の指定を持つ。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceReference {
    path: String,
    line_span: SourceLineSpan,
}

impl SourceReference {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn line_span(&self) -> &SourceLineSpan {
        &self.line_span
    }

    /// crates/graphite/src/lib.rs・crates/graphite/src/lib.rs:12・
    /// crates/graphite/src/lib.rs:12-20 のいずれかの形を分解する。
    ///
    /// 行番号部分が数字でない、綴りにワイルドカード (`*`) やプレースホルダ
    /// (`<...>`) を含む等、実在するファイル1個を指しているとは限らない綴りは
    /// `None` を返す。呼び出し側 (`ReferenceTarget::classify`) はその場合
    /// ソース参照として扱わない。「複数のテストファイルの総称」のような
    /// 散文中の言及まで検査対象にすると、実在しない綴りとして誤検出する。
    pub fn parse(token: &str) -> Option<Self> {
        let (path, line_span) = match token.split_once(".rs:") {
            Some((stem, rest)) => (format!("{stem}.rs"), parse_line_span(rest)?),
            None if token.ends_with(".rs") => (token.to_string(), SourceLineSpan::Unspecified),
            None => return None,
        };
        if !is_literal_path(&path) {
            return None;
        }
        Some(Self { path, line_span })
    }
}

/// 実在するファイル1個を指しうる綴りか。ワイルドカードやプレースホルダを含む
/// 綴りは「該当するファイル群」を総称する散文であり、個別のファイルを指さない。
fn is_literal_path(path: &str) -> bool {
    !path.contains(['*', '<', '>'])
}

impl fmt::Display for SourceReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.path, self.line_span)
    }
}

fn parse_line_span(rest: &str) -> Option<SourceLineSpan> {
    match rest.split_once('-') {
        Some((start, end)) => Some(SourceLineSpan::Range(
            start.parse().ok()?,
            end.parse().ok()?,
        )),
        None => Some(SourceLineSpan::Single(rest.parse().ok()?)),
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceLineSpan, SourceReference};

    #[test]
    fn 行番号なしの参照を分解する() {
        let reference = SourceReference::parse("crates/graphite/src/lib.rs").unwrap();
        assert_eq!(reference.path(), "crates/graphite/src/lib.rs");
        assert_eq!(reference.line_span(), &SourceLineSpan::Unspecified);
    }

    #[test]
    fn 単一行の参照を分解する() {
        let reference = SourceReference::parse("crates/graphite/src/lib.rs:12").unwrap();
        assert_eq!(reference.line_span(), &SourceLineSpan::Single(12));
        assert_eq!(reference.line_span().last_line(), Some(12));
    }

    #[test]
    fn 範囲の参照を分解する() {
        let reference = SourceReference::parse("crates/graphite/src/lib.rs:12-20").unwrap();
        assert_eq!(reference.line_span(), &SourceLineSpan::Range(12, 20));
        assert_eq!(reference.line_span().last_line(), Some(20));
    }

    #[test]
    fn 行番号が数字でなければ分解に失敗する() {
        assert!(SourceReference::parse("crates/graphite/src/lib.rs:abc").is_none());
    }

    #[test]
    fn rsで終わらないトークンは分解に失敗する() {
        assert!(SourceReference::parse("crates/graphite/src/lib.toml").is_none());
    }

    #[test]
    fn ワイルドカードを含む綴りは分解に失敗する() {
        assert!(SourceReference::parse("crates/graphite/tests/compute_graph_*.rs").is_none());
    }

    #[test]
    fn プレースホルダを含む綴りは分解に失敗する() {
        assert!(SourceReference::parse("crates/graphite/tests/generated/<名前>.rs").is_none());
    }
}
