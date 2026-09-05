//! `cargo xtask generate [--check]`・`cargo xtask check-external`・
//! `cargo xtask check-docs`・`cargo xtask check-doc-comments`・
//! `cargo xtask check-line-counts` のコマンドライン入口。
//!
//! 実処理は `lib.rs` (`xtask` ライブラリ) に集約し、使い方の説明文は `usage.rs`
//! が持つ。ここは引数解析とプロセス終了コードだけを担う。

use std::env;
use std::error::Error;

use xtask::RepositoryRoot;

use crate::usage::USAGE;

mod usage;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

// `generate` は生成ファイルを更新し、`generate --check` は差分をエラーにし、
// `check-external` は外部 crate からの生成経路を実走で検査し、`check-docs` は
// 文書参照と索引を検査し、`check-doc-comments` は doc コメントの網羅と撤去を、
// `check-line-counts` は1ファイル100行の原則と例外台帳を検査する。
enum Command {
    Generate,
    Check,
    CheckExternalCrate,
    CheckDocuments,
    CheckDocComments,
    CheckLineCounts,
}

impl Command {
    fn from_arguments(arguments: &[String]) -> Result<Self, Box<dyn Error>> {
        match arguments {
            [command] if command == "generate" => Ok(Self::Generate),
            [command, option] if command == "generate" && option == "--check" => Ok(Self::Check),
            [command] if command == "check-external" => Ok(Self::CheckExternalCrate),
            [command] if command == "check-docs" => Ok(Self::CheckDocuments),
            [command] if command == "check-doc-comments" => Ok(Self::CheckDocComments),
            [command] if command == "check-line-counts" => Ok(Self::CheckLineCounts),
            _ => Err(USAGE.into()),
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let command = Command::from_arguments(&arguments)?;

    let root = RepositoryRoot::from_current_directory()?;
    match command {
        Command::Generate => xtask::generate(&root),
        Command::Check => xtask::verify(&root),
        Command::CheckExternalCrate => xtask::check_external_crate(&root),
        Command::CheckDocuments => xtask::check_documents(&root),
        Command::CheckDocComments => xtask::check_doc_comments(&root),
        Command::CheckLineCounts => xtask::check_line_counts(&root),
    }
}
