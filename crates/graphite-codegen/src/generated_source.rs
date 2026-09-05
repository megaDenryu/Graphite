//! 生成した schema module を、追跡可能な Rust ソースの本文へ整形する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::declaration_site::DeclarationSite;

// 生成ファイルへ書き出す本文を、先頭の案内コメントごと組み立てる。
pub(crate) fn 生成ファイルの本文(
    body: &TokenStream,
    fingerprint: [u64; 4],
    site: &DeclarationSite,
) -> syn::Result<String> {
    let generated: syn::File = syn::parse2(quote! {
        #[allow(unused_imports)]
        use super::*;

        #[doc(hidden)]
        pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
            #(#fingerprint),*
        ];

        #body
    })?;
    let formatted = prettyplease::unparse(&generated);
    // 再生成コマンドの実行場所は絶対パスで書かない。ここに機械固有のパスを
    // 埋めると、別の作業環境で生成した内容が一致せず `--check` が落ちる。
    //
    // 注意: 案内する再生成コマンドは、どの入口から生成しても同じ文言にする。
    // 入口ごとに書き分けると、`cargo graphite generate` が書いたファイルを
    // `cargo xtask generate --check` が古いと判定する (逆も同じ)。
    let site = site.display();
    Ok(format!(
        "// このファイルは Graphite が生成したため手編集しないこと。\n\
         // 生成元: {site}\n\
         // 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する\n\
         //         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。\n\n\
         {formatted}"
    ))
}

// 指紋の材料にする整形済みの生成コードを返す。指紋そのものを埋め込む前の形である。
//
// 注意: 指紋は prettyplease の整形結果をハッシュするため版差が指紋差になる。
// ルートの Cargo.toml で prettyplease を `=0.2.37` に固定して抑えている。
pub(crate) fn 指紋の材料になる整形済み本文(
    body: &TokenStream,
) -> syn::Result<String> {
    let normalized: syn::File = syn::parse2(quote! {
        #[allow(unused_imports)]
        use super::*;
        #body
    })?;
    Ok(prettyplease::unparse(&normalized))
}
