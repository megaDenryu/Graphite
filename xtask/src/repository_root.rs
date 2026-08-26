use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use graphite_cli::{relative_display, with_path_context, GenerationTree};

use crate::document_reference::DocumentPath;

/// 生成と文書検査の基準となるリポジトリルート。
///
/// 生成の走査開始点 (`crates/*/src`・`crates/*/tests`・`examples/*/src`) を
/// 決めて `GenerationTree` を組み立てるのがこの型の役目であり、schema宣言の
/// 抽出・生成計画・書き込み・検査そのものは `graphite-cli` が担う。
///
/// 注意: 綴りと表示はこの型のメソッドへ閉じる。呼び出し側が裸の `PathBuf` を
/// 組み立てると、宣言元との相対関係が場所ごとにずれる。
pub struct RepositoryRoot {
    path: PathBuf,
    tree: GenerationTree,
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
        ensure_workspace_root(&path)?;
        let tree = GenerationTree::new(path.clone(), scan_roots(&path)?);
        Ok(Self { path, tree })
    }

    /// 生成コアへ渡す走査対象。抽出・計画・検査は `graphite-cli` が行う。
    pub fn generation_tree(&self) -> &GenerationTree {
        &self.tree
    }

    /// `docs/` 配下の文書がその綴りで実在するか。
    pub fn document_exists(&self, document: &DocumentPath) -> bool {
        self.path.join(document.spelling()).is_file()
    }

    /// 索引ファイル (docs/README.md) の本文を読む。
    pub fn document_index_text(&self) -> Result<String, Box<dyn Error>> {
        let path = self.path.join("docs").join("README.md");
        with_path_context(fs::read_to_string(&path), &self.relative_display(&path))
    }

    /// `docs/` 配下に実在する索引対象ファイル (`.md`・`.html`) を、ルート相対の
    /// 綴りで列挙する。
    ///
    /// 索引 (docs/README.md) との過不足の突き合わせに使う。索引側
    /// (`ReferenceTarget::classify`) が `.md`・`.html` しか受理しないため、
    /// ここで同じ拡張子へ絞らないと、他拡張子のファイルを `docs/` へ置いた
    /// 時点でどう索引に書いても登録できず検査が恒久的に落ちる。
    pub fn document_files(&self) -> Result<Vec<DocumentPath>, Box<dyn Error>> {
        let mut paths = Vec::new();
        self.collect_all_files(&self.path.join("docs"), &mut paths)?;
        paths.retain(is_indexable_document_file);
        let mut documents: Vec<DocumentPath> = paths
            .iter()
            .map(|path| DocumentPath::from_relative_display(&self.relative_display(path)))
            .collect();
        documents.sort();
        Ok(documents)
    }

    /// 文書参照を書きうるファイルを、順序を固定して列挙する。
    ///
    /// 走査対象は `README.md`・`CLAUDE.md`・`docs/` 配下の Markdown・
    /// `examples/*/README.md`・`crates`/`xtask`/`examples` 配下の Rust ファイル
    /// である。生成ファイルも rustdoc に文書参照を持つため除外しない。
    pub fn document_reference_sources(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut candidates = Vec::new();
        push_if_file(&self.path.join("README.md"), &mut candidates);
        push_if_file(&self.path.join("CLAUDE.md"), &mut candidates);
        self.collect_all_files(&self.path.join("docs"), &mut candidates)?;
        for example_directory in subdirectories(&self.path, &self.path.join("examples"))? {
            push_if_file(&example_directory.join("README.md"), &mut candidates);
        }
        for area in ["crates", "xtask", "examples"] {
            self.collect_all_files(&self.path.join(area), &mut candidates)?;
        }
        candidates.retain(is_scannable_text_file);
        candidates.sort();
        candidates.dedup();
        Ok(candidates)
    }

    /// リポジトリルートからの相対パスを、環境によらない綴りで表示する。
    pub fn relative_display(&self, path: &Path) -> String {
        relative_display(&self.path, path)
    }

    /// ビルド生成物を除いて、ディレクトリ配下の全ファイルを再帰的に集める。
    fn collect_all_files(
        &self,
        directory: &Path,
        paths: &mut Vec<PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        if !directory.is_dir() {
            return Ok(());
        }
        for entry in with_path_context(fs::read_dir(directory), &self.relative_display(directory))? {
            let path = entry?.path();
            if path.is_dir() {
                if path.file_name().and_then(OsStr::to_str) == Some("target") {
                    continue;
                }
                self.collect_all_files(&path, paths)?;
            } else {
                paths.push(path);
            }
        }
        Ok(())
    }
}

/// `[workspace]` を持つ `Cargo.toml` がある場所だけをリポジトリルートとして通す。
fn ensure_workspace_root(path: &Path) -> Result<(), Box<dyn Error>> {
    let manifest_path = path.join("Cargo.toml");
    let has_workspace_table = fs::read_to_string(&manifest_path)
        .map(|text| text.lines().any(|line| line.trim() == "[workspace]"))
        .unwrap_or(false);
    if has_workspace_table {
        Ok(())
    } else {
        Err(format!("リポジトリルートで実行してください(現在: {})", path.display()).into())
    }
}

/// schema探索・生成ファイル探索の両方が使う走査開始ディレクトリの一覧。
///
/// 走査対象は workspace 全体 (`crates/*/src`・`crates/*/tests`・
/// `examples/*/src`) であり、`docs/code_generation.md` の生成先の記述と
/// 一致させている。
fn scan_roots(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut roots = Vec::new();
    for crate_directory in subdirectories(path, &path.join("crates"))? {
        push_if_directory(&crate_directory.join("src"), &mut roots);
        push_if_directory(&crate_directory.join("tests"), &mut roots);
    }
    for example_directory in subdirectories(path, &path.join("examples"))? {
        push_if_directory(&example_directory.join("src"), &mut roots);
    }
    Ok(roots)
}

/// 指定ディレクトリの直下のディレクトリ一覧 (存在しなければ空)。
fn subdirectories(base: &Path, directory: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    for entry in with_path_context(fs::read_dir(directory), &relative_display(base, directory))? {
        let path = entry?.path();
        if path.is_dir() {
            found.push(path);
        }
    }
    found.sort();
    Ok(found)
}

fn push_if_directory(path: &Path, roots: &mut Vec<PathBuf>) {
    if path.is_dir() {
        roots.push(path.to_path_buf());
    }
}

fn push_if_file(path: &Path, paths: &mut Vec<PathBuf>) {
    if path.is_file() {
        paths.push(path.to_path_buf());
    }
}

/// 文書参照の出典として読むのは Markdown と Rust のソースだけである。
///
/// `design_journal.html` のような添付は、参照を書く場所として扱わない。
fn is_scannable_text_file(path: &PathBuf) -> bool {
    matches!(path.extension().and_then(OsStr::to_str), Some("md" | "rs"))
}

/// 索引 (docs/README.md) が受理する拡張子 (`.md`・`.html`) だけを通す。
///
/// `ReferenceTarget::classify` の受理拡張子と一致させる。ここを広げると、
/// 索引に登録できない拡張子のファイルが `docs/` へ紛れ込んだ時点で
/// `check-docs` が恒久的に失敗する。
fn is_indexable_document_file(path: &PathBuf) -> bool {
    matches!(path.extension().and_then(OsStr::to_str), Some("md" | "html"))
}
