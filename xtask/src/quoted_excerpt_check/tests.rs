use super::ExcerptMismatch;
use crate::document_reference::ReferenceOrigin;
use crate::quoted_excerpt::QuotedExcerpt;
use crate::source_reference::SourceReference;

// 1行目から順に、範囲の指定と対応させて読むための試験用ソース。
const SOURCE: &str = "fn 先頭(
    引数: usize,
) -> bool {
    true
}
struct 五行目より後;
";

// `lines[0]` を参照が書かれた行とみなして引用を取り込み、2つの判定を掛けた
// 結果の表示を返す。違反が無ければ `None`。
fn 引用の違反の表示を求める(token: &str, lines: &[&str]) -> Option<String> {
    let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
    let target = SourceReference::parse(token).unwrap();
    let excerpt = QuotedExcerpt::following_fence(lines, 0, origin, target)
        .expect("引用として取り込むこと");
    let source_lines: Vec<String> = SOURCE.lines().map(str::to_string).collect();
    let mismatch = ExcerptMismatch::judge(&excerpt, &source_lines, SOURCE)?;
    Some(mismatch.to_string())
}

#[test]
fn 範囲内の引用は違反にしない() {
    let lines = ["参照", "", "```rust", "fn 先頭(", "    引数: usize,", "```"];
    assert!(引用の違反の表示を求める("xtask/src/lib.rs:1-5", &lines).is_none());
}

#[test]
fn 範囲の外の行を引用していれば行範囲の判定が違反にする() {
    let lines = ["参照", "```rust", "struct 五行目より後;", "```"];
    let 表示 = 引用の違反の表示を求める("xtask/src/lib.rs:1-5", &lines).expect("違反になること");
    assert!(表示.contains("指定の行範囲にありません"));
    assert!(表示.contains("struct 五行目より後;"));
}

#[test]
fn 複数行へ折った署名を1行へ畳んだ引用を受理する() {
    let lines = ["参照", "```rust", "fn 先頭(引数: usize) -> bool;", "```"];
    assert!(引用の違反の表示を求める("xtask/src/lib.rs:1-5", &lines).is_none());
}

#[test]
fn 省略記号の行は照合しない() {
    let lines = ["参照", "```rust", "// ...", "fn 先頭(", "...", "```"];
    assert!(引用の違反の表示を求める("xtask/src/lib.rs:1-5", &lines).is_none());
}

#[test]
fn 複数範囲はいずれか1つに含まれれば合格になる() {
    let lines = ["参照", "```rust", "fn 先頭(", "struct 五行目より後;", "```"];
    assert!(引用の違反の表示を求める("xtask/src/lib.rs:1-2, 6", &lines).is_none());
}

#[test]
fn 行範囲の判定が照合するのは先頭3行までである() {
    let lines =
        ["参照", "```rust", "fn 先頭(", "    引数: usize,", ") -> bool {", "    true", "```"];
    assert!(引用の違反の表示を求める("xtask/src/lib.rs:1-3", &lines).is_none());
}

#[test]
fn 四行目以降がファイルのどこにも無ければ鮮度の判定が違反にする() {
    let lines = [
        "参照",
        "```rust",
        "fn 先頭(",
        "    引数: usize,",
        ") -> bool {",
        "    ファイルに無い行();",
        "```",
    ];
    let 表示 = 引用の違反の表示を求める("xtask/src/lib.rs:1-5", &lines).expect("違反になること");
    assert!(表示.contains("参照先ファイルのどこにもありません"));
    assert!(表示.contains("ファイルに無い行();"));
}
