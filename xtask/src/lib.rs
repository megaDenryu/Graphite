//! Graphite リポジトリ自身の開発用入口のライブラリ側。
//!
//! 生成そのものは `graphite-cli` が担い、ここはワークスペース全体という走査
//! 開始点を決めてそれを呼ぶ。文書参照と索引の検査 (`check-docs`) はこの
//! リポジトリ固有の検査であり、ここに置く。バイナリ (`main.rs`) は引数解析と
//! プロセス終了コードだけを担う。`xtask/tests/` の統合テストはこのライブラリを
//! 経由して `cargo xtask generate --check` 相当を `cargo test` から検査する。

mod document_index;
mod document_reference;
mod reference_scan;
mod repository_root;

use std::error::Error;

pub use document_reference::DocumentPath;
pub use repository_root::RepositoryRoot;

use crate::document_index::DocumentIndex;
use crate::reference_scan::ReferenceScan;

/// `cargo xtask generate` 相当: 期待する生成ファイルを更新する。
pub fn generate(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    graphite_cli::generate(root.generation_tree())
}

/// `cargo xtask generate --check` 相当: 差分と孤児生成ファイルをエラーにする。
pub fn verify(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    graphite_cli::verify(root.generation_tree())
}

/// `cargo xtask check-docs` 相当: 文書参照の綴りが実在するかを検査する。
///
/// 参照が数百件の規模になったため、目視での確認は成立しない。検査の範囲と
/// 限界は `main.rs` の使い方の説明に書いてある。
pub fn check_documents(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    let scan = ReferenceScan::over(root)?;
    let existing = root.document_files()?;
    let mismatch = DocumentIndex::read_from(root)?.compare_with(&existing);
    let missing = scan.missing_targets();
    println!(
        "文書参照 {}件と docs 配下の {}ファイルを検査しました(別リポジトリを指す参照 {}件は検査対象外)",
        scan.reference_count(),
        existing.len(),
        scan.external_reference_count()
    );
    if missing.is_empty() && mismatch.is_empty() {
        return Ok(());
    }
    Err(format!("{missing}{mismatch}").into())
}
