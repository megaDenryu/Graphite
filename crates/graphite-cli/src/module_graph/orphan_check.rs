//! `src_root`配下の全`.rs`ファイルを集め、`mod`のどの根からも辿れなかった
//! ファイル (孤立ファイル) を扱う。検査器は解析できなかった入力を黙って
//! 対象から外してはならない、というリポジトリ全体の規約により、孤立
//! ファイルも黙って無視はしない。ただし違反にするのは孤立ファイルの中身が
//! `generated = "...";` から始まるGraphiteの宣言 (schema・instance) を
//! 含む場合だけである (`schema_macro_collector::追跡形式らしいか`で判定する。
//! schema宣言・instance宣言はどちらもこの形を先頭に持つため、マクロ名が
//! 利用者ごとに違うinstanceも、静的schema名簿を作る前のこの時点で
//! 名前を知らずに判定できる)。`include!`で読み込むだけの純粋なデータ・
//! 補助関数のファイルのように、Graphiteの宣言を1件も含まない孤立ファイルは
//! 対象外にし、`到達表`へ仮のCargo targetを差して以降の解決を通す
//! (`docs/code_generation.md`「宣言の種類」に検査していない範囲を書く)。
//! 除外するディレクトリ名は`generation_tree::collect_rust_files`と同じ3つ
//! (`target`・`generated`・`ui`) である。

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cargo_target::CargoTarget;
use crate::schema_macro_collector::{collect_macro_calls, 追跡形式らしいか};

// 孤立ファイルのうち、Graphiteの宣言を含まない (対象外の) ものへ差す仮の
// Cargo target。実在のtarget表示 (`src/lib`等) と綴りが衝突しないよう
// 丸括弧付きにしてあり、静的schema名簿には決して現れないので、instanceの
// 照合 (`static_resolution::instance_resolution`) がこの値と実在のtargetを
// 取り違えることはない。
fn 孤立ファイルの仮target() -> CargoTarget {
    CargoTarget::new("(mod木から辿れないファイル)")
}

pub(super) fn 辿れなかったファイルを解決する(
    src_root: &Path,
    到達表: &mut BTreeMap<PathBuf, CargoTarget>,
) -> Result<(), Box<dyn Error>> {
    let mut 全ファイル = Vec::new();
    集める(src_root, &mut 全ファイル)?;
    let mut 違反 = Vec::new();
    for path in 全ファイル {
        if 到達表.contains_key(&path) {
            continue;
        }
        if 宣言らしい呼び出しを含むか(&path)? {
            違反.push(path.display().to_string());
        } else {
            到達表.insert(path, 孤立ファイルの仮target());
        }
    }
    if 違反.is_empty() {
        return Ok(());
    }
    let 一覧 = 違反.join("\n  ");
    Err(format!(
        "`lib.rs`・`main.rs`・`bin/*.rs` のどの `mod` 木からも辿り着けない `src/` 配下のファイルに、`generated = \"...\";` から始まるGraphiteの宣言 (schema・instance) があります。`mod` 宣言で到達できるようにしてください:\n  {一覧}"
    )
    .into())
}

// 孤立ファイル1件が、`dynamic_graph_schema!`・`static_graph_schema!`という
// 固定名の呼び出し、または`generated = "...";`から始まる呼び出し (instance
// 宣言を含む) を1件でも持つかを見る。ファイル自体の読み取り・構文解析が
// 失敗した場合も、検査器の規約に従い黙って対象から外さず違反として報告する。
fn 宣言らしい呼び出しを含むか(path: &Path) -> Result<bool, Box<dyn Error>> {
    let ソース = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let 構文木 = syn::parse_file(&ソース)
        .map_err(|error| format!("{}: Rustとして解析できません: {error}", path.display()))?;
    Ok(collect_macro_calls(&構文木).iter().any(|call| {
        matches!(call.name.as_str(), "dynamic_graph_schema" | "static_graph_schema") || 追跡形式らしいか(&call.tokens)
    }))
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
