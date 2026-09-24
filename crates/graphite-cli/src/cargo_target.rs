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
//!
//! `src` 配下のtarget表は `module_graph::srcのCargoターゲット表を求める`が
//! `src`全体のファイルを読み構文解析して1回で組み立てる。呼び出しのたびに
//! 作り直すとパッケージ内のファイル数の2乗に比例する読み取り・構文解析が
//! 発生するため、`GenerationTree`が持つ`src_target_cache`が1回だけ作って
//! 保持し、以降の`ファイルの属するCargoターゲットを求める`呼び出しは
//! この保持済みの表を引くだけにする。

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::module_graph;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CargoTarget(String);

// `src`配下のtarget表のキャッシュ。`GenerationTree`が1件保持し、同じ
// `GenerationTree`から呼ぶ`cargo_target`呼び出し全体で使い回す。
pub(crate) type SrcTargetCache = RefCell<Option<BTreeMap<PathBuf, CargoTarget>>>;

pub(crate) fn 空のキャッシュ() -> SrcTargetCache {
    RefCell::new(None)
}

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
// どの走査開始点にも属さない場合は、判定不能を黙って1つのtargetへ束ねずに
// エラーにする。`src`配下のtarget表は`cache`が保持し、1つの`GenerationTree`
// に属する呼び出しの間は`module_graph::srcのCargoターゲット表を求める`を
// 呼び直さない (呼び出しのたびに作り直すと、パッケージ内のファイル数の
// 2乗に比例する読み取り・構文解析が発生する)。
//
// `Cargo`はツール名の固有名詞であり訳さない (プロジェクト全体の表記に合わせる)。
// 識別子中の大文字がRustのsnake_case規約検査に引っかかるため許可する。
#[allow(non_snake_case)]
pub(crate) fn ファイルの属するCargoターゲットを求める(
    scan_roots: &[std::path::PathBuf],
    path: &Path,
    cache: &SrcTargetCache,
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
    let mut キャッシュ = cache.borrow_mut();
    if キャッシュ.is_none() {
        *キャッシュ = Some(module_graph::srcのCargoターゲット表を求める(root)?);
    }
    // 直前の`is_none`チェックで`Some`にしたので、この`unwrap`は必ず成功する。
    キャッシュ.as_ref().unwrap().get(path).cloned().ok_or_else(|| {
        format!(
            "{}: `lib.rs`・`main.rs`・`bin/*.rs` のどの `mod` 木からも辿り着けません。`mod` 宣言で到達できるようにしてください",
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
mod tests;
