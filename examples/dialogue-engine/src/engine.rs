//! プレイロジック。
//!
//! stdin/stdout に直接依存すると自動テストが書けなくなるため、「入力」と
//! 「出力」の両方をクロージャとして抽象化する:
//! - `choose: FnMut(&[String]) -> usize` — 選択肢ラベルの一覧を受け取り、
//!   選ぶインデックスを返す。stdin から読む実装 (CLI) と、あらかじめ決めた
//!   選択列を順に返す実装 (テスト、[`scripted_choices`]) の両方をこの型
//!   1つで表現できる。
//! - `narrate: FnMut(&str)` — 1行相当のテキストを出力する。

use crate::schema::{DialogueGraph, SceneId};

// 1 プレイの結果。
//
// `ending_title` は到達したエンディングのタイトルである。デッドエンドに落ちた場合、
// または (壊れたシナリオの循環にはまって) `MAX_STEPS` を超えた場合は `None` になる
// (本編シナリオでは `validate` がデッドエンド無し・全エンディング到達可能を保証する
// ので起きない想定だが、壊れたシナリオを誤ってプレイした場合の安全弁)。
#[derive(Debug, Clone, PartialEq)]
pub struct PlayOutcome {
    pub visited: Vec<SceneId>, // 訪れたシーンキーの列 (ループで同じシーンに複数回訪れれば重複して積まれる)。
    pub ending_title: Option<String>,
}

// `play` が打ち切りと判断するまでの最大シーン遷移数。
//
// グラフには意図的なループ (合流・往復) があるため、`choose` が常に
// ループへ戻る選択を返し続けると理論上は無限にプレイが終わらない
// (例: 選択列を使い切った `scripted_choices` のフェイルセーフが
// たまたま自己ループを選び続けるケース)。CLI やテストが無限ループで
// ハングするのを防ぐため、一定歩数で強制終了する。
const MAX_STEPS: usize = 10_000;

impl PlayOutcome {
    // 既訪シーン数 (ループで同じシーンに複数回訪れても 1 回と数える)。
    pub fn unique_scene_count(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        self.visited
            .iter()
            .filter(|id| seen.insert((*id).clone()))
            .count()
    }
}

// シナリオをプレイする。
//
// `start` から開始し、各シーンで `narrate` に話者+本文を渡す。finale に
// 達したらエンディングのタイトル+エピローグを `narrate` し終了する。
// finale が無いシーンでは選択肢ラベルの一覧を `choose` に渡し、返って
// きたインデックスの行き先へ進む (範囲外を渡された場合は最後の選択肢に
// クランプするフェイルセーフ)。
pub fn play(
    schema: &DialogueGraph::Graph,
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
        let scene = schema
            .scene_by_id(&current)
            .unwrap_or_else(|| panic!("プレイ中に未知のシーンキーに到達しました: {current:?}"));

        narrate(&format!("[{}] {}", scene.speaker, scene.text));

        if let Some(finale) = scene.finale_as_scene() {
            let ending = finale.ending();
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

// テスト・自動プレイ用: あらかじめ決めた選択列を順に返す `choose` を作る。
// 選択列を使い切った後にさらに呼ばれたら 0 番目を選ぶ (フェイルセーフ。
// テストのシナリオ設計ミスで無限ループするより、決定的に完走する方を選ぶ)。
pub fn scripted_choices(script: Vec<usize>) -> impl FnMut(&[String]) -> usize {
    let mut iter = script.into_iter();
    move |_labels| iter.next().unwrap_or(0)
}

#[cfg(test)]
mod tests;
