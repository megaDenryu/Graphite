//! 計算グラフの遅延評価と差分再計算 (どのノードが何回評価されるか) を検査する。

use std::cell::RefCell;
use std::rc::Rc;

use graphite::{ComputeGraph, ComputeGraphBuilder};

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
