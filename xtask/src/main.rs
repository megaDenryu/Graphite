//! `cargo xtask generate [--check]`・`cargo xtask check-external`・
//! `cargo xtask check-docs`・`cargo xtask check-doc-comments` のコマンドライン入口。
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

// `generate` は生成ファイルを更新し、`generate --check` は差分をエラーにし、
// `check-external` は外部 crate からの生成経路を実走で検査し、`check-docs` は
// 文書参照と索引を検査し、`check-doc-comments` は doc コメントの網羅と撤去を
// 検査する。
enum Command {
    Generate,
    Check,
    CheckExternalCrate,
    CheckDocuments,
    CheckDocComments,
}

// 使い方と、`check-docs` が検査しないことの説明。
//
// 検査の限界を書いておかないと、通ったことを「文書の内容が正しい」と読み違える。
const USAGE: &str = "\
使い方: リポジトリルートで次のいずれかを実行してください
  cargo xtask generate            生成ファイルを更新する
  cargo xtask generate --check    生成ファイルの差分と孤児 (どの schema 宣言からも参照されなくなった生成ファイル) をエラーにする
  cargo xtask check-external      ワークスペースの外の検証用パッケージで、生成の差分検査とビルドとテストを実行する
  cargo xtask check-docs          文書参照とリポジトリ内Rustソース参照の綴りの実在・行数範囲、docs/README.md 索引の網羅を検査する
  cargo xtask check-doc-comments  doc コメントが公開面に網羅され、内部領域に1件も無いことを検査する

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
挟んで) コードフェンスが始まる場合、check-docs はそのフェンス本文を引用とみなし、
2つの判定を行う。1つ目は行範囲の妥当性であり、引用の先頭3行が参照先の「指定された
行範囲」に実在することを見る。2つ目は引用の鮮度である。引用の鮮度とは、引用の
「全行」が参照先の「ファイル全体」に実在することである。前者は引用がどの範囲から
取られたかを見て、後者は引用が現在のコードから取られたかを見る。先頭3行の上限は前者だけのもので
あり、後者には適用しない。後者が対象をファイル全体にするため、途中を省略した引用も
複数行の署名を1行へ畳んだ引用も、省略しなかった行がファイルのどこかに実在する限り
違反にならない。どちらの判定も照合しない行は3種類ある。空行、省略記号
(`...`・`// ...`) だけの行、および正規化すると空になる行 (`{` だけの行のように、
空白と末尾の区切りを落とすと何も残らない行。どの本文にも含まれてしまい、照合
しても常に一致になるため判定にならない) である。フェンス本文がこの3種類だけで
できている引用は、照合の件数にも数えない。一致とみなす差は3つあり、空白の違い・
引数列の末尾コンマ・署名を `;` で打ち切った引用である。カンマ区切りの複数範囲
では、行範囲の妥当性はいずれか1つの範囲に含まれれば一致とみなす。

引用として取り込むのは、情報文字列が `rust` のコードフェンスだけである。報告に出す
「照合しなかった Rust コードフェンス」は、走査した Markdown にある `rust` フェンスの
開始行の総数から、引用として取り込んだ件数を引いたものである (`powershell` や
`text` のフェンスは出典を持つ引用ではないため、この総数に数えない)。check-docs は、
検査が届かなかった範囲を件数で見えるようにするために、この件数を報告する。

文書の側は、引用をコードから改変せずに写す必要がある。鮮度の判定が見るのは「その行が
参照先ファイルに実在するか」であり、文書の書き手が変数名を短くする・型引数を省くと
いった改変をすると、鮮度の判定はその引用を違反にする。
`docs/desugaring_reference.md` の冒頭が宣言しているとおり、原文との違いは4つ (先頭
の字下げの除去・doc コメント記号の除去・「(署名のみ抜粋)」と記した箇所での本体省略
とシグネチャの1行化・出典範囲内の一部メソッドの無印省略) に限る。

check-docs が検査しないもの:
  文書の節番号と、Markdown の見出しへ飛ぶリンク (`#見出し` の形) の実在
  宣言した行範囲の開始位置が引用の先頭と揃っていること (63件の引用のうち41件は
  引用が範囲の先頭から始まっておらず、揃いを要求すると41件が一斉に違反になる)
  引用行が現れる順序 (引用の2行を入れ替えても check-docs は違反にしない)
  ファイルの綴りを持たない裸の行範囲 (`docs/desugaring_reference.md` が同一
  ファイル内を指すときに使う `764-775` の形) と、その直後のコードフェンス
  (綴りが無いため参照として拾われず、照合もされない)
  参照とコードフェンスの間に散文が挟まる引用 (参照の行から空行だけを読み飛ばした
  先がフェンスでなければ、その参照に引用は続いていないとみなす。散文を挟んだまま
  腐った引用が実際に2件あり、issue #31 で参照をフェンスの直前へ移して照合の対象へ
  入れた。文書の側は、引用したいフェンスの直前の行に参照を書く)
  1つの参照に続けて並ぶ2つ目以降のコードフェンスの本文 (`docs/desugaring_reference.md`
  は1つの出典範囲が続けて並ぶ複数のコードブロックを覆うことがあると宣言している。
  照合するのは1つ目のフェンスだけである)
  コードフェンスの中に書かれたソース参照の引用本文 (フェンスを閉じる記号を引用の
  開始と読み違えるため、check-docs は照合の対象にしない。ファイルの実在と行数
  範囲は従来どおり検査する)
  引用が指定の行範囲から一字一句そのまま取られていること (行範囲の妥当性が照合
  するのは引用の先頭3行までであり、4行目以降がその範囲の中にあるかは検査しない。
  4行目以降については、引用の鮮度が「ファイルのどこかに実在するか」だけを見る。
  省略された部分と畳み込まれた空白の差はどちらの判定も検査しない。生成物そのものの
  一致は generate --check が別に担当する)
  Rust の doc コメントに書いたコードフェンスの本文 (照合の対象は Markdown 文書だけである)
  行番号を持たないソース参照の引用本文 (照合すべき行範囲が無い)
  外部URL
  `../` で始まる別リポジトリの文書 (件数だけ報告する)
  ワイルドカード・プレースホルダを含むソースらしき綴り (ファイル群の総称として扱う)

check-doc-comments が検査するもの: 2つある。1つは内部領域
(crates/graphite-codegen・crates/graphite-cli・crates/graphite-macros・xtask・
examples) に項目の doc コメント (`///` と `#[doc = \"...\"]`) が1件も無いこと。
もう1つは生成コードの公開面 (crates・examples・verification の下でディレクトリ名が
generated の場所) の、非 #[doc(hidden)] な公開項目に doc コメントが付いていること。
どちらも syn で構文解析して判定し、読めなかったファイルと構文解析に失敗した
ファイルは違反として数える。報告には領域ごとに、検査したファイル数・公開項目数・
見つかった doc コメントの件数を出す。

check-doc-comments が対象外にするもの (対象外である条件を書き下せるものだけ):
  公開面の3領域 (crates/graphite の非 #[doc(hidden)] な公開項目、生成コードの
  公開面、graphite-macros の #[proc_macro]・#[proc_macro_derive]・
  #[proc_macro_attribute] が付いた関数)。前2つは doc の網羅を要求する側であり、
  3つ目は内部領域の中にありながら利用者の rustdoc に出るため撤去の対象にしない
  ファイル冒頭の `//!` (モジュールの説明。1ファイルに1つであり網羅・非網羅の
  問題を持たないため、内部領域でも残す)
  マクロ呼び出し (`quote!` 等) のトークン列の中に書かれた doc コメント。これは
  生成されるコードに付く doc であって、その領域自身の項目に付いたものではない。
  syn はマクロ本体を不透明なトークン列として持つため、構文解析の結果として自然に
  外れる (graphite-codegen の 861行の `///` のうち 155行、graphite-macros の
  267行のうち一部がこれに当たる)
  #[doc(hidden)] が付いた項目とその中身

check-doc-comments が検査しないもの:
  crates/graphite の公開面の網羅 (rustc の #![warn(missing_docs)] が検査する)
  doc コメントの中身が項目の説明として正しいか
  クレート境界を越えた再公開 (pub use) による公開到達性。公開到達性は
  「囲むモジュールが全て pub であること」だけで判定する
  タプル構造体の名前を持たないフィールド (rustc の missing_docs も要求しない)
  private な型に対する inherent impl の pub なメソッド (rustc は要求しないが、
  この検査は公開項目として数える。生成コードには現れない形である)

`xtask/tests/doc_comments_check.rs` がこの検査を呼ぶため、
`cargo test --workspace` も同じ違反を検出する。";

impl Command {
    fn from_arguments(arguments: &[String]) -> Result<Self, Box<dyn Error>> {
        match arguments {
            [command] if command == "generate" => Ok(Self::Generate),
            [command, option] if command == "generate" && option == "--check" => Ok(Self::Check),
            [command] if command == "check-external" => Ok(Self::CheckExternalCrate),
            [command] if command == "check-docs" => Ok(Self::CheckDocuments),
            [command] if command == "check-doc-comments" => Ok(Self::CheckDocComments),
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
    }
}
