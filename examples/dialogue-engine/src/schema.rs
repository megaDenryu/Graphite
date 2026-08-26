//! スキーマ宣言 (`graph_schema!`) と、そこで参照されるノード型・積み荷型。
//!
//! `graph!` リテラルによるシナリオ本体は `story.rs` (本編) と
//! `broken_story.rs` (検証テスト用) に、完成済みグラフへの導出クエリは
//! `derived_query.rs` にある。v3 (`docs/history/graph_literal_v3.md` §4) で
//! ハンドシェイクマクロを全廃したため、`graph_schema!` と `graph!` を同一
//! ファイルに置く必要は無い (`graph!` が参照するのは通常の型・メソッドだけに
//! なったため、別モジュールから `use` すれば足りる。実証は
//! `crates/graphite/tests/graph_cross_module.rs`)。

// ============================================================
// スキーマ宣言 (`docs/schema_v4.md`)
// ============================================================
//
// node Scene:  1 場面。話者と本文を持つ。
// node Ending: 1 エンディング。タイトルとエピローグ本文を持つ。
// edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene) — 選択肢。制約なし。
// edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1 — エンディングへの
//               到達。各シーンにつき高々1つの結末。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub speaker: String,
    pub text: String,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Ending {
    pub title: String,
    pub epilogue: String,
}

/// `Choice` 辺の積み荷 (選択肢のラベル文字列)。
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceEdge {
    pub label: String,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod DialogueGraph {
    include!("generated/schema_dialogue_graph.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/schema_dialogue_graph.rs";
    schema DialogueGraph {
        node Scene;
        node Ending;

        // Choice に `where unique pair` を付けない理由: 同じ (from, to) の
        // 対に対して、文言 (ラベル) が異なる複数の選択肢が正当にありうる
        // 設計 (例: 別々の経緯で同じシーンへ合流する選択肢が2つあっても
        // おかしくない) ため、平行辺を積極的に許す。
        edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene);
        edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1;
    }
}

// 綴り短縮のための再輸出。同名edgeを持つschemaを足したらこの行を消す。
pub use DialogueGraph::{Choice, EndingId, Finale, SceneId};

impl PartialOrd for SceneId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SceneId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for EndingId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EndingId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
