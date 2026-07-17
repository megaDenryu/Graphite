//! シナリオ構造検証。
//!
//! `DialogueGraph::scene_graph()` (choice 辺だけを射影した汎用
//! `graphite::Graph<SceneId, String, SceneId>`) の `reachable_from` /
//! `filter_nodes` / `has_cycle` を使って、以下を検出する:
//!
//! 1. 開始シーンから到達不能なシーン
//! 2. デッドエンド (選択肢が0本かつ finale も無いシーン)
//! 3. どのエンディングにも到達できないシーン群、かつそれが閉じたループ
//!    (グラフに循環がある) になっているケース
//! 4. どの到達可能シーンからも finale されない = 到達不能なエンディング

use std::collections::HashSet;

use crate::schema::{DialogueGraph, DialogueGraphNode, Ending, EndingId, Finale, Scene, SceneId};

/// 検証結果一式。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidationReport {
    /// 開始シーンから到達不能なシーン (キー昇順)。
    pub unreachable_scenes: Vec<SceneId>,
    /// 選択肢もfinaleも無いデッドエンドシーン (キー昇順)。
    pub dead_end_scenes: Vec<SceneId>,
    /// 到達可能などのシーンからも finale されない = 到達不能なエンディング
    /// (キー昇順)。
    pub unreachable_endings: Vec<EndingId>,
    /// どのエンディングにも到達できず、かつ循環 (閉じたループ) を形成して
    /// いるシーン群 (キー昇順)。単なるデッドエンドとは異なり、「無限に
    /// さまよい続けられるが決して終わらない」構造上の罠を指す。
    pub trapped_scenes: Vec<SceneId>,
}

impl ValidationReport {
    /// 4種のいずれの問題も無ければ true。
    pub fn is_clean(&self) -> bool {
        self.unreachable_scenes.is_empty()
            && self.dead_end_scenes.is_empty()
            && self.unreachable_endings.is_empty()
            && self.trapped_scenes.is_empty()
    }
}

/// `schema` を `start` シーンを起点に検証する。
pub fn validate(schema: &DialogueGraph, start: &SceneId) -> ValidationReport {
    let scene_graph = schema.scene_graph();

    // 1. 到達不能シーン: 全シーン - reachable_from(start)
    let reachable: HashSet<SceneId> = scene_graph
        .reachable_from(start)
        .into_iter()
        .cloned()
        .collect();
    let mut unreachable_scenes: Vec<SceneId> = Scene::ids(schema)
        .filter(|id| !reachable.contains(*id))
        .cloned()
        .collect();
    unreachable_scenes.sort();

    // 2. デッドエンド
    let mut dead_end_scenes: Vec<SceneId> = Scene::ids(schema)
        .filter(|id| schema.is_dead_end(id))
        .cloned()
        .collect();
    dead_end_scenes.sort();

    // 3-a. finale を持つシーンの集合 (到達可能性チェックの終点候補)。
    let finale_scene_ids: HashSet<SceneId> = Finale::iter(schema)
        .map(|(_key, edge)| edge.from().clone())
        .collect();

    // 3-b. 「そのシーンから、finale を持つシーンへ到達できるか」を全シーン
    //      について計算する (自分自身が finale シーンなら当然到達できる —
    //      reachable_from は反射的なので finale_scene_ids に自身が含まれて
    //      いれば自動的に true になる)。
    let can_reach_ending: HashSet<SceneId> = Scene::ids(schema)
        .filter(|id| {
            scene_graph
                .reachable_from(id)
                .into_iter()
                .any(|reached| finale_scene_ids.contains(reached))
        })
        .cloned()
        .collect();

    // 4. 到達不能なエンディング: reachable な finale シーンが指す先だけを
    //    「到達可能エンディング」とし、その補集合を報告する。
    let reachable_endings: HashSet<EndingId> = Finale::iter(schema)
        .filter(|(_key, edge)| reachable.contains(edge.from()))
        .map(|(_key, edge)| edge.to().clone())
        .collect();
    let mut unreachable_endings: Vec<EndingId> = Ending::ids(schema)
        .filter(|id| !reachable_endings.contains(*id))
        .cloned()
        .collect();
    unreachable_endings.sort();

    // 5. 閉じたループ: 「エンディングに到達できない」シーンだけを残した
    //    部分グラフ (filter_nodes) を作り、その中で実際に循環に参加して
    //    いるシーンだけを報告する (単に「行き止まりへ向かう片道路」の
    //    途中にあるだけのシーンは、循環の一部ではないので trapped には
    //    含めない — 単独のデッドエンドと区別するため)。
    let stuck_graph = scene_graph.filter_nodes(|id| !can_reach_ending.contains(id));
    let mut trapped_scenes: Vec<SceneId> = stuck_graph
        .keys()
        .filter(|id| scene_is_in_cycle(&stuck_graph, id))
        .cloned()
        .collect();
    trapped_scenes.sort();

    ValidationReport {
        unreachable_scenes,
        dead_end_scenes,
        unreachable_endings,
        trapped_scenes,
    }
}

/// `id` が `graph` 上で (長さ1以上の) 循環に参加しているか。
/// 「`id` の隣接先のどれかから `id` 自身へ戻れるか」で判定する
/// (自己ループも `out_neighbors` に `id` 自身が含まれるため自然に拾える)。
fn scene_is_in_cycle(graph: &graphite::Graph<SceneId, String, SceneId>, id: &SceneId) -> bool {
    graph
        .out_neighbors(id)
        .into_iter()
        .any(|next| graph.reachable_from(next).into_iter().any(|k| k == id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{build_broken_story, build_story, start_scene_id};

    #[test]
    fn 本編シナリオは検証クリーンである() {
        let story = build_story().expect("本編シナリオは構築に成功するはず");
        let report = validate(&story, &start_scene_id());

        assert!(
            report.is_clean(),
            "本編シナリオは検証クリーンのはずだが: {report:?}"
        );
    }

    #[test]
    fn 壊れたシナリオは到達不能とデッドエンドを検出する() {
        let broken = build_broken_story().expect("壊れたシナリオ自体は構築に成功するはず");
        let report = validate(&broken, &SceneId("br_start".to_string()));

        assert!(!report.is_clean());
        assert_eq!(
            report.unreachable_scenes,
            vec![SceneId("br_unreachable".to_string())]
        );
        assert_eq!(report.dead_end_scenes, vec![SceneId("br_dead".to_string())]);
        // br_dead は単独の行き止まりで循環していないので trapped には出ない。
        assert!(report.trapped_scenes.is_empty());
        // br_end は br_ok から到達可能なので unreachable_endings は空。
        assert!(report.unreachable_endings.is_empty());
    }

    #[test]
    fn 全シーンがエンディングに到達できないループはtrappedとして検出される() {
        // t_start -> t_loop_a -> t_loop_b -> t_loop_a (どのエンディングにも
        // 繋がらない孤立した循環) を持つ最小フィクスチャで trapped_scenes を
        // 確認する (`graph!` はスキーマと同じファイルでしか呼べないため、
        // 実体は `schema::build_pure_loop_story` に定義してある)。
        let g = crate::schema::build_pure_loop_story()
            .expect("循環のみのテストシナリオは構築に成功するはず (エンディング0個も許容される)");

        let report = validate(&g, &SceneId("t_start".to_string()));
        assert!(!report.is_clean());

        let mut trapped = report.trapped_scenes.clone();
        trapped.sort();
        assert_eq!(
            trapped,
            vec![
                SceneId("t_loop_a".to_string()),
                SceneId("t_loop_b".to_string())
            ]
        );
        // t_start 自体は循環に含まれない (循環に入るだけの片道シーン)。
        assert!(!report.trapped_scenes.contains(&SceneId("t_start".to_string())));
    }
}
