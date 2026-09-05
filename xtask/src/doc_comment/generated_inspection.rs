//! 生成コードの検査。非 `#[doc(hidden)]` な公開項目に doc コメントがあることを
//! 確かめる。生成ファイルは利用者のクレートへ include されるため、その公開面は
//! 利用者の hover と rustdoc に出る。

use std::fmt::Write;

use syn::visit::Visit;

use super::public_item_visitor::PublicItemVisitor;
use super::rust_source::{ParsedRustSource, RustSource};

// 生成領域1件の検査結果。
pub(super) struct GeneratedAreaReport {
    spelling: String,
    file_count: usize,
    public_item_count: usize,
    items_without_doc: Vec<String>,
    unreadable_files: Vec<String>,
}

impl GeneratedAreaReport {
    pub(super) fn inspect(spelling: String, sources: Vec<RustSource>) -> Self {
        let file_count = sources.len();
        let mut public_item_count = 0;
        let mut items_without_doc = Vec::new();
        let mut unreadable_files = Vec::new();
        for source in sources {
            match source.parse() {
                ParsedRustSource::Unreadable { spelling, reason } => {
                    unreadable_files.push(format!("{spelling}: {reason}"));
                }
                ParsedRustSource::Parsed { spelling, syntax } => {
                    let mut visitor = PublicItemVisitor::new();
                    visitor.visit_file(&syntax);
                    public_item_count += visitor.public_item_count();
                    for item in visitor.items_without_doc() {
                        items_without_doc.push(format!("{spelling}: {item}"));
                    }
                }
            }
        }
        Self {
            spelling,
            file_count,
            public_item_count,
            items_without_doc,
            unreadable_files,
        }
    }

    pub(super) fn is_clean(&self) -> bool {
        self.items_without_doc.is_empty() && self.unreadable_files.is_empty()
    }

    pub(super) fn render(&self) -> String {
        let mut text = format!(
            "  {} — {}ファイル、公開項目 {}件、doc の欠落 {}件、解析できないファイル {}件\n",
            self.spelling,
            self.file_count,
            self.public_item_count,
            self.items_without_doc.len(),
            self.unreadable_files.len()
        );
        for item in &self.items_without_doc {
            let _ = writeln!(text, "    doc がありません: {item}");
        }
        for failure in &self.unreadable_files {
            let _ = writeln!(text, "    解析できません: {failure}");
        }
        text
    }
}
