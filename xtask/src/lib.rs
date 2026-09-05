//! Graphite リポジトリ自身の開発用入口のライブラリ側。
//!
//! 生成そのものは `graphite-cli` が担い、ここは対象が `crates/*` と `examples/*`
//! の全パッケージであることだけを決めてそれをパッケージごとに呼ぶ。文書参照と
//! 索引の検査 (`check-docs`) はこのリポジトリ固有の検査であり、ここに置く。バイナリ (`main.rs`) は引数解析と
//! プロセス終了コードだけを担う。`xtask/tests/` の統合テストはこのライブラリを
//! 経由して `cargo xtask generate --check` 相当を `cargo test` から検査する。

mod doc_comment;
mod document_index;
mod document_reference;
mod excerpt_file_body;
mod excerpt_range_body;
mod excerpt_inspection;
mod external_verification;
mod missing_references;
mod quoted_excerpt;
mod quoted_excerpt_check;
mod reference_scan;
mod repository_package;
mod repository_root;
mod source_reference;
mod source_reference_check;

use std::error::Error;

pub use document_reference::DocumentPath;
pub use repository_package::RepositoryPackage;
pub use repository_root::RepositoryRoot;

use crate::doc_comment::DocCommentInspection;
use crate::document_index::DocumentIndex;
use crate::external_verification::ExternalVerificationPackage;
use crate::reference_scan::ReferenceScan;

// `cargo xtask generate` 相当: 期待する生成ファイルをパッケージごとに更新する。
pub fn generate(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    for package in root.generation_packages()? {
        println!("対象パッケージ: {}", package.spelling());
        graphite_cli::generate(package.generation_tree())?;
    }
    Ok(())
}

// `cargo xtask generate --check` 相当: 差分と孤児生成ファイルをエラーにする。
pub fn verify(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    for package in root.generation_packages()? {
        println!("対象パッケージ: {}", package.spelling());
        graphite_cli::verify(package.generation_tree())?;
    }
    Ok(())
}

// `cargo xtask check-external` 相当: 外部 crate からの生成経路を実走で検査する。
//
// 生成の差分検査は `cargo graphite generate --check` と同じ経路を通り、続けて
// 検証用パッケージのビルドとテストを実行する。
pub fn check_external_crate(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    ExternalVerificationPackage::new(root.external_verification_package()?).check()
}

// `cargo xtask check-docs` 相当: 文書参照の綴りが実在するかを検査する。
//
// 参照が数百件の規模になったため、目視での確認は成立しない。検査の範囲と
// 限界は `main.rs` の使い方の説明に書いてある。
pub fn check_documents(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    let scan = ReferenceScan::over(root)?;
    let existing = root.document_files()?;
    let mismatch = DocumentIndex::read_from(root)?.compare_with(&existing);
    let missing = scan.missing_targets();
    let invalid_sources = scan.invalid_source_references();
    let excerpts = scan.inspect_excerpts();
    println!(
        "文書参照 {}件・ソース参照 {}件と docs 配下の {}ファイルを検査しました\
         (別リポジトリを指す参照 {}件は検査対象外)",
        scan.reference_count(),
        scan.source_reference_count(),
        existing.len(),
        scan.external_reference_count()
    );
    println!(
        "直前のソース参照から引用として照合したコードフェンスは {}件・{}行です\
         (照合しなかった Rust コードフェンス {}件、\
         参照先が実在せず照合できなかった引用 {}件)",
        excerpts.compared_excerpt_count(),
        excerpts.compared_line_count(),
        scan.unquoted_rust_fence_count(),
        excerpts.unreadable_excerpt_count()
    );
    if missing.is_empty() && mismatch.is_empty() && invalid_sources.is_empty() && excerpts.is_empty()
    {
        return Ok(());
    }
    Err(format!("{missing}{mismatch}{invalid_sources}{excerpts}").into())
}

// `cargo xtask check-doc-comments` 相当: doc コメントの網羅と撤去を検査する。
//
// `xtask/tests/doc_comments_check.rs` がこの関数を呼ぶため、`cargo test --workspace`
// でも同じ検査が走る。検査の範囲と限界は `main.rs` の使い方の説明に書いてある。
pub fn check_doc_comments(root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
    DocCommentInspection::new(root).run()
}
