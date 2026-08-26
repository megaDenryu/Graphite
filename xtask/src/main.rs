//! `cargo xtask generate [--check]` と `cargo xtask check-docs` のコマンドライン入口。
//!
//! 実処理は `lib.rs` (`xtask` ライブラリ) に集約する。ここは引数解析と
//! プロセス終了コードだけを担う。

use std::env;
use std::error::Error;

use xtask::RepositoryRoot;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

/// `generate` は生成ファイルを更新し、`generate --check` は差分をエラーにし、
/// `check-docs` は文書参照と索引を検査する。
enum Command {
    Generate,
    Check,
    CheckDocuments,
}

/// 使い方と、`check-docs` が検査しないことの説明。
///
/// 検査の限界を書いておかないと、通ったことを「文書の内容が正しい」と読み違える。
const USAGE: &str = "\
使い方: リポジトリルートで次のいずれかを実行してください
  cargo xtask generate            生成ファイルを更新する
  cargo xtask generate --check    生成ファイルの差分と孤児をエラーにする
  cargo xtask check-docs          文書参照の綴りの実在と docs/README.md 索引の網羅を検査する

check-docs が検査しないもの:
  節番号とアンカーの実在
  crates・examples 等のソースファイルを指す参照
  行番号つき引用の中身 (generate --check が担当する)
  外部URL
  `../` で始まる別リポジトリの文書 (件数だけ報告する)";

impl Command {
    fn from_arguments(arguments: &[String]) -> Result<Self, Box<dyn Error>> {
        match arguments {
            [command] if command == "generate" => Ok(Self::Generate),
            [command, option] if command == "generate" && option == "--check" => Ok(Self::Check),
            [command] if command == "check-docs" => Ok(Self::CheckDocuments),
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
        Command::CheckDocuments => xtask::check_documents(&root),
    }
}
