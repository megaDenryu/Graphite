//! 統合テスト: 選択列を与えた自動プレイが、意図したエンディングへ到達する。

use dialogue_engine::engine;
use dialogue_engine::schema::SceneId;
use dialogue_engine::story::{build_story, start_scene_id};

#[test]
fn スクリプト化した選択列で犠牲エンディングに到達する() {
    let story = build_story().expect("本編シナリオの構築は成功するはず");
    let start = start_scene_id();

    // start -> arrival -> airlock -> quarters -> quarters_diary
    //   -> quarters_rooms -> quarters_locked -> quarters_takashi
    //   -> takashi_seal -> seal_sacrifice -> ending_sacrifice
    let script = vec![
        0, // start -> arrival
        0, // arrival -> airlock
        2, // airlock -> quarters
        0, // quarters -> quarters_diary
        0, // quarters_diary -> quarters_rooms
        0, // quarters_rooms -> quarters_locked
        0, // quarters_locked -> quarters_takashi
        0, // quarters_takashi -> takashi_seal
        1, // takashi_seal -> seal_sacrifice
    ];

    let outcome = engine::play(&story, &start, engine::scripted_choices(script), |_| {});

    assert_eq!(outcome.ending_title.as_deref(), Some("犠牲による静寂"));
    assert_eq!(
        outcome.visited.last(),
        Some(&SceneId("seal_sacrifice".to_string()))
    );
}
