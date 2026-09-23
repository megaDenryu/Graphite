use super::*;
use crate::schema_macro_collector::collect_macro_calls;
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

// 一時ファイルへ書いてから `parse` + `collect_dynamic_into` を通す (parseが
// ファイル読み取りを含むため)。テスト後に一時ファイルを削除する。
fn 一時ファイルで試す(内容: &str, 試す: impl FnOnce(&GenerationTree, &SchemaSourceFile, &str, &[MacroCall])) {
    let dir = std::env::temp_dir();
    let path = dir.join(format!(
        "graphite_schema_source_file_test_{}_{}.rs",
        std::process::id(),
        内容.len()
    ));
    std::fs::write(&path, 内容).unwrap();

    let tree = tree_at(&dir);
    let source = SchemaSourceFile::new(path.clone());
    let (display_path, parsed_file) = source.parse(&tree).unwrap();
    let calls = collect_macro_calls(&parsed_file);
    試す(&tree, &source, &display_path, &calls);

    std::fs::remove_file(&path).unwrap();
}

#[test]
fn 旧名graph_schemaの宣言は改名を促すエラーになる() {
    一時ファイルで試す(
        "graphite::graph_schema! { generated = \"generated/x.rs\"; schema X { node Person; } }",
        |tree, source, display_path, calls| {
            let mut plan = GenerationPlan::new();
            let error = source
                .collect_dynamic_into(tree, display_path, calls, &mut plan)
                .unwrap_err();
            let message = error.to_string();
            assert!(message.contains("graph_schema!"));
            assert!(message.contains("dynamic_graph_schema!"));
            assert!(message.contains("issue #40"));
            // 互換の別名として生成してはならない: 計画に何も積まれていない。
            assert_eq!(plan.declaration_count(), 0);
        },
    );
}

#[test]
fn 解析できないファイルは違反になる() {
    let dir = std::env::temp_dir();
    let path = dir.join(format!(
        "graphite_unparseable_test_{}.rs",
        std::process::id()
    ));
    std::fs::write(&path, "fn broken( {").unwrap();

    let tree = tree_at(&dir);
    let source = SchemaSourceFile::new(path.clone());
    let error = source.parse(&tree).err().unwrap();

    std::fs::remove_file(&path).unwrap();

    assert!(error.to_string().contains("Rustとして解析できません"));
}
