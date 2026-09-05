//! `chain` サブコマンド: 指定した社員から `Boss` 辺を根まで辿る。

use std::collections::{HashMap, HashSet};

use crate::schema::{EmployeeId, OrgChart};

// 管理チェーン中の 1 エントリ。
#[derive(Debug, Clone, PartialEq)]
pub struct ChainEntry {
    pub depth: usize, // 起点からの深さ (起点自身は0)。
    pub employee: EmployeeId,
    pub name: String,
    pub title: String,
    pub since: Option<i32>, // このエントリの上司との在任年 (起点自身は `None`)。
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChainResult {
    pub entries: Vec<ChainEntry>,
    pub cycle_back_to: Option<EmployeeId>, // 途中で循環を検出して打ち切った場合 `Some(戻り先のキー)`。
}

// 指定した社員から `Boss` 辺を根 (トップ層) まで辿る。
//
// `boss_iter` が返すEdgeRefの役割名getterから両端のNodeRefを取り、
// `EmployeeId -> (EmployeeId, since)` の索引を先に作って辿る。
//
// 訪問済み集合を持ちながら辿ることで循環を検出する。循環に突入したら
// そこで打ち切り、`cycle_back_to` にループの戻り先キーを記録する
// (`anomalies` コマンドの循環検出とは独立した、チェーン単体での安全対策)。
pub fn management_chain(org: &OrgChart::Graph, start: &EmployeeId) -> Option<ChainResult> {
    let start_employee = org.employee_by_id(start)?;

    let boss_of: HashMap<EmployeeId, (EmployeeId, i32)> = org
        .boss_iter()
        .map(|edge| {
            (
                edge.subordinate().id().clone(),
                (edge.superior().id().clone(), edge.payload().since),
            )
        })
        .collect();

    let mut entries = vec![ChainEntry {
        depth: 0,
        employee: start.clone(),
        name: start_employee.name.clone(),
        title: start_employee.title.clone(),
        since: None,
    }];
    let mut visited: HashSet<EmployeeId> = HashSet::new();
    visited.insert(start.clone());

    let mut current = start.clone();
    let mut depth = 1usize;
    let mut cycle_back_to = None;

    loop {
        match boss_of.get(&current) {
            None => break, // トップ層に到達 (これ以上の上司なし)
            Some((boss_id, since)) => {
                if visited.contains(boss_id) {
                    cycle_back_to = Some(boss_id.clone());
                    break;
                }
                let boss_employee = org
                    .employee_by_id(boss_id)
                    .expect("Boss::iterの終点は必ずemployeeに存在するはず");
                entries.push(ChainEntry {
                    depth,
                    employee: boss_id.clone(),
                    name: boss_employee.name.clone(),
                    title: boss_employee.title.clone(),
                    since: Some(*since),
                });
                visited.insert(boss_id.clone());
                current = boss_id.clone();
                depth += 1;
            }
        }
    }

    Some(ChainResult {
        entries,
        cycle_back_to,
    })
}
