//! `src_root`配下の全`.rs`ファイルを集め、`mod`のどの根からも辿れなかった
//! ファイルをエラーとして報告する (検査器は解析できなかった入力を黙って
//! 対象から外してはならない、というリポジトリ全体の規約の適用)。除外する
//! ディレクトリ名は`generation_tree::collect_rust_files`と同じ3つ
//! (`target`・`generated`・`ui`) である。

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cargo_target::CargoTarget;

pub(super) fn 辿れなかったファイルを検査する(
    src_root: &Path,
    到達表: &BTreeMap<PathBuf, CargoTarget>,
) -> Result<(), Box<dyn Error>> {
    let mut 全ファイル = Vec::new();
    集める(src_root, &mut 全ファイル)?;
    let 孤立: Vec<&PathBuf> = 全ファイル.iter().filter(|path| !到達表.contains_key(*path)).collect();
    if 孤立.is_empty() {
        return Ok(());
    }
    let 一覧 = 孤立.iter().map(|path| path.display().to_string()).collect::<Vec<_>>().join("\n  ");
    Err(format!(
        "`lib.rs`・`main.rs`・`bin/*.rs` のどの `mod` 木からも辿り着けない `src/` 配下のファイルがあります。`mod` 宣言で到達できるようにするか、削除してください:\n  {一覧}"
    )
    .into())
}

fn 集める(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))? {
        let path = entry.map_err(|error| format!("{}: {error}", directory.display()))?.path();
        if path.is_dir() {
            if matches!(path.file_name().and_then(OsStr::to_str), Some("target" | "generated" | "ui")) {
                continue;
            }
            集める(&path, files)?;
        } else if path.extension() == Some(OsStr::new("rs")) {
            files.push(path);
        }
    }
    Ok(())
}
