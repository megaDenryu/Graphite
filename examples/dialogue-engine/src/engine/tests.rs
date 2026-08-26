//! プレイロジックの振る舞いを、選択列を与えた自動プレイで固定する。

use super::*;
use crate::story::build_story;

#[test]
fn 選択列で真実エンディングに到達できる() {
    let story = build_story().expect("本編シナリオは構築に成功するはず");
    let start = SceneId("start".to_string());

    // start -> arrival -> airlock -> hangar -> central(?) ... 実際には
    // scene_choices の並び順に依存するため、まず選択肢ラベルを目視して
    // 経路を決める (下記アサーションで並び順の前提を明示する)。
    let opts = story.scene_choices(&SceneId("airlock".to_string()));
    assert_eq!(opts.len(), 3, "airlock の選択肢は3本のはず");

    // airlock -> lab -> lab_samples -> lab_computer -> lab_echo
    //   -> lower_hatch -> lower_hall -> control_room -> control_analysis
    //   -> crisis_truth -> truth_sent -> ending_truth
    let script = vec![
        0, // start -> arrival (唯一)
        0, // arrival -> airlock (唯一)
        1, // airlock -> lab
        0, // lab -> lab_samples
        0, // lab_samples -> lab_computer
        0, // lab_computer -> lab_echo
        1, // lab_echo -> lower_hatch
        0, // lower_hatch -> lower_hall
        2, // lower_hall -> control_room
        0, // control_room -> control_analysis
        2, // control_analysis -> crisis_truth
        0, // crisis_truth -> truth_sent (唯一)
    ];

    let outcome = play(&story, &start, scripted_choices(script), |_| {});
    assert_eq!(outcome.ending_title.as_deref(), Some("真実の伝播"));
    assert!(outcome.visited.contains(&SceneId("truth_sent".to_string())));
}

#[test]
fn 既訪シーン数はユニークカウントである() {
    let story = build_story().expect("本編シナリオは構築に成功するはず");
    let start = SceneId("start".to_string());

    // hangar_log で自己ループを2回踏んでから central に戻る経路。
    let script = vec![
        0, // start -> arrival
        0, // arrival -> airlock
        0, // airlock -> hangar
        0, // hangar -> hangar_log
        0, // hangar_log -> hangar_log (ループ1回目)
        0, // hangar_log -> hangar_log (ループ2回目)
        1, // hangar_log -> central
        3, // central -> lower_hatch
        0, // lower_hatch -> lower_hall
        1, // lower_hall -> comms
        0, // comms -> lower_hall (戻る)
        2, // lower_hall -> control_room
        0, // control_room -> control_analysis
        0, // control_analysis -> crisis_evacuate
        0, // crisis_evacuate -> shuttle_bay (唯一)
    ];

    let outcome = play(&story, &start, scripted_choices(script), |_| {});
    assert_eq!(outcome.ending_title.as_deref(), Some("生存者、脱出"));
    // hangar_log を3回訪れているが visited は延べ数、unique は1回分だけ数える。
    let hangar_log_visits = outcome
        .visited
        .iter()
        .filter(|id| id.0 == "hangar_log")
        .count();
    assert_eq!(hangar_log_visits, 3);
    assert!(outcome.unique_scene_count() < outcome.visited.len());
}
