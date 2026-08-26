//! `cargo xtask check-docs` 相当を `cargo test` からも検査する統合テスト。
//!
//! 文書間の参照はバッククォートで囲んだプレーンテキストであり、コンパイラも
//! rustdoc も追随を検査しない。文書を移動したり名前を変えたりした瞬間に、
//! 誰にも気付かれないまま綴りだけが取り残される。このテストを
//! `cargo test --workspace` に含めることで、通常のテスト実行で検出できる
//! ようにする。

use std::path::PathBuf;

#[test]
fn 文書参照の綴りは全て実在する() {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let root = xtask::RepositoryRoot::at(repository_root)
        .expect("xtaskの実行場所からリポジトリルートを解決できること");
    xtask::check_documents(&root).expect(
        "文書参照か docs/README.md 索引が実態と食い違っています。リポジトリルートで `cargo xtask check-docs` を実行してください",
    );
}
