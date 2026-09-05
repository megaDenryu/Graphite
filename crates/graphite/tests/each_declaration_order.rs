//! `where each` を「終点役割 → 始点役割」の順に書いた schema の非回帰テスト。
//!
//! 多重度違反の variant は側 (出次数/入次数) ではなく `where` 節の**記述順**に
//! 並ぶ。他のテスト用 schema はいずれも始点役割の制約しか書かないか、始点側を
//! 先に書いているため、記述順と側順のどちらで並べても同じ生成物になってしまい、
//! 並べ替えの誤りを検出できない。この schema は終点役割の制約を先に書くことで
//! 両者を区別する。
//!
//! 生成物 (`generated/each_declaration_order_declaration_order.rs`) が正本であり、
//! `Violation` の variant は `記事ごとの著者` (終点側) が `著者ごとの記事` (始点側)
//! より先に並ぶ。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。検証
//! 対象1つ (`where each` の記述順) に対するテスト関数の列である。本体は29行であ
//! る。超過を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Author {
    pub name: String,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub title: String,
}

/// 積み荷型。
#[derive(Debug, Clone, PartialEq)]
pub struct Byline {
    pub year: i32,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod DeclarationOrder {
    include!("generated/each_declaration_order_declaration_order.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/each_declaration_order_declaration_order.rs";
    schema DeclarationOrder {
        node Author;
        node Article;

        edge Wrote = (writer: Author) -[byline: Byline]-> (article: Article)
            where each article: 1, each writer: 0..1;
    }
}

use DeclarationOrder::{ArticleId, AuthorId, Violation, Wrote, WroteId};

#[cfg(test)]
mod tests {
    use super::*;

    fn 著者(id: &str) -> AuthorId {
        AuthorId(id.to_string())
    }

    fn 記事(id: &str) -> ArticleId {
        ArticleId(id.to_string())
    }

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
}
