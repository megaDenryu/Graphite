//! graphitets-by-hand 用の静的グラフ! マクロ (issue #24 段階1)。
//!
//! `静的グラフ! { graph G; node N: T; ... edge E = K(N1 -> N2); ... }` から、
//! 手書きの到達点 (`src/bin/static_graph.rs`) が人力で書いていた「具体・参照
//! の層」(個体タグ・ノード達・辺達・型別名・所属辺メソッド・参照の層・
//! グラフ本体) を生成する。仕組み (台帳・ノード参照・辺参照・ノードタグ・
//! 辺・結ぶ) と、種別・実体型・種別ごとの端点アクセサはマクロの外
//! (呼び出し側のスコープ) にある前提で展開する。

mod codegen;
mod input;
mod validate;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use input::静的グラフ入力;

#[proc_macro]
pub fn 静的グラフ(入力: TokenStream) -> TokenStream {
    let 入力 = parse_macro_input!(入力 as 静的グラフ入力);
    if let Err(エラー) = 入力.検証する() {
        return エラー.to_compile_error().into();
    }
    codegen::生成する(&入力).into()
}
