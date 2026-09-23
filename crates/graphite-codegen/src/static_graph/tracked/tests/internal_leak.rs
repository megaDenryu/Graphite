// 設計書 §6 の4 (issue #41 是正6で対象を広げる): 生成本文を syn で読み戻し、
// `__` で始まる識別子を持つ全項目 (トップレベルの struct・fn・const に加え、
// impl 内のメソッド・関連定数) が「非公開であるか #[doc(hidden)] が付いて
// いるか」を確かめ、公開項目の型・シグネチャのテキストに内部生成識別子
// (`__`) が現れないことを確かめる。積み荷ありの辺 (`instance入力`) を
// 含むことで、積み荷アクセサのシグネチャも検査対象に入る。

use super::super::{parse_tracked_static_instance, parse_tracked_static_schema};
use super::{instance入力, schema入力};

#[test]
fn c分類の識別子は公開項目の型へ漏れない() {
    let schema = parse_tracked_static_schema(schema入力()).unwrap();
    let instance = parse_tracked_static_instance(schema.型入力(), instance入力()).unwrap();
    let site = crate::declaration_site::DeclarationSite::new("src/main.rs".to_string(), 1);

    let mut 検査した項目数 = 0;
    for 本文 in [
        schema.render_module_source(&site).unwrap(),
        instance.render_module_source(&site).unwrap(),
    ] {
        let file: syn::File = syn::parse_str(&本文).expect("生成本文はRustとして解析できる");
        for item in &file.items {
            検査した項目数 += 項目を検査する(item);
        }
    }
    // 検査そのものが空振り (対象0件) にならないことを確かめる。impl内の
    // メソッド (`ImplItem::Fn`) を検査対象に含めたことの錨でもある。
    assert!(検査した項目数 > 0, "検査対象の項目が1件も無い");
}

// 1件検査するたびに1を返す (検査件数の合計を呼び出し側で確かめるため)。
fn 項目を検査する(item: &syn::Item) -> usize {
    match item {
        syn::Item::Struct(構造体) => {
            名前を確かめる(&構造体.ident, &構造体.vis, &構造体.attrs);
            if 公開か(&構造体.vis) {
                let 型文字列 =
                    構造体.fields.iter().map(|f| quote::quote! { #f }.to_string()).collect::<Vec<_>>().join(" ");
                内部識別子が無いことを確かめる(&型文字列, "struct");
            }
            1
        }
        syn::Item::Fn(関数) => {
            名前を確かめる(&関数.sig.ident, &関数.vis, &関数.attrs);
            if 公開か(&関数.vis) {
                let sig = &関数.sig;
                内部識別子が無いことを確かめる(&quote::quote! { #sig }.to_string(), "fn");
            }
            1
        }
        syn::Item::Const(定数) => {
            名前を確かめる(&定数.ident, &定数.vis, &定数.attrs);
            if 公開か(&定数.vis) {
                let ty = &定数.ty;
                内部識別子が無いことを確かめる(&quote::quote! { #ty }.to_string(), "const");
            }
            1
        }
        syn::Item::Impl(実装) => 実装.items.iter().map(impl項目を検査する).sum(),
        _ => 0,
    }
}

fn impl項目を検査する(項目: &syn::ImplItem) -> usize {
    match 項目 {
        syn::ImplItem::Fn(関数) => {
            名前を確かめる(&関数.sig.ident, &関数.vis, &関数.attrs);
            if 公開か(&関数.vis) {
                let sig = &関数.sig;
                内部識別子が無いことを確かめる(&quote::quote! { #sig }.to_string(), "impl内のfn");
            }
            1
        }
        syn::ImplItem::Const(定数) => {
            名前を確かめる(&定数.ident, &定数.vis, &定数.attrs);
            if 公開か(&定数.vis) {
                let ty = &定数.ty;
                内部識別子が無いことを確かめる(&quote::quote! { #ty }.to_string(), "impl内のconst");
            }
            1
        }
        _ => 0,
    }
}

fn 公開か(可視性: &syn::Visibility) -> bool {
    matches!(可視性, syn::Visibility::Public(_))
}

fn doc_hiddenが付いているか(属性列: &[syn::Attribute]) -> bool {
    属性列
        .iter()
        .any(|属性| 属性.path().is_ident("doc") && quote::quote! { #属性 }.to_string().contains("hidden"))
}

// 名前が `__` で始まる項目だけを対象に、非公開か `#[doc(hidden)]` かを
// 確かめる (issue #41 是正6)。
fn 名前を確かめる(名前: &syn::Ident, 可視性: &syn::Visibility, 属性列: &[syn::Attribute]) {
    let 名前文字列 = 名前.to_string();
    if !名前文字列.starts_with("__") {
        return;
    }
    assert!(
        !公開か(可視性) || doc_hiddenが付いているか(属性列),
        "内部生成識別子 `{名前文字列}` が非公開でも#[doc(hidden)]でもない (公開の追跡経路へ漏れている)"
    );
}

fn 内部識別子が無いことを確かめる(文字列: &str, 種類: &str) {
    assert!(
        !文字列.contains("__"),
        "公開{種類}の型/シグネチャに内部生成識別子(__)が現れている: {文字列}"
    );
}
