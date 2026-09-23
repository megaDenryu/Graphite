use std::error::Error;
use std::fs;
use std::path::PathBuf;

use super::PackageManifestFacts;

// 実在するパッケージを固定具にする。書き込みが要らず、リポジトリの実際の
// Cargo.toml が導出規則の入力であることも同時に確かめられる。
fn 対象マニフェスト(relative_directory: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(relative_directory)
        .join("Cargo.toml")
}

// この関数は、一時ディレクトリへ `manifest_body` を Cargo.toml として書き、
// 読み取り結果を返す。この関数は、読み取り後に一時ディレクトリを削除する。
fn 読み取る(suffix: &str, manifest_body: &str) -> Result<PackageManifestFacts, Box<dyn Error>> {
    let directory = std::env::temp_dir().join(format!(
        "graphite-xtask-package-manifest-test-{}-{suffix}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).expect("一時ディレクトリを作成できること");
    fs::write(directory.join("Cargo.toml"), manifest_body).expect("Cargo.tomlを書けること");
    let result = PackageManifestFacts::read(&directory.join("Cargo.toml"));
    fs::remove_dir_all(&directory).ok();
    result
}

#[test]
fn publish_falseを持つxtask自身は内部領域と判定する() {
    let facts = PackageManifestFacts::read(&対象マニフェスト(".")).expect("読み取れること");
    assert!(facts.is_internal());
}

#[test]
fn proc_macro_trueを持つgraphite_macrosは内部領域と判定する() {
    let facts = PackageManifestFacts::read(&対象マニフェスト("../crates/graphite-macros"))
        .expect("読み取れること");
    assert!(facts.is_internal());
}

#[test]
fn publish_falseもproc_macroも無いgraphiteは公開面と判定する() {
    let facts = PackageManifestFacts::read(&対象マニフェスト("../crates/graphite"))
        .expect("読み取れること");
    assert!(!facts.is_internal());
}

#[test]
fn cargo_tomlが無いディレクトリはエラーになる() {
    let result = PackageManifestFacts::read(&対象マニフェスト("src"));
    assert!(result.is_err());
}

#[test]
fn 構文解析に失敗するcargo_tomlはエラーになる() {
    assert!(読み取る("syntax-error", "[package\nname = ").is_err());
}

#[test]
fn publishの空配列は内部領域と判定する() {
    let manifest = "[package]\nname = \"x\"\nversion = \"0.1.0\"\npublish = []\n";
    assert!(読み取る("publish-empty-array", manifest)
        .expect("読み取れること")
        .is_internal());
}

#[test]
fn publishの非空配列は公開面と判定する() {
    let manifest = "[package]\nname = \"x\"\nversion = \"0.1.0\"\npublish = [\"my-registry\"]\n";
    assert!(!読み取る("publish-registries", manifest)
        .expect("読み取れること")
        .is_internal());
}

#[test]
fn publishのワークスペース継承はエラーになる() {
    let manifest = "[package]\nname = \"x\"\nversion = \"0.1.0\"\npublish.workspace = true\n";
    assert!(読み取る("publish-inherit", manifest).is_err());
}

#[test]
fn publishが文字列なら予期しない型としてエラーになる() {
    let manifest = "[package]\nname = \"x\"\nversion = \"0.1.0\"\npublish = \"x\"\n";
    assert!(読み取る("publish-string", manifest).is_err());
}

#[test]
fn proc_macroの下線表記も内部領域と判定する() {
    let manifest = "[package]\nname = \"x\"\nversion = \"0.1.0\"\n\n[lib]\nproc_macro = true\n";
    assert!(読み取る("proc-macro-underscore", manifest)
        .expect("読み取れること")
        .is_internal());
}
