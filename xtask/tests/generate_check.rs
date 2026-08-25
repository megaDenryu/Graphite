//! `cargo xtask generate --check` 相当を `cargo test` からも検査する統合テスト。
//!
//! xtaskを手動で叩かない限り、生成ファイルのヘッダに埋め込んだ元DSLの行番号が
//! ずれても (schema宣言の前後に行を足し引きしても) 誰も気付けない。このテストを
//! `cargo test --workspace` に含めることで、CIの通常のテスト実行だけで陳腐化を
//! 検出できるようにする。

use std::path::PathBuf;

#[test]
fn 生成ファイルは全て最新である() {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let root = xtask::RepositoryRoot::at(repository_root)
        .expect("xtaskの実行場所からリポジトリルートを解決できること");
    xtask::verify(&root).expect(
        "生成ファイルが古いか孤立しています。リポジトリルートで `cargo xtask generate` を実行してください",
    );
}
