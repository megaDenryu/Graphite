//! 生成元の探索と通常Rustファイルの読み書きを担う開発用入口のライブラリ側。
//!
//! バイナリ (`main.rs`) は引数解析とプロセス終了コードだけを担い、実処理は
//! ここへ集約する。`xtask/tests/` の統合テストはこのライブラリを経由して
//! `cargo xtask generate --check` 相当を `cargo test` から検査する。

mod generated_target_path;
mod generation_plan;
mod io_context;
mod repository_root;
mod schema_source_file;

use std::error::Error;

pub use generation_plan::GenerationPlan;
pub use repository_root::RepositoryRoot;

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
