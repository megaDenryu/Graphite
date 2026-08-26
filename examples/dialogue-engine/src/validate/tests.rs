//! 4種の構造上の問題を、本編シナリオと壊れたシナリオの両方で確かめる。

use super::*;
use crate::broken_story::{build_broken_story, build_pure_loop_story};
use crate::story::{build_story, start_scene_id};

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
    // 確認する (実体は
    // `broken_story::build_pure_loop_story` に定義してある)。
    let g = build_pure_loop_story()
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
    assert!(!report
        .trapped_scenes
        .contains(&SceneId("t_start".to_string())));
}
