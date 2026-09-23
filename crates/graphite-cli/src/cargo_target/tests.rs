use super::*;
use std::path::PathBuf;

fn roots() -> Vec<PathBuf> {
    vec![PathBuf::from("/repo/src"), PathBuf::from("/repo/tests")]
}

#[test]
fn tests直下のファイルはファイルごとに別targetになる() {
    let a = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/foo.rs"), &空のキャッシュ()).unwrap();
    let b = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/bar.rs"), &空のキャッシュ()).unwrap();
    assert_eq!(a.表示(), "tests/foo");
    assert_eq!(b.表示(), "tests/bar");
    assert_ne!(a, b);
}

#[test]
fn tests配下のサブディレクトリは先頭の階層名でまとまる() {
    let a = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/foo/helper.rs"), &空のキャッシュ())
        .unwrap();
    let b = ファイルの属するCargoターゲットを求める(&roots(), Path::new("/repo/tests/foo.rs"), &空のキャッシュ()).unwrap();
    assert_eq!(a.表示(), "tests/foo");
    assert_eq!(a, b);
}

#[test]
fn どの走査開始点にも属さないパスはエラーになる() {
    let error =
        ファイルの属するCargoターゲットを求める(&roots(), Path::new("/other/x.rs"), &空のキャッシュ()).err().unwrap();
    assert!(error.to_string().contains("走査開始点"));
}

#[test]
fn 同じキャッシュを使うとsrc配下の表を作り直さない() {
    // `module_graph::srcのCargoターゲット表を求める`は実在のディレクトリを
    // 要求するため、一時ディレクトリへ最小のsrcツリーを作って実測する。
    // 同じ`cache`で2回解決し、2回目もキャッシュから同じ値を返すこと
    // (2回目の呼び出しで`root`が実在しなくても解決できること) を確かめる。
    let dir = std::env::temp_dir().join(format!("graphite_cargo_target_cache_test_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("lib.rs"), "pub fn 甲() {}").unwrap();
    let scan_roots = vec![src.clone()];
    let cache = 空のキャッシュ();
    let 一回目 = ファイルの属するCargoターゲットを求める(&scan_roots, &src.join("lib.rs"), &cache).unwrap();
    assert!(cache.borrow().is_some());
    // srcディレクトリごと削除しても、キャッシュ済みなら2回目は
    // ファイルシステムへ触らずに解決できる。
    std::fs::remove_dir_all(&dir).unwrap();
    let 二回目 = ファイルの属するCargoターゲットを求める(&scan_roots, &src.join("lib.rs"), &cache).unwrap();
    assert_eq!(一回目, 二回目);
}
