use std::path::PathBuf;

use super::ExcerptInspection;
use crate::document_reference::ReferenceOrigin;
use crate::quoted_excerpt::QuotedExcerpt;
use crate::repository_root::RepositoryRoot;
use crate::source_reference::SourceReference;

fn root() -> RepositoryRoot {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    RepositoryRoot::at(repository_root)
        .expect("xtaskの実行場所からリポジトリルートを解決できること")
}

// `lines[0]` を参照が書かれた行とみなして引用1件を取り込む。
fn excerpt(token: &str, lines: &[&str]) -> Vec<QuotedExcerpt> {
    let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
    let target = SourceReference::parse(token).unwrap();
    vec![QuotedExcerpt::following_fence(lines, 0, origin, target)
        .expect("引用として取り込むこと")]
}

#[test]
fn 参照先が実在する引用は照合した件数と行数に数える() {
    let lines = ["参照", "```rust", "mod quoted_excerpt;", "```"];
    let excerpts = excerpt("xtask/src/lib.rs:9-25", &lines);
    let inspection = ExcerptInspection::over(&excerpts, &root());
    assert_eq!(inspection.compared_excerpt_count(), 1);
    assert_eq!(inspection.compared_line_count(), 1);
    assert_eq!(inspection.unreadable_excerpt_count(), 0);
    assert!(inspection.is_empty());
}

#[test]
fn 参照先が実在しない引用は照合した行数に数えない() {
    let lines = ["参照", "```rust", "mod quoted_excerpt;", "```"];
    let excerpts = excerpt("xtask/src/存在しない.rs:1-5", &lines);
    let inspection = ExcerptInspection::over(&excerpts, &root());
    assert_eq!(inspection.compared_excerpt_count(), 0);
    assert_eq!(inspection.compared_line_count(), 0);
    assert_eq!(inspection.unreadable_excerpt_count(), 1);
    assert!(inspection.is_empty());
}
