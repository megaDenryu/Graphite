//! ミニスプレッドシートのスキーマ定義。
//!
//! ノード種別は `Cell` の1種のみ。エッジは3種類:
//!
//! - `Feeds = Cell -> Cell where unique pair;` — 可換な演算 (`Mul`/`Sum`)
//!   の被演算子。「`from` の値が `to` の入力になる」という向き (依存元→
//!   依存先) で読む。乗算・合計はどちらの演算子を先に書いても値が変わらない
//!   (可換) ので、被演算子どうしの役割を区別する必要が無い — `docs/modeling_guide.md`
//!   §5 が言う「対称な役割は辺種別を分けなくてよい」場合そのもの。
//! - `Lhs = Cell -> Cell where unique pair;` — 減算 (`Sub`) の**被減数**
//!   (「引かれる方」。`Lhs(a -> d)` は「`a` が `d` の被減数である」)。
//! - `Rhs = Cell -> Cell where unique pair;` — 減算 (`Sub`) の**減数**
//!   (「引く方」)。
//!
//! `Sub` (減算) は非可換 (`a - b != b - a`) なので、Lhs/Rhs という**辺種別
//! そのもの**が被減数/減数の区別を運ぶ — `docs/modeling_guide.md` §5
//! 「同種の辺の間の役割差は辺種別を分ける」の直接適用。旧設計では
//! `Feeds` 1種で全ての依存を張り、被減数/減数の区別は
//! `Formula::Sub(CellId, CellId)` という enum の引数順序**だけ**が持って
//! いた (グラフ構造には無い情報だった)。これが `README.md` 旧「実装の
//! 割り切り」節が指摘していた二重管理: 依存関係そのもの (どのセルがどの
//! セルに依存するか) を `Formula` とグラフの両方に別々に手で書き、両者を
//! 手で一致させ続ける必要があった。
//!
//! v4 (現行) では `Formula` から `CellId` を完全に取り除き、「どの演算か」
//! (`Input`/`Mul`/`Sub`/`Sum`) だけを持つ。演算対象の具体的なセルは
//! [`crate::engine::Engine::eval_formula`] がこのグラフ (`Feeds`/`Lhs`/`Rhs`
//! エッジをそのセルを終点として絞り込む) から直接読み取る。これにより
//! 「どのセルがどのセルに依存するか」という**同一性+接続性を持つ情報**
//! (`docs/modeling_guide.md` §1) はグラフだけが持ち、二重管理は完全に
//! 解消されている。
//!
//! `graph_schema!` はこの `Cell`/`Formula` 型を生成せず参照するだけである。
//! ID型を省略したため、`Sheet` module 内には `CellId`・`FeedsId`・`LhsId`・
//! `RhsId` が生成される (`docs/node_id_v4_2.md`)。

/// 1つのセルが「どう値を求めるか」— **どの演算を適用するか**だけを表す。
///
/// 依存先セル (演算対象) は保持しない。演算対象は `Sheet` の `Feeds`/
/// `Lhs`/`Rhs` エッジがそのセルの識別性 + 接続性として既に持っている情報
/// であり、`Formula` 側に同じ情報を複製すると二重管理になる
/// (`docs/modeling_guide.md` §1「同一性+接続性を持つものだけをグラフの
/// 要素にする」の裏返し: セル間の依存はグラフの要素、演算の種類は
/// `Cell` のフィールド)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formula {
    /// 入力セル。演算を持たず、値は [`crate::engine::Engine::set_input`]
    /// で外部から直接設定される。
    Input,
    /// このセルへ `Feeds` エッジで入ってくる全セルの積 (可換なので何本
    /// あってもよい。このexampleの `default_sheet` では常に2本)。
    Mul,
    /// このセルへ `Lhs`/`Rhs` エッジで入ってくる2セルの差
    /// (`Lhs` の値 − `Rhs` の値)。非可換なので `Lhs`/`Rhs` それぞれ
    /// ちょうど1本 (`crate::engine::Engine::new` が構築時に検査する)。
    Sub,
    /// このセルへ `Feeds` エッジで入ってくる全セルの和 (可換)。
    Sum,
}

/// スプレッドシートのセル。値そのものは持たない — 値は
/// [`crate::engine::Engine`] が別途 `HashMap<CellId, f64>` として持つ
/// (`../Bullet/docs/graph_design_sketches.md` 決定2: グラフは構築後不変。可変な
/// 「今の値」を不変な依存構造から分離する)。
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub formula: Formula,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Sheet {
    include!("generated/schema_sheet.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/schema_sheet.rs";
    schema Sheet {
        node Cell;

        edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair;
        edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair;
        edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair;
    }
}

// 綴り短縮のための再輸出。同名edgeを持つschemaを足したらこの行を消す。
pub use Sheet::{CellId, Feeds, Lhs, Rhs};
