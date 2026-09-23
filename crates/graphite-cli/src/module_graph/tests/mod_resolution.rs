// `mod`宣言 (`#[path]`込み) の解決と、どの根からも辿れないファイルの検出の
// 単体試験 (`module_graph`本体の`mod宣言を辿る`・`orphan_check`が対象)。

use super::一時srcで試す;

#[test]
fn path属性で指したファイルとその先の入れ子modを解決する() {
    一時srcで試す(
        &[
            ("lib.rs", "#[path = \"仕組み/mod.rs\"] mod 仕組み;"),
            ("仕組み/mod.rs", "#[path = \"辺.rs\"] mod 辺モジュール;"),
            ("仕組み/辺.rs", "pub struct 辺;"),
        ],
        |dir, 結果| {
            let 表 = 結果.unwrap();
            let lib = 表.get(&dir.join("lib.rs")).unwrap();
            let mod_rs = 表.get(&dir.join("仕組み/mod.rs")).unwrap();
            let edge_rs = 表.get(&dir.join("仕組み/辺.rs")).unwrap();
            assert_eq!(lib, mod_rs);
            assert_eq!(lib, edge_rs);
        },
    );
}

#[test]
fn インラインmodの中のpath属性はmod名を積み重ねた位置を基準にする() {
    // Rust referenceの規則: インラインmod `mod a { .. }` の中に書いた
    // `#[path]`は、宣言が物理的に書かれているファイルのディレクトリでは
    // なく、`a`という名前を積み重ねた仮想ディレクトリを基準に解決する
    // (`module_graph`本体の`mod宣言を辿る`参照)。この基準を物理ファイルの
    // ディレクトリのまま解決すると、`src/a/x.rs`を探すべきところを
    // `src/x.rs`で探してしまい、実在するファイルを孤立ファイル扱いにする。
    一時srcで試す(
        &[
            ("lib.rs", "mod a { #[path = \"x.rs\"] pub mod b; }"),
            ("a/x.rs", "pub struct 辺;"),
        ],
        |dir, 結果| {
            let 表 = 結果.unwrap();
            let lib = 表.get(&dir.join("lib.rs")).unwrap();
            let x = 表.get(&dir.join("a/x.rs")).unwrap();
            assert_eq!(lib, x);
        },
    );
}

#[test]
fn どの根からも辿れないファイルは違反になる() {
    一時srcで試す(
        &[("lib.rs", "pub fn 甲() {}"), ("orphan.rs", "pub fn 迷子() {}")],
        |_dir, 結果| {
            let error = 結果.err().unwrap();
            assert!(error.to_string().contains("orphan.rs"));
        },
    );
}
