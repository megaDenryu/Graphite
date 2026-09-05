use super::QuotedExcerpt;
use crate::document_reference::ReferenceOrigin;
use crate::source_reference::SourceReference;

// `lines[0]` を参照が書かれた行とみなして引用を取り込む。
fn excerpt(token: &str, lines: &[&str]) -> Option<QuotedExcerpt> {
    let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
    QuotedExcerpt::following_fence(lines, 0, origin, SourceReference::parse(token).unwrap())
}

#[test]
fn 行番号のない参照は照合の対象にならない() {
    let lines = ["参照", "```rust", "fn 先頭(", "```"];
    assert!(excerpt("xtask/src/lib.rs", &lines).is_none());
}

#[test]
fn 直後にコードフェンスが無ければ照合の対象にならない() {
    let lines = ["参照", "続く散文", "```rust", "fn 先頭(", "```"];
    assert!(excerpt("xtask/src/lib.rs:1-5", &lines).is_none());
}

#[test]
fn 正規化すると空になる行だけのフェンスは引用として取り込まない() {
    let lines = ["参照", "```rust", "{", ",", "```"];
    assert!(excerpt("xtask/src/lib.rs:1-5", &lines).is_none());
}

#[test]
fn 情報文字列がrustでないフェンスは引用として取り込まない() {
    let lines = ["参照", "```text", "fn 先頭(", "```"];
    assert!(excerpt("xtask/src/lib.rs:1-5", &lines).is_none());
}

#[test]
fn 空行と省略記号の行は照合の対象にしない() {
    let lines = ["参照", "", "```rust", "// ...", "", "fn 先頭(", "...", "```"];
    let excerpt = excerpt("xtask/src/lib.rs:1-5", &lines).expect("引用として取り込むこと");
    assert_eq!(excerpt.line_count(), 1);
}
