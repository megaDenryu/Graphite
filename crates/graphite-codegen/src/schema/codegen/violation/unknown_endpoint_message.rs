//! 未知の端点キーを参照した違反の表示を組み立てる。
//!
//! 有向の始点・終点と無向の端点の3ケースは同じ文の形を共有し、辺のIDと端点の
//! キーがそれぞれ表示できるかで4通りへ分かれる。文の先頭には解決できなかった
//! 綴りを置く。辺のキーは入力形式によっては利用者が機械的に採番するだけの値で
//! あり、原因の定義を指さないためである (issue #26)。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;

// 未知のキーを参照した端点が、辺のどの位置にあるか。
#[derive(Clone, Copy)]
pub(crate) enum 未知端点の位置 {
    有向辺の始点,
    有向辺の終点,
    無向辺の端点,
}

impl 未知端点の位置 {
    // 端点のキーを保持する variant のフィールド名。
    fn 端点の欄の名前(self) -> Ident {
        let 綴り = match self {
            Self::有向辺の始点 => "source",
            Self::有向辺の終点 => "target",
            Self::無向辺の端点 => "endpoint",
        };
        Ident::new(綴り, proc_macro2::Span::call_site())
    }

    // 診断文へ書く位置の呼び名。
    fn 診断文での呼び名(self) -> &'static str {
        match self {
            Self::有向辺の始点 => "始点",
            Self::有向辺の終点 => "終点",
            Self::無向辺の端点 => "端点",
        }
    }

    // この違反を表す variant 名。
    fn variantの名前(self, 辺: &EdgeInfo<'_>) -> Ident {
        match self {
            Self::有向辺の始点 => 辺.unknown_source_variant(),
            Self::有向辺の終点 => 辺.unknown_target_variant(),
            Self::無向辺の端点 => 辺.unknown_endpoint_variant(),
        }
    }

    // この位置の端点が指すノード。無向辺は両端が同型のため始点側で代表する。
    fn 端点が指すノード<'a>(self, 辺: &'a EdgeInfo<'a>) -> &'a NodeInfo<'a> {
        match self {
            Self::有向辺の終点 => 辺.to_node,
            Self::有向辺の始点 | Self::無向辺の端点 => 辺.from_node,
        }
    }

    // 表示できるキーだけを載せた `Display` の分岐。
    //
    // 辺のIDと端点のキーは表示できるかを別々に判定する。利用者が宣言したID型へ
    // `Debug` を要求しない契約 (`docs/schema_v4.md`) のため、片方だけが生成ID型で
    // ある構成が起こり、論理積で判定すると表示できる側の綴りまで落ちる。
    pub(crate) fn 表示の腕(
        self, 違反列挙型の名前: &Ident, 辺: &EdgeInfo<'_>
    ) -> TokenStream {
        let variant = self.variantの名前(辺);
        let 端点の欄 = self.端点の欄の名前();
        let 辺種別の綴り = 辺.kind.to_string();
        let ノード = self.端点が指すノード(辺);
        let ノード型の綴り = ノード.type_ident.to_string();
        let 位置の呼び名 = self.診断文での呼び名();
        match (
            ノード.id_ty.is_debug_printable(),
            辺.id_ty.is_debug_printable(),
        ) {
            (true, true) => quote! {
                #違反列挙型の名前::#variant { edge, #端点の欄 } => write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    #端点の欄, #ノード型の綴り, #辺種別の綴り, edge, #位置の呼び名
                )
            },
            (true, false) => quote! {
                #違反列挙型の名前::#variant { #端点の欄, .. } => write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` の{})",
                    #端点の欄, #ノード型の綴り, #辺種別の綴り, #位置の呼び名
                )
            },
            (false, true) => quote! {
                #違反列挙型の名前::#variant { edge, .. } => write!(
                    f,
                    "未知のキーが {} として解決できません (辺 `{}` {:?} の{})",
                    #ノード型の綴り, #辺種別の綴り, edge, #位置の呼び名
                )
            },
            (false, false) => quote! {
                #違反列挙型の名前::#variant { .. } => write!(
                    f,
                    "未知のキーが {} として解決できません (辺 `{}` の{})",
                    #ノード型の綴り, #辺種別の綴り, #位置の呼び名
                )
            },
        }
    }
}
