//! 実行の記録 — サービス1件の起動記録と、1回の実行全体の報告。

use std::time::Duration;

use crate::schema::ServiceId;

/// 1サービスの起動記録 (実測)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRecord {
    pub service: ServiceId,
    /// 1始まりの波番号。
    pub wave: usize,
    /// エンジン開始からの起動開始時刻。
    pub start: Duration,
    /// エンジン開始からの起動完了時刻。
    pub end: Duration,
}

/// `run_waves` の実行結果一式。
#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub waves: Vec<Vec<ServiceId>>,
    pub records: Vec<ExecutionRecord>,
    pub total: Duration,
}

impl ExecutionReport {
    /// `id` の起動記録を引く (`run_waves` に渡した波に含まれるキーなら必ず存在する)。
    pub fn record_of(&self, id: &ServiceId) -> &ExecutionRecord {
        self.records
            .iter()
            .find(|r| &r.service == id)
            .unwrap_or_else(|| panic!("{id:?} の実行記録が見つからない"))
    }
}
