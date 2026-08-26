//! 統合テスト: パーサの異常系が行番号付きで報告されること。

use build_pipeline::parser;

#[test]
fn パーサ異常系_コロンがない() {
    let e = parser::parse("task foo bar (1s)\n").unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn パーサ異常系_秒数の括弧がない() {
    let e = parser::parse("task foo: cargo build\n").unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn パーサ異常系_秒数の単位sがない() {
    let e = parser::parse("task foo: cargo build (10)\n").unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn パーサ異常系_未知キーワード() {
    let e = parser::parse("task foo: cargo build (1s)\nfoo touches target/x\n").unwrap_err();
    assert_eq!(e.line, 2);
}

#[test]
fn パーサ異常系_行番号は複数行にまたがっても正しい() {
    let input = "\
# comment
task ok: cargo build (1s)
ok produces target/a

task broken cargo test (2s)
";
    let e = parser::parse(input).unwrap_err();
    assert_eq!(e.line, 5);
}
