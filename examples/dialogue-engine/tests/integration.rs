//! 統合テスト。`dialogue_engine` ライブラリ (`src/lib.rs` 経由で公開された
//! `schema`/`engine`/`validate`/`report`) をブラックボックスではなく通常の
//! ライブラリ利用者として使い、モジュールをまたいだ挙動を確認する。

use dialogue_engine::schema::{
    build_broken_story, build_story, broken_start_scene_id, start_scene_id, EndingId, SceneId,
};
use dialogue_engine::{engine, report, validate};

#[test]
fn 壊れたシナリオは到達不能シーンを検出する() {
    let broken = build_broken_story().expect("壊れたシナリオ自体の構築は成功するはず");
    let result = validate::validate(&broken, &broken_start_scene_id());

    assert_eq!(
        result.unreachable_scenes,
        vec![SceneId("br_unreachable".to_string())]
    );
}

#[test]
fn 壊れたシナリオはデッドエンドシーンを検出する() {
    let broken = build_broken_story().expect("壊れたシナリオ自体の構築は成功するはず");
    let result = validate::validate(&broken, &broken_start_scene_id());

    assert_eq!(result.dead_end_scenes, vec![SceneId("br_dead".to_string())]);
}

#[test]
fn 本編シナリオは検証クリーンである() {
    let story = build_story().expect("本編シナリオの構築は成功するはず");
    let result = validate::validate(&story, &start_scene_id());

    assert!(result.is_clean(), "{result:?}");
}

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
    assert_eq!(outcome.visited.last(), Some(&SceneId("seal_sacrifice".to_string())));
}

/// `route` が返す経路は「実際にその選択肢ラベル通りに選び続けたら本当に
/// そのエンディングへ到達するか」で検算できる。全4エンディングについて、
/// route の出力ラベル列をそのまま `engine::play` の入力に変換し、
/// 実プレイの結果と突き合わせる。
#[test]
fn route_の示す経路を実際に辿ると同じエンディングに到達する() {
    let story = build_story().expect("本編シナリオの構築は成功するはず");
    let start = start_scene_id();

    let endings = [
        ("ending_evacuate", "生存者、脱出"),
        ("ending_sacrifice", "犠牲による静寂"),
        ("ending_truth", "真実の伝播"),
        ("ending_isolation", "沈黙する基地"),
    ];

    for (ending_key, expected_title) in endings {
        let ending_id = EndingId(ending_key.to_string());
        let route = report::route_to_ending(&story, &start, &ending_id)
            .unwrap_or_else(|| panic!("{ending_key} への経路が見つからない"));

        assert!(route.len() >= 2, "{ending_key}: 経路が短すぎる");
        assert_eq!(route.first().unwrap().0, start);
        assert!(
            route.last().unwrap().1.is_none(),
            "{ending_key}: 最後のシーンはfinaleなのでラベルは無いはず"
        );

        // route の各ステップのラベルを、そのシーンの scene_choices 内での
        // インデックスに変換してスクリプト化する。
        let mut script = Vec::new();
        for (scene_id, label) in &route[..route.len() - 1] {
            let label = label.as_ref().expect("finale 以外は選択肢ラベルを持つはず");
            let options = story.scene_choices(scene_id);
            let idx = options
                .iter()
                .position(|(_, l)| l == label)
                .unwrap_or_else(|| {
                    panic!("{ending_key}: シーン {scene_id:?} に選択肢 `{label}` が見つからない")
                });
            script.push(idx);
        }

        let outcome = engine::play(&story, &start, engine::scripted_choices(script), |_| {});
        assert_eq!(
            outcome.ending_title.as_deref(),
            Some(expected_title),
            "{ending_key}: route通りに選んだのに期待したエンディングに到達しなかった"
        );
    }
}

#[test]
fn statsとmapはシナリオ全体を反映する() {
    let story = build_story().expect("本編シナリオの構築は成功するはず");
    let start = start_scene_id();

    let stats = report::compute_stats(&story, &start);
    assert_eq!(stats.scene_count, 30);
    assert_eq!(stats.ending_count, 4);
    assert_eq!(stats.shortest_routes.len(), 4, "全エンディングが到達可能なはず");

    let mermaid = report::to_mermaid(&story);
    for (ending_key, _) in [
        ("ending_evacuate", ()),
        ("ending_sacrifice", ()),
        ("ending_truth", ()),
        ("ending_isolation", ()),
    ] {
        assert!(
            mermaid.contains(ending_key),
            "mermaid出力に {ending_key} が含まれていない"
        );
    }
}
