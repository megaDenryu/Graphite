use super::{is_package_table_header, nearest_manifest_directory, PackageRoot};
use std::path::PathBuf;

fn this_package() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn パッケージのマニフェストがあればsrcを走査対象にする() {
    let root = PackageRoot::at(this_package()).unwrap();
    let sources = root.generation_tree().schema_source_files().unwrap();
    assert!(!sources.is_empty());
}

#[test]
fn 仮想ワークスペースのマニフェストは拒否する() {
    let workspace_root = this_package().join("..").join("..");
    let Err(error) = PackageRoot::at(workspace_root) else {
        panic!("[package] を持たないマニフェストは拒否されること");
    };
    assert!(error.to_string().contains("[package] がありません"));
}

#[test]
fn 行末コメント付きのpackage見出しを受理する() {
    assert!(is_package_table_header("[package]"));
    assert!(is_package_table_header("[package] # 生成の対象"));
    assert!(is_package_table_header("  [package]\t"));
}

#[test]
fn 先頭行のバイト順マークがあってもpackage見出しとして受理する() {
    assert!(is_package_table_header("\u{feff}[package]"));
    assert!(is_package_table_header("\u{feff}[package] # 説明"));
}

#[test]
fn package以外のテーブル見出しは受理しない() {
    assert!(!is_package_table_header("[package.metadata]"));
    assert!(!is_package_table_header("[workspace]"));
    assert!(!is_package_table_header("[dependencies]"));
    assert!(!is_package_table_header("[package] extra"));
}

#[test]
fn マニフェストの無いディレクトリからは上へ辿る() {
    let source_directory = this_package().join("src");
    assert_eq!(
        nearest_manifest_directory(&source_directory).as_deref(),
        Some(this_package().as_path())
    );
}
