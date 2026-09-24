// このファイルはschemaファイル本文 (issue #41 §2) を組み立て、種別ごとの
// 型アンカー `pub struct {種別}Edge<'a>` を並べる。DSLの種別トークン
// (`所属(太郎 -> 開発部)`の`所属`等) がF12で着地する先であり、どのinstance
// もこの型を構築しない (`docs/static_graph.md`「生成される名前の公開契約」)。
// 役割・積み荷のフィールドは端点の役割・積み荷の形を示すだけの情報であり
// `pub(crate)` にする (型自体は `pub` で公開契約)。呼び出し側
// (`static_graph::tracked::schema`) が、schemaの指紋定数を
// `crate::generated_source::生成ファイルの本文` 経由で別途足す。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::辺値型名;
use crate::static_graph::schema::input::{積み荷宣言, 静的グラフ型入力, 辺形状};
use crate::static_graph::semantic::{辺種別列をschemaから組み立てる, 辺種別};

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(crate) fn schema本体を組み立てる(
    schema: &静的グラフ型入力,
    宣言元: &宣言元ファイルの綴り,
) -> TokenStream {
    let 辺種別列 = 辺種別列をschemaから組み立てる(schema);
    let 辺値struct列 = 辺種別列.iter().map(|辺種別| 一種別分を組み立てる(辺種別, 宣言元));
    quote! { #(#辺値struct列)* }
}

fn 一種別分を組み立てる(辺種別: &辺種別, 宣言元: &宣言元ファイルの綴り) -> TokenStream {
    let 型名 = 辺値型名(辺種別, 宣言元);
    let doc = doc属性を組み立てる(型名.追跡());
    let 積み荷フィールド = 積み荷フィールドを組み立てる(辺種別.積み荷());
    match 辺種別.形状() {
        辺形状::有向 { 始点役割, 始点型, 終点役割, 終点型, .. } => quote! {
            #doc
            pub struct #型名<'a> {
                pub(crate) #始点役割: &'a #始点型,
                pub(crate) #終点役割: &'a #終点型,
                #積み荷フィールド
            }
        },
        辺形状::無向 { 第1役割, 第1型, 第2役割, 第2型, .. } => quote! {
            #doc
            pub struct #型名<'a> {
                pub(crate) #第1役割: &'a #第1型,
                pub(crate) #第2役割: &'a #第2型,
                #積み荷フィールド
            }
        },
    }
}

fn 積み荷フィールドを組み立てる(積み荷: Option<&積み荷宣言>) -> TokenStream {
    match 積み荷 {
        Some(積み荷宣言 { 役割, 型 }) => quote! { pub(crate) #役割: #型, },
        None => TokenStream::new(),
    }
}
