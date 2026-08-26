use std::fmt;

use crate::source_reference::SourceReference;

/// リポジトリ内 Rust ソースを置くディレクトリの先頭綴り。
///
/// この一覧にある先頭を持つトークンだけをソース参照として分類する。
/// `docs/` 配下の文書からの相対パスのような、ソースを指さない `.rs` に
/// 似た綴りを誤って拾わないための絞り込みである。
const SOURCE_AREAS: [&str; 4] = ["crates/", "examples/", "xtask/", "verification/"];

/// 抽出したトークン1個が指す先の分類。
///
/// このリポジトリの文書間参照は、ほぼ全てがバッククォートで囲んだリポジトリ
/// ルート相対のプレーンテキスト (`docs/schema_v4.md` の形) である。先頭が
/// `../` のものは別リポジトリ (Bullet) の文書を指すため、このリポジトリでは
/// 実在を判定できない。`crates/`・`examples/`・`xtask/`・`verification/` 始まりで
/// `.rs` に終わるものはリポジトリ内 Rust ソースへの参照である。
///
/// 注意: 分類はトークン全体で行う。部分文字列として `docs/` を切り出すと、
/// 別リポジトリを正しく指した `../Bullet/docs/...` と、先頭の `../Bullet` が
/// 欠けたまま自リポジトリを指してしまう綴りを区別できなくなる。
pub enum ReferenceTarget {
    /// このリポジトリの `docs/` 配下を指す綴り。実在を検査する。
    RepositoryDocument(DocumentPath),
    /// このリポジトリ内 Rust ソースを指す綴り。実在と行番号範囲を検査する。
    SourceCode(SourceReference),
    /// `../` で始まる別リポジトリの文書を指す綴り。件数だけ数える。
    ExternalDocument,
}

impl ReferenceTarget {
    /// トークン1個を分類する。文書・ソースのいずれも指さないトークンは
    /// `None` を返す。
    pub fn classify(token: &str) -> Option<Self> {
        if token.ends_with(".md") || token.ends_with(".html") {
            if token.starts_with("../") {
                return Some(Self::ExternalDocument);
            }
            if token.starts_with("docs/") {
                return Some(Self::RepositoryDocument(DocumentPath::new(token)));
            }
            return None;
        }
        if SOURCE_AREAS.iter().any(|area| token.starts_with(area)) {
            return SourceReference::parse(token).map(Self::SourceCode);
        }
        None
    }
}

/// リポジトリルートからの相対で `docs/` 配下の文書を指す綴り。
///
/// 索引との突き合わせと実在判定はこの綴りの一致で行うため、区切り文字は
/// スラッシュへ正規化した形だけを保持する。
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocumentPath(String);

impl DocumentPath {
    fn new(spelling: &str) -> Self {
        Self(spelling.replace('\\', "/"))
    }

    /// 実ファイルの走査結果から、索引と突き合わせられる綴りを作る。
    pub fn from_relative_display(display: &str) -> Self {
        Self::new(display)
    }

    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DocumentPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// 参照が書かれていた場所。リポジトリルート相対のファイル綴りと行番号を持つ。
pub struct ReferenceOrigin {
    file: String,
    line_number: usize,
}

impl ReferenceOrigin {
    pub fn new(file: String, line_number: usize) -> Self {
        Self { file, line_number }
    }
}

impl fmt::Display for ReferenceOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.file, self.line_number)
    }
}

/// 1箇所に書かれた自リポジトリ文書への参照。
pub struct DocumentReference {
    origin: ReferenceOrigin,
    target: DocumentPath,
}

impl DocumentReference {
    pub fn new(origin: ReferenceOrigin, target: DocumentPath) -> Self {
        Self { origin, target }
    }

    pub fn target(&self) -> &DocumentPath {
        &self.target
    }
}

impl fmt::Display for DocumentReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} -> {}", self.origin, self.target)
    }
}

/// 1箇所に書かれた自リポジトリソースへの参照。
pub struct SourceCodeReference {
    origin: ReferenceOrigin,
    target: SourceReference,
}

impl SourceCodeReference {
    pub fn new(origin: ReferenceOrigin, target: SourceReference) -> Self {
        Self { origin, target }
    }

    pub fn target(&self) -> &SourceReference {
        &self.target
    }
}

impl fmt::Display for SourceCodeReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} -> {}", self.origin, self.target)
    }
}
