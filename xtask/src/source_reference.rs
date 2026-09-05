use std::fmt;

// 行番号1個分の指定。単一行か範囲かを判別共用体で表す。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LineExtent {
    Single(usize),
    Range(usize, usize),
}

impl LineExtent {
    // `1 <= 開始 <= 終了` を満たすときだけ受理する。`:0` や `:34-12`
    // (開始が終了より大きい逆転範囲) は構築時に弾く。黙って受理すると、
    // 実ファイルの行数と比較しようがない指定が検査対象から消える。
    fn parse(part: &str) -> Option<Self> {
        match part.split_once('-') {
            Some((start, end)) => {
                let start: usize = start.parse().ok()?;
                let end: usize = end.parse().ok()?;
                (start >= 1 && start <= end).then_some(Self::Range(start, end))
            }
            None => {
                let line: usize = part.parse().ok()?;
                (line >= 1).then_some(Self::Single(line))
            }
        }
    }

    fn end(&self) -> usize {
        match *self {
            Self::Single(line) => line,
            Self::Range(_, end) => end,
        }
    }

    // この指定が指すソース行だけを取り出す。終了行が実ファイルの末尾を
    // 超える場合は末尾で止める。行数の超過そのものは
    // `SourceCodeReference::evaluate` が別に違反として報告するため、ここで
    // 二重に報告しない。
    pub fn lines_within<'a>(&self, lines: &'a [String]) -> &'a [String] {
        let (start, end) = match *self {
            Self::Single(line) => (line, line),
            Self::Range(start, end) => (start, end),
        };
        let begin = (start - 1).min(lines.len());
        &lines[begin..end.min(lines.len()).max(begin)]
    }
}

impl fmt::Display for LineExtent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(line) => write!(formatter, "{line}"),
            Self::Range(start, end) => write!(formatter, "{start}-{end}"),
        }
    }
}

// リポジトリ内 Rust ソース参照が指す行番号の指定。
//
// 行番号なしの参照 (ファイル全体を指す)・単一行・範囲・カンマ区切りの
// 複数指定 (`3-4, 9-11` や `336, 365, 376` のように単一行と範囲が混在
// してよい) の4種を判別共用体で表す。文字列のまま保持すると、範囲の
// 開始・終了や「指定なし」の判定を使用箇所ごとにパースし直すことになる。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceLineSpan {
    Unspecified,
    Single(usize),
    Range(usize, usize),
    List(Vec<LineExtent>), // カンマ区切りで単一行・範囲が混在する複数指定
}

impl SourceLineSpan {
    // `.rs:` の直後 (`rest`) を分解する。`3-4, 9-11` のようなカンマ区切りは
    // `List` へ、単発の指定は従来どおり `Single`/`Range` へ振り分ける。
    // いずれかの要素が `1 <= 開始 <= 終了` を満たさなければ全体を `None` に
    // する。
    pub fn parse(rest: &str) -> Option<Self> {
        if !rest.contains(',') {
            return match LineExtent::parse(rest)? {
                LineExtent::Single(line) => Some(Self::Single(line)),
                LineExtent::Range(start, end) => Some(Self::Range(start, end)),
            };
        }
        let mut extents = Vec::new();
        for part in rest.split(',') {
            extents.push(LineExtent::parse(part.trim())?);
        }
        Some(Self::List(extents))
    }

    // 範囲ごとに分けた指定の一覧。行番号なしの参照では空になる。
    //
    // カンマ区切りの複数指定を1本へ連結せずに範囲ごとへ分けるのは、引用本文の
    // 照合が範囲ごとに本文を作る必要があるためである。連結すると、飛ばした
    // 区間の境界をまたぐ偽の一致が起きる。
    pub fn extents(&self) -> Vec<LineExtent> {
        match self {
            Self::Unspecified => Vec::new(),
            Self::Single(line) => vec![LineExtent::Single(*line)],
            Self::Range(start, end) => vec![LineExtent::Range(*start, *end)],
            Self::List(extents) => extents.clone(),
        }
    }

    // 実ファイルの行数と比較すべき最終行番号。指定が無ければ比較不要。
    // 複数指定では列内の最大の終了行で比較する。
    pub fn last_line(&self) -> Option<usize> {
        match self {
            Self::Unspecified => None,
            Self::Single(line) => Some(*line),
            Self::Range(_, end) => Some(*end),
            Self::List(extents) => extents.iter().map(LineExtent::end).max(),
        }
    }
}

impl fmt::Display for SourceLineSpan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => Ok(()),
            Self::Single(line) => write!(formatter, ":{line}"),
            Self::Range(start, end) => write!(formatter, ":{start}-{end}"),
            Self::List(extents) => {
                write!(formatter, ":")?;
                for (index, extent) in extents.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, ", ")?;
                    }
                    write!(formatter, "{extent}")?;
                }
                Ok(())
            }
        }
    }
}

// このリポジトリ内 Rust ソースを指す1個の参照。リポジトリルート相対の
// 綴りと行の指定を持つ。
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

    // crates/graphite/src/lib.rs・crates/graphite/src/lib.rs:12・
    // crates/graphite/src/lib.rs:12-20・
    // crates/graphite/src/lib.rs:12-20, 30 のいずれかの形を分解する。
    //
    // 行番号部分が数字でない・`1 <= 開始 <= 終了` を満たさない、綴りに
    // ワイルドカード (`*`) やプレースホルダ (`<...>`) を含む等、実在する
    // ファイル1個を指しているとは限らない綴りは `None` を返す。呼び出し側
    // (`ReferenceTarget::classify`) はその場合、綴りがファイル群の総称
    // (ワイルドカード・プレースホルダ) かどうかを見て、総称でなければ
    // 「解析できないソース参照」として検査対象にする。黙って検査から
    // 消してよいのは総称の場合だけである。
    pub fn parse(token: &str) -> Option<Self> {
        let (path, line_span) = match token.split_once(".rs:") {
            Some((stem, rest)) => (format!("{stem}.rs"), SourceLineSpan::parse(rest)?),
            None if token.ends_with(".rs") => (token.to_string(), SourceLineSpan::Unspecified),
            None => return None,
        };
        if !is_literal_path(&path) {
            return None;
        }
        Some(Self { path, line_span })
    }
}

impl fmt::Display for SourceReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.path, self.line_span)
    }
}

// 実在するファイル1個を指しうる綴りか。ワイルドカードやプレースホルダを含む
// 綴りは「該当するファイル群」を総称する散文であり、個別のファイルを指さない。
pub(crate) fn is_literal_path(path: &str) -> bool {
    !path.contains(['*', '<', '>'])
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
    fn カンマ区切りの複数範囲を分解する() {
        let reference = SourceReference::parse("crates/graphite/src/lib.rs:3-4, 9-11").unwrap();
        assert_eq!(reference.line_span().last_line(), Some(11));
        assert_eq!(reference.to_string(), "crates/graphite/src/lib.rs:3-4, 9-11");
    }

    #[test]
    fn カンマ区切りの単一行の列を分解する() {
        let reference = SourceReference::parse("crates/graphite/src/lib.rs:336, 365, 376").unwrap();
        assert_eq!(reference.line_span().last_line(), Some(376));
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

    #[test]
    fn 行番号0の指定は分解に失敗する() {
        assert!(SourceReference::parse("crates/graphite/src/lib.rs:0").is_none());
    }

    #[test]
    fn 開始が終了より大きい範囲は分解に失敗する() {
        assert!(SourceReference::parse("crates/graphite/src/lib.rs:34-12").is_none());
    }

    #[test]
    fn カンマ区切りの列に無効な要素があれば全体が分解に失敗する() {
        assert!(SourceReference::parse("crates/graphite/src/lib.rs:3-4, 0").is_none());
    }
}
