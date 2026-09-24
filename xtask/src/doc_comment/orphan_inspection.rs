//! どのパッケージにも属さない Rust ソースの検出 (issue #32 検収の是正)。
//!
//! パッケージ単位の走査 (`doc_comment_packages`) は、Cargo.toml を持つ
//! ディレクトリの配下しか見ない。呼び出し元 (`DocCommentInspection`) は、
//! `crates`・`examples`・`verification` の配下を丸ごと再走査してこの型へ渡し、
//! この型は既知のどのパッケージの配下にも収まらない Rust ソースを検査結果として
//! 持つ。この型が走査そのものを担わないのは、兄弟の `GeneratedAreaReport` と
//! 同じく、走査を呼び出し元のメソッドに閉じるためである。

#[cfg(test)]
mod tests;

use std::fmt::Write;

use crate::inspected_area::InspectedArea;
use crate::rust_source::RustSource;

// どのパッケージにも属さない Rust ソースの検査結果。
pub(super) struct OrphanSourceReport {
    spellings: Vec<String>,
}

impl OrphanSourceReport {
    // この関数は、`known_areas` のどれの配下にも無い Rust ソースを集める。
    // 走査は呼び出し元が既に済ませ、この関数は取得済みの `sources` だけを見る。
    pub(super) fn inspect(sources: Vec<RustSource>, known_areas: &[&InspectedArea]) -> Self {
        let mut spellings = Vec::new();
        for source in sources {
            let spelling = source.spelling().to_string();
            let belongs = known_areas
                .iter()
                .any(|area| area.contains_spelling(&spelling));
            if !belongs {
                spellings.push(spelling);
            }
        }
        spellings.sort();
        Self { spellings }
    }

    pub(super) fn is_clean(&self) -> bool {
        self.spellings.is_empty()
    }

    pub(super) fn render(&self) -> String {
        let mut text = format!(
            "所属パッケージが無い Rust ソース (パッケージ単位の走査が取りこぼしていないかを確かめる。{}件):\n",
            self.spellings.len()
        );
        for spelling in &self.spellings {
            let _ = writeln!(text, "  {spelling}");
        }
        text
    }
}
