//! ドメイン検証・実行計画・クリティカルパス計算。
//!
//! `graph_schema!` が保証する「図式適合」(端点種別・多重度) はあくまで
//! グラフの形が正しいかどうかであり、「誰も produce しない artifact を
//! consume している」「同じ artifact を2つのタスクが produce している」
//! 「タスク依存が循環している」といった *ビルドパイプラインとしての意味の
//! 妥当性* は検査しない。これらはこのモジュールで、`{label}().iter()` /
//! `{node_snake}_ids()` イテレータと汎用 `graphite::Graph<TaskId>` への
//! 射影を使って別レイヤーとして実装する
//! (README「導出エッジ」節が想定する使い分けそのもの)。
//!
//! 検査・実行計画・クリティカルパスはそれぞれ同名のサブモジュールが持ち、
//! 3者が共有するタスク依存グラフの射影は `task_dependency_graph` が持つ。

mod critical_path;
mod domain_issue;
mod plan;
mod task_dependency_graph;
mod validation;

pub use critical_path::{critical_path, CriticalPath};
pub use domain_issue::DomainIssue;
pub use plan::{plan, Wave};
pub use task_dependency_graph::{task_dependency_graph, TaskDependencyGraph};
pub use validation::validate;
