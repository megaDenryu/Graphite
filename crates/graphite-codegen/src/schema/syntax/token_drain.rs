//! デリミタの途中でパースを諦めたときに、残りのトークンを読み捨てる。

use syn::parse::ParseStream;

/// 残りのトークンを丸ごと読み捨てて `ParseStream` を空にする。
///
/// syn の `ParseBuffer` は drop 時に「まだトークンが残っているか」を
/// チェックし、残っていれば共有の `Unexpected` セルにその位置を記録する
/// (`syn::parse::Parser::parse2` はこのセルを最終チェックで読み、"unexpected
/// token" エラーとして再浮上させる)。宣言単位のエラー回復 (G4) では、内側の
/// `Parse` 実装がデリミタの途中でエラーを返した後もそのデリミタの中身が
/// 未消費のまま残ることがあり、これを放置すると「回復して続行したはず」の
/// 箇所で無関係な "unexpected token" が幽霊のように出る。そのため、
/// デリミタ内 (`( .. )`/`[ .. ]`) でエラーを返す全ての箇所は、返す前に
/// この関数で中身を空にしておく。
pub(super) fn drain_rest(content: ParseStream) {
    let _ = content.parse::<proc_macro2::TokenStream>();
}
