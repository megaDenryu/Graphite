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
//! テストは検証する側ごとに `each_declaration_order/` の2ファイルへ分けてある。
//! `tests/` は統合テストの根のモジュールディレクトリであり、裸の `mod` はこの
//! ディレクトリ直下を探して cargo が別のテスト対象として組み立てるため、
//! モジュールの綴りを `#[path]` で明示する。

#[cfg(test)]
#[path = "each_declaration_order/traversal.rs"]
mod traversal;

#[cfg(test)]
#[path = "each_declaration_order/each_violation.rs"]
mod each_violation;

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

fn 著者(id: &str) -> AuthorId {
    AuthorId(id.to_string())
}

fn 記事(id: &str) -> ArticleId {
    ArticleId(id.to_string())
}
