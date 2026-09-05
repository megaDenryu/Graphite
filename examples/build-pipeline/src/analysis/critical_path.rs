//! クリティカルパス (タスク時間の重み付き最長経路) の計算。

use std::collections::HashMap;

use graphite::CycleError;

use super::task_dependency_graph::task_dependency_graph;
use crate::schema::{BuildPipeline, TaskId};

// クリティカルパス (タスク時間の重み付き最長経路) の計算結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPath {
    pub path: Vec<TaskId>,
    pub total_secs: u32,
    pub total_work_secs: u32,
}

impl CriticalPath {
    // 全体並列度 = 全タスクの所要時間合計 / クリティカルパス長。
    // 1.0 に近いほど並列化の余地が無い (直列に近い) パイプラインであることを
    // 意味し、大きいほど並列実行による短縮効果が大きいことを意味する。
    pub fn parallelism(&self) -> f64 {
        if self.total_secs == 0 {
            0.0
        } else {
            self.total_work_secs as f64 / self.total_secs as f64
        }
    }
}

// クリティカルパスを計算する。
//
// トポロジカル順序に沿って `dist[v] = max(dist[v], dist[u] + secs(v))`
// (`u -> v` の辺ごと) と緩和していく DAG 上の最長経路 DP。`Graph::out_neighbors`
// だけで書けるため、常に「順方向に伝播する」形にしているのがポイント
// (`Graph::in_neighbors` はフェーズ5で追加されたが、この DP は前進伝播で
// 完結するため使っていない)。同種の計算を汎用的に行いたいだけなら
// `graphite::Graph::critical_path_by` (フェーズ5追加) に委譲することも
// できるが、`total_work_secs`/`parallelism` などこのアプリ固有の付随
// データを持つ `CriticalPath` を組み立てる都合上、ここでは専用の DP
// を保持している。
pub fn critical_path(g: &BuildPipeline::Graph) -> Result<CriticalPath, CycleError<TaskId>> {
    let dep_graph = task_dependency_graph(g);
    let order = dep_graph.topological_sort()?;

    let secs_of = |id: &TaskId| -> u32 { g.task_by_id(id).map(|t| t.secs).unwrap_or(0) };

    if order.is_empty() {
        return Ok(CriticalPath {
            path: Vec::new(),
            total_secs: 0,
            total_work_secs: 0,
        });
    }

    let mut dist: HashMap<TaskId, u32> = HashMap::new();
    let mut pred: HashMap<TaskId, TaskId> = HashMap::new();

    for &id in &order {
        dist.entry(id.clone()).or_insert_with(|| secs_of(id));
    }

    for &id in &order {
        let cur = dist[id];
        for succ in dep_graph.out_neighbors(id) {
            let candidate = cur + secs_of(succ);
            if candidate > *dist.get(succ).unwrap_or(&0) {
                dist.insert(succ.clone(), candidate);
                pred.insert(succ.clone(), id.clone());
            }
        }
    }

    let end = order
        .iter()
        .max_by_key(|&&id| dist[id])
        .map(|&id| id.clone())
        .expect("orderは空でないことを上で確認済み");

    let total_secs = dist[&end];
    let mut path = vec![end.clone()];
    let mut cur = end;
    while let Some(p) = pred.get(&cur) {
        path.push(p.clone());
        cur = p.clone();
    }
    path.reverse();

    let total_work_secs: u32 = g.task_ids().map(secs_of).sum();

    Ok(CriticalPath {
        path,
        total_secs,
        total_work_secs,
    })
}

#[cfg(test)]
mod tests;
