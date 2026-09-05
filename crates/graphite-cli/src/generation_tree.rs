use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generated_target_path::GeneratedTargetPath;
use crate::io_context::with_path_context;
use crate::relative_display::relative_display;
use crate::schema_source_file::SchemaSourceFile;

// 生成が対象にするディレクトリ木。基準ディレクトリと走査開始点の一覧を持つ。
//
// 組み立て口は `PackageRoot` だけであり、基準ディレクトリは常にパッケージルート、
// 走査開始点は常にパッケージ直下の `src`・`tests` である。どのパッケージを対象に
// するかだけが入口ごとに違い (`cargo graphite` は実行した場所の1つ、Graphite
// リポジトリの `xtask` は `crates/*`・`examples/*` の全部)、schema宣言の抽出・
// 生成計画・書き込み・検査はこの型を通して共有する。
//
// 注意: 生成先の綴りと表示はこの型のメソッドへ閉じる。呼び出し側が裸の
// `PathBuf` を組み立てると、宣言元との相対関係が場所ごとにずれる。
pub struct GenerationTree {
    base: PathBuf,
    scan_roots: Vec<PathBuf>,
}

impl GenerationTree {
    // 基準ディレクトリと、その配下の走査開始点から組み立てる。
    //
    // 前提: `scan_roots` は実在するディレクトリであり、`base` の配下にある。
    // 唯一の組み立て口である `PackageRoot` が列挙時に確かめる。
    pub(crate) fn new(base: PathBuf, scan_roots: Vec<PathBuf>) -> Self {
        Self { base, scan_roots }
    }

    // 基準ディレクトリからの相対パスを、環境によらない綴りで表示する。
    pub(crate) fn relative_display(&self, path: &Path) -> String {
        relative_display(&self.base, path)
    }

    // schema宣言を探す対象のRustファイルを、順序を固定して列挙する。
    //
    // `target`・`generated`・`ui` (trybuild フィクスチャ) は除外する。
    pub(crate) fn schema_source_files(&self) -> Result<Vec<SchemaSourceFile>, Box<dyn Error>> {
        let mut paths = Vec::new();
        for root in &self.scan_roots {
            self.collect_rust_files(root, &mut paths)?;
        }
        paths.sort();
        if paths.is_empty() {
            return Err(format!(
                "schemaを探すRustファイルが1件も見つかりません(走査対象: {})",
                self.scan_root_display()
            )
            .into());
        }
        Ok(paths.into_iter().map(SchemaSourceFile::new).collect())
    }

    // `generated/` 配下に実在する生成ファイルを、走査対象の全域から列挙する。
    //
    // schema宣言の削除・移動で取り残された孤児生成ファイルを検出するために使う
    // (`GenerationPlan::verify` 参照)。
    pub(crate) fn existing_generated_files(
        &self,
    ) -> Result<Vec<GeneratedTargetPath>, Box<dyn Error>> {
        let mut files = Vec::new();
        for root in &self.scan_roots {
            self.collect_generated_files(root, &mut files)?;
        }
        files.sort();
        Ok(files)
    }

    // 走査開始点が1件も無いときに「対象なし」と読めるよう、一覧を綴りで並べる。
    fn scan_root_display(&self) -> String {
        if self.scan_roots.is_empty() {
            return "なし".to_string();
        }
        self.scan_roots
            .iter()
            .map(|root| self.relative_display(root))
            .collect::<Vec<_>>()
            .join(" ")
    }

    // 生成物と compile-fail 用のソースを除いて、Rustファイルを再帰的に集める。
    fn collect_rust_files(
        &self,
        directory: &Path,
        paths: &mut Vec<PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in with_path_context(fs::read_dir(directory), &self.relative_display(directory))? {
            let path = entry?.path();
            if path.is_dir() {
                if matches!(
                    path.file_name().and_then(OsStr::to_str),
                    Some("target" | "generated" | "ui")
                ) {
                    continue;
                }
                self.collect_rust_files(&path, paths)?;
            } else if path.extension() == Some(OsStr::new("rs")) {
                paths.push(path);
            }
        }
        Ok(())
    }

    // `generated` という名前のディレクトリを探し、その直下の `.rs` ファイルを集める。
    //
    // `collect_rust_files` と対の関係にある: そちらは `generated`/`ui`/`target` を
    // 除外して宣言元を集め、こちらは `ui`/`target` を除外しつつ `generated` の
    // 中身だけを集める。
    fn collect_generated_files(
        &self,
        directory: &Path,
        files: &mut Vec<GeneratedTargetPath>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in with_path_context(fs::read_dir(directory), &self.relative_display(directory))? {
            let path = entry?.path();
            if !path.is_dir() {
                continue;
            }
            let name = path.file_name().and_then(OsStr::to_str);
            if matches!(name, Some("target" | "ui")) {
                continue;
            }
            if name == Some("generated") {
                self.collect_generated_leaf_files(&path, files)?;
            } else {
                self.collect_generated_files(&path, files)?;
            }
        }
        Ok(())
    }

    fn collect_generated_leaf_files(
        &self,
        generated_directory: &Path,
        files: &mut Vec<GeneratedTargetPath>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in with_path_context(
            fs::read_dir(generated_directory),
            &self.relative_display(generated_directory),
        )? {
            let path = entry?.path();
            if path.extension() == Some(OsStr::new("rs")) {
                files.push(GeneratedTargetPath::new(path));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::GenerationTree;
    use std::path::PathBuf;

    #[test]
    fn 走査開始点が無ければ対象なしと表示する() {
        let tree = GenerationTree::new(PathBuf::from("/repo"), Vec::new());
        assert_eq!(tree.scan_root_display(), "なし");
    }
}
