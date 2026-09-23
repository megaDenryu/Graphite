//! 静的グラフのschema・instanceの2段階の解決 (issue #41 段階3 §2)。
//!
//! 第1段階でパッケージ内の全ファイルの全マクロ呼び出しを1回ずつ見て、
//! `static_graph_schema!` から「静的schema名簿」を作る (`schema_registry`。
//! 名前の重複はここで検出する)。第2段階で、名簿に名前が無い残りの呼び出し
//! のうち、名簿の名前と一致するものをinstanceとみなして解決する
//! (`instance_resolution`)。全部品 (名簿) が揃ってからinstanceを解決する
//! ため、名簿の組み立てはビルダー (`静的schema名簿ビルダー`) にする。
//! `embedded_detection` は、名簿の名前が他のマクロの入力の中に埋め込まれて
//! 書かれていないかを別に検査する。

mod embedded_detection;
mod instance_resolution;
mod schema_registry;
#[cfg(test)]
mod tests;

pub(crate) use embedded_detection::埋め込まれたinstanceを検査する;
pub(crate) use instance_resolution::instanceを解決する;
pub(crate) use schema_registry::静的schema名簿ビルダー;

use crate::cargo_target::CargoTarget;
use crate::schema_macro_collector::MacroCall;
use crate::schema_source_file::SchemaSourceFile;

// ファイル1件分の、静的解決に必要な材料 (動的グラフの解決と共有する
// `parse`/`collect_macro_calls` の結果)。`target` はこのファイルが属する
// Cargo target (`cargo_target` 参照)。静的schemaの名簿とinstanceの照合は
// targetの中だけで閉じる。
pub(crate) struct FileMacros<'a> {
    pub(crate) source: &'a SchemaSourceFile,
    pub(crate) display_path: String,
    pub(crate) calls: Vec<MacroCall>,
    pub(crate) target: CargoTarget,
}
