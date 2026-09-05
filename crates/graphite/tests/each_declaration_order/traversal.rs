//! `where each` の記述順が、終点側の探索の形へ現れることの検証。

use super::*;

#[test]
fn 終点側は1本確定_始点側は高々1本として探索できる() {
    let graph = DeclarationOrder::Graph::create(|b| {
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
        b.wrote(
            WroteId("w1".to_string()),
            Wrote::new(著者("a1"), 記事("t1"), Byline { year: 2026 }),
        );
    })
    .unwrap();

    let 記事の参照 = graph.article_by_id(&記事("t1")).unwrap();
    // `each article: 1` なので終点側の探索は `Option` を挟まず辺参照を返す。
    let 辺 = 記事の参照.wrote_as_article();
    assert_eq!(辺.byline().year, 2026);

    let 著者の参照 = graph.author_by_id(&著者("a1")).unwrap();
    // `each writer: 0..1` なので始点側の探索は `Option` を返す。
    let 辺 = 著者の参照.wrote_as_writer().unwrap();
    assert_eq!(辺.article().id(), &記事("t1"));
}
