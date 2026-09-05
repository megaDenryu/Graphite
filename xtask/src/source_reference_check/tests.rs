use std::path::PathBuf;

use super::{InvalidSourceReferences, SourceCodeReference, UnparsableSourceReference, Violation};
use crate::document_reference::ReferenceOrigin;
use crate::repository_root::RepositoryRoot;
use crate::source_reference::SourceReference;

fn root() -> RepositoryRoot {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    RepositoryRoot::at(repository_root)
        .expect("xtaskの実行場所からリポジトリルートを解決できること")
}

fn reference(token: &str) -> SourceCodeReference {
    let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
    SourceCodeReference::new(origin, SourceReference::parse(token).unwrap())
}

#[test]
fn 実在し行数内のソース参照は違反にしない() {
    let root = root();
    assert!(reference("xtask/src/main.rs").evaluate(&root).is_none());
}

#[test]
fn 実在しないファイルは違反になる() {
    let root = root();
    let reference = reference("xtask/src/存在しない.rs");
    let violation = reference.evaluate(&root).expect("違反になること");
    let invalid = InvalidSourceReferences::new(vec![violation]);
    assert!(invalid.to_string().contains("ファイルが実在しません"));
}

#[test]
fn 行数を超える指定は違反になる() {
    let root = root();
    let reference = reference("xtask/src/source_reference_check.rs:999999");
    let violation = reference.evaluate(&root).expect("違反になること");
    let invalid = InvalidSourceReferences::new(vec![violation]);
    assert!(invalid.to_string().contains("行までです"));
}

#[test]
fn 解析できないソース参照は違反として表示される() {
    let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
    let unparsable = UnparsableSourceReference::new(origin, "xtask/src/lib.rs:0".to_string());
    let invalid = InvalidSourceReferences::new(vec![Violation::Unparsable(&unparsable)]);
    assert!(invalid.to_string().contains("解析できないソース参照です"));
}
