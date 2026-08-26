//! 計算グラフの呼び出し規約違反 (未知のキー・計算ノードへの代入) を検査する。

use graphite::{ComputeGraph, ComputeGraphBuilder};

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
