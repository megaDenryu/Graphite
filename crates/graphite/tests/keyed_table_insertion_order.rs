//! 回帰テスト: `KeyedTable` (`crates/graphite/src/keyed_table.rs`) は挿入順を
//! 保持する仕様であり (`docs/schema_v4.md` §3「順序保証」)、これを土台にした
//! 制約なし辺種別の `{Kind}::of`/`iter` は格納順 (構築時の追加順) を保持する
//! はず、という約束を検証する。
//!
//! 発覚の経緯: dialogue-engine 移行中、同一始点から平行辺が複数ある種別
//! (Choice 相当) の `of()`/`iter()` の順序がプロセスごとに変わり、テストが
//! flaky になった。原因は (1) `KeyedTable` の内部が素の `HashMap` で反復順序
//! が未規定だったこと、(2) freeze 時の `from_index` 構築が builder の挿入順
//! ではなく、出来上がった `KeyedTable` (HashMap) の `iter` 順で行われていた
//! こと、の2点。`KeyedTable` を `Vec<(K, V)>` 本体 + `HashMap<K, usize>` 索引
//! の構造に変えることで両方解消される (from_index の構築源である
//! `#accessor.iter()` が挿入順を返すようになるため)。
//!
//! builder 直接経由・`graph!` リテラル経由の両方で確認する。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpeakerId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Speaker {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub text: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema Dialogue {
        node Speaker;
        node Line;

        // 制約なし (each も unique pair も無し): 同一始点からの平行辺が自由。
        edge Choice = Speaker -> Line;
    }
}

/// 記述順どおりの `line{i}` テキスト列を作る補助関数。
fn expected_texts(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("line{i}")).collect()
}

#[cfg(test)]
mod tests {
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
    fn choiceのofとiterは挿入順を保持する_builder経由() {
        const N: usize = 7;

        let g = Dialogue::create(|b| {
            b.speaker(speaker_id(), Speaker { name: "S".to_string() });
            for i in 0..N {
                b.line(line_id(i), Line { text: format!("line{i}") });
            }
            for i in 0..N {
                b.choice(ChoiceId(format!("c{i}")), Choice(speaker_id(), line_id(i)));
            }
        })
        .expect("制約なし辺種別なので必ず構築に成功する");

        let of_texts: Vec<String> = Choice::of(&g, &speaker_id())
            .into_iter()
            .map(|line: &Line| line.text.clone())
            .collect();
        assert_eq!(of_texts, expected_texts(N));

        let iter_ids: Vec<String> = Choice::iter(&g).map(|(id, _)| id.0.clone()).collect();
        let expected_ids: Vec<String> = (0..N).map(|i| format!("c{i}")).collect();
        assert_eq!(iter_ids, expected_ids);

        let ids_only: Vec<String> = Choice::ids(&g).map(|id| id.0.clone()).collect();
        assert_eq!(ids_only, expected_ids);

        let between_texts: Vec<String> = Choice::between(&g, &speaker_id(), &line_id(3))
            .into_iter()
            .map(|c| c.to().0.clone())
            .collect();
        assert_eq!(between_texts, vec!["l3".to_string()]);
    }

    /// `graph!` リテラル経由でも同じ順序保証が成り立つことを確認する。
    #[test]
    #[rustfmt::skip]
    fn choiceのofは挿入順を保持する_graphリテラル経由() {
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

        let of_texts: Vec<String> = Choice::of(&g, &SpeakerId("s".to_string()))
            .into_iter()
            .map(|line: &Line| line.text.clone())
            .collect();
        assert_eq!(of_texts, expected_texts(6));

        let iter_ids: Vec<String> = Choice::iter(&g).map(|(id, _)| id.0.clone()).collect();
        assert_eq!(
            iter_ids,
            vec!["c0", "c1", "c2", "c3", "c4", "c5"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
    }
}
