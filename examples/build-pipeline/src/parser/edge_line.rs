//! `<タスク名> produces|consumes <パス>` 行のパース。

use super::parse_error::{err, ParseError};
use super::parsed_pipeline::{EdgeKind, ParsedEdge};

// `<タスク名> produces|consumes <パス>` をパースする。
pub(super) fn parse_edge_line(line: &str, line_no: usize) -> Result<ParsedEdge, ParseError> {
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
                format!(
                "2番目のトークンは `produces` か `consumes` である必要があります (実際: {other:?})"
            ),
            ))
        }
    };

    Ok(ParsedEdge {
        task_name: tokens[0].to_string(),
        kind,
        path: tokens[2].to_string(),
    })
}
