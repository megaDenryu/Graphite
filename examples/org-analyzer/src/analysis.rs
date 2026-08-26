//! 集計 (`summary`)・管理チェーン追跡 (`chain`)・構造異常検出 (`anomalies`)
//! のロジック。
//!
//! CLI からの呼び出しと表示整形 (`report.rs`) を分離し、この module は
//! 「`OrgChart` を読んで構造化データを返す」ことだけに専念する。
//!
//! サブコマンドごとにサブモジュールを分け、このファイルは公開面をまとめる。

mod anomaly;
mod boss_anomaly;
mod management_chain;
mod project_anomaly;
mod span_of_control;
mod summary;

pub use anomaly::{detect_anomalies, AnomalyReport, CrossDepartmentBoss};
pub use management_chain::{management_chain, ChainEntry, ChainResult};
pub use span_of_control::SpanOfControlStats;
pub use summary::{summarize, DeptCount, GradeCount, ProjectAssignmentCount, SummaryReport};
