use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

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
        Ok(Self {
            path: env::current_dir()?,
        })
    }

    /// schema宣言を探す対象のRustファイルを、順序を固定して列挙する。
    pub fn schema_source_files(&self) -> Result<Vec<SchemaSourceFile>, Box<dyn Error>> {
        let mut paths = Vec::new();
        collect_rust_files(&self.path.join("crates/graphite/tests"), &mut paths)?;
        collect_rust_files(&self.path.join("examples"), &mut paths)?;
        paths.sort();
        Ok(paths.into_iter().map(SchemaSourceFile::new).collect())
    }

    /// リポジトリルートからの相対パスを、環境によらない綴りで表示する。
    pub fn relative_display(&self, path: &Path) -> String {
        path.strip_prefix(&self.path)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }
}

/// 生成物と compile-fail 用のソースを除いて、Rustファイルを再帰的に集める。
fn collect_rust_files(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    if !directory.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            if matches!(
                path.file_name().and_then(OsStr::to_str),
                Some("target" | "generated" | "ui")
            ) {
                continue;
            }
            collect_rust_files(&path, paths)?;
        } else if path.extension() == Some(OsStr::new("rs")) {
            paths.push(path);
        }
    }
    Ok(())
}
