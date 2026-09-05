//! 検査対象の Rust ソース1件と、その構文解析の結果。
//!
//! 解析に失敗したファイルを黙って読み飛ばさないために、失敗を値として持つ。

use std::fs;
use std::path::PathBuf;

use syn::File;

// 検査対象の Rust ソース1件。綴り (リポジトリルート相対) と実体の位置を持つ。
pub(crate) struct RustSource {
    spelling: String,
    path: PathBuf,
}

// 構文解析の結果。読めなかった場合も値として残し、違反として数える。
pub(crate) enum ParsedRustSource {
    Parsed { spelling: String, syntax: File },
    Unreadable { spelling: String, reason: String },
}

impl RustSource {
    pub(crate) fn new(spelling: String, path: PathBuf) -> Self {
        Self { spelling, path }
    }

    pub(crate) fn spelling(&self) -> &str {
        &self.spelling
    }

    // 読み込みと構文解析をまとめて行い、どちらの失敗も `Unreadable` にする。
    pub(crate) fn parse(self) -> ParsedRustSource {
        let text = match fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(error) => {
                return ParsedRustSource::Unreadable {
                    spelling: self.spelling,
                    reason: format!("読み込みに失敗しました: {error}"),
                }
            }
        };
        match syn::parse_file(&text) {
            Ok(syntax) => ParsedRustSource::Parsed {
                spelling: self.spelling,
                syntax,
            },
            Err(error) => ParsedRustSource::Unreadable {
                spelling: self.spelling,
                reason: format!("構文解析に失敗しました: {error}"),
            },
        }
    }
}
