// 生成物5: 辺インスタンスごとの具象参照struct。役割アクセサ (始点役割/終点
// 役割、または端点1/端点2) は戻り値を「個体の具象参照型」にする (前版の
// タグ付きジェネリック辺参照からの強化点)。積み荷を持つ種別だけ積み荷
// アクセサ (役割名) を持つ。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::literal::input::{辺形状 as 具体形状, 辺宣言 as 具体辺宣言, 静的グラフ入力};
use crate::schema::input::{辺形状 as 型形状, 静的グラフ型入力};

pub(super) fn 生成する(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> TokenStream {
    let 生成達 = instance.辺宣言達.iter().map(|辺| {
        let 型宣言 = schema.辺宣言を種別名で探す(&辺.種別).expect("相互検証済みなので種別は必ず実在する");
        一辺分を生成する(辺, 型宣言)
    });
    quote! { #(#生成達)* }
}

fn 一辺分を生成する(辺: &具体辺宣言, 型宣言: &crate::schema::input::辺宣言) -> TokenStream {
    let 参照名 = format_ident!("{}参照", 辺.名前);
    let 型名 = format_ident!("{}の辺", 辺.種別);

    let ロールアクセサ達 = ロールアクセサ達を生成する(辺, 型宣言);
    let 積み荷アクセサ = 積み荷アクセサを生成する(型宣言);

    quote! {
        #[derive(Clone, Copy)]
        struct #参照名<'a> {
            実体: &'a #型名<'a>,
            ノード達: &'a ノード達,
            辺達: &'a 辺達<'a>,
        }
        impl<'a> #参照名<'a> {
            #ロールアクセサ達
            #積み荷アクセサ
        }
    }
}

fn ロールアクセサ達を生成する(辺: &具体辺宣言, 型宣言: &crate::schema::input::辺宣言) -> TokenStream {
    let (第一役割, 第一個体, 第二役割, 第二個体) = match (&辺.形状, &型宣言.形状) {
        (具体形状::有向 { 始点, 終点, .. }, 型形状::有向 { 始点役割, 終点役割, .. }) => {
            (始点役割.clone(), 始点, 終点役割.clone(), 終点)
        }
        (具体形状::無向 { 端点1, 端点2, .. }, 型形状::無向 { .. }) => {
            (format_ident!("端点1"), 端点1, format_ident!("端点2"), 端点2)
        }
        _ => unreachable!("相互検証済みなので向きは一致している"),
    };
    let 第一アクセサ = 一アクセサを生成する(&第一役割, 第一個体);
    let 第二アクセサ = 一アクセサを生成する(&第二役割, 第二個体);
    quote! {
        #第一アクセサ
        #第二アクセサ
    }
}

// self.実体 (辺値struct) が既に持つ端点参照 (#役割名 フィールド) を読んで
// 個体参照へ包む。self.ノード達 を個体名で引き直さない (辺が辿りの根拠、
// static_graph.rs 冒頭コメントの方針と同じ)。
fn 一アクセサを生成する(役割名: &proc_macro2::Ident, 個体名: &proc_macro2::Ident) -> TokenStream {
    let 戻り値型 = format_ident!("{}参照", 個体名);
    quote! {
        fn #役割名(&self) -> #戻り値型<'a> {
            #戻り値型 { 実体: self.実体.#役割名, ノード達: self.ノード達, 辺達: self.辺達 }
        }
    }
}

fn 積み荷アクセサを生成する(型宣言: &crate::schema::input::辺宣言) -> TokenStream {
    match 型宣言.形状.積み荷() {
        Some((役割, 型)) => quote! {
            fn #役割(&self) -> &'a #型 { &self.実体.#役割 }
        },
        None => TokenStream::new(),
    }
}
