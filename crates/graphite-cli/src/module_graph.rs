//! `src/`配下のCargo target (lib・main・bin) を、各ルートファイルから
//! `mod`宣言 (`#[path]`込み) を辿って求める (issue #46 前段)。
//!
//! 裁定 (2026-09-24): 生成器はRustのmodule解決を完全には再実装しない。属性
//! (`#[cfg(..)]`等) の真偽は評価せず、`mod 名前;`が構文上に存在する限り
//! 辿る (実際にそのビルドへ含まれるかは問わない、安全側の近似)。複数の根
//! から同じファイルへ到達できる場合は lib → main → bin (ファイル名昇順) の
//! 優先順で最初に見つかった根へ属させる。このリポジトリの実際のクレート
//! 構成では根どうしが専用の子moduleを共有しないため、この優先順めは現状の
//! どのファイルの帰属も変えない。`mod`から一度も辿り着けない`src/`配下の
//! ファイルは黙って束ねず、`generate`/`generate --check`を止める違反にする
//! (`docs/static_graph.md`「制約」節)。
//!
//! `tests/`配下は1ファイル1targetという既存の近似のままパスだけで判定でき
//! るため対象外 (`cargo_target`)。`examples/`・`benches/`は生成器がそもそも
//! 走査しない (`docs/code_generation.md`「宣言と配線」)。

mod mod_resolution;
mod orphan_check;
mod root_discovery;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use syn::Item;

use crate::cargo_target::CargoTarget;
use mod_resolution::{path属性の値, 子ファイルを解決する};
use root_discovery::ルート一覧を求める;

// `src_root`配下の全`.rs`ファイルについて、属するCargo targetを求める。
// `mod`のどの根の木からも辿れなかったファイルが1件でもあれば、束ねずに
// エラーにする (`orphan_check`)。`Cargo`はツール名の固有名詞であり訳さない
// (`cargo_target`のdoc参照)。
#[allow(non_snake_case)]
pub(crate) fn srcのCargoターゲット表を求める(
    src_root: &Path,
) -> Result<BTreeMap<PathBuf, CargoTarget>, Box<dyn Error>> {
    let mut 表 = BTreeMap::new();
    for (root, target) in ルート一覧を求める(src_root)? {
        let 基準 = root.parent().unwrap_or(src_root).to_path_buf();
        for ファイル in 到達ファイルを辿る(&root, &基準)? {
            表.entry(ファイル).or_insert_with(|| target.clone());
        }
    }
    orphan_check::辿れなかったファイルを検査する(src_root, &表)?;
    Ok(表)
}

// 1つの根から`mod`宣言 (`#[path]`込み) をBFSで辿り、到達した全ファイルの
// 絶対パス集合を返す。インラインの`mod 名前 { .. }`は新しいファイルを持た
// ないが、その中の`mod 内側;`を解決する基準ディレクトリ (`mod_dir`) を
// 1段深くする。`#[path]`は仕様上「宣言が書かれている物理ファイル自身の
// ディレクトリ」から解決する (`mod_dir`とは別、inline mod化しても遡って
// 物理ファイルの位置を使う)。そのため物理ファイルのパスも一緒に運ぶ。
fn 到達ファイルを辿る(root: &Path, 基準: &Path) -> Result<HashSet<PathBuf>, Box<dyn Error>> {
    let mut 到達済み = HashSet::new();
    let mut 未処理 = VecDeque::new();
    未処理.push_back((root.to_path_buf(), 基準.to_path_buf()));
    while let Some((ファイル, mod_dir)) = 未処理.pop_front() {
        if !到達済み.insert(ファイル.clone()) {
            continue;
        }
        let ソース = fs::read_to_string(&ファイル).map_err(|error| format!("{}: {error}", ファイル.display()))?;
        let 構文木 = syn::parse_file(&ソース)
            .map_err(|error| format!("{}: Rustとして解析できません: {error}", ファイル.display()))?;
        let 物理ディレクトリ = ファイル.parent().unwrap_or(&mod_dir).to_path_buf();
        mod宣言を辿る(&構文木.items, &mod_dir, &物理ディレクトリ, &mut 未処理)?;
    }
    Ok(到達済み)
}

fn mod宣言を辿る(
    items: &[Item],
    mod_dir: &Path,
    物理ディレクトリ: &Path,
    未処理: &mut VecDeque<(PathBuf, PathBuf)>,
) -> Result<(), Box<dyn Error>> {
    for item in items {
        let Item::Mod(宣言) = item else { continue };
        let 名前 = 宣言.ident.to_string();
        let path_attr = path属性の値(&宣言.attrs);
        match &宣言.content {
            Some((_, 内側の項目)) => {
                let 内側の基準 = match &path_attr {
                    Some(相対) => 物理ディレクトリ.join(相対),
                    None => mod_dir.join(&名前),
                };
                mod宣言を辿る(内側の項目, &内側の基準, 物理ディレクトリ, 未処理)?;
            }
            None => {
                let (子ファイル, 子の基準) =
                    子ファイルを解決する(mod_dir, 物理ディレクトリ, &名前, path_attr.as_deref())?;
                未処理.push_back((子ファイル, 子の基準));
            }
        }
    }
    Ok(())
}
