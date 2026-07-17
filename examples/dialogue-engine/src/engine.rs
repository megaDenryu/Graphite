//! プレイロジック。
//!
//! stdin/stdout に直接依存すると自動テストが書けなくなるため、「入力」と
//! 「出力」の両方をクロージャとして抽象化する:
//! - `choose: FnMut(&[String]) -> usize` — 選択肢ラベルの一覧を受け取り、
//!   選ぶインデックスを返す。stdin から読む実装 (CLI) と、あらかじめ決めた
//!   選択列を順に返す実装 (テスト、[`scripted_choices`]) の両方をこの型
//!   1つで表現できる。
//! - `narrate: FnMut(&str)` — 1行相当のテキストを出力する。

use crate::schema::{DialogueGraph, DialogueGraphNode, Finale, Scene, SceneId};

/// 1 プレイの結果。
#[derive(Debug, Clone, PartialEq)]
pub struct PlayOutcome {
    /// 訪れたシーンキーの列 (ループで同じシーンに複数回訪れれば重複して積まれる)。
    pub visited: Vec<SceneId>,
    /// 到達したエンディングのタイトル。デッドエンドに落ちた場合、または
    /// (壊れたシナリオの循環にはまって) [`MAX_STEPS`] を超えた場合は
    /// `None` (本編シナリオでは `validate` がデッドエンド無し・全エンディ
    /// ング到達可能を保証するので起きない想定だが、壊れたシナリオを誤って
    /// プレイした場合の安全弁)。
    pub ending_title: Option<String>,
}

/// `play` が打ち切りと判断するまでの最大シーン遷移数。
///
/// グラフには意図的なループ (合流・往復) があるため、`choose` が常に
/// ループへ戻る選択を返し続けると理論上は無限にプレイが終わらない
/// (例: 選択列を使い切った [`scripted_choices`] のフェイルセーフが
/// たまたま自己ループを選び続けるケース)。CLI やテストが無限ループで
/// ハングするのを防ぐため、一定歩数で強制終了する。
const MAX_STEPS: usize = 10_000;

impl PlayOutcome {
    /// 既訪シーン数 (ループで同じシーンに複数回訪れても 1 回と数える)。
    pub fn unique_scene_count(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        self.visited
            .iter()
            .filter(|id| seen.insert((*id).clone()))
            .count()
    }
}

/// シナリオをプレイする。
///
/// `start` から開始し、各シーンで `narrate` に話者+本文を渡す。finale に
/// 達したらエンディングのタイトル+エピローグを `narrate` し終了する。
/// finale が無いシーンでは選択肢ラベルの一覧を `choose` に渡し、返って
/// きたインデックスの行き先へ進む (範囲外を渡された場合は最後の選択肢に
/// クランプするフェイルセーフ)。
pub fn play(
    schema: &DialogueGraph,
    start: &SceneId,
    mut choose: impl FnMut(&[String]) -> usize,
    mut narrate: impl FnMut(&str),
) -> PlayOutcome {
    let mut current = start.clone();
    let mut visited = Vec::new();

    loop {
        if visited.len() >= MAX_STEPS {
            narrate(&format!(
                "(打ち切り: {MAX_STEPS}ステップ経過してもエンディングに到達しませんでした。シナリオ内のループにはまっている可能性があります)"
            ));
            return PlayOutcome {
                visited,
                ending_title: None,
            };
        }

        visited.push(current.clone());
        let scene = Scene::get(schema, &current)
            .unwrap_or_else(|| panic!("プレイ中に未知のシーンキーに到達しました: {current:?}"));

        narrate(&format!("[{}] {}", scene.speaker, scene.text));

        if let Some(ending) = Finale::of(schema, &current) {
            narrate(&format!("=== {} ===", ending.title));
            narrate(&ending.epilogue);
            return PlayOutcome {
                visited,
                ending_title: Some(ending.title.clone()),
            };
        }

        let options = schema.scene_choices(&current);
        if options.is_empty() {
            // finale も choice も無いデッドエンド (壊れたシナリオを誤って
            // プレイした場合のみ起きうる)。安全側として打ち切る。
            narrate("(この先に選択肢がありません。行き止まりです)");
            return PlayOutcome {
                visited,
                ending_title: None,
            };
        }

        let labels: Vec<String> = options.iter().map(|(_, label)| label.clone()).collect();
        let picked = choose(&labels);
        let picked = picked.min(options.len() - 1);
        current = options[picked].0.clone();
    }
}

/// テスト・自動プレイ用: あらかじめ決めた選択列を順に返す `choose` を作る。
/// 選択列を使い切った後にさらに呼ばれたら 0 番目を選ぶ (フェイルセーフ。
/// テストのシナリオ設計ミスで無限ループするより、決定的に完走する方を選ぶ)。
pub fn scripted_choices(script: Vec<usize>) -> impl FnMut(&[String]) -> usize {
    let mut iter = script.into_iter();
    move |_labels| iter.next().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::build_story;

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
}
