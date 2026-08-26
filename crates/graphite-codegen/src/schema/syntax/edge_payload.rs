//! 辺の柄に書かれた積み荷 `[役割名: 型パス]` を読む。

use syn::parse::ParseStream;
use syn::{Ident, Path, Token};

/// `edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;`
/// `edge Reports = (reporter: Employee) -> (recipient: Employee) where unique pair;`
/// `edge Friends = Person -- Person where unique pair;`
///
/// 属性型 (`BossEdge` 等) はユーザーが `graph_schema!` の外で宣言した普通の
/// struct への参照であり、このマクロは生成しない。
///
/// 役割名中心構文:
/// - 有向端点は役割名つき (`(役割名: 型名)`) が必須。
/// - 積み荷も役割名つき (`[役割名: 型パス]`) が必須。
/// - 無向辺には端点の役割名を書けない。
/// - 柄が4形になる: `->` / `-[役割名: Attrs]->` (有向) / `--` / `-[役割名: Attrs]-` (無向)。
#[derive(Clone)]
pub struct EdgePayload {
    pub role: Ident,
    pub ty: Path,
}

/// `-[役割名: 型パス]->` / `-[役割名: 型パス]-` の `[ .. ]` の中身。
/// `edges::BossEdge` のようなモジュール修飾も許す (ノード型名と違い端点照合
/// に使わないため、単純 `Ident` に制限する必要がない)。
pub(super) fn parse_edge_bracket_body(content: ParseStream) -> syn::Result<EdgePayload> {
    let role: Ident = content.parse()?;
    if !content.peek(Token![:]) {
        return Err(syn::Error::new(
            role.span(),
            "積み荷には役割名が必要です。`-[役割名: 型パス]->` または `-[役割名: 型パス]-` と書いてください",
        ));
    }
    content.parse::<Token![:]>()?;
    let path: Path = content.parse()?;
    if !content.is_empty() {
        return Err(content
            .error("`-[役割名: 型パス]->` または `-[役割名: 型パス]-` の形式で指定してください"));
    }
    Ok(EdgePayload { role, ty: path })
}
