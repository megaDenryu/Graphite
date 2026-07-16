//! ミニスプレッドシートのスキーマ定義。
//!
//! ノード種別は `Cell` の1種のみ。エッジは `feeds: Cell -> Cell (0..*)`
//! の1種のみ — 「`from` の値が `to` の入力になる」という向きで読む
//! (`from` が `to` に値を feed する)。多重度 `(0..*)` は「1つのセルは
//! 0個以上の他セルに値を供給できる」ことを表す (逆方向・つまり「1つの
//! セルが何個の入力を持つか」は `feeds().iter()` を終点キーで集計すれば
//! 分かる — `engine.rs` の依存グラフ射影で使う)。
//!
//! `graph_schema!` はこの `Cell`/`Formula` 型を生成せず参照するだけ
//! (`docs/edge_syntax_v3.md` 参照)。生成されるのはグラフ機械 (`CellId`
//! newtype・`Sheet` 構造体・`SheetBuilder`・`SheetViolation`・
//! `feeds()` ビューアクセサ) だけ。

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

        edge feeds: Cell -> Cell (0..*);
    }
}
