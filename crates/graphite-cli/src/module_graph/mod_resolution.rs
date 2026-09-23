//! `mod`宣言1件を、それが指す子ファイルとその子ファイル自身の`mod`解決基準
//! ディレクトリへ変換する。`module_graph`本体のBFSはこのファイルの関数だけを
//! 呼び、ファイルシステムの探索順序 (`foo.rs`優先、次に`foo/mod.rs`) と
//! `#[path]`属性の扱いをこの1箇所へ閉じる。

use std::error::Error;
use std::path::{Path, PathBuf};

use syn::{Attribute, Expr, Lit, Meta};

// `mod名;`が指す子ファイルの絶対パスと、その子ファイルが自分の`mod`を解決する
// ときの基準ディレクトリ (`mod_dir`) を返す。`path_attr`は`#[path = "..."]`が
// あればその文字列。Rustの仕様では`#[path]`は`mod_dir`ではなく「宣言が書かれ
// ている物理ファイル自身のディレクトリ」(`物理ディレクトリ`) から解決する
// (`module_graph`の`mod宣言を辿る`参照)。
pub(super) fn 子ファイルを解決する(
    mod_dir: &Path,
    物理ディレクトリ: &Path,
    名前: &str,
    path_attr: Option<&str>,
) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
    if let Some(相対) = path_attr {
        let ファイル = 物理ディレクトリ.join(相対);
        if !ファイル.is_file() {
            return Err(format!(
                "{}: `#[path = \"{相対}\"]` の参照先ファイルがありません",
                ファイル.display()
            )
            .into());
        }
        let 基準 = ファイル.parent().unwrap_or(mod_dir).to_path_buf();
        return Ok((ファイル, 基準));
    }

    let ファイル形式 = mod_dir.join(format!("{名前}.rs"));
    if ファイル形式.is_file() {
        return Ok((ファイル形式, mod_dir.join(名前)));
    }
    let ディレクトリ形式 = mod_dir.join(名前).join("mod.rs");
    if ディレクトリ形式.is_file() {
        return Ok((ディレクトリ形式, mod_dir.join(名前)));
    }
    Err(format!(
        "{}: `mod {名前};` の参照先が見つかりません(`{名前}.rs`・`{名前}/mod.rs`のどちらも無し)",
        mod_dir.display()
    )
    .into())
}

// `#[path = "..."]`属性から文字列リテラルを取り出す。無ければ`None`。
pub(super) fn path属性の値(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        let Meta::NameValue(名前と値) = &attr.meta else { return None };
        if !名前と値.path.is_ident("path") {
            return None;
        }
        let Expr::Lit(リテラル式) = &名前と値.value else { return None };
        let Lit::Str(文字列) = &リテラル式.lit else { return None };
        Some(文字列.value())
    })
}
