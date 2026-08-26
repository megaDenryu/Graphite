//! `cargo graphite generate [--check]` のコマンドライン入口。
//!
//! 実処理は `lib.rs` (`graphite-cli` ライブラリ) に集約する。ここは引数解析と
//! プロセス終了コードだけを担う。Graphite リポジトリ自身の開発では、同じ生成
//! コアをワークスペース全体の走査で呼ぶ `cargo xtask generate` を使う。

use std::env;
use std::error::Error;

use graphite_cli::PackageRoot;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

/// `generate` は生成ファイルを更新し、`generate --check` は差分をエラーにする。
enum Command {
    Generate,
    Check,
}

/// 使い方と、走査する範囲の説明。
///
/// 走査範囲を書いておかないと、宣言を置いた場所が対象外で1件も生成されない
/// ときに、原因が実行場所なのか宣言なのか読み手が決められない。
const USAGE: &str = "\
使い方: 生成したいパッケージのディレクトリで次のいずれかを実行してください
  cargo graphite generate          生成ファイルを更新する
  cargo graphite generate --check  生成ファイルの差分と孤児をエラーにする

走査する範囲:
  パッケージ直下の src と tests の配下にある Rust ファイル
  (generated・ui・target の各ディレクトリは対象外)";

impl Command {
    fn from_arguments(arguments: &[String]) -> Result<Self, Box<dyn Error>> {
        match arguments {
            [command] if command == "generate" => Ok(Self::Generate),
            [command, option] if command == "generate" && option == "--check" => Ok(Self::Check),
            _ => Err(USAGE.into()),
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let command = Command::from_arguments(&command_arguments())?;
    let package = PackageRoot::from_current_directory()?;
    println!("対象パッケージ: {}", package.display());
    match command {
        Command::Generate => graphite_cli::generate(package.generation_tree()),
        Command::Check => graphite_cli::verify(package.generation_tree()),
    }
}

/// 実行ファイル名を除いた引数を、cargo サブコマンド形式と直接実行の両方から集める。
///
/// `cargo graphite generate` で起動されると、cargo は `cargo-graphite graphite
/// generate` という並びで実行する。先頭の `graphite` を落とさないと、直接
/// `cargo-graphite generate` と打ったときと引数の位置がずれる。
fn command_arguments() -> Vec<String> {
    let mut arguments = env::args().skip(1);
    let Some(first) = arguments.next() else {
        return Vec::new();
    };
    let rest = arguments.collect::<Vec<_>>();
    if first == "graphite" {
        return rest;
    }
    let mut all = vec![first];
    all.extend(rest);
    all
}
