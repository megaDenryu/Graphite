//! ミニスプレッドシートのスキーマ定義。
//!
//! ノード種別は `Cell` の1種のみ。エッジは `Feeds = Cell -> Cell where
//! unique pair;` の1種のみ — 「`from` の値が `to` の入力になる」という
//! 向きで読む (`from` が `to` に値を feed する)。`where unique pair` を
//! 付けている: 「あるセルが別のセルへ値を供給する」という依存関係は
//! 有るか無いかの二値であり、同じ (from, to) の対に2本目の `Feeds`
//! エッジを張ることに意味は無いため (`examples/async-dag` の
//! `DependsOn` と同じ判断)。逆方向 (1つのセルが何個の入力を持つか) は
//! `Feeds::iter` を終点キーで集計すれば分かる (`engine.rs` の依存グラフ
//! 射影で使う)。
//!
//! `graph_schema!` はこの `Cell`/`Formula` 型を生成せず参照するだけ
//! (`docs/schema_v4.md` 参照)。生成されるのはグラフ機械 (`CellId`/
//! `FeedsId` newtype・`Sheet` 構造体・`SheetBuilder`・`SheetViolation`・
//! `Feeds` 固有 impl) だけ。

/// 1つのセルが「どう値を求めるか」を表す式。
///
/// 依存先セルは `CellId` で直接参照する (値ではなくキー)。これにより
/// 「セル `d` は `b`・`c` に依存する」という事実が `Formula` の値そのもの
/// に型として現れる — observer パターンのコールバック購読 (実行時にしか
/// 分からない) との対比が [`crate::antipattern`] の要点。
///
/// `Formula` 自体は依存グラフの構造 (`feeds` エッジ) と重複する情報を
/// 持つ (このexampleでは両方を手で書いて一致させている。実運用では
/// `Formula` から `feeds` エッジを自動導出するのが自然だが、今回は
/// `graph!` リテラルの型付けをそのまま読めることを優先し、あえて手書き
/// のままにしている — `README.md` 「実装の割り切り」節参照)。
#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    /// 入力セル。式を持たず、値は [`crate::engine::Engine::set_input`] で
    /// 外部から直接設定される。
    Input,
    /// 2つのセルの積。
    Mul(CellId, CellId),
    /// 2つのセルの差 (`Sub(a, b)` は `a - b`)。
    Sub(CellId, CellId),
    /// 複数セルの合計。
    Sum(Vec<CellId>),
}

/// スプレッドシートのセル。値そのものは持たない — 値は
/// [`crate::engine::Engine`] が別途 `HashMap<CellId, f64>` として持つ
/// (`docs/graph_design_sketches.md` 決定2: グラフは構築後不変。可変な
/// 「今の値」を不変な依存構造から分離する)。
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub formula: Formula,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema Sheet {
        node Cell;

        edge Feeds = Cell -> Cell where unique pair;
    }
}
