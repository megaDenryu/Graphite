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
  cargo xtask generate --check    生成ファイルの差分と孤児 (どの schema 宣言からも参照されなくなった生成ファイル) をエラーにする
  cargo xtask check-external      ワークスペースの外の検証用パッケージで、生成の差分検査とビルドとテストを実行する
  cargo xtask check-docs          文書参照とリポジトリ内Rustソース参照の綴りの実在・行数範囲、docs/README.md 索引の網羅を検査する

check-docs が検査するもの (crates・examples・xtask・verification 配下の Rust
ソースを指す参照。docs/history 配下 (書き換えず追記のみで運用する、過去の設計
記録) も対象に含む): ファイルの実在と、行番号 (範囲や `3-4, 9-11` のようなカンマ
区切りの複数指定なら最大の終了行) がその実ファイルの行数に収まっていること。`:0` や `:34-12` (開始が
終了より大きい逆転範囲) のような無効な行指定、およびワイルドカード・
プレースホルダを含まないのに解析できない綴りは「解析できないソース参照」として
検査する。ワイルドカード (`*`) やプレースホルダ (`<名前>` 等) を含む綴りだけは
「該当するファイル群」を総称する散文とみなし、個別ファイルへの参照として検査
しない。

行番号を持つソース参照が Markdown 文書に書かれ、その行の直後に (空行だけを
挟んで) コードフェンスが始まる場合、check-docs はフェンス本文の先頭3行が参照先の
行範囲に実在することも照合する。照合しない行は3種類ある。空行、省略記号
(`...`・`// ...`) だけの行、および正規化すると空になる行 (`{` だけの行のように、
空白と末尾の区切りを落とすと何も残らない行。どの本文にも含まれてしまい、照合
しても常に一致になるため判定にならない) である。フェンス本文がこの3種類だけで
できている引用は、照合の件数にも数えない。一致とみなす差は3つあり、空白の違い・
引数列の末尾コンマ・署名を `;` で打ち切った引用である。カンマ区切りの複数範囲
では、check-docs はいずれか1つの範囲に含まれれば一致とみなす。

check-docs が検査しないもの:
  文書の節番号と、Markdown の見出しへ飛ぶリンク (`#見出し` の形) の実在
  宣言した行範囲の開始位置が引用の先頭と揃っていること (63件の引用のうち41件は
  引用が範囲の先頭から始まっておらず、揃いを要求すると41件が一斉に違反になる)
  引用行が現れる順序 (引用の2行を入れ替えても check-docs は違反にしない)
  ファイルの綴りを持たない裸の行範囲 (`docs/desugaring_reference.md` が同一
  ファイル内を指すときに使う `764-775` の形) と、その直後のコードフェンス
  (綴りが無いため参照として拾われず、照合もされない)
  コードフェンスの中に書かれたソース参照の引用本文 (フェンスを閉じる記号を引用の
  開始と読み違えるため、check-docs は照合の対象にしない。ファイルの実在と行数
  範囲は従来どおり検査する)
  引用本文とコードの完全な一致 (check-docs が照合するのは引用の先頭3行までで
  あり、それより後ろ・省略された部分・畳み込まれた空白の差は検査しない。生成物
  そのものの一致は generate --check が別に担当する)
  Rust の doc コメントに書いたコードフェンスの本文 (照合の対象は Markdown 文書だけである)
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
