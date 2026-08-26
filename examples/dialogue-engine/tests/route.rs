//! 統合テスト: `route` が返す経路を実際にプレイして検算する。

use dialogue_engine::schema::EndingId;
use dialogue_engine::story::{build_story, start_scene_id};
use dialogue_engine::{engine, route};

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
        let route = route::route_to_ending(&story, &start, &ending_id)
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
