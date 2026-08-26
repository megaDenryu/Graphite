//! `validate` の検出能力を確かめるために、意図的に壊してあるシナリオ2本。
//!
//! 到達不能シーンとデッドエンドを持つシナリオと、どのエンディングにも
//! 到達できない閉じたループだけを持つシナリオを分けて用意する。どちらも
//! `graph_schema!` の検証 (`create`) は通過し、`validate` が構造上の問題
//! として検出する側の題材である。

use crate::schema::{ChoiceEdge, DialogueGraph, Ending, Scene, SceneId};

// - `br_unreachable` は誰からも choice/finale で参照されない (到達不能)。
// - `br_dead` は選択肢もfinaleも持たない (デッドエンド)。
#[rustfmt::skip]
pub fn build_broken_story() -> Result<DialogueGraph::Graph, DialogueGraph::Violation> {
    graphite::graph!(DialogueGraph {
        br_start = Scene {
            speaker: "テスト".to_string(),
            text: "壊れたシナリオの開始".to_string()
        },
        br_ok = Scene {
            speaker: "テスト".to_string(),
            text: "普通に続いてエンディングへ到達する".to_string()
        },
        br_dead = Scene {
            speaker: "テスト".to_string(),
            text: "選択肢もfinaleも無い、行き止まり".to_string()
        },
        br_unreachable = Scene {
            speaker: "テスト".to_string(),
            text: "誰からも参照されない孤立シーン".to_string()
        },
        br_end = Ending {
            title: "テスト終了".to_string(),
            epilogue: "壊れたシナリオのエンディング".to_string()
        },

        c_brstart_brok = Choice(br_start -[ChoiceEdge { label: "進む".to_string() }]-> br_ok),
        c_brstart_brdead = Choice(br_start -[ChoiceEdge { label: "行き止まりへ向かう".to_string() }]-> br_dead),
        f_brok = Finale(br_ok -> br_end),
        // br_dead は意図的に何の辺も出さない (デッドエンド)。
        c_brunreachable_brok = Choice(br_unreachable -[ChoiceEdge { label: "戻る".to_string() }]-> br_ok),
        // br_unreachable は意図的に誰からも参照しない (到達不能)。
    })
    .map(|graph| graph.into_graph())
}

pub fn broken_start_scene_id() -> SceneId {
    SceneId("br_start".to_string())
}

// 「エンディングに到達できない閉じたループ」だけを持つテスト用シナリオ。
// `validate` の trapped_scenes (循環 + どのエンディングにも到達できない)
// 検出だけをピンポイントで確認するための最小フィクスチャ。
#[rustfmt::skip]
pub fn build_pure_loop_story() -> Result<DialogueGraph::Graph, DialogueGraph::Violation> {
    graphite::graph!(DialogueGraph {
        t_start = Scene { speaker: "テスト".to_string(), text: "開始".to_string() },
        t_loop_a = Scene { speaker: "テスト".to_string(), text: "ループA".to_string() },
        t_loop_b = Scene { speaker: "テスト".to_string(), text: "ループB".to_string() },

        c_tstart_tloopa = Choice(t_start -[ChoiceEdge { label: "ループへ".to_string() }]-> t_loop_a),
        c_tloopa_tloopb = Choice(t_loop_a -[ChoiceEdge { label: "Bへ".to_string() }]-> t_loop_b),
        c_tloopb_tloopa = Choice(t_loop_b -[ChoiceEdge { label: "Aへ".to_string() }]-> t_loop_a),
        // どのシーンにも finale が無い = t_loop_a/t_loop_b は循環しつつ
        // どのエンディングにも到達できない「罠」になる。
    })
    .map(|graph| graph.into_graph())
}

pub fn pure_loop_start_scene_id() -> SceneId {
    SceneId("t_start".to_string())
}
