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

fn tree() -> GenerationTree {
    GenerationTree::new(
        PathBuf::from("/repo"),
        vec![PathBuf::from("/repo/src"), PathBuf::from("/repo/tests")],
    )
}

fn source() -> SchemaSourceFile {
    SchemaSourceFile::new(PathBuf::from("/repo/src/main.rs"))
}

fn target(tree: &GenerationTree, source: &SchemaSourceFile) -> CargoTarget {
    source.cargo_target(tree)
}

fn call(name: &str, tokens: proc_macro2::TokenStream, line: usize) -> MacroCall {
    MacroCall { name: name.to_string(), tokens, line }
}
