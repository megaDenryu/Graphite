//! パッケージルートの決定と、そこから作る走査対象の組み立て。

#[cfg(test)]
mod tests;

use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generation_tree::GenerationTree;

// 生成の基準になる cargo パッケージのルート (`[package]` を持つ `Cargo.toml`
// があるディレクトリ)。
//
// Graphite リポジトリの外から使うときの走査開始点を決めるのがこの型の役目で
// あり、`xtask::RepositoryRoot` と対の位置にある。走査開始点はパッケージ直下の
// `src` と `tests` に固定する。cargo が Rust ソースを置く場所はこの2つであり、
// 生成先の規約 (`docs/code_generation.md`) もこの2つを前提に書いてある。
pub struct PackageRoot {
    path: PathBuf,
    tree: GenerationTree,
}

impl PackageRoot {
    // 実行時の作業ディレクトリから上へ辿り、最も近いパッケージを基準にする。
    //
    // cargo 自身と同じ探し方にすることで、パッケージ内のどのディレクトリから
    // 実行しても同じ結果になる。
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

    // 指定したディレクトリをパッケージルートとして受け取る。
    //
    // `[package]` を持たない `Cargo.toml` (メンバーだけを列挙する仮想
    // ワークスペース) を拒むのは、そこには `src`・`tests` が無く、走査対象が
    // 1件も無いまま無言で成功したように見えるためである。
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

    // 生成の中核へ渡す走査対象。schema宣言の抽出・生成計画・検査は
    // `generate`・`verify` が行う。
    pub fn generation_tree(&self) -> &GenerationTree {
        &self.tree
    }

    // 実行対象をそのまま示すための綴り。どのパッケージを生成したかを表示する。
    pub fn display(&self) -> String {
        self.path.display().to_string()
    }

    // このパッケージのディレクトリ。
    //
    // 注意: 生の `Path` へ戻すのは外部APIとの境界だけである。この口は
    // `std::process::Command::current_dir` へ渡すために開けてある。生成先の
    // 綴りと表示を組み立てる用途に使ってはならない (それは `GenerationTree` の
    // メソッドが持つ)。
    pub fn directory(&self) -> &Path {
        &self.path
    }
}

// `[package]` を持つ `Cargo.toml` がある場所だけをパッケージルートとして通す。
fn ensure_package_manifest(path: &Path) -> Result<(), Box<dyn Error>> {
    let manifest_path = path.join("Cargo.toml");
    let Ok(text) = fs::read_to_string(&manifest_path) else {
        return Err(format!("Cargo.toml を読めません: {}", manifest_path.display()).into());
    };
    if text.lines().any(is_package_table_header) {
        Ok(())
    } else {
        Err(format!(
            "{} には [package] がありません。生成の対象になるパッケージのディレクトリで実行してください",
            manifest_path.display()
        )
        .into())
    }
}

// `Cargo.toml` の1行が `[package]` テーブルの見出しかを判定する。
//
// 行末コメント (`[package] # 説明`) と、先頭行のバイト順マーク (BOM。ファイルの
// 文字符号化と並び順を示す不可視の先頭バイト列。Windows の編集器が付けることが
// ある) を受理する。素の一致比較だとこの2つを取りこぼし、実在するパッケージを
// 「[package] がありません」と拒む。
//
// 注意: `[package.metadata]` を受理してはならない。`[package]` の直後が終わりか
// コメントであることまで確かめる。
fn is_package_table_header(line: &str) -> bool {
    let line = line.trim_start_matches('\u{feff}').trim();
    let Some(rest) = line.strip_prefix("[package]") else {
        return false;
    };
    let rest = rest.trim_start();
    rest.is_empty() || rest.starts_with('#')
}

// 与えたディレクトリから上へ辿り、`Cargo.toml` を持つ最初のディレクトリを返す。
fn nearest_manifest_directory(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|directory| directory.join("Cargo.toml").is_file())
        .map(Path::to_path_buf)
}
