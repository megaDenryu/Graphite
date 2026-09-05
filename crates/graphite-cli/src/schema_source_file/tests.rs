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
