//! schema宣言から通常のRustソースを生成する道具のライブラリ側。
//!
//! 純粋層 (`graphite-codegen`) が作った本文をファイルへ書き出し、作業ツリーとの
//! 差分を検査するところまでを担う。走査開始点の決め方だけが入口ごとに違い、
//! 抽出・計画・書き込み・検査は `GenerationTree` を通して共有する。入口は
//! `cargo graphite` (`main.rs`) と Graphite リポジトリ自身の `xtask` の2つである。

mod generated_target_path;
mod generation_plan;
mod generation_tree;
mod io_context;
mod package_root;
mod relative_display;
mod schema_source_file;

use std::error::Error;

use crate::generation_plan::GenerationPlan;

pub use generation_tree::GenerationTree;
pub use io_context::with_path_context;
pub use package_root::PackageRoot;
pub use relative_display::relative_display;

/// 全schema宣言から `GenerationPlan` を組み立てる。
fn build_plan(tree: &GenerationTree) -> Result<GenerationPlan, Box<dyn Error>> {
    let mut plan = GenerationPlan::new();
    for source in tree.schema_source_files()? {
        source.collect_into(tree, &mut plan)?;
    }
    Ok(plan)
}

/// `generate` 相当: 期待する生成ファイルを更新する。
pub fn generate(tree: &GenerationTree) -> Result<(), Box<dyn Error>> {
    build_plan(tree)?.write_stale_files(tree)
}

/// `generate --check` 相当: 差分と孤児生成ファイルをエラーにする。
pub fn verify(tree: &GenerationTree) -> Result<(), Box<dyn Error>> {
    build_plan(tree)?.verify(tree)
}
