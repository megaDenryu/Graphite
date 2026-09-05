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
//!
//! 行の種別判定だけをこのファイルが持ち、行1本の解釈は `task_line`/
//! `edge_line` が、結果の型と誤りの型は `parsed_pipeline`/`parse_error` が持つ。

mod edge_line;
mod parse_error;
mod parsed_pipeline;
mod task_line;

#[cfg(test)]
mod tests;

use edge_line::parse_edge_line;
use parse_error::err;
use task_line::parse_task_line;

pub use parse_error::ParseError;
pub use parsed_pipeline::{EdgeKind, ParsedEdge, ParsedPipeline, ParsedTask};

// `pipeline.txt` の内容全体をパースする。
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
