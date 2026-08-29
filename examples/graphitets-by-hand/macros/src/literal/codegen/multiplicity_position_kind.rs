// multiplicity_asserts.rs が生成する位置ごとのconst assertが使う、macro展開
// 時の内部判定専用の位置区分。表示名への変換とマクロトークンへの変換という
// 独立した2つの振る舞いを持つ、それ自体で完結した概念のため別ファイルへ
// 切り出している。仕組み層 (src/仕組み/位置.rs) の始点位置/終点位置 (多重度
// trait のジェネリック引数として使うマーカー型) とは別物であり、両者の名前が
// 同じでも指す層が違う。

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum 位置区分 {
    始点,
    終点,
}

impl 位置区分 {
    pub(super) fn 表示名(&self) -> &'static str {
        match self {
            位置区分::始点 => "始点",
            位置区分::終点 => "終点",
        }
    }

    pub(super) fn トークンを生成する(&self) -> TokenStream {
        match self {
            位置区分::始点 => quote! { 始点位置 },
            位置区分::終点 => quote! { 終点位置 },
        }
    }
}
