//! ファイルが属する Cargo target の識別子。
//!
//! `static_graph_schema!`/instance の名前解決は、パッケージ全体ではなく
//! この単位で閉じる (issue #46 前段のPR #45レビュー、B)。生成器はRustの
//! moduleの名前解決を再実装しないため、`src/` 配下 (lib・bin) を1つの
//! target、`tests/` 配下は最初の1階層 (`tests/foo.rs`・`tests/foo/` の
//! どちらも "foo") ごとに別のtargetとして近似する。この近似は
//! `tests/共通ヘルパー/mod.rs` のように複数のtest実行ファイルへ
//! `#[path]`/`mod` で読み込まれる補助ファイルを正しく追跡できないが、
//! 生成器はRustのmodule解決を再実装しない方針 (`docs/code_generation.md`
//! 「宣言と配線」) を優先し、この既知の制約を許容する。

use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CargoTarget(String);

impl CargoTarget {
    pub(crate) fn 表示(&self) -> &str {
        &self.0
    }
}

// `scan_roots` ( `[パッケージ]/src` と `[パッケージ]/tests` ) のどちらの
// 配下にあるかを見て、このファイルが属するtargetを求める。
pub(crate) fn ファイルの所属targetを求める(scan_roots: &[std::path::PathBuf], path: &Path) -> CargoTarget {
    for root in scan_roots {
        let Ok(relative) = path.strip_prefix(root) else {
            continue;
        };
        let root_name = root.file_name().and_then(OsStr::to_str).unwrap_or("");
        if root_name != "tests" {
            return CargoTarget(root_name.to_string());
        }
        let first = relative
            .components()
            .next()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .unwrap_or_default();
        let stem = first.strip_suffix(".rs").unwrap_or(&first).to_string();
        return CargoTarget(format!("tests/{stem}"));
    }
    // scan_rootsのどれとも一致しない (単体試験のfixtureがscan_rootsを省略した
    // 場合など)。全ファイルを1つのtargetへ落とし、判定不能を隠さない。
    CargoTarget("?".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn roots() -> Vec<PathBuf> {
        vec![PathBuf::from("/repo/src"), PathBuf::from("/repo/tests")]
    }

    #[test]
    fn src配下は1つのtargetにまとまる() {
        let a = ファイルの所属target求める_helper(&roots(), "/repo/src/main.rs");
        let b = ファイルの所属target求める_helper(&roots(), "/repo/src/domain/model.rs");
        assert_eq!(a.表示(), "src");
        assert_eq!(b.表示(), "src");
    }

    #[test]
    fn tests直下のファイルはファイルごとに別targetになる() {
        let a = ファイルの所属target求める_helper(&roots(), "/repo/tests/foo.rs");
        let b = ファイルの所属target求める_helper(&roots(), "/repo/tests/bar.rs");
        assert_eq!(a.表示(), "tests/foo");
        assert_eq!(b.表示(), "tests/bar");
        assert_ne!(a, b);
    }

    #[test]
    fn tests配下のサブディレクトリは先頭の階層名でまとまる() {
        let a = ファイルの所属target求める_helper(&roots(), "/repo/tests/foo/helper.rs");
        let b = ファイルの所属target求める_helper(&roots(), "/repo/tests/foo.rs");
        assert_eq!(a.表示(), "tests/foo");
        assert_eq!(a, b);
    }

    fn ファイルの所属target求める_helper(roots: &[PathBuf], path: &str) -> CargoTarget {
        ファイルの所属targetを求める(roots, Path::new(path))
    }
}
