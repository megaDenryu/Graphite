//! 計算グラフの凍結時検証 (循環・未宣言依存・キー重複) を検査する。

use std::collections::HashSet;

use graphite::{ComputeGraph, ComputeGraphBuilder, ComputeGraphError, GraphError};

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
