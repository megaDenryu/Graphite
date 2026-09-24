//! 内部領域と公開面の判定 (issue #32)。
//!
//! この検査器は、内部領域をハードコードした一覧ではなく、走査対象の
//! パッケージそれぞれの Cargo.toml から機械的に導出する。呼び出し元は、
//! 生成コードの公開面 (`generated` ディレクトリ) の扱いをこことは別に行う。

#[cfg(test)]
mod tests;

use std::error::Error;

use super::package_manifest::PackageManifestFacts;
use crate::inspected_area::InspectedArea;
use crate::repository_root::RepositoryRoot;

// 走査対象のパッケージを、Cargo.toml から読み取った事実で内部領域と公開面へ
// 分類した結果。この型は、内部領域・公開面のどちらも綴りへ剥がさず
// `InspectedArea` のまま保持する。呼び出し元だけが、表示境界
// (`PublicPackageReport::render` 等) で綴りへ剥がしてよい。
pub(super) struct PackageClassification {
    internal: Vec<InspectedArea>,
    public: Vec<InspectedArea>,
}

impl PackageClassification {
    // 内部領域と判定されたパッケージ。呼び出し元は、この領域それぞれで
    // `///` が1件も無いことを確かめる。
    pub(super) fn internal_areas(&self) -> &[InspectedArea] {
        &self.internal
    }

    // 公開面と判定されたパッケージ。呼び出し元は、内部領域の件数と合わせて
    // パッケージの総数を突き合わせるための一覧として報告へ出す。
    pub(super) fn public_areas(&self) -> &[InspectedArea] {
        &self.public
    }

    // 内部領域・公開面のどちらかに属すると分かっているパッケージの領域。
    // 呼び出し元は、この一覧を使ってどのパッケージにも属さない Rust ソースを
    // 検出する (`OrphanSourceReport`)。
    pub(super) fn known_areas(&self) -> Vec<&InspectedArea> {
        self.internal.iter().chain(self.public.iter()).collect()
    }
}

// この関数は、走査対象のパッケージを Cargo.toml の内容で内部領域と公開面へ
// 分類する。
pub(super) fn classify_packages(
    root: &RepositoryRoot,
) -> Result<PackageClassification, Box<dyn Error>> {
    let mut internal = Vec::new();
    let mut public = Vec::new();
    for package in root.doc_comment_packages()? {
        let area = InspectedArea::at(package.spelling());
        if PackageManifestFacts::read(&package.manifest_path())?.is_internal() {
            internal.push(area);
        } else {
            public.push(area);
        }
    }
    Ok(PackageClassification { internal, public })
}
