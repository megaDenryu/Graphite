use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generated_target_path::GeneratedTargetPath;
use crate::io_context::with_path_context;
use crate::schema_source_file::SchemaSourceFile;

/// 生成の基準となるリポジトリルート。
///
/// 注意: 生成先の綴りと表示はこの型のメソッドへ閉じる。呼び出し側が裸の
/// `PathBuf` を組み立てると、宣言元との相対関係が場所ごとにずれる。
pub struct RepositoryRoot {
    path: PathBuf,
}

impl RepositoryRoot {
    /// 実行時の作業ディレクトリをリポジトリルートとして受け取る。
    pub fn from_current_directory() -> Result<Self, Box<dyn Error>> {
        Self::at(env::current_dir()?)
    }

    /// 指定したディレクトリをリポジトリルートとして受け取る。
    ///
    /// `path` の `Cargo.toml` に `[workspace]` が無ければ、リポジトリルート
    /// 以外 (サブディレクトリ等) を指しているとみなしてエラーにする。検査
    /// せずに受け取ると、サブディレクトリ実行時に探索対象が1件も見つからず
    /// 無言で `exit 0` する (偽陰性)。`cargo test` からの検査
    /// (`xtask/tests/`) は、作業ディレクトリを書き換えずにこの入口を直接
    /// 使う。
    pub fn at(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let root = Self { path };
        root.ensure_workspace_root()?;
        Ok(root)
    }

    fn ensure_workspace_root(&self) -> Result<(), Box<dyn Error>> {
        let manifest_path = self.path.join("Cargo.toml");
        let has_workspace_table = fs::read_to_string(&manifest_path)
            .map(|text| text.lines().any(|line| line.trim() == "[workspace]"))
            .unwrap_or(false);
        if has_workspace_table {
            Ok(())
        } else {
            Err(format!(
                "リポジトリルートで実行してください(現在: {})",
                self.path.display()
            )
            .into())
        }
    }

    /// schema宣言を探す対象のRustファイルを、順序を固定して列挙する。
    ///
    /// 走査対象は workspace 全体 (`crates/*/src`・`crates/*/tests`・
    /// `examples/*/src`) であり、`docs/code_generation.md` の生成先の記述と
    /// 一致させている。`target`・`generated`・`ui` (trybuild フィクスチャ) は
    /// 除外する。
    pub fn schema_source_files(&self) -> Result<Vec<SchemaSourceFile>, Box<dyn Error>> {
        let mut paths = Vec::new();
        for root in self.scan_roots()? {
            collect_rust_files(self, &root, &mut paths)?;
        }
        paths.sort();
        if paths.is_empty() {
            return Err("schema宣言が1件も見つかりません。実行場所を確認してください".into());
        }
        Ok(paths.into_iter().map(SchemaSourceFile::new).collect())
    }

    /// `generated/` 配下に実在する生成ファイルを、走査対象の全域から列挙する。
    ///
    /// schema宣言の削除・移動で取り残された孤児生成ファイルを検出するために使う
    /// (`GenerationPlan::verify` 参照)。
    pub fn existing_generated_files(&self) -> Result<Vec<GeneratedTargetPath>, Box<dyn Error>> {
        let mut files = Vec::new();
        for root in self.scan_roots()? {
            collect_generated_files(self, &root, &mut files)?;
        }
        files.sort();
        Ok(files)
    }

    /// 単体テスト専用: 実ファイルシステムを介さずに構築する。
    #[cfg(test)]
    pub(crate) fn for_tests(path: PathBuf) -> Self {
        Self { path }
    }

    /// リポジトリルートからの相対パスを、環境によらない綴りで表示する。
    pub fn relative_display(&self, path: &Path) -> String {
        path.strip_prefix(&self.path)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    /// schema探索・生成ファイル探索の両方が使う走査開始ディレクトリの一覧。
    fn scan_roots(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut roots = Vec::new();
        for crate_directory in subdirectories(self, &self.path.join("crates"))? {
            push_if_exists(&crate_directory.join("src"), &mut roots);
            push_if_exists(&crate_directory.join("tests"), &mut roots);
        }
        for example_directory in subdirectories(self, &self.path.join("examples"))? {
            push_if_exists(&example_directory.join("src"), &mut roots);
        }
        Ok(roots)
    }
}

/// 指定ディレクトリの直下のディレクトリ一覧 (存在しなければ空)。
fn subdirectories(root: &RepositoryRoot, directory: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    for entry in with_path_context(fs::read_dir(directory), &root.relative_display(directory))? {
        let path = entry?.path();
        if path.is_dir() {
            found.push(path);
        }
    }
    Ok(found)
}

fn push_if_exists(path: &Path, roots: &mut Vec<PathBuf>) {
    if path.is_dir() {
        roots.push(path.to_path_buf());
    }
}

/// 生成物と compile-fail 用のソースを除いて、Rustファイルを再帰的に集める。
fn collect_rust_files(
    root: &RepositoryRoot,
    directory: &Path,
    paths: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in with_path_context(fs::read_dir(directory), &root.relative_display(directory))? {
        let path = entry?.path();
        if path.is_dir() {
            if matches!(
                path.file_name().and_then(OsStr::to_str),
                Some("target" | "generated" | "ui")
            ) {
                continue;
            }
            collect_rust_files(root, &path, paths)?;
        } else if path.extension() == Some(OsStr::new("rs")) {
            paths.push(path);
        }
    }
    Ok(())
}

/// `generated` という名前のディレクトリを探し、その直下の `.rs` ファイルを集める。
///
/// `collect_rust_files` と対の関係にある: そちらは `generated`/`ui`/`target` を
/// 除外して宣言元を集め、こちらは `ui`/`target` を除外しつつ `generated` の
/// 中身だけを集める。
fn collect_generated_files(
    root: &RepositoryRoot,
    directory: &Path,
    files: &mut Vec<GeneratedTargetPath>,
) -> Result<(), Box<dyn Error>> {
    for entry in with_path_context(fs::read_dir(directory), &root.relative_display(directory))? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }
        let name = path.file_name().and_then(OsStr::to_str);
        if matches!(name, Some("target" | "ui")) {
            continue;
        }
        if name == Some("generated") {
            collect_generated_leaf_files(root, &path, files)?;
        } else {
            collect_generated_files(root, &path, files)?;
        }
    }
    Ok(())
}

fn collect_generated_leaf_files(
    root: &RepositoryRoot,
    generated_directory: &Path,
    files: &mut Vec<GeneratedTargetPath>,
) -> Result<(), Box<dyn Error>> {
    for entry in with_path_context(
        fs::read_dir(generated_directory),
        &root.relative_display(generated_directory),
    )? {
        let path = entry?.path();
        if path.extension() == Some(OsStr::new("rs")) {
            files.push(GeneratedTargetPath::new(path));
        }
    }
    Ok(())
}
