//! graphitets-by-hand 用のマクロ (issue #24 段階2)。2層に分かれる。
//!
//! `静的グラフ型!` (レイヤー1、`graph_schema!` 相当) は型の骨組み (種別ラベル・
//! 種別契約・役割アクセサ・多重度の契約) をグラフ名に依存しない形で1回生成
//! する。`静的グラフ!` (レイヤー2、`graph!` リテラル相当) は個体タグ・
//! ノード達・辺達・参照の層・グラフ本体を生成し、レイヤー1が用意した骨組みに
//! 具体的な個体を接続する。両マクロは独立にexpandされるため、レイヤー2は
//! レイヤー1が何を生成したかをトークンレベルで参照できない (詳細は各
//! moduleのコメント参照)。
//!
//! 仕組み (台帳・ノード参照・辺参照・無向辺・無向辺参照・ノードタグ・
//! 種別契約・辺・結ぶ) と、種別・実体型はマクロの外 (呼び出し側のスコープ)
//! にある前提で展開する。

mod literal;
mod schema;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn 静的グラフ型(入力: TokenStream) -> TokenStream {
    let 入力 = parse_macro_input!(入力 as schema::input::静的グラフ型入力);
    if let Err(エラー) = 入力.検証する() {
        return エラー.to_compile_error().into();
    }
    schema::codegen::生成する(&入力).into()
}

#[proc_macro]
pub fn 静的グラフ(入力: TokenStream) -> TokenStream {
    let 入力 = parse_macro_input!(入力 as literal::input::静的グラフ入力);
    if let Err(エラー) = 入力.検証する() {
        return エラー.to_compile_error().into();
    }
    literal::codegen::生成する(&入力).into()
}
