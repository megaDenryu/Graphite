//! schema宣言から通常のRustソースを生成する道具のライブラリ側。
//!
//! 純粋層 (`graphite-codegen`) が作った本文をファイルへ書き出し、作業ツリーとの
//! 差分を検査するところまでを担う。走査開始点の決め方だけが入口ごとに違い、
//! 抽出・計画・書き込み・検査は `GenerationTree` を通して共有する。入口は
//! `cargo graphite` (`main.rs`) と Graphite リポジトリ自身の `xtask` の2つである。
//!
//! schema宣言は3種類ある。動的グラフの `dynamic_graph_schema!` はファイル
//! 単体で解決できるが、静的グラフの `static_graph_schema!`/instanceは
//! パッケージ全体を横断する2段階の解決 (`static_resolution`) を要る。

mod cargo_target;
mod generated_target_path;
mod generation_plan;
mod generation_tree;
mod io_context;
mod package_root;
mod relative_display;
mod schema_macro_collector;
mod schema_source_file;
mod static_resolution;

use std::error::Error;

use crate::generation_plan::GenerationPlan;
use crate::schema_macro_collector::collect_macro_calls;
use crate::static_resolution::{
    埋め込まれたinstanceを検査する, instanceを解決する, FileMacros, 静的schema名簿ビルダー,
};

pub use generation_tree::GenerationTree;
pub use io_context::with_path_context;
pub use package_root::PackageRoot;
pub use relative_display::relative_display;

// `build_plan` の結果。生成計画そのものに加え、件数の報告に使う内訳を持つ。
struct 解決結果 {
    plan: GenerationPlan,
    動的schemaの宣言数: usize,
    静的schemaの宣言数: usize,
    静的instanceの宣言数: usize,
    解析したファイル件数: usize,
}

impl 解決結果 {
    fn 内訳を表示する(&self) -> String {
        format!(
            "dynamic schema {}件、static schema {}件、static instance {}件",
            self.動的schemaの宣言数, self.静的schemaの宣言数, self.静的instanceの宣言数
        )
    }
}

// 全schema宣言から `解決結果` を組み立てる。
//
// パッケージ内の全ファイルを1回ずつ構文解析し (解析できなければ違反にする)、
// 動的グラフのschemaはその場で計画へ積む。静的グラフは、全ファイルの
// マクロ呼び出しを集め終えてから (1) `static_graph_schema!` の名簿を作り
// (2) 名簿を使ってinstanceを解決する、という2段階を踏む。名簿が全部品
// (パッケージ内の全静的schema) を持つまで instance を解決しないのはこの
// ためである。
fn build_plan(tree: &GenerationTree) -> Result<解決結果, Box<dyn Error>> {
    let mut plan = GenerationPlan::new();
    let sources = tree.schema_source_files()?;

    let mut files: Vec<FileMacros> = Vec::with_capacity(sources.len());
    let mut 動的schemaの宣言数 = 0;
    for source in &sources {
        let (display_path, parsed_file) = source.parse(tree)?;
        let calls = collect_macro_calls(&parsed_file);
        let 動的schemaを積む前の宣言数 = plan.declaration_count();
        source.collect_dynamic_into(tree, &display_path, &calls, &mut plan)?;
        動的schemaの宣言数 += plan.declaration_count() - 動的schemaを積む前の宣言数;
        let target = source.cargo_target(tree);
        files.push(FileMacros { source, display_path, calls, target });
    }

    let mut 名簿ビルダー = 静的schema名簿ビルダー::default();
    for file in &files {
        for call in file.calls.iter().filter(|call| call.name == "static_graph_schema") {
            名簿ビルダー.追加する(tree, file.source, &file.display_path, &file.target, call, &mut plan)?;
        }
    }
    let 名簿 = 名簿ビルダー.完成する();
    let 静的schemaの宣言数 = 名簿.len();

    let 静的instanceの宣言数 = instanceを解決する(tree, &files, &名簿, &mut plan)?;
    埋め込まれたinstanceを検査する(&files, &名簿)?;

    Ok(解決結果 {
        plan,
        動的schemaの宣言数,
        静的schemaの宣言数,
        静的instanceの宣言数,
        解析したファイル件数: sources.len(),
    })
}

// `generate` 相当: 期待する生成ファイルを更新する。
//
// 何件の宣言を読み何件書いたかを必ず1行で表示する。表示しないと、宣言が0件の
// パッケージ (置き場所を間違えた・拡張子を間違えた) でも無言で成功したように
// 見え、生成されていないことに気付けない。
pub fn generate(tree: &GenerationTree) -> Result<(), Box<dyn Error>> {
    let 結果 = build_plan(tree)?;
    let written = 結果.plan.write_stale_files(tree)?;
    println!(
        "{}、生成 {written}件 (解析したファイル {}件)",
        結果.内訳を表示する(),
        結果.解析したファイル件数
    );
    Ok(())
}

// `generate --check` 相当: 差分と孤児生成ファイルをエラーにする。
//
// 差分が無ければ、読んだ宣言の件数を1行で表示する。`generate` と同じ理由で、
// 対象が0件のまま成功したことを黙って通さない。
pub fn verify(tree: &GenerationTree) -> Result<(), Box<dyn Error>> {
    let 結果 = build_plan(tree)?;
    結果.plan.verify(tree)?;
    let count = 結果.plan.declaration_count();
    println!(
        "{}、最新 {count}件 (解析したファイル {}件)",
        結果.内訳を表示する(),
        結果.解析したファイル件数
    );
    Ok(())
}
