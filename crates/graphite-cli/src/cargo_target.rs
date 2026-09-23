//! ファイルが属する Cargo target の識別子。
//!
//! `static_graph_schema!`/instance の名前解決は、パッケージ全体ではなく
//! この単位で閉じる (issue #46 前段)。
//!
//! 裁定 (2026-09-24): `src/` 配下は`lib.rs`・
//! `main.rs`・`bin/*.rs`をそれぞれ別のCargo targetとして扱う (Cargoの自動
//! target発見規則と同じ区別。1つに束ねると、同じパッケージのlibとbinが
//! 同名のschemaを持てなくなるという実害がある、`module_graph`のモジュール
//! doc参照)。区別には`mod`宣言 (`#[path]`込み) を各rootから辿る
//! (`module_graph::srcのCargoターゲット表を求める`)。`tests/` 配下は
//! 最初の1階層 (`tests/foo.rs`・`tests/foo/` のどちらも "foo") ごとに別の
//! targetとして近似し、こちらはパスの形だけで判定できるためファイルを
//! 読まない。この近似は `tests/共通ヘルパー/mod.rs` のように複数のtest
//! 実行ファイルへ `#[path]`/`mod` で読み込まれる補助ファイルを正しく
//! 追跡できないが、生成器はRustのmodule解決を再実装しない方針
//! (`docs/code_generation.md` 「宣言と配線」) を優先し、この既知の制約を
//! 許容する。

use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::module_graph;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CargoTarget(String);

impl CargoTarget {
    pub(crate) fn 表示(&self) -> &str {
        &self.0
    }

    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

// `scan_roots` ( `[パッケージ]/src` と `[パッケージ]/tests` ) のうち
// `path` が属する走査開始点を求め、その種類に応じてCargo targetを求める。
// どの走査開始点にも属さない場合・`src`配下で`mod`のどの根の木からも
// 辿れない場合は、判定不能を黙って1つのtargetへ束ねずエラーにする。
//
// `Cargo`はツール名の固有名詞であり訳さない (プロジェクト全体の表記に合わせる)。
// 識別子中の大文字がRustのsnake_case規約検査に引っかかるため許可する。
#[allow(non_snake_case)]
pub(crate) fn ファイルの属するCargoターゲットを求める(
    scan_roots: &[std::path::PathBuf],
    path: &Path,
) -> Result<CargoTarget, Box<dyn Error>> {
    let root = scan_roots.iter().find(|root| path.strip_prefix(root).is_ok()).ok_or_else(|| {
        format!("{}: 走査開始点 (src・tests) のどれにも属しません。Cargo targetを判定できません", path.display())
    })?;
    let root_name = root
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("{}: 走査開始点のディレクトリ名をUTF-8として読めません", root.display()))?;
    if root_name == "tests" {
        let relative = path.strip_prefix(root).expect("直前のfindでstrip_prefixが成功することを確かめた");
        return Ok(tests配下のtargetを求める(relative));
    }
    let 表 = module_graph::srcのCargoターゲット表を求める(root)?;
    表.get(path).cloned().ok_or_else(|| {
        format!(
            "{}: `lib.rs`・`main.rs`・`bin/*.rs` のどの `mod` 木からも辿り着けません。`mod` 宣言で到達できるようにするか、このファイルを削除してください",
            path.display()
        )
        .into()
    })
}

fn tests配下のtargetを求める(relative: &Path) -> CargoTarget {
    let first = relative
        .components()
        .next()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .unwrap_or_default();
    let stem = first.strip_suffix(".rs").unwrap_or(&first).to_string();
    CargoTarget::new(format!("tests/{stem}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn roots() -> Vec<PathBuf> {
        vec![PathBuf::from("/repo/src"), PathBuf::from("/repo/tests")]
    }

    #[test]
    fn tests直下のファイルはファイルごとに別targetになる() {
        let a = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/foo.rs")).unwrap();
        let b = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/bar.rs")).unwrap();
        assert_eq!(a.表示(), "tests/foo");
        assert_eq!(b.表示(), "tests/bar");
        assert_ne!(a, b);
    }

    #[test]
    fn tests配下のサブディレクトリは先頭の階層名でまとまる() {
        let a = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/foo/helper.rs")).unwrap();
        let b = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/foo.rs")).unwrap();
        assert_eq!(a.表示(), "tests/foo");
        assert_eq!(a, b);
    }

    #[test]
    fn どの走査開始点にも属さないパスはエラーになる() {
        let error = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/other/x.rs")).err().unwrap();
        assert!(error.to_string().contains("走査開始点"));
    }
}
