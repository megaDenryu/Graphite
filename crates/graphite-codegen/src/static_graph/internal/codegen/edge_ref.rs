// 生成物5: 辺インスタンスごとの具象参照struct (`{辺名}Ref`、旧
// `{辺名}参照`)。役割アクセサ (有向は始点役割/終点役割、無向は第1役割/
// 第2役割) は戻り値を「個体の具象参照型 (`{個体名}Ref`)」にする。積み荷を
// 持つ種別だけ積み荷アクセサ (役割名) を持つ。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::static_graph::literal::input::{辺形状 as 具体形状, 辺宣言 as 具体辺宣言, 静的グラフ入力};
use crate::static_graph::schema::input::{辺形状 as 型形状, 静的グラフ型入力};

pub(super) fn 辺インスタンス参照達を生成する(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> TokenStream {
    let 生成達 = instance.辺宣言達.iter().map(|辺| {
        let 型宣言 = schema.辺宣言を種別名で探す(&辺.種別).expect("相互検証済みなので種別は必ず実在する");
        一辺分を生成する(辺, 型宣言)
    });
    quote! { #(#生成達)* }
}

fn 一辺分を生成する(辺: &具体辺宣言, 型宣言: &crate::static_graph::schema::input::辺宣言) -> TokenStream {
    let 参照名 = format_ident!("{}Ref", 辺.名前, span = 辺.名前.span());
    let 型名 = format_ident!("{}Edge", 辺.種別, span = 辺.種別.span());

    let ロールアクセサ達 = ロールアクセサ達を生成する(辺, 型宣言);
    let 積み荷アクセサ = 積み荷アクセサを生成する(型宣言);

    quote! {
        #[derive(Clone, Copy)]
        struct #参照名<'a> {
            entity: &'a #型名<'a>,
            nodes: &'a Nodes,
            edges: &'a Edges<'a>,
        }
        impl<'a> #参照名<'a> {
            #ロールアクセサ達
            #積み荷アクセサ
        }
    }
}

fn ロールアクセサ達を生成する(辺: &具体辺宣言, 型宣言: &crate::static_graph::schema::input::辺宣言) -> TokenStream {
    let (第一役割, 第一個体, 第二役割, 第二個体) = match (&辺.形状, &型宣言.形状) {
        (具体形状::有向 { 始点, 終点, .. }, 型形状::有向 { 始点役割, 終点役割, .. }) => {
            (始点役割.clone(), 始点, 終点役割.clone(), 終点)
        }
        (具体形状::無向 { 端点1, 端点2, .. }, 型形状::無向 { 第1役割, 第2役割, .. }) => {
            (第1役割.clone(), 端点1, 第2役割.clone(), 端点2)
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

// self.entity (辺値struct) が既に持つ端点参照 (#役割名 フィールド) を読んで
// 個体参照へ包む。self.nodes を個体名で引き直さない (辺が辿りの根拠、
// static_graph.rs 冒頭コメントの方針と同じ)。
fn 一アクセサを生成する(役割名: &proc_macro2::Ident, 個体名: &proc_macro2::Ident) -> TokenStream {
    let 戻り値型 = format_ident!("{}Ref", 個体名, span = 個体名.span());
    quote! {
        fn #役割名(&self) -> #戻り値型<'a> {
            #戻り値型 { entity: self.entity.#役割名, nodes: self.nodes, edges: self.edges }
        }
    }
}

fn 積み荷アクセサを生成する(型宣言: &crate::static_graph::schema::input::辺宣言) -> TokenStream {
    match 型宣言.形状.積み荷() {
        Some((役割, 型)) => quote! {
            fn #役割(&self) -> &'a #型 { &self.entity.#役割 }
        },
        None => TokenStream::new(),
    }
}
