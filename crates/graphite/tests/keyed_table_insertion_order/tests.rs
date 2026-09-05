use super::*;

fn speaker_id() -> SpeakerId {
    SpeakerId("s".to_string())
}

fn line_id(i: usize) -> LineId {
    LineId(format!("l{i}"))
}

/// builder 経由で同一始点から7本の平行辺を記述順に張り、`of`/`iter` が
/// その順を保持することを確認する。
#[test]
fn choiceの役割探索とiterは挿入順を保持する_builder経由() {
    const N: usize = 7;

    let g = Dialogue::Graph::create(|b| {
        b.speaker(
            speaker_id(),
            Speaker {
                name: "S".to_string(),
            },
        );
        for i in 0..N {
            b.line(
                line_id(i),
                Line {
                    text: format!("line{i}"),
                },
            );
        }
        for i in 0..N {
            b.choice(
                ChoiceId(format!("c{i}")),
                Choice::new(speaker_id(), line_id(i)),
            );
        }
    })
    .expect("制約なし辺種別なので必ず構築に成功する");

    let speaker = g.speaker_by_id(&speaker_id()).unwrap();
    let 役割探索のテキスト: Vec<String> = speaker
        .choice_as_speaker()
        .map(|edge| edge.line().text.clone())
        .collect();
    assert_eq!(役割探索のテキスト, expected_texts(N));

    let iter_ids: Vec<String> = g.choice_iter().map(|edge| edge.id().0.clone()).collect();
    let expected_ids: Vec<String> = (0..N).map(|i| format!("c{i}")).collect();
    assert_eq!(iter_ids, expected_ids);

    let ids_only: Vec<String> = g.choice_ids().map(|id| id.0.clone()).collect();
    assert_eq!(ids_only, expected_ids);

    let between_texts: Vec<String> = speaker
        .choice_between(g.line_by_id(&line_id(3)).unwrap())
        .map(|edge| edge.line().id().0.clone())
        .collect();
    assert_eq!(between_texts, vec!["l3".to_string()]);
}

/// `graph!` リテラル経由でも同じ順序保証が成り立つことを確認する。
#[test]
#[rustfmt::skip]
fn choiceの役割探索は挿入順を保持する_graphリテラル経由() {
    let g = graphite::graph!(Dialogue {
        s  = Speaker { name: "S".into() },
        l0 = Line { text: "line0".into() },
        l1 = Line { text: "line1".into() },
        l2 = Line { text: "line2".into() },
        l3 = Line { text: "line3".into() },
        l4 = Line { text: "line4".into() },
        l5 = Line { text: "line5".into() },

        c0 = Choice(s -> l0),
        c1 = Choice(s -> l1),
        c2 = Choice(s -> l2),
        c3 = Choice(s -> l3),
        c4 = Choice(s -> l4),
        c5 = Choice(s -> l5),
    })
    .expect("制約なし辺種別なので必ず構築に成功する");

    let 役割探索のテキスト: Vec<String> = g.s().choice_as_speaker()
        .map(|edge| edge.line().text.clone())
        .collect();
    assert_eq!(役割探索のテキスト, expected_texts(6));

    let iter_ids: Vec<String> = g.choice_iter().map(|edge| edge.id().0.clone()).collect();
    assert_eq!(
        iter_ids,
        vec!["c0", "c1", "c2", "c3", "c4", "c5"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    );
}
