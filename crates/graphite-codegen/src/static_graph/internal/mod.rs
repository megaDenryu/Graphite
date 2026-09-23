//! `__static_graph_impl!` (issue #24 段階2、`#[doc(hidden)]` の内部proc
//! macro、`graphite_macros::__static_graph_impl` から呼ばれる)。
//! `static_graph_schema!` がmacro_rules!転送で焼き込んだschemaの生トークンと、
//! 利用側が `<schema名>! { .. }` で書いたinstanceの生トークンを1回の展開で
//! 同時に受け取り、両者の相互検証 (validate module) と具象コード生成
//! (codegen module) をここで配線する。
//!
//! 受け取るトークン列は `schema <名前> { .. } instance { .. }` の形。schema側
//! (`静的グラフ型入力`) は `static_graph_schema!` と同じ構文をそのままparseし直す
//! (macro_rules!転送で生トークンのままspanを保って届くため)。
//!
//! 検証前に意味モデルを組み立てることを型状態で防ぐ (issue #41 是正8)。
//! `静的グラフ内部入力::検証する` は `self` を消費して `検証済み静的グラフ内部入力`
//! を返し、意味モデルを組み立てられるのは検証済みの型だけである。

mod codegen;
mod validate;

use syn::parse::{Parse, ParseStream};

use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;
use crate::static_graph::semantic::意味モデル;

syn::custom_keyword!(instance);

pub struct 静的グラフ内部入力 {
    pub schema: 静的グラフ型入力,
    pub instance: 静的グラフ入力,
}

impl Parse for 静的グラフ内部入力 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let schema: 静的グラフ型入力 = input.parse()?;
        input.parse::<instance>()?;
        let 本体;
        syn::braced!(本体 in input);
        let instance: 静的グラフ入力 = 本体.parse()?;
        Ok(静的グラフ内部入力 { schema, instance })
    }
}

impl 静的グラフ内部入力 {
    // schema単体の構造検証・instance単体の構造検証・両者を突き合わせる相互
    // 検証の3段で行う。相互検証は前の2段が通っている前提 (端点が宣言済み等)
    // に依存するため、この順序を変えない。`self` を消費し、通れば
    // `検証済み静的グラフ内部入力` を返す (型状態、issue #41 是正8)。
    pub fn 検証する(self) -> syn::Result<検証済み静的グラフ内部入力> {
        self.schema.検証する()?;
        self.instance.検証する()?;
        validate::相互検証する(&self.schema, &self.instance)?;
        Ok(検証済み静的グラフ内部入力 { schema: self.schema, instance: self.instance })
    }
}

// 相互検証を通った静的グラフ内部入力。意味モデルを組み立てられるのは
// この型だけであり、`静的グラフ内部入力` (未検証) からは組み立てられない
// (issue #41 是正8)。
pub struct 検証済み静的グラフ内部入力 {
    schema: 静的グラフ型入力,
    instance: 静的グラフ入力,
}

impl 検証済み静的グラフ内部入力 {
    pub fn 意味モデルを組み立てる(&self) -> 意味モデル {
        意味モデル::組み立てる(&self.schema, &self.instance)
    }

    pub fn コードを生成する(&self) -> proc_macro2::TokenStream {
        codegen::コードを生成する(&self.意味モデルを組み立てる())
    }
}
