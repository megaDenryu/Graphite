//! `cargo xtask check-doc-comments` 相当を `cargo test` からも検査する統合テスト。
//!
//! doc コメントの網羅と撤去は、どちらもコンパイラが検査しない。生成コードの
//! 公開面は利用者の rustdoc に出るのに、doc を書き忘れても何も言われない。
//! 内部領域の `///` は誰にも読まれないまま増える。このテストを
//! `cargo test --workspace` に含めることで、通常のテスト実行で両方を検出できる
//! ようにする。

use std::path::PathBuf;

#[test]
fn 公開面のdocコメントは網羅され内部領域には1件も無い() {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let root = xtask::RepositoryRoot::at(repository_root)
        .expect("xtaskの実行場所からリポジトリルートを解決できること");
    xtask::check_doc_comments(&root).expect(
        "生成コードの公開面に doc コメントが欠けているか、内部領域に項目の `///` が残っています。リポジトリルートで `cargo xtask check-doc-comments` を実行してください",
    );
}
