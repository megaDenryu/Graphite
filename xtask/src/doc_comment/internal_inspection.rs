//! 内部領域の検査。項目に付いた `///` を1件も残さないことを確かめる。

use std::fmt::Write;

use syn::visit::{self, Visit};
use syn::{Attribute, Item};

use super::attribute_facts::{is_outer_doc_comment, is_procedural_macro_entry};
use crate::rust_source::{ParsedRustSource, RustSource};

// 内部領域1件の検査結果。撤去作業の一覧としてそのまま読めるよう、ファイルごとの
// 件数を残す。
pub(super) struct InternalAreaReport {
    spelling: String,
    file_count: usize,
    files_with_doc_comments: Vec<(String, usize)>,
    doc_comment_count: usize,
    unreadable_files: Vec<String>,
}

impl InternalAreaReport {
    pub(super) fn inspect(spelling: String, sources: Vec<RustSource>) -> Self {
        let file_count = sources.len();
        let mut files_with_doc_comments = Vec::new();
        let mut doc_comment_count = 0;
        let mut unreadable_files = Vec::new();
        for source in sources {
            match source.parse() {
                ParsedRustSource::Unreadable { spelling, reason } => {
                    unreadable_files.push(format!("{spelling}: {reason}"));
                }
                ParsedRustSource::Parsed { spelling, syntax } => {
                    let mut counter = OuterDocCommentCounter::default();
                    counter.visit_file(&syntax);
                    if counter.count > 0 {
                        doc_comment_count += counter.count;
                        files_with_doc_comments.push((spelling, counter.count));
                    }
                }
            }
        }
        Self {
            spelling,
            file_count,
            files_with_doc_comments,
            doc_comment_count,
            unreadable_files,
        }
    }

    pub(super) fn is_clean(&self) -> bool {
        self.doc_comment_count == 0 && self.unreadable_files.is_empty()
    }

    pub(super) fn render(&self) -> String {
        let mut text = format!(
            "  {} — {}ファイル、項目の `///` {}件、解析できないファイル {}件\n",
            self.spelling,
            self.file_count,
            self.doc_comment_count,
            self.unreadable_files.len()
        );
        for (spelling, count) in &self.files_with_doc_comments {
            let _ = writeln!(text, "    {spelling} — {count}件");
        }
        for failure in &self.unreadable_files {
            let _ = writeln!(text, "    解析できません: {failure}");
        }
        text
    }
}

// `///` の総数を数える。手続き型マクロの入口 (関数にしか付かない) は公開面なので
// その項目ごと数えない。
#[derive(Default)]
struct OuterDocCommentCounter {
    count: usize,
}

impl<'ast> Visit<'ast> for OuterDocCommentCounter {
    fn visit_item(&mut self, item: &'ast Item) {
        if let Item::Fn(function) = item {
            if is_procedural_macro_entry(&function.attrs) {
                return;
            }
        }
        visit::visit_item(self, item);
    }

    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        if is_outer_doc_comment(attribute) {
            self.count += 1;
        }
    }
}
