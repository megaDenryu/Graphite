//! `task <名前>: <コマンド...> (<秒数>s)` 行のパース。

use super::parse_error::{err, ParseError};
use super::parsed_pipeline::ParsedTask;

/// `<名前>: <コマンド...> (<秒数>s)` (先頭の `task ` は既に剥がされている) をパースする。
pub(super) fn parse_task_line(rest: &str, line_no: usize) -> Result<ParsedTask, ParseError> {
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
        return Err(err(line_no, format!("task 名に空白は使えません: {name:?}")));
    }

    let remainder = remainder.trim();
    let open = remainder
        .rfind('(')
        .ok_or_else(|| err(line_no, "末尾に想定実行時間 `(<秒数>s)` が見つかりません"))?;
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
