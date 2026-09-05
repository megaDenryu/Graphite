use std::path::{Path, PathBuf};

// 検査済みの生成先パス (`generated/<名前>.rs` の形式を満たす絶対パス)。
//
// `GenerationPlan` はこの型をキーと引数に使い、裸の `PathBuf` を引き回さない。
// 生成先の絶対パスをどう組み立てるかは `SchemaSourceFile::generated_target`
// (絶対パスへの変換の唯一の入口) と、実在ファイルの発見箇所
// (`GenerationTree::existing_generated_files`) の2箇所に閉じる。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneratedTargetPath(PathBuf);

impl GeneratedTargetPath {
    // 絶対パスへ組み立てた後の生成先を包む。
    //
    // 呼び出し元 (`schema_source_file`・`generation_tree`) が既に
    // `generated/<名前>.rs` の形式を検査済みであることを前提とする
    // (このモジュール自身は検査しない — 検査は `graphite_codegen::validate_generated_relative_path`
    // に1箇所だけ置く)。
    pub(crate) fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}
