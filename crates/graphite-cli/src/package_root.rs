use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generation_tree::GenerationTree;

/// 生成の基準になる cargo パッケージのルート (`[package]` を持つ `Cargo.toml`
/// があるディレクトリ)。
///
/// Graphite リポジトリの外から使うときの走査開始点を決めるのがこの型の役目で
/// あり、`xtask::RepositoryRoot` と対の位置にある。走査開始点はパッケージ直下の
/// `src` と `tests` に固定する。cargo が Rust ソースを置く場所はこの2つであり、
/// 生成先の規約 (`docs/code_generation.md`) もこの2つを前提に書いてある。
pub struct PackageRoot {
    path: PathBuf,
    tree: GenerationTree,
}

impl PackageRoot {
    /// 実行時の作業ディレクトリから上へ辿り、最も近いパッケージを基準にする。
    ///
    /// cargo 自身と同じ探し方にすることで、パッケージ内のどのディレクトリから
    /// 実行しても同じ結果になる。
    pub fn from_current_directory() -> Result<Self, Box<dyn Error>> {
        let start = env::current_dir()?;
        let Some(found) = nearest_manifest_directory(&start) else {
            return Err(format!(
                "Cargo.toml が見つかりません。cargo パッケージの中で実行してください(現在: {})",
                start.display()
            )
            .into());
        };
        Self::at(found)
    }

    /// 指定したディレクトリをパッケージルートとして受け取る。
    ///
    /// `[package]` を持たない `Cargo.toml` (メンバーだけを列挙する仮想
    /// ワークスペース) を拒むのは、そこには `src`・`tests` が無く、走査対象が
    /// 1件も無いまま無言で成功したように見えるためである。
    pub fn at(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        ensure_package_manifest(&path)?;
        let mut scan_roots = Vec::new();
        for area in ["src", "tests"] {
            let directory = path.join(area);
            if directory.is_dir() {
                scan_roots.push(directory);
            }
        }
        let tree = GenerationTree::new(path.clone(), scan_roots);
        Ok(Self { path, tree })
    }

    /// 生成コアへ渡す走査対象。抽出・計画・検査は `generate`・`verify` が行う。
    pub fn generation_tree(&self) -> &GenerationTree {
        &self.tree
    }

    /// 実行対象をそのまま示すための綴り。どのパッケージを生成したかを表示する。
    pub fn display(&self) -> String {
        self.path.display().to_string()
    }
}

/// `[package]` を持つ `Cargo.toml` がある場所だけをパッケージルートとして通す。
fn ensure_package_manifest(path: &Path) -> Result<(), Box<dyn Error>> {
    let manifest_path = path.join("Cargo.toml");
    let Ok(text) = fs::read_to_string(&manifest_path) else {
        return Err(format!("Cargo.toml を読めません: {}", manifest_path.display()).into());
    };
    if text.lines().any(|line| line.trim() == "[package]") {
        Ok(())
    } else {
        Err(format!(
            "{} には [package] がありません。生成の対象になるパッケージのディレクトリで実行してください",
            manifest_path.display()
        )
        .into())
    }
}

/// 与えたディレクトリから上へ辿り、`Cargo.toml` を持つ最初のディレクトリを返す。
fn nearest_manifest_directory(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|directory| directory.join("Cargo.toml").is_file())
        .map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use super::{nearest_manifest_directory, PackageRoot};
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
    fn マニフェストの無いディレクトリからは上へ辿る() {
        let source_directory = this_package().join("src");
        assert_eq!(
            nearest_manifest_directory(&source_directory).as_deref(),
            Some(this_package().as_path())
        );
    }
}
