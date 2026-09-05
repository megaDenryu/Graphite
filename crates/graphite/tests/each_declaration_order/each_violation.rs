//! `where each` の記述順が、両側の多重度違反の報告へ現れることの検証。

use super::*;

#[test]
fn 両側の多重度違反をそれぞれ報告する() {
    // 記事 t1 には辺が無く (入次数0 なので `each article: 1` に反する)、
    // 著者 a1 は2本書いている (出次数2 なので `each writer: 0..1` に反する)。
    let 結果 = DeclarationOrder::Graph::create_collecting(|b| {
        b.author(
            著者("a1"),
            Author {
                name: "a1".to_string(),
            },
        );
        b.article(
            記事("t1"),
            Article {
                title: "t1".to_string(),
            },
        );
        b.article(
            記事("t2"),
            Article {
                title: "t2".to_string(),
            },
        );
        b.article(
            記事("t3"),
            Article {
                title: "t3".to_string(),
            },
        );
        b.wrote(
            WroteId("w1".to_string()),
            Wrote::new(著者("a1"), 記事("t2"), Byline { year: 2026 }),
        );
        b.wrote(
            WroteId("w2".to_string()),
            Wrote::new(著者("a1"), 記事("t3"), Byline { year: 2026 }),
        );
    });
    let Err(違反) = 結果 else {
        panic!("多重度違反があるため凍結は失敗するはず");
    };

    let 記事側 = 違反
        .iter()
        .filter(|v| matches!(v, Violation::WroteArticleEachViolation { .. }))
        .count();
    let 著者側 = 違反
        .iter()
        .filter(|v| matches!(v, Violation::WroteWriterEachViolation { .. }))
        .count();
    assert_eq!(記事側, 1, "入次数0 の記事 t1 が1件報告される");
    assert_eq!(著者側, 1, "出次数2 の著者 a1 が1件報告される");

    let 記事側の文言 = 違反
        .iter()
        .find(|v| matches!(v, Violation::WroteArticleEachViolation { .. }))
        .unwrap()
        .to_string();
    assert!(
        記事側の文言.contains("入次数 ちょうど1 を期待しますが実際は 0 本です"),
        "実際の文言: {記事側の文言}"
    );
    let 著者側の文言 = 違反
        .iter()
        .find(|v| matches!(v, Violation::WroteWriterEachViolation { .. }))
        .unwrap()
        .to_string();
    assert!(
        著者側の文言.contains("出次数 0..1 を期待しますが実際は 2 本です"),
        "実際の文言: {著者側の文言}"
    );
}
