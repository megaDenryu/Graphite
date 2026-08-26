//! 統合テスト: `stats` と `map` の出力がシナリオ全体を反映する。

use dialogue_engine::story::{build_story, start_scene_id};
use dialogue_engine::{mermaid, stats};

#[test]
fn statsとmapはシナリオ全体を反映する() {
    let story = build_story().expect("本編シナリオの構築は成功するはず");
    let start = start_scene_id();

    let stats = stats::compute_stats(&story, &start);
    assert_eq!(stats.scene_count, 30);
    assert_eq!(stats.ending_count, 4);
    assert_eq!(
        stats.shortest_routes.len(),
        4,
        "全エンディングが到達可能なはず"
    );

    let mermaid = mermaid::to_mermaid(&story);
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
