//! `schema 名 { ... }` 全体を、宣言単位で回復しながら読む。

use proc_macro2::TokenTree;
use syn::parse::ParseStream;
use syn::{braced, Ident};

use super::edge_declaration::EdgeDecl;
use super::keywords as kw;
use super::node_declaration::NodeDecl;

// `schema Org { ... }` 全体。
pub struct SchemaInput {
    pub schema_name: Ident,
    pub nodes: Vec<NodeDecl>,
    pub edges: Vec<EdgeDecl>,
}

// 宣言単位で回復パースした結果 (`docs/development/ide_support_spec.md` G4a)。
// `errors` が空であることは、全宣言が正常にパースできたことを意味する。
pub struct SchemaParse {
    pub schema: SchemaInput,
    pub errors: Vec<syn::Error>, // 個々の宣言のパースに失敗した箇所を蓄積したもの
}

impl SchemaInput {
    // 宣言単位の回復パーサ (G4a)。
    //
    // 回復は次の戦略で行う。
    //
    // - ヘッダ (`schema Name {`) 自体が壊れている場合は回復せず `Err` を
    //   返す (`schema` キーワード・スキーマ名・開きブレースが揃わないと
    //   ボディの走査自体を始められないため)。
    // - ボディ内は `node`/`edge` 宣言単位でパースする。1宣言のパースに
    //   失敗したら、その `syn::Error` を `errors` に蓄積し、次の宣言境界
    //   まで読み飛ばして続行する。
    // - 境界の定義: ボディの `ParseStream` からトークン木を1つずつ
    //   読み飛ばし、次に `node`/`edge` キーワードが先頭に現れるか入力が
    //   尽きるまで進める。`node`/`edge` いずれの宣言も `;` で終わるため
    //   `;` 区切りの境界定義も選べるが、キーワード探索は proc_macro2 の
    //   `( .. )`/`[ .. ]` がまるごと1つの `Group` トークン木として扱われる
    //   性質にただ乗りできる (where 節・エッジラベルの中身にどんなトークンが
    //   あっても、Group 単位で一括に読み飛ばされるので誤って途中で止まらない)
    //   うえ、両宣言に共通して使え実装も単純で誤爆しにくいためこちらを
    //   採用した。
    pub fn parse_recovering(input: ParseStream) -> syn::Result<SchemaParse> {
        input.parse::<kw::schema>()?;
        let schema_name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut errors = Vec::new();

        while !content.is_empty() {
            if content.peek(kw::node) {
                match content.parse::<NodeDecl>() {
                    Ok(n) => nodes.push(n),
                    Err(e) => {
                        errors.push(e);
                        skip_to_decl_boundary(&content);
                    }
                }
            } else if content.peek(kw::edge) {
                match content.parse::<EdgeDecl>() {
                    Ok(ed) => edges.push(ed),
                    Err(e) => {
                        errors.push(e);
                        skip_to_decl_boundary(&content);
                    }
                }
            } else {
                errors.push(content.error("`node` または `edge` 宣言を期待しました"));
                skip_to_decl_boundary(&content);
            }
        }

        Ok(SchemaParse {
            schema: SchemaInput {
                schema_name,
                nodes,
                edges,
            },
            errors,
        })
    }
}

// 次の `node`/`edge` キーワード (もしくは入力終端) まで、トークン木を
// 1つずつ読み飛ばす。`SchemaInput::parse_recovering` のコメントが書いた
// 境界の定義を参照。
pub(super) fn skip_to_decl_boundary(content: ParseStream) {
    while !content.is_empty() && !content.peek(kw::node) && !content.peek(kw::edge) {
        // `content.parse::<TokenTree>()` は必ず1つトークン木を消費する
        // (`content` が空でないことは while 条件で保証済み) ので、
        // 無限ループにはならない。
        let _ = content.parse::<TokenTree>();
    }
}
