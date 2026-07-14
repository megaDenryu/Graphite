//! `pipeline.txt` 用の簡易行形式パーサ。
//!
//! 文法 (詳細は `pipeline.txt` 冒頭のコメントも参照):
//! ```text
//! task <名前>: <コマンド...> (<秒数>s)
//! <タスク名> produces <パス>
//! <タスク名> consumes <パス>
//! ```
//! `#` 始まりの行・空行は無視する。エラーは行番号付きで報告し、
//! どのパイプライン定義行が壊れているかをユーザーが即座に特定できるように
//! する。

use std::fmt;

/// パース済みタスク 1 件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedTask {
    pub name: String,
    pub cmd: String,
    pub secs: u32,
}

/// `produces` / `consumes` の種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Produces,
    Consumes,
}

/// パース済みエッジ (タスク → 成果物パス) 1 件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedEdge {
    pub task_name: String,
    pub kind: EdgeKind,
    pub path: String,
}

/// パース結果全体。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedPipeline {
    pub tasks: Vec<ParsedTask>,
    pub edges: Vec<ParsedEdge>,
}

/// 行番号付きパースエラー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}行目: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

fn err(line: usize, message: impl Into<String>) -> ParseError {
    ParseError {
        line,
        message: message.into(),
    }
}

/// `pipeline.txt` の内容全体をパースする。
pub fn parse(input: &str) -> Result<ParsedPipeline, ParseError> {
    let mut pipeline = ParsedPipeline::default();

    for (i, raw_line) in input.lines().enumerate() {
        let line_no = i + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix("task ") {
            pipeline.tasks.push(parse_task_line(rest, line_no)?);
        } else if line.starts_with("task") {
            // "task" にはマッチしたが直後に半角スペースがない (例: "task:foo") 。
            return Err(err(
                line_no,
                "task 行は `task <名前>: <コマンド...> (<秒数>s)` の形式である必要があります",
            ));
        } else {
            pipeline.edges.push(parse_edge_line(line, line_no)?);
        }
    }

    Ok(pipeline)
}

/// `<名前>: <コマンド...> (<秒数>s)` (先頭の `task ` は既に剥がされている) をパースする。
fn parse_task_line(rest: &str, line_no: usize) -> Result<ParsedTask, ParseError> {
    let (name_part, remainder) = rest.split_once(':').ok_or_else(|| {
        err(
            line_no,
            "task 行に ':' が見つかりません (`task <名前>: <コマンド...> (<秒数>s)`)",
        )
    })?;

    let name = name_part.trim();
    if name.is_empty() {
        return Err(err(line_no, "task 名が空です"));
    }
    if name.contains(char::is_whitespace) {
        return Err(err(
            line_no,
            format!("task 名に空白は使えません: {name:?}"),
        ));
    }

    let remainder = remainder.trim();
    let open = remainder.rfind('(').ok_or_else(|| {
        err(
            line_no,
            "末尾に想定実行時間 `(<秒数>s)` が見つかりません",
        )
    })?;
    if !remainder.ends_with(')') {
        return Err(err(
            line_no,
            "想定実行時間の括弧が閉じていません (`(<秒数>s)` の形式で末尾に置くこと)",
        ));
    }

    let cmd = remainder[..open].trim();
    if cmd.is_empty() {
        return Err(err(line_no, "コマンドが空です"));
    }

    let secs_part = &remainder[open + 1..remainder.len() - 1];
    let secs_digits = secs_part.strip_suffix('s').ok_or_else(|| {
        err(
            line_no,
            format!("実行時間は `<数値>s` の形式である必要があります (実際: {secs_part:?})"),
        )
    })?;
    let secs: u32 = secs_digits.parse().map_err(|_| {
        err(
            line_no,
            format!("実行時間の数値部分が解釈できません: {secs_digits:?}"),
        )
    })?;

    Ok(ParsedTask {
        name: name.to_string(),
        cmd: cmd.to_string(),
        secs,
    })
}

/// `<タスク名> produces|consumes <パス>` をパースする。
fn parse_edge_line(line: &str, line_no: usize) -> Result<ParsedEdge, ParseError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(err(
            line_no,
            format!(
                "produces/consumes 行は `<タスク名> produces|consumes <パス>` の3トークン形式である必要があります (実際は{}トークン)",
                tokens.len()
            ),
        ));
    }

    let kind = match tokens[1] {
        "produces" => EdgeKind::Produces,
        "consumes" => EdgeKind::Consumes,
        other => {
            return Err(err(
                line_no,
                format!("2番目のトークンは `produces` か `consumes` である必要があります (実際: {other:?})"),
            ))
        }
    };

    Ok(ParsedEdge {
        task_name: tokens[0].to_string(),
        kind,
        path: tokens[2].to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
