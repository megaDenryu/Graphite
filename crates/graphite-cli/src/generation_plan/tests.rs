use super::*;
use std::path::PathBuf;

fn target(path: &str) -> GeneratedTargetPath {
    GeneratedTargetPath::new(PathBuf::from(path))
}

fn tree_at(base: &str) -> GenerationTree {
    GenerationTree::new(PathBuf::from(base), Vec::new())
}

#[test]
fn 同じ生成先を2回追加すると重複エラーになる() {
    let tree = tree_at("/repo");
    let mut plan = GenerationPlan::new();
    plan.add(&tree, target("/repo/generated/a.rs"), "one".to_string())
        .unwrap();
    let error = plan
        .add(&tree, target("/repo/generated/a.rs"), "two".to_string())
        .unwrap_err();
    assert!(error.to_string().contains("生成先が重複しています"));
}

#[test]
fn 異なる生成先は両方追加できる() {
    let tree = tree_at("/repo");
    let mut plan = GenerationPlan::new();
    plan.add(&tree, target("/repo/generated/a.rs"), "one".to_string())
        .unwrap();
    plan.add(&tree, target("/repo/generated/b.rs"), "two".to_string())
        .unwrap();
    assert_eq!(plan.expected.len(), 2);
}
