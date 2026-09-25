//! 複製できる schema の完成済みグラフと辺レコードへ付ける `#[derive(Clone)]` を組み立てる。
//!
//! 生成器は `Clone` の実装を手で書かず、導出だけを付ける。複製の意味を
//! 利用者が手で書く `#[derive(Clone)]` と同じにする (構築印も値のまま写す)
//! ためである (`docs/schema_v4.md` §3.1.3、`docs/development/design_principles.md`
//! 原則5・原則6)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::semantic::完成したグラフの複製可否;

// 複製できる schema なら `#[derive(Clone)]` を、そうでなければ空を返す。
// 意味層はトークンを作らないため、この写像はコード生成層に置く。
pub(crate) fn 複製の導出属性を組み立てる(
    複製可否: &完成したグラフの複製可否,
) -> TokenStream {
    match 複製可否 {
        完成したグラフの複製可否::複製できる => quote! { #[derive(Clone)] },
        完成したグラフの複製可否::複製できない => TokenStream::new(),
    }
}
