//! `ComputeGraph<V>` — 遅延実行・差分再計算する計算グラフ (`docs/compute_graph.md`)。
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
//! - 計算ノード表 (`node_table.rs`) — キーからノード種別・依存キー列・計算を引く
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GraphError;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::rc::Rc;

    /// `key` の評価回数を数えるカウンタ付きの計算ノードを積む。
    fn computed_counting<D, S>(
        b: &mut ComputeGraphBuilder<f64>,
        key: &str,
        deps: D,
        counter: Rc<RefCell<usize>>,
        f: impl Fn(&[&f64]) -> f64 + 'static,
    ) where
        D: IntoIterator<Item = S>,
        S: Into<String>,
    {
        b.computed(key, deps, move |args| {
            *counter.borrow_mut() += 1;
            f(args)
        });
    }

    #[test]
    fn ダイヤモンド依存でも各ノードちょうど1回だけ再計算される() {
        // price -> a -> c
        // price -> b -> c
        let mut b = ComputeGraph::builder();
        b.input("price", 10.0);

        let count_a = Rc::new(RefCell::new(0));
        let count_b = Rc::new(RefCell::new(0));
        let count_c = Rc::new(RefCell::new(0));

        computed_counting(&mut b, "a", ["price"], count_a.clone(), |args| {
            args[0] * 2.0
        });
        computed_counting(&mut b, "b", ["price"], count_b.clone(), |args| {
            args[0] + 100.0
        });
        computed_counting(&mut b, "c", ["a", "b"], count_c.clone(), |args| {
            args[0] + args[1]
        });

        let mut g = b.freeze().expect("循環が無いので成功するはず");

        assert_eq!(*g.get("c"), 20.0 + 110.0);
        assert_eq!(*count_a.borrow(), 1, "aはちょうど1回だけ再計算されるはず");
        assert_eq!(*count_b.borrow(), 1, "bはちょうど1回だけ再計算されるはず");
        assert_eq!(*count_c.borrow(), 1, "cはちょうど1回だけ再計算されるはず");

        // 差分更新でも各ノードちょうど1回。
        g.set_input("price", 20.0);
        assert_eq!(*g.get("c"), 40.0 + 120.0);
        assert_eq!(*count_a.borrow(), 2);
        assert_eq!(*count_b.borrow(), 2);
        assert_eq!(*count_c.borrow(), 2);
    }

    #[test]
    fn 遅延評価はgetしていない枝を再計算しない() {
        let mut b = ComputeGraph::builder();
        b.input("x", 1.0);

        let count_y = Rc::new(RefCell::new(0));
        let count_z = Rc::new(RefCell::new(0));
        computed_counting(&mut b, "y", ["x"], count_y.clone(), |args| args[0] * 2.0);
        computed_counting(&mut b, "z", ["x"], count_z.clone(), |args| args[0] * 3.0);

        let mut g = b.freeze().unwrap();

        // set_inputだけでは計算が走らない (freeze直後、getを一度も呼んでいない)。
        assert_eq!(*count_y.borrow(), 0);
        assert_eq!(*count_z.borrow(), 0);

        // yだけgetする。zは一度も評価されない。
        assert_eq!(*g.get("y"), 2.0);
        assert_eq!(*count_y.borrow(), 1);
        assert_eq!(*count_z.borrow(), 0, "getしていないzは再計算されないはず");

        // 入力を書き換えてもgetしなければzは動かない。
        g.set_input("x", 5.0);
        assert_eq!(*count_y.borrow(), 1);
        assert_eq!(*count_z.borrow(), 0);
    }

    #[test]
    fn 差分更新は影響外のノードを再計算しない() {
        // a -> b (aはinput)
        // d -> e (dはinput、aとは無関係な別枝)
        let mut b = ComputeGraph::builder();
        b.input("a", 1.0);
        b.input("d", 100.0);

        let count_b = Rc::new(RefCell::new(0));
        let count_e = Rc::new(RefCell::new(0));
        computed_counting(&mut b, "b", ["a"], count_b.clone(), |args| args[0] * 2.0);
        computed_counting(&mut b, "e", ["d"], count_e.clone(), |args| args[0] + 1.0);

        let mut g = b.freeze().unwrap();

        // 両方一度getしてキャッシュ済みの状態を作る。
        assert_eq!(*g.get("b"), 2.0);
        assert_eq!(*g.get("e"), 101.0);
        assert_eq!(*count_b.borrow(), 1);
        assert_eq!(*count_e.borrow(), 1);

        // aだけ変更する -> 影響が及ぶのはbのみ、eは無関係。
        g.set_input("a", 10.0);
        assert_eq!(*g.get("b"), 20.0);
        assert_eq!(*count_b.borrow(), 2, "bは再計算されるはず");

        // eをgetしても再計算されない (dirtyになっていないため)。
        assert_eq!(*g.get("e"), 101.0);
        assert_eq!(
            *count_e.borrow(),
            1,
            "eは影響を受けていないので再計算されないはず"
        );
    }

    #[test]
    fn freezeは循環をパスつきcycleerrorで拒否する() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.computed("a", ["b"], |args| *args[0]);
        b.computed("b", ["a"], |args| *args[0]);

        let err = match b.freeze() {
            Err(err) => err,
            Ok(_) => panic!("循環があるので失敗するはず"),
        };

        match err {
            ComputeGraphError::Cycle(cycle_err) => {
                let members: HashSet<String> = cycle_err.cycle.into_iter().collect();
                assert_eq!(members, HashSet::from(["a".to_string(), "b".to_string()]));
            }
            other => panic!("Cycleエラーになるはずが: {other:?}"),
        }
    }

    #[test]
    fn freezeは未宣言依存をエラーで拒否する() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.input("x", 1.0);
        b.computed("y", ["z"], |args| *args[0]); // "z"は未宣言

        let err = match b.freeze() {
            Err(err) => err,
            Ok(_) => panic!("未宣言依存があるので失敗するはず"),
        };

        match err {
            ComputeGraphError::Graph(GraphError::UnknownEndpoint { missing, .. }) => {
                assert_eq!(missing, "z");
            }
            other => panic!("UnknownEndpointエラーになるはずが: {other:?}"),
        }
    }

    #[test]
    fn freezeはキー重複をエラーで拒否する() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.input("x", 1.0);
        b.input("x", 2.0); // 重複

        let err = match b.freeze() {
            Err(err) => err,
            Ok(_) => panic!("キー重複があるので失敗するはず"),
        };

        assert_eq!(
            err,
            ComputeGraphError::Graph(GraphError::DuplicateKey("x".to_string()))
        );
    }

    #[test]
    #[should_panic(expected = "未知のキーです")]
    fn getは未知のキーでパニックする() {
        let b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        let mut g = b.freeze().unwrap();
        let _ = g.get("no_such_key");
    }

    #[test]
    #[should_panic(expected = "未知のキーです")]
    fn set_inputは未知のキーでパニックする() {
        let b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        let mut g = b.freeze().unwrap();
        g.set_input("no_such_key", 1.0);
    }

    #[test]
    #[should_panic(expected = "計算ノードであり入力ノードではありません")]
    fn set_inputは計算ノードに対してパニックする() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.input("x", 1.0);
        b.computed("y", ["x"], |args| *args[0]);
        let mut g = b.freeze().unwrap();
        g.set_input("y", 999.0);
    }
}
