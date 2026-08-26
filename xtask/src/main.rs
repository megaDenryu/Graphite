//! `cargo xtask generate [--check]`・`cargo xtask check-external`・
//! `cargo xtask check-docs` のコマンドライン入口。
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
/// `check-external` は外部 crate からの生成経路を実走で検査し、`check-docs` は
/// 文書参照と索引を検査する。
enum Command {
    Generate,
    Check,
    CheckExternalCrate,
    CheckDocuments,
}

/// 使い方と、`check-docs` が検査しないことの説明。
///
/// 検査の限界を書いておかないと、通ったことを「文書の内容が正しい」と読み違える。
const USAGE: &str = "\
使い方: リポジトリルートで次のいずれかを実行してください
  cargo xtask generate            生成ファイルを更新する
  cargo xtask generate --check    生成ファイルの差分と孤児をエラーにする
  cargo xtask check-external      ワークスペースの外の検証用パッケージで、生成の差分検査とビルドとテストを実行する
  cargo xtask check-docs          文書参照の綴りの実在と docs/README.md 索引の網羅を検査する

check-docs が検査するもの (crates・examples・xtask・verification 配下の Rust
ソースを指す参照): ファイルの実在と、行番号 (範囲なら終了行) がその実ファイルの
行数に収まっていること。ただし docs/history 配下 (ログ型の歴史文書) は除く。
歴史文書は当時の綴りをそのまま保存する運用であり、ファイル移動や行の増減で
参照が腐っても現在の実体へ追随させない。ワイルドカード (`*`) やプレースホルダ
(`<名前>` 等) を含む綴りは「該当するファイル群」を総称する散文とみなし、
個別ファイルへの参照として検査しない。

check-docs が検査しないもの:
  節番号とアンカーの実在
  ソース参照が引用している本文とコードの一致 (generate --check が別に担当する)
  外部URL
  `../` で始まる別リポジトリの文書 (件数だけ報告する)
  docs/history 配下のソース参照 (歴史文書の当時の綴りは是正しない)
  ワイルドカード・プレースホルダを含むソースらしき綴り (ファイル群の総称として扱う)";

impl Command {
    fn from_arguments(arguments: &[String]) -> Result<Self, Box<dyn Error>> {
        match arguments {
            [command] if command == "generate" => Ok(Self::Generate),
            [command, option] if command == "generate" && option == "--check" => Ok(Self::Check),
            [command] if command == "check-external" => Ok(Self::CheckExternalCrate),
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
        Command::CheckExternalCrate => xtask::check_external_crate(&root),
        Command::CheckDocuments => xtask::check_documents(&root),
    }
}
