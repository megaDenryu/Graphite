use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use graphite_cli::{relative_display, with_path_context, PackageRoot};

use crate::doc_comment::{InspectedArea, RustSource};
use crate::document_reference::DocumentPath;
use crate::repository_package::RepositoryPackage;
use crate::source_reference::SourceReference;

// 生成と文書検査の基準となるリポジトリルート。
//
// 生成の対象になるパッケージ (`crates/*`・`examples/*`) を列挙するのがこの型の
// 役目であり、走査開始点の決め方と、schema宣言の抽出・生成計画・書き込み・検査
// そのものは `graphite-cli` が担う。
//
// 注意: 綴りと表示はこの型のメソッドへ閉じる。呼び出し側が裸の `PathBuf` を
// 組み立てると、宣言元との相対関係が場所ごとにずれる。
pub struct RepositoryRoot {
    path: PathBuf,
}

impl RepositoryRoot {
    // 実行時の作業ディレクトリをリポジトリルートとして受け取る。
    pub fn from_current_directory() -> Result<Self, Box<dyn Error>> {
        Self::at(env::current_dir()?)
    }

    // 指定したディレクトリをリポジトリルートとして受け取る。
    //
    // `path` の `Cargo.toml` に `[workspace]` が無ければ、リポジトリルート
    // 以外 (サブディレクトリ等) を指しているとみなしてエラーにする。検査
    // せずに受け取ると、サブディレクトリ実行時に探索対象が1件も見つからず
    // 無言で `exit 0` する (偽陰性)。`cargo test` からの検査
    // (`xtask/tests/`) は、作業ディレクトリを書き換えずにこの入口を直接
    // 使う。
    pub fn at(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        ensure_workspace_root(&path)?;
        Ok(Self { path })
    }

    // 生成の対象になるパッケージ (`crates/*`・`examples/*`) を綴り順で列挙する。
    //
    // 1パッケージにつき1つの走査対象を作るのは、外部crate向けの
    // `cargo graphite generate` が作るものと基準ディレクトリを揃えるためである。
    // リポジトリルートを基準にした1つの走査対象で全パッケージをまとめて処理すると、
    // 生成ファイルへ書く宣言元の綴りが両入口で食い違う。
    pub fn generation_packages(&self) -> Result<Vec<RepositoryPackage>, Box<dyn Error>> {
        let mut packages = Vec::new();
        for area in ["crates", "examples"] {
            for directory in subdirectories(&self.path, &self.path.join(area))? {
                if !directory.join("Cargo.toml").is_file() {
                    continue;
                }
                let spelling = self.relative_display(&directory);
                packages.push(RepositoryPackage::new(spelling, PackageRoot::at(directory)?));
            }
        }
        if packages.is_empty() {
            return Err(format!(
                "生成の対象になるパッケージが crates/ と examples/ の下に1件もありません(現在: {})",
                self.path.display()
            )
            .into());
        }
        Ok(packages)
    }

    // ワークスペースの外に置いた検証用パッケージ。
    //
    // 綴りをここ1箇所に閉じる。呼び出し側が `verification/external-crate` を
    // 組み立て直すと、移動したときに直し漏れる。裸の `PathBuf` ではなく
    // `PackageRoot` を返すのは、受け取る側が同じ役割の型を作り直さずに済ませる
    // ためである。
    pub fn external_verification_package(&self) -> Result<PackageRoot, Box<dyn Error>> {
        PackageRoot::at(self.path.join("verification").join("external-crate"))
    }

    // `docs/` 配下の文書がその綴りで実在するか。
    pub fn document_exists(&self, document: &DocumentPath) -> bool {
        self.path.join(document.spelling()).is_file()
    }

    // リポジトリ内 Rust ソースの実際の行数。実在しなければ `None`。
    //
    // 実在判定と行数取得を1回の読み込みで済ませる。存在確認だけを別の
    // `is_file` 呼び出しで行うと、走査対象が増えたときに二度手間になる。
    pub fn source_file_line_count(&self, reference: &SourceReference) -> Option<usize> {
        self.source_file_lines(reference).map(|lines| lines.len())
    }

    // リポジトリ内 Rust ソースの本文を行ごとに読む。実在しなければ `None`。
    //
    // 引用本文の照合が指定の行範囲を切り出すために使う。
    pub fn source_file_lines(&self, reference: &SourceReference) -> Option<Vec<String>> {
        self.source_file_text(reference)
            .map(|text| text.lines().map(str::to_string).collect())
    }

    // リポジトリ内 Rust ソースの本文を丸ごと読む。実在しなければ `None`。
    //
    // 引用の鮮度の照合が、行範囲によらずファイル全体を対象にするために使う。
    // 綴りの組み立てをこの型へ閉じるため、行ごとの読み出しもこのメソッドを通す。
    pub fn source_file_text(&self, reference: &SourceReference) -> Option<String> {
        fs::read_to_string(self.path.join(reference.path())).ok()
    }

    // 索引ファイル (docs/README.md) の本文を読む。
    pub fn document_index_text(&self) -> Result<String, Box<dyn Error>> {
        let path = self.path.join("docs").join("README.md");
        with_path_context(fs::read_to_string(&path), &self.relative_display(&path))
    }

    // `docs/` 配下に実在する索引対象ファイル (`.md`・`.html`) を、ルート相対の
    // 綴りで列挙する。
    //
    // 索引 (docs/README.md) との過不足の突き合わせに使う。索引側
    // (`ReferenceTarget::classify`) が `.md`・`.html` しか受理しないため、
    // ここで同じ拡張子へ絞らないと、他拡張子のファイルを `docs/` へ置いた
    // 時点でどう索引に書いても登録できず検査が恒久的に落ちる。
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

    // 文書参照を書きうるファイルを、順序を固定して列挙する。
    //
    // 走査対象は `README.md`・`CLAUDE.md`・`docs/` 配下の Markdown・
    // `examples/*/README.md`・`crates`/`xtask`/`examples`/`verification` 配下の
    // Rust ファイルである。生成ファイルも rustdoc に文書参照を持つため除外しない。
    pub fn document_reference_sources(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut candidates = Vec::new();
        push_if_file(&self.path.join("README.md"), &mut candidates);
        push_if_file(&self.path.join("CLAUDE.md"), &mut candidates);
        self.collect_all_files(&self.path.join("docs"), &mut candidates)?;
        for example_directory in subdirectories(&self.path, &self.path.join("examples"))? {
            push_if_file(&example_directory.join("README.md"), &mut candidates);
        }
        for area in ["crates", "xtask", "examples", "verification"] {
            self.collect_all_files(&self.path.join(area), &mut candidates)?;
        }
        candidates.retain(is_scannable_text_file);
        candidates.sort();
        candidates.dedup();
        Ok(candidates)
    }

    // 指定した領域の配下にある Rust ソースを、綴り順で列挙する。
    //
    // doc コメントの検査 (`check-doc-comments`) が使う。綴りの組み立てをこの型へ
    // 閉じるのは、領域の指定と表示の綴りが呼び出し側ごとにずれないようにするため
    // である。
    pub(crate) fn rust_source_files(
        &self,
        area: &InspectedArea,
    ) -> Result<Vec<RustSource>, Box<dyn Error>> {
        let mut paths = Vec::new();
        self.collect_all_files(&self.path.join(area.spelling()), &mut paths)?;
        paths.retain(|path| path.extension().and_then(OsStr::to_str) == Some("rs"));
        paths.sort();
        Ok(paths
            .into_iter()
            .map(|path| RustSource::new(self.relative_display(&path), path))
            .collect())
    }

    // リポジトリルートからの相対パスを、環境によらない綴りで表示する。
    pub fn relative_display(&self, path: &Path) -> String {
        relative_display(&self.path, path)
    }

    // ビルド生成物を除いて、ディレクトリ配下の全ファイルを再帰的に集める。
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

// `[workspace]` を持つ `Cargo.toml` がある場所だけをリポジトリルートとして通す。
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

// 指定ディレクトリの直下のディレクトリ一覧 (存在しなければ空)。
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

fn push_if_file(path: &Path, paths: &mut Vec<PathBuf>) {
    if path.is_file() {
        paths.push(path.to_path_buf());
    }
}

// 文書参照の出典として読むのは Markdown と Rust のソースだけである。
//
// `design_journal.html` のような添付は、参照を書く場所として扱わない。
fn is_scannable_text_file(path: &PathBuf) -> bool {
    matches!(path.extension().and_then(OsStr::to_str), Some("md" | "rs"))
}

// 索引 (docs/README.md) が受理する拡張子 (`.md`・`.html`) だけを通す。
//
// `ReferenceTarget::classify` の受理拡張子と一致させる。ここを広げると、
// 索引に登録できない拡張子のファイルが `docs/` へ紛れ込んだ時点で
// `check-docs` が恒久的に失敗する。
fn is_indexable_document_file(path: &PathBuf) -> bool {
    matches!(path.extension().and_then(OsStr::to_str), Some("md" | "html"))
}
