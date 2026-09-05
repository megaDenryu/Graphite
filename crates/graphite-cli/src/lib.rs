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

// 全schema宣言から `GenerationPlan` を組み立てる。
fn build_plan(tree: &GenerationTree) -> Result<GenerationPlan, Box<dyn Error>> {
    let mut plan = GenerationPlan::new();
    for source in tree.schema_source_files()? {
        source.collect_into(tree, &mut plan)?;
    }
    Ok(plan)
}

// `generate` 相当: 期待する生成ファイルを更新する。
//
// 何件の宣言を読み何件書いたかを必ず1行で表示する。表示しないと、宣言が0件の
// パッケージ (置き場所を間違えた・拡張子を間違えた) でも無言で成功したように
// 見え、生成されていないことに気付けない。
pub fn generate(tree: &GenerationTree) -> Result<(), Box<dyn Error>> {
    let plan = build_plan(tree)?;
    let written = plan.write_stale_files(tree)?;
    println!("schema宣言 {}件、生成 {written}件", plan.declaration_count());
    Ok(())
}

// `generate --check` 相当: 差分と孤児生成ファイルをエラーにする。
//
// 差分が無ければ、読んだ宣言の件数を1行で表示する。`generate` と同じ理由で、
// 対象が0件のまま成功したことを黙って通さない。
pub fn verify(tree: &GenerationTree) -> Result<(), Box<dyn Error>> {
    let plan = build_plan(tree)?;
    plan.verify(tree)?;
    let count = plan.declaration_count();
    println!("schema宣言 {count}件、最新 {count}件");
    Ok(())
}
