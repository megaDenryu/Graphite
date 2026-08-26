//! 統合テスト: 循環依存サンプルが `has_cycle` と `compute_waves` の両方で
//! 拒否され、具体的な循環パスが得られること。

use std::collections::HashSet;

use async_dag::depgraph::{self, build_dependency_graph};
use async_dag::fixtures::cyclic_demo;

#[test]
fn 循環依存サンプルはhas_cycleでtrueになる() {
    let g = cyclic_demo();
    let dep_graph = build_dependency_graph(&g);
    assert!(dep_graph.has_cycle());
}

#[test]
fn 循環依存サンプルはcompute_wavesが具体的な循環パスつきで拒否する() {
    let g = cyclic_demo();
    let err = depgraph::compute_waves(&g).expect_err("循環があるのでErrになるはず");
    assert_eq!(err.cycle.len(), 3, "循環パス: {:?}", err.cycle);
    let cycle_names: HashSet<String> = err.cycle.iter().map(|i| i.0.clone()).collect();
    assert_eq!(
        cycle_names,
        ["a", "b", "c"].iter().map(|s| s.to_string()).collect()
    );
}
