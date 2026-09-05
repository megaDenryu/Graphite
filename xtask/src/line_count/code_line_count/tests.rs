use super::CodeLineCount;

#[test]
fn 空行とコメントだけの行を数えない() {
    let text = "fn main() {}\n\n// 説明\n/// doc\n//! module\n";
    assert_eq!(CodeLineCount::of_text(text).value(), 1);
}

#[test]
fn 行末のコメントはコード行として数える() {
    assert_eq!(CodeLineCount::of_text("let x = 1; // 説明\n").value(), 1);
}

#[test]
fn ブロックコメントの途中の行を数えない() {
    let text = "/* 開始\n途中\n終わり */\nlet x = 1;\n";
    assert_eq!(CodeLineCount::of_text(text).value(), 1);
}

#[test]
fn ブロックコメントの後ろに残ったコードを数える() {
    assert_eq!(CodeLineCount::of_text("/* 説明 */ let x = 1;\n").value(), 1);
}

#[test]
fn 行の途中で開いたブロックコメントの本文を数えない() {
    let text = format!("let x = 1; /* 説明\n{}*/\n", "コメント本文\n".repeat(99));
    assert_eq!(CodeLineCount::of_text(&text).value(), 1);
}

#[test]
fn 行の途中で閉じたブロックコメントの後ろのコードを数える() {
    let text = "let x = 1; /* 説明\n途中\n*/ let y = 2;\nlet z = 3;\n";
    assert_eq!(CodeLineCount::of_text(text).value(), 3);
}

#[test]
fn 行コメントの中のブロックコメントの開始を開始とみなさない() {
    let text = "let x = 1; // /* 説明\nlet y = 2;\n";
    assert_eq!(CodeLineCount::of_text(text).value(), 2);
}

#[test]
fn 文字列リテラルの中のブロックコメントの開始を開始とみなさない() {
    let text = "let pattern = \"tests/ui/*.rs\";\nlet x = 1;\nlet y = 2;\n";
    assert_eq!(CodeLineCount::of_text(text).value(), 3);
}

#[test]
fn 文字列リテラルの中の行コメントの開始を開始とみなさない() {
    let text = "let url = \"https://example.com\"; let x = 1;\n";
    assert_eq!(CodeLineCount::of_text(text).value(), 1);
}

#[test]
fn 打ち消した引用符を閉じ引用符とみなさない() {
    let text = "let s = \"\\\"/*\"; /* 説明\n本文\n*/ let x = 1;\n";
    assert_eq!(CodeLineCount::of_text(text).value(), 2);
}
