//! 指紋照合のコンパイル時コードを組み立てる。「再生成の案内」の文言を
//! ここへ1つだけ置き、動的グラフ (`dynamic_graph_schema!`)・静的グラフの
//! schema・静的グラフのinstanceの3つが同じ文言・同じ組み立て関数を使う
//! (issue #41 段階2)。生成ファイル先頭の案内コメント
//! (`crate::generated_source`) はこの文言とは別に持つ (先に生成した
//! ファイルを再生成の対象へ広げないための現状維持、issue #41 段階2の
//! スコープ外)。

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote_spanned;

// 全ての生成ファイルの再生成コマンド案内で共有する文言。
const 再生成の案内: &str =
    "パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)";

// 以下の3つの関数は、生成ファイルへ埋め込む指紋定数の名前
// (`crate::generated_source::生成ファイルの本文` へ渡す) を返す。動的グラフ
// のschema・静的グラフのschema・静的グラフのinstanceの3種類で名前を分け、
// 同じパッケージへ両方を生成しても定数名が衝突しないようにする。この名前は
// 利用者トークンの由来を持たないC分類の名前なので、`Span::call_site()` で
// 作る。
pub fn 動的schema指紋定数名() -> Ident {
    Ident::new("__GRAPHITE_SCHEMA_FINGERPRINT", Span::call_site())
}

pub fn 静的schema指紋定数名() -> Ident {
    Ident::new("__GRAPHITE_STATIC_SCHEMA_FINGERPRINT", Span::call_site())
}

pub fn 静的instance指紋定数名() -> Ident {
    Ident::new("__GRAPHITE_STATIC_INSTANCE_FINGERPRINT", Span::call_site())
}

// 指紋照合の警告文に書く対象名。動的グラフのschemaの1種類だけを今のところ
// ここへ置く (静的グラフのschema/instanceは、macroの公開の振る舞いをまだ
// 生成ファイル照合へ切り替えていないため対象名を持たない。issue #41 段階3の
// 仕事)。`graphite-macros` はこの文字列を直書きしない (issue #41 是正16)。
pub fn 動的schema対象文言() -> &'static str {
    "Graphite schema"
}

// 生成ファイルが古いときの警告文
// (「{対象} の生成ファイルが古いため、{再生成の案内}」)。
fn 古い生成ファイルの警告文(対象: &str) -> String {
    format!("{対象} の生成ファイルが古いため、{再生成の案内}")
}

// 指紋照合のコンパイル時コード (`const _: () = { .. };`) を組み立てる。
//
// `定数パス` は生成ファイルに埋め込まれた指紋定数への参照
// (例: `#schema_name::__GRAPHITE_SCHEMA_FINGERPRINT`)。ここへ渡す前に
// 呼び出し側で `quote!` へ組み立てるため、パス中の識別子は呼び出し側が
// 持つ元トークンのspanをそのまま保つ (このspanは書き換えない)。
//
// `span` はこのブロック全体の足場トークン (`const`・`if`・`panic!` 等) へ
// 付けるspanである。動的グラフは現行動作を保つため
// `proc_macro2::Span::call_site()` を渡す (挙動を変えない)。静的グラフの
// schema/instanceは `generated = "..."` リテラルのspanを渡し、include の
// 置き忘れ等のエラーをその行へ向ける。
pub fn 指紋照合コードを生成する(
    定数パス: TokenStream,
    期待指紋: [u64; 4],
    対象: &str,
    span: Span,
) -> TokenStream {
    let [第一, 第二, 第三, 第四] = 期待指紋;
    let 警告文 = 古い生成ファイルの警告文(対象);
    quote_spanned! { span =>
        const _: () = {
            let actual = #定数パス;
            if !(actual[0] == #第一
                && actual[1] == #第二
                && actual[2] == #第三
                && actual[3] == #第四)
            {
                panic!(#警告文);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn 動的グラフと同じ対象文言はバイト単位で一致する() {
        let コード = 指紋照合コードを生成する(
            quote! { World::__GRAPHITE_SCHEMA_FINGERPRINT },
            [1, 2, 3, 4],
            "Graphite schema",
            Span::call_site(),
        );
        let 文言 = コード.to_string();
        assert!(文言.contains(
            "Graphite schema の生成ファイルが古いため、パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)"
        ));
    }

    #[test]
    fn 静的グラフの対象文言はschema名を含む() {
        let コード = 指紋照合コードを生成する(
            quote! { 組織::__GRAPHITE_STATIC_SCHEMA_FINGERPRINT },
            [1, 2, 3, 4],
            "Graphite 静的グラフの schema `組織`",
            Span::call_site(),
        );
        assert!(コード.to_string().contains("Graphite 静的グラフの schema"));
    }
}
