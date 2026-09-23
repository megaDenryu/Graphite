// instance側からの参照専用の修飾パス (issue #41 是正2・是正13)。定義箇所
// (`type_names::辺値型名`・`fixed_vocabulary` 等) と違い、参照側は doc を
// 持たない生の `TokenStream` を返す。instanceファイルの本文とDSLトークンの
// 錨は、参照先の型が別module (schema module・instanceのグラフmodule) に
// あるため、`use super::*;` だけでは解決できず修飾パスが要る:
//
// - `辺値参照パス`: `{schema名}::{種別}Edge`。`所属Edge` はschema module
//   (`mod 組織`) の中にあり、instance module (`mod 開発チーム`) の
//   `use super::*;` は `組織` というmodule名だけを持ち込み、その内側の
//   `所属Edge` までは持ち込まない。
// - `個体参照パス`・`辺参照パス`: `{グラフ名}::{個体名/辺名}Ref`。DSL
//   トークンの錨はマクロ呼び出し位置 (instance moduleの外) に置かれる
//   ため、instance module越しの修飾が要る。
//
// spanは呼び出し元が渡すIdentから自動継承する (`format_ident!` の最初の
// 補間Identのspan継承、`proc-macro-dev` スキル「スパンポリシー」参照)。
// instanceの本文からの参照は具体辺が持つ「instanceに書かれた種別トークン」
// (`具体辺.種別トークン()`) を使い、schemaの宣言位置を代用しない
// (issue #41 是正1)。

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::static_graph::semantic::{具体辺, 意味モデル};

pub(crate) fn 辺値参照パス(意味モデル: &意味モデル, 具体辺: &具体辺) -> TokenStream {
    let schema名 = 意味モデル.schema名();
    let 型名 = format_ident!("{}Edge", 具体辺.種別トークン());
    quote! { #schema名::#型名 }
}

pub(crate) fn 個体参照パス(意味モデル: &意味モデル, 個体名トークン: &Ident) -> TokenStream {
    let グラフ名 = 意味モデル.グラフ名();
    let 型名 = format_ident!("{}Ref", 個体名トークン);
    quote! { #グラフ名::#型名 }
}

pub(crate) fn 辺参照パス(意味モデル: &意味モデル, 辺名トークン: &Ident) -> TokenStream {
    let グラフ名 = 意味モデル.グラフ名();
    let 型名 = format_ident!("{}Ref", 辺名トークン);
    quote! { #グラフ名::#型名 }
}
