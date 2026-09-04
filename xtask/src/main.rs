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
  cargo xtask check-docs          文書参照とリポジトリ内Rustソース参照の綴りの実在・行数範囲、docs/README.md 索引の網羅を検査する

check-docs が検査するもの (crates・examples・xtask・verification 配下の Rust
ソースを指す参照。docs/history 配下 (ログ型の歴史文書) も対象に含む): ファイルの
実在と、行番号 (範囲や `3-4, 9-11` のようなカンマ区切りの複数指定なら最大の
終了行) がその実ファイルの行数に収まっていること。`:0` や `:34-12` (開始が
終了より大きい逆転範囲) のような無効な行指定、およびワイルドカード・
プレースホルダを含まないのに解析できない綴りは「解析できないソース参照」として
検査する。ワイルドカード (`*`) やプレースホルダ (`<名前>` 等) を含む綴りだけは
「該当するファイル群」を総称する散文とみなし、個別ファイルへの参照として検査
しない。

行番号を持つソース参照が Markdown 文書に書かれ、その行の直後に (空行だけを
挟んで) コードフェンスが始まる場合は、フェンス本文の先頭3行が参照先の行範囲に
実在することも照合する。空行と省略記号 (`...`・`// ...`) だけの行は照合しない。
空白の違い・引数列の末尾コンマ・署名を `;` で打ち切った引用は一致とみなす。
カンマ区切りの複数範囲では、いずれか1つの範囲に含まれれば一致とみなす。

check-docs が検査しないもの:
  節番号とアンカーの実在
  引用本文とコードの完全な一致 (照合するのは引用の先頭3行までであり、それより
  後ろ・省略された部分・畳み込まれた空白の差は検査しない。生成物そのものの
  一致は generate --check が別に担当する)
  Rust の doc コメントに書いたコードフェンスの本文 (照合は Markdown 文書だけを対象にする)
  行番号を持たないソース参照の引用本文 (照合すべき行範囲が無い)
  外部URL
  `../` で始まる別リポジトリの文書 (件数だけ報告する)
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
