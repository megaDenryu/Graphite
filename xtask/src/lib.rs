//! 生成元の探索と通常Rustファイルの読み書きを担う開発用入口のライブラリ側。
//!
//! バイナリ (`main.rs`) は引数解析とプロセス終了コードだけを担い、実処理は
//! ここへ集約する。`xtask/tests/` の統合テストはこのライブラリを経由して
//! `cargo xtask generate --check` 相当を `cargo test` から検査する。

mod document_index;
mod document_reference;
mod generated_target_path;
mod generation_plan;
mod io_context;
mod reference_scan;
mod repository_root;
mod schema_source_file;

use std::error::Error;

pub use document_reference::DocumentPath;
pub use generation_plan::GenerationPlan;
pub use repository_root::RepositoryRoot;

use crate::document_index::DocumentIndex;
use crate::reference_scan::ReferenceScan;

/// 全schema宣言から `GenerationPlan` を組み立てる。
fn build_plan(root: &RepositoryRoot) -> Result<GenerationPlan, Box<dyn Error>> {
    let mut plan = GenerationPlan::new();
    for source in root.schema_source_files()? {
        source.collect_into(root, &mut plan)?;
    }
    Ok(plan)
}

/// `cargo xtask generate` 相当: 期待する生成ファイルを更新する。
pub fn generate(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    build_plan(root)?.write_stale_files(root)
}

/// `cargo xtask generate --check` 相当: 差分と孤児生成ファイルをエラーにする。
pub fn verify(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    build_plan(root)?.verify(root)
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
