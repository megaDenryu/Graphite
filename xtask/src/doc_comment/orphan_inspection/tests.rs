use crate::doc_comment::internal_area_derivation::classify_packages;
use crate::doc_comment::temporary_repository::TemporaryRepository;
use crate::inspected_area::InspectedArea;
use crate::repository_root::RepositoryRoot;
use crate::rust_source::RustSource;

use super::OrphanSourceReport;

const SCAN_ROOTS: [&str; 3] = ["crates", "examples", "verification"];

fn 走査済みソースを集める(root: &RepositoryRoot) -> Vec<RustSource> {
    let mut sources = Vec::new();
    for area in SCAN_ROOTS {
        sources.extend(
            root.rust_source_files(&InspectedArea::at(area))
                .expect("走査できること"),
        );
    }
    sources
}

#[test]
fn パッケージに属さないrustソースを違反として検出する() {
    let repository = TemporaryRepository::new("orphan-inspection-test");
    repository.write_package("crates/pkg-a", "");
    repository.write_package("examples/pkg-b", "");
    repository.write_file("examples/shared/common.rs", "");
    repository.write_package("xtask", "publish = false\n");

    let root = RepositoryRoot::at(repository.path().to_path_buf())
        .expect("疑似リポジトリを読み取れること");
    let classification = classify_packages(&root).expect("分類できること");
    let known_areas = classification.known_areas();
    let sources = 走査済みソースを集める(&root);

    let report = OrphanSourceReport::inspect(sources, &known_areas);

    assert!(!report.is_clean());
    assert!(report.render().contains("examples/shared/common.rs"));
}
