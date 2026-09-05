//! パース結果を表す型。行形式のテキストを読み終えた時点の姿であり、
//! グラフへの変換 (`builder.rs`) はここでは行わない。

// パース済みタスク 1 件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedTask {
    pub name: String,
    pub cmd: String,
    pub secs: u32,
}

// `produces` / `consumes` の種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Produces,
    Consumes,
}

// パース済みエッジ (タスク → 成果物パス) 1 件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedEdge {
    pub task_name: String,
    pub kind: EdgeKind,
    pub path: String,
}

// パース結果全体。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedPipeline {
    pub tasks: Vec<ParsedTask>,
    pub edges: Vec<ParsedEdge>,
}
