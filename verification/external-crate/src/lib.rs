//! Graphite のワークスペースの外にある crate から schema を使えることを確かめる
//! 検証用パッケージ。
//!
//! このパッケージはルートの `Cargo.toml` の `[workspace] exclude` に入っており、
//! ワークスペースの `cargo build` も `cargo test --workspace` もここを見ない。
//! 外部の利用者と同じ条件、つまり `cargo xtask` が無く、走査開始点がパッケージ
//! 直下の `src` である条件を再現するためである。
//!
//! 検査は `cargo xtask check-external` が行う。生成し直すときは、このディレクトリで
//! `cargo graphite generate` を実行する。

/// ノード型: 蔵書。
#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub title: String,
}

/// ノード型: 利用者。
#[derive(Debug, Clone, PartialEq)]
pub struct Reader {
    pub name: String,
}

/// `Borrowed` 辺が1本ごとに運ぶ積み荷。
#[derive(Debug, Clone, PartialEq)]
pub struct Loan {
    pub day: u32,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(
    clippy::needless_lifetimes,
    clippy::wrong_self_convention,
    clippy::clone_on_copy,
    clippy::write_literal
)]
pub mod Library {
    include!("generated/library.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/library.rs";
    schema Library {
        node Book;
        node Reader;

        edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1;
    }
}

/// 貸出中の蔵書を1件だけ持つグラフを組み立てる。
///
/// 生成物を `include!` するだけでは、公開APIが生えていない不整合をこの crate の
/// ビルドが見逃す。組み立てから読み出しまで通すことで、生成した型・辺の役割
/// アクセサ・多重度検査が外部 crate でも働くことを確かめる。
pub fn 貸出中の蔵書を1件持つ図書グラフを組み立てる() -> Library::Graph {
    graphite::graph!(Library {
        本 = Book { title: "型で守るグラフ".to_string() },
        利用者 = Reader { name: "検証".to_string() },
        貸出 = Borrowed(本 -[Loan { day: 1 }]-> 利用者),
    })
    .expect("多重度を満たすグラフは構築に成功する")
    .into_graph()
}

// static_schema! (issue #24、全個体がコンパイル時に確定するグラフ) が外部
// crateでも動くことを確かめる。graph_schema!/graph! と違いインライン展開の
// まま完結し、ファイル生成トラッキングに参加しないため (docs/static_graph.md
// 参照)、include! も generated/ への出力も不要。既存の Book/Reader 型を
// そのまま再利用する。

#[rustfmt::skip]
graphite::static_schema! {
    schema ReadingCircle {
        node Book;
        node Reader;
        edge Assigned = (book: Book) -> (reader: Reader) where each book: 1;
    }
}

#[rustfmt::skip]
ReadingCircle! {
    graph Circle;
    node 本 = Book { title: "型で守るグラフ".to_string() };
    node 読者 = Reader { name: "検証".to_string() };
    edge 割り当て = Assigned(本 -> 読者);
}

/// `static_schema!` で組み立てた読書会グラフの割り当て (どの本を誰が読むか)
/// を返す。`graph_schema!`/`graph!` と異なり、個体・辺の集合自体がコンパイル
/// 時に固定されているため `freeze()` を呼ばない。
pub fn 読書会グラフの割り当てを求める() -> (String, String) {
    let nodes = Nodes::new();
    let edges = Edges::new(&nodes);
    let g = Circle::new(&nodes, &edges);
    let 割り当て = g.edge_refs.割り当て;
    (割り当て.book().entity().title.clone(), 割り当て.reader().entity().name.clone())
}

#[cfg(test)]
mod tests {
    use super::{貸出中の蔵書を1件持つ図書グラフを組み立てる, 読書会グラフの割り当てを求める};

    #[test]
    fn 外部crateから生成した公開apiを呼べる() {
        let graph = 貸出中の蔵書を1件持つ図書グラフを組み立てる();
        assert_eq!(graph.book_len(), 1);
        assert_eq!(graph.borrowed_len(), 1);
        let 貸出 = graph.borrowed_iter().next().expect("辺が1本ある");
        assert_eq!(貸出.loan().day, 1);
        assert_eq!(貸出.reader().name, "検証");
    }

    #[test]
    fn 外部crateからstatic_schemaの公開apiを呼べる() {
        assert_eq!(読書会グラフの割り当てを求める(), ("型で守るグラフ".to_string(), "検証".to_string()));
    }
}
