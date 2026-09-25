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

// 以下の4つの型は、生成コードが要求するトレイト (`docs/schema_v4.md` §3.1.2) を
// 何も導出しない。ノード値型も積み荷型も、生成コードが要求する固有のトレイトを
// 持たない。`Impression` は無向辺 `Recommended` の積み荷であり、無向の
// 積み荷ありの辺値型にも同じ見張りを掛ける。この検証用パッケージのビルドが、
// 生成コードが利用者の型へトレイトを要求しないことを機械で確かめる。この
// パッケージは導出を足すと保証が消えるため足さない (issue #27, #35)。

// ノード型: 蔵書。
pub struct Book {
    pub title: String,
}

// ノード型: 利用者。
pub struct Reader {
    pub name: String,
}

// `Borrowed` 辺が1本ごとに運ぶ積み荷。
pub struct Loan {
    pub day: u32,
}

// `Recommended` (無向辺) が1本ごとに運ぶ積み荷。
pub struct Impression {
    pub text: String,
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
graphite::dynamic_graph_schema! {
    generated = "generated/library.rs";
    schema Library {
        node Book;
        node Reader;

        edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1;
        edge Recommended = Reader -[note: Impression]- Reader;
    }
}

// 貸出中の蔵書を1件だけ持つグラフを組み立てる。
//
// 生成物を `include!` するだけでは、公開APIが生えていない不整合をこの crate の
// ビルドが見逃す。組み立てから読み出しまで通すことで、生成した型・辺の役割
// アクセサ・多重度検査が外部 crate でも働くことを確かめる。
pub fn 貸出中の蔵書を1件持つ図書グラフを組み立てる() -> Library::Graph {
    let 構築の結果 = graphite::graph!(Library {
        本 = Book { title: "型で守るグラフ".to_string() },
        利用者 = Reader { name: "検証".to_string() },
        感想相手 = Reader { name: "検証2".to_string() },
        貸出 = Borrowed(本 -[Loan { day: 1 }]-> 利用者),
        推薦 = Recommended(利用者 -[Impression { text: "面白い".to_string() }]- 感想相手),
    });
    let Ok(構築済み) = 構築の結果 else {
        panic!("多重度を満たすグラフは構築に成功する")
    };
    構築済み.into_graph()
}

// static_graph_schema! (issue #24、全個体がコンパイル時に確定するグラフ) が外部
// crateでも動くことを確かめる。動的グラフと同じ生成ファイル・指紋照合の
// 方式で公開APIを追跡する (issue #41、`docs/static_graph.md` 参照)。既存の
// Book/Reader 型をそのまま再利用する。

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(
    clippy::needless_lifetimes,
    clippy::wrong_self_convention,
    clippy::clone_on_copy,
    clippy::write_literal
)]
mod ReadingCircle {
    include!("generated/reading_circle.rs");
}

#[rustfmt::skip]
graphite::static_graph_schema! {
    generated = "generated/reading_circle.rs";
    schema ReadingCircle {
        node Book;
        node Reader;
        edge Assigned = (book: Book) -> (reader: Reader) where each book: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(
    clippy::needless_lifetimes,
    clippy::wrong_self_convention,
    clippy::clone_on_copy,
    clippy::write_literal
)]
mod Circle {
    include!("generated/circle.rs");
}

#[rustfmt::skip]
ReadingCircle! {
    generated = "generated/circle.rs";
    graph Circle;
    node 本 = Book { title: "型で守るグラフ".to_string() };
    node 読者 = Reader { name: "検証".to_string() };
    edge 割り当て = Assigned(本 -> 読者);
}

// `static_graph_schema!` で組み立てた読書会グラフの割り当て (どの本を誰が読むか)
// を返す。`dynamic_graph_schema!`/`graph!` と異なり、個体・辺の集合自体がコンパイル
// 時に固定されているため `freeze()` を呼ばない。
pub fn 読書会グラフの割り当てを求める() -> (String, String) {
    let g = Circle::construct!();
    let 割り当て = g.edge_refs().割り当て();
    (割り当て.book().entity().title.clone(), 割り当て.reader().entity().name.clone())
}

#[cfg(test)]
mod tests;
