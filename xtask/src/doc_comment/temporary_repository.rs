//! テストが使う疑似リポジトリの一時ディレクトリ (issue #32 検収の是正)。
//!
//! この型は、internal_area_derivation と orphan_inspection のテストが個別に
//! 組み立てていた固定具を1箇所へ集約する。この型は Drop で一時ディレクトリを
//! 削除するため、テストが途中で失敗しても一時ディレクトリが残らない。

use std::fs;
use std::path::{Path, PathBuf};

pub(super) struct TemporaryRepository {
    root: PathBuf,
}

impl TemporaryRepository {
    // この関数は、空の疑似リポジトリを一時ディレクトリへ作り、ルートへ
    // `[workspace]` を持つ Cargo.toml だけを置く。
    pub(super) fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("graphite-xtask-{name}-{}", std::process::id()));
        fs::create_dir_all(&root).expect("一時ディレクトリを作成できること");
        fs::write(root.join("Cargo.toml"), "[workspace]\n")
            .expect("ルートのCargo.tomlを書けること");
        Self { root }
    }

    pub(super) fn path(&self) -> &Path {
        &self.root
    }

    // この関数は、指定した相対ディレクトリへ、Cargo.toml と空の src/lib.rs を
    // 持つパッケージを置く。呼び出し元は `manifest_extra` に、`[package]`
    // テーブルへ追記する行 (末尾に改行を含む) を渡す。呼び出し元は、
    // `publish = false\n` のように内部領域の条件を足すためにこれを使う。
    pub(super) fn write_package(&self, relative_directory: &str, manifest_extra: &str) {
        let directory = self.root.join(relative_directory);
        fs::create_dir_all(directory.join("src")).expect("パッケージのsrcを作成できること");
        let name = relative_directory
            .rsplit('/')
            .next()
            .unwrap_or(relative_directory);
        fs::write(
            directory.join("Cargo.toml"),
            format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n{manifest_extra}"),
        )
        .expect("Cargo.tomlを書けること");
        fs::write(directory.join("src/lib.rs"), "").expect("src/lib.rsを書けること");
    }

    // この関数は、指定した相対パスへ任意の内容のファイルを置く。呼び出し元は、
    // 所属パッケージを持たないソースのような、パッケージ構造に沿わない
    // ファイルを置くためにこの関数を使う。
    pub(super) fn write_file(&self, relative_path: &str, content: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("親ディレクトリを作成できること");
        }
        fs::write(path, content).expect("ファイルを書けること");
    }
}

impl Drop for TemporaryRepository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
