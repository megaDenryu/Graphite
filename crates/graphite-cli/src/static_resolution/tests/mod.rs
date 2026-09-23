// `static_resolution` の単体試験のフィクスチャを共有する module本体
// (1ファイル100行の原則。`schema_registry`・`instance_resolution` それぞれ
// 別の責務を検査する独立したファイルへ分ける)。

mod embedded_detection;
mod instance_resolution;
mod instance_resolution_target_scope;
mod schema_registry;

use std::path::PathBuf;

use super::*;
use crate::cargo_target::CargoTarget;
use crate::generation_plan::GenerationPlan;
use crate::generation_tree::GenerationTree;

// Cargo targetの判定は`src`配下について実ファイルの`mod`宣言を辿る
// (`module_graph`) ため、このフィクスチャは実在するこのパッケージ自身
// (`graphite-cli`) のディレクトリを基準にする。`crates/graphite-cli/src/
// main.rs`は`mod`宣言を持たないので、`lib.rs`の木とは別targetになる
// (`schema_registry.rs`等の「別のcargo_targetなら同名のschemaを許す」の
// 検査対象そのもの)。`tests`側はパスの形だけで判定でき実在しなくてよい。
fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn tree() -> GenerationTree {
    let base = manifest_dir();
    GenerationTree::new(base.clone(), vec![base.join("src"), base.join("tests")])
}

fn source() -> SchemaSourceFile {
    SchemaSourceFile::new(manifest_dir().join("src").join("main.rs"))
}

fn target(tree: &GenerationTree, source: &SchemaSourceFile) -> CargoTarget {
    source.cargo_target(tree).unwrap()
}

fn call(name: &str, tokens: proc_macro2::TokenStream, line: usize) -> MacroCall {
    MacroCall { name: name.to_string(), tokens, line }
}
