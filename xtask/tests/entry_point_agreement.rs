//! 2つの生成の入口が同じ本文を書くことを、リポジトリ内のパッケージで実測する。
//!
//! 生成ファイルの2行目に書く宣言元の綴りは走査対象の基準ディレクトリからの相対で
//! ある。基準が入口ごとに違うと、`cargo graphite generate` が書いた本文を
//! `cargo xtask generate --check` が古いと判定する (逆も同じ)。このテストは
//! `cargo xtask generate` が書いた作業ツリーに対して、外部crate向けの入口
//! (`graphite_cli::PackageRoot`) から差分検査を通し、両者が一致することを
//! `cargo test --workspace` の中で確かめる。

use std::fs;
use std::path::PathBuf;

use graphite_cli::PackageRoot;

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// `crates/*` と `examples/*` のうち `Cargo.toml` を持つディレクトリを綴り順で集める。
fn package_directories() -> Vec<PathBuf> {
    let root = repository_root();
    let mut directories = Vec::new();
    for area in ["crates", "examples"] {
        let entries = fs::read_dir(root.join(area)).expect("crates と examples は実在する");
        for entry in entries {
            let path = entry.expect("ディレクトリ項目を読めること").path();
            if path.join("Cargo.toml").is_file() {
                directories.push(path);
            }
        }
    }
    directories.sort();
    directories
}

#[test]
fn 外部crate向けの入口で検査してもリポジトリ内の生成ファイルは最新である() {
    let directories = package_directories();
    assert!(
        !directories.is_empty(),
        "生成の対象になるパッケージが1件も見つかりません"
    );
    for directory in directories {
        let package = PackageRoot::at(directory.clone())
            .expect("crates と examples の各ディレクトリはcargoパッケージであること");
        graphite_cli::verify(package.generation_tree()).unwrap_or_else(|error| {
            panic!(
                "{} を `cargo graphite generate --check` 相当で検査すると差分が出ました。\
                 2つの入口の生成本文が食い違っています: {error}",
                directory.display()
            )
        });
    }
}
