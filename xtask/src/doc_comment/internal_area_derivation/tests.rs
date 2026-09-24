use crate::doc_comment::temporary_repository::TemporaryRepository;
use crate::inspected_area::InspectedArea;
use crate::repository_root::RepositoryRoot;

use super::classify_packages;

#[test]
fn verification配下の2件目のパッケージも内部領域として分類する() {
    let repository = TemporaryRepository::new("internal-area-derivation-test");
    repository.write_package("crates/pkg-a", "");
    repository.write_package("verification/pkg-b", "publish = false\n");
    repository.write_package("verification/pkg-c", "publish = false\n");
    repository.write_package("xtask", "publish = false\n");

    let root = RepositoryRoot::at(repository.path().to_path_buf())
        .expect("疑似リポジトリを読み取れること");
    let classification = classify_packages(&root).expect("分類できること");

    let internal_spellings: Vec<&str> = classification
        .internal_areas()
        .iter()
        .map(InspectedArea::spelling)
        .collect();
    assert!(internal_spellings.contains(&"verification/pkg-b"));
    assert!(internal_spellings.contains(&"verification/pkg-c"));

    let public_spellings: Vec<&str> = classification
        .public_areas()
        .iter()
        .map(InspectedArea::spelling)
        .collect();
    assert!(public_spellings.contains(&"crates/pkg-a"));
}
