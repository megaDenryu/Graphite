//! `node 型名;` の宣言を読む。

use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::{Ident, Path, Token};

use super::identifier_type::parse_optional_id_type;
use super::keywords as kw;

// `node Person;`
//
// `Person` はユーザーが `graph_schema!` の外で宣言した普通の struct への
// 参照であり、このマクロは生成しない。型名は単純 `Ident` のみを受け付ける
// (エッジ端点の型名照合に文字列比較で使うため、`syn::Path` にすると
// `crate::Person` と `Person` を同一視できず照合が破綻する。モジュール
// 修飾したい場合は `use` でこのスコープに名前を持ち込むのが Rust の作法
// どおりの解決)。
//
// 内部ストレージの複数形フィールド名を明示指定する `node 型名(複数形);`
// 構文はかつて存在したが、v4 でストレージ名が内部専用 (利用者から不可視)
// になり明示する意義が消えたため廃止した (`docs/graph_splice.md` §3)。
// 検出・移行診断は行わない。内部フィールド名はノード名から機械的な
// 私有名として生成する。
pub struct NodeDecl {
    pub name: Ident,
    pub id_ty: Option<Path>, // 既存の公開 ID 型。`None` なら `{name}Id` newtype を生成する
}

impl Parse for NodeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::node>()?;
        let name: Ident = input.parse()?;
        let id_ty = parse_optional_id_type(input)?;
        input.parse::<Token![;]>()?;
        Ok(NodeDecl { name, id_ty })
    }
}
