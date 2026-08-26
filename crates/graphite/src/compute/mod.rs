//! `ComputeGraph<V>` は遅延実行・差分再計算する計算グラフを実行時の値として提供する
//! (`docs/compute_graph.md`)。
//!
//! `flow!` (`docs/flow_macro.md`) が「書いた瞬間に実行される即時実行の脱糖」
//! であるのに対し、`ComputeGraph` は計算グラフを**実行時の値として持ち**、
//! pull 型で遅延評価・差分再計算するランタイムエンジンである
//! (`examples/reactive-cells` の `Engine` の一般化 — あちらは `f64` +
//! `Formula` enum に特化した example、こちらは汎用ライブラリ)。
//!
//! ```
//! use graphite::ComputeGraph;
//!
//! let mut b = ComputeGraph::builder();
//! b.input("price", 100.0);
//! b.input("qty", 3.0);
//! b.computed("subtotal", ["price", "qty"], |args| args[0] * args[1]);
//! b.computed("tax", ["subtotal"], |args| args[0] * 0.1);
//! b.computed("total", ["subtotal", "tax"], |args| args[0] + args[1]);
//! let mut g = b.freeze()?; // 循環は ComputeGraphError::Cycle (循環パスつき) で拒否
//!
//! assert_eq!(*g.get("total"), 330.0); // 遅延: ここで初めて必要分だけ計算
//! g.set_input("qty", 5.0); // 差分: 影響ノードだけ dirty に
//! assert_eq!(*g.get("total"), 550.0); // 再計算は影響分をトポロジカル順に各1回
//! # Ok::<(), graphite::ComputeGraphError>(())
//! ```
//!
//! ## 設計決定
//!
//! - **値型は単一のジェネリック `V`** (`ComputeGraph<V>`)。異種の値はユーザーが
//!   enum で表現する想定であり、`ComputeGraph` 自体は実行時リフレクションを
//!   持ち込まない (`reactive-cells` の `Engine` と同じ整理)。
//! - **関数は動的ディスパッチ** (`node_kind.rs` の `値を求める関数` に理由を書いた)。
//! - **依存は位置引数** (`args[0]` = 依存キー列の 0 番目)。非可換な演算
//!   (減算等) の左右は依存リストの並び順で表現する — `flow!` の fan-in
//!   タプルと同じ規則。`docs/modeling_guide.md` §5 の「役割は名前で」は
//!   **グラフデータ**の規律 (`reactive-cells` の `Lhs`/`Rhs` エッジ種別分離
//!   参照) であり、関数適用の引数そのものは Rust の関数呼び出しと同じ
//!   位置渡しが正道 — グラフ構造の設計判断とクロージャ引数の設計判断は
//!   別の関心事である。
//! - **キーは名前 (`String`)。**
//! - **pull 型の遅延 + 差分。** [`ComputeGraph::set_input`] は値の書き込みと
//!   未再計算印の伝播のみを行い、実際の再計算は一切行わない。[`ComputeGraph::get`]
//!   が「未再計算な祖先だけ」をトポロジカル順に各 1 回再計算する。glitch-free に
//!   なる理由は `recomputation.rs` に書いた。
//!
//! ## 部品の分担
//!
//! - 計算ノード表 (`node_table.rs`) — キーからノード種別・依存キー列・計算を読み出す
//! - 依存構造 (`dependency_structure.rs`) — 凍結時に検証して確定した依存グラフと
//!   トポロジカル位置。既存の [`crate::Graph`] を再利用する範囲もここに書いた
//! - 評価状態 (`evaluation_state.rs`) — 現在値と未再計算集合。可変なのはここだけ
//! - 再計算器 (`recomputation.rs`) — 上の3つを借りて再計算を1回分実行する

mod builder;
mod dependency_structure;
mod error;
mod evaluation_state;
mod node_kind;
mod node_table;
mod recomputation;

use std::collections::HashMap;

pub use builder::ComputeGraphBuilder;
pub use error::ComputeGraphError;

use dependency_structure::依存構造;
use evaluation_state::評価状態;
use node_table::{キーの照会結果, 計算ノード表};
use recomputation::再計算器;

/// 遅延実行・差分再計算する計算グラフ (モジュール doc 参照)。
///
/// 構築後は計算ノード表と依存構造 (どのノードがどのノードに依存するか) が不変で、
/// 可変なのは評価状態 (今の値と、未再計算かどうか) だけ。
pub struct ComputeGraph<V> {
    ノード表: 計算ノード表<V>,
    依存構造: 依存構造,
    評価状態: 評価状態<V>,
}

impl<V> ComputeGraph<V> {
    /// 構築用 builder を作る。
    pub fn builder() -> ComputeGraphBuilder<V> {
        ComputeGraphBuilder::空のbuilderから始める()
    }

    /// 凍結を通った部品から計算グラフを組み立てる、この型の唯一の私有
    /// コンストラクタ。全ての計算ノードは未再計算の状態で始まる — 「遅延:
    /// [`Self::get`] するまで何も計算しない」がこの初期状態そのもの。
    pub(in crate::compute) fn 部品から組み立てる(
        ノード表: 計算ノード表<V>,
        依存構造: 依存構造,
        入力値: HashMap<String, V>,
    ) -> Self {
        let 評価状態 = 評価状態::入力値と未再計算のキー列から生成する(
            入力値,
            ノード表
                .計算ノードのキー列()
                .cloned()
                .collect::<Vec<String>>(),
        );
        Self {
            ノード表,
            依存構造,
            評価状態,
        }
    }

    /// `key` の現在値を返す。未再計算な祖先 (`key` 自身を含む) だけを
    /// トポロジカル順に各1回再計算してから返す (pull 型の遅延評価)。
    ///
    /// # Panics
    /// `key` がこのグラフに存在しないキーの場合 (呼び出し規約違反。
    /// `docs/design_principles.md` 原則2)。
    pub fn get(&mut self, key: &str) -> &V {
        assert!(
            self.ノード表.キーを含むか(key),
            "get: 未知のキーです: {key:?}"
        );
        再計算器::部品を借りて始める(
            &self.ノード表,
            &self.依存構造,
            &mut self.評価状態,
        )
        .必要なら実行する(key);
        self.評価状態.現在値(key)
    }

    /// 入力ノード `key` に新しい値を書き込み、影響を受ける計算ノードを
    /// 未再計算にする (再計算そのものは行わない — 差分は「書き込み + 未再計算印の
    /// 伝播」のみで完結し、実際の再計算は次の [`Self::get`] まで遅延する)。
    ///
    /// # Panics
    /// - `key` がこのグラフに存在しないキーの場合。
    /// - `key` が入力ノードではない (計算ノードである) 場合 — 計算ノードの
    ///   値は依存元ノードの更新から常に自動的に決まるべきであり、直接代入は
    ///   依存構造と値ストアの不整合を招く契約違反 (`docs/design_principles.md`
    ///   原則2、`reactive-cells` の `Engine::set_input` と同じ整理)。
    pub fn set_input(&mut self, key: &str, value: V) {
        match self.ノード表.キーを照会する(key) {
            キーの照会結果::未知のキー => panic!("set_input: 未知のキーです: {key:?}"),
            キーの照会結果::計算ノード => panic!(
                "set_input: {key:?} は計算ノードであり入力ノードではありません。\
                 計算ノードの値は依存元ノードの更新から自動的に決まります。"
            ),
            キーの照会結果::入力ノード => {}
        }

        let 影響先 = self.依存構造.影響を受けるキー列(key);
        self.評価状態
            .入力値を書き込んで影響先を未再計算にする(key, value, 影響先);
    }
}
