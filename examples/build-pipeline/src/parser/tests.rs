//! 正常系1件と、行番号を報告する異常系のテスト。

use super::{parse, EdgeKind, ParsedEdge, ParsedTask};

#[test]
fn 正常なパイプラインをパースできる() {
    let input = "\
# comment
task build_core: cargo build -p core (120s)
build_core produces target/core.rlib

task test_core: cargo test -p core (70s)
test_core consumes target/core.rlib
test_core produces target/test-results/core.xml
";
    let parsed = parse(input).expect("パースに成功するはず");
    assert_eq!(parsed.tasks.len(), 2);
    assert_eq!(parsed.edges.len(), 3);
    assert_eq!(
        parsed.tasks[0],
        ParsedTask {
            name: "build_core".to_string(),
            cmd: "cargo build -p core".to_string(),
            secs: 120,
        }
    );
    assert_eq!(
        parsed.edges[0],
        ParsedEdge {
            task_name: "build_core".to_string(),
            kind: EdgeKind::Produces,
            path: "target/core.rlib".to_string(),
        }
    );
}

#[test]
fn コロンがないtask行はエラーで行番号を報告する() {
    let input = "task build_core cargo build (120s)\n";
    let e = parse(input).unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn 秒数の単位がないとエラーになる() {
    let input = "task build_core: cargo build (120)\n";
    let e = parse(input).unwrap_err();
    assert_eq!(e.line, 1);
    assert!(e.message.contains('s'));
}

#[test]
fn 秒数が数値でないとエラーになる() {
    let input = "task build_core: cargo build (abcs)\n";
    let e = parse(input).unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn produces行のトークン数が不正だとエラーになる() {
    let input = "task t: cmd (1s)\nt produces\n";
    let e = parse(input).unwrap_err();
    assert_eq!(e.line, 2);
}

#[test]
fn 未知のキーワードはエラーになる() {
    let input = "task t: cmd (1s)\nt uses target/x\n";
    let e = parse(input).unwrap_err();
    assert_eq!(e.line, 2);
    assert!(e.message.contains("produces"));
}

#[test]
fn 空行とコメントは無視される() {
    let input = "\n# comment\n\ntask t: cmd (1s)\n\n# another comment\nt produces x\n";
    let parsed = parse(input).unwrap();
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.edges.len(), 1);
}

#[test]
fn task名が空だとエラーになる() {
    let input = "task : cmd (1s)\n";
    let e = parse(input).unwrap_err();
    assert_eq!(e.line, 1);
}
