//! 敵: observer パターン (コールバック購読) で書いたナイーブなリアクティブ
//! セル。
//!
//! [`NaiveCell`] は「値が変わったら購読者へ即座に通知する」だけの、
//! よくある実装 (Excel の再計算・フロントエンドの signal ライブラリの
//! 초期実装・自作の「観察可能な値」でしばしば見る形)。依存関係は
//! 「誰が誰を `subscribe` したか」という**実行時のコールバック登録**
//! としてしか存在せず、静的な全体像 (どのセルがどのセルに依存するか)
//! はどこにも書かれていない。
//!
//! このモジュールは README「敵の紹介」節で説明する3つの問題を実際に
//! 動くコードで再現する:
//!
//! - [`build_diamond_demo`] — (a) グリッチ。ダイヤモンド依存
//!   (`a→b, a→c, b→d, c→d`) で `d` が2回再計算され、1回目は
//!   矛盾した中間状態を観測する。
//! - [`build_infinite_loop_demo`] — (b) 無限ループ。循環購読
//!   (`x→y→x→..`) に誰も気づかず notify が回り続ける (実際に無限に
//!   回すとスタックオーバーフローするため、デモでは `cap` で強制停止
//!   させ「本来なら止まらない」ことを回数で示す)。
//! - [`build_diamond_demo`] (引数 `swap_registration_order`) — (c) 更新
//!   順序が購読登録順に依存して非決定になる。


mod diamond_demo;
mod infinite_loop_demo;
mod naive_cell;

pub use diamond_demo::{build_diamond_demo, DiamondDemo};
pub use infinite_loop_demo::build_infinite_loop_demo;
pub use naive_cell::NaiveCell;
