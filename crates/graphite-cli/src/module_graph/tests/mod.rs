// `srcのCargoターゲット表を求める`の単体試験を担当ごとに分ける
// (`root_separation`はlib/main/binのtarget分離、`mod_resolution`は
// `mod`宣言・`#[path]`属性の解決と孤立ファイルの検出)。両方が使う
// フィクスチャ組み立て (`一時srcで試す`) だけをこのファイルに置く。
// 実際のファイルシステムへ一時ディレクトリを作り、`mod`宣言込みの最小src
// ツリーを書いてから読む (mod-graph探索はファイル読み取りを要るため、
// パスだけの純粋関数では検査できない)。テスト後に一時ディレクトリを
// 削除する。

mod mod_resolution;
mod root_separation;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::srcのCargoターゲット表を求める;

static 連番: AtomicUsize = AtomicUsize::new(0);

pub(super) fn 一時srcで試す(
    ファイル群: &[(&str, &str)],
    試す: impl FnOnce(&Path, Result<BTreeMap<PathBuf, crate::cargo_target::CargoTarget>, Box<dyn std::error::Error>>),
) {
    let dir = std::env::temp_dir().join(format!(
        "graphite_module_graph_test_{}_{}",
        std::process::id(),
        連番.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&dir);
    for (相対パス, 内容) in ファイル群 {
        let path = dir.join(相対パス);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, 内容).unwrap();
    }
    let 結果 = srcのCargoターゲット表を求める(&dir);
    試す(&dir, 結果);
    fs::remove_dir_all(&dir).unwrap();
}
