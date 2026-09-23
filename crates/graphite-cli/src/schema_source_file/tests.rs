use super::*;
use std::path::Path;

fn tree_at(path: &Path) -> GenerationTree {
    GenerationTree::new(path.to_path_buf(), Vec::new())
}

#[test]
fn generatedディレクトリ配下の相対パスを受理する() {
    let tree = tree_at(Path::new("/repo"));
    let source = SchemaSourceFile::new(PathBuf::from("/repo/crates/graphite/tests/x.rs"));
    let target = source
        .generated_target(&tree, "generated/world.rs")
        .unwrap();
    assert_eq!(
        target.as_path(),
        Path::new("/repo/crates/graphite/tests/generated/world.rs")
    );
}

#[test]
fn 絶対パスを拒否する() {
    let tree = tree_at(Path::new("/repo"));
    let source = SchemaSourceFile::new(PathBuf::from("/repo/crates/graphite/tests/x.rs"));
    assert!(source.generated_target(&tree, "/etc/evil.rs").is_err());
}

#[test]
fn 上位ディレクトリへの脱出を拒否する() {
    let tree = tree_at(Path::new("/repo"));
    let source = SchemaSourceFile::new(PathBuf::from("/repo/crates/graphite/tests/x.rs"));
    assert!(source
        .generated_target(&tree, "generated/../../evil.rs")
        .is_err());
}

#[test]
fn 拡張子がrs以外なら拒否する() {
    let tree = tree_at(Path::new("/repo"));
    let source = SchemaSourceFile::new(PathBuf::from("/repo/crates/graphite/tests/x.rs"));
    assert!(source
        .generated_target(&tree, "generated/world.txt")
        .is_err());
}

#[test]
fn 旧名graph_schemaの宣言は改名を促すエラーになる() {
    let dir = std::env::temp_dir();
    let path = dir.join(format!(
        "graphite_legacy_name_test_{}.rs",
        std::process::id()
    ));
    std::fs::write(
        &path,
        "graphite::graph_schema! { generated = \"generated/x.rs\"; schema X { node Person; } }",
    )
    .unwrap();

    let tree = tree_at(&dir);
    let source = SchemaSourceFile::new(path.clone());
    let mut plan = GenerationPlan::new();
    let error = source.collect_into(&tree, &mut plan).unwrap_err();

    std::fs::remove_file(&path).unwrap();

    let message = error.to_string();
    assert!(message.contains("graph_schema!"));
    assert!(message.contains("dynamic_graph_schema!"));
    assert!(message.contains("issue #40"));
    // 互換の別名として生成してはならない: 計画に何も積まれていない。
    assert_eq!(plan.declaration_count(), 0);
}
