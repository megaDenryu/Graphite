//! 統合テスト: `validate` がシナリオ構造の問題を検出することを、ライブラリの
//! 公開APIだけを使って確かめる。

use dialogue_engine::broken_story::{broken_start_scene_id, build_broken_story};
use dialogue_engine::schema::SceneId;
use dialogue_engine::story::{build_story, start_scene_id};
use dialogue_engine::validate;

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
