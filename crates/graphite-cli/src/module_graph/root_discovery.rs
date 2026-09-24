//! `src/`配下のCargo target roots (`lib.rs`・`main.rs`・`bin/*.rs`・
//! `bin/*/main.rs`) の列挙。Cargoの自動target発見規則と同じ区別であり、
//! `[[bin]]`の明示指定 (`Cargo.toml`側の設定) は対象にしない
//! (`module_graph`本体のdoc参照)。

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cargo_target::CargoTarget;

pub(super) fn ルート一覧を求める(src_root: &Path) -> Result<Vec<(PathBuf, CargoTarget)>, Box<dyn Error>> {
    let mut 一覧 = Vec::new();
    let lib = src_root.join("lib.rs");
    if lib.is_file() {
        一覧.push((lib, CargoTarget::new("src/lib")));
    }
    let main = src_root.join("main.rs");
    if main.is_file() {
        一覧.push((main, CargoTarget::new("src/main")));
    }
    let bin_dir = src_root.join("bin");
    if bin_dir.is_dir() {
        let mut bin項目: Vec<PathBuf> = fs::read_dir(&bin_dir)
            .map_err(|error| format!("{}: {error}", bin_dir.display()))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .collect();
        bin項目.sort();
        for 項目 in bin項目 {
            if 項目.is_file() && 項目.extension() == Some(OsStr::new("rs")) {
                let 名前 = 項目.file_stem().and_then(OsStr::to_str).unwrap_or("").to_string();
                一覧.push((項目, CargoTarget::new(format!("src/bin/{名前}"))));
            } else if 項目.is_dir() {
                let main_rs = 項目.join("main.rs");
                if main_rs.is_file() {
                    let 名前 = 項目.file_name().and_then(OsStr::to_str).unwrap_or("").to_string();
                    一覧.push((main_rs, CargoTarget::new(format!("src/bin/{名前}"))));
                }
            }
        }
    }
    Ok(一覧)
}
