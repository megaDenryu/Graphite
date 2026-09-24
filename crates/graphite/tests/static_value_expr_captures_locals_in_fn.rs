//! instanceを関数の中 (文の位置) に `graph <名前> in fn;` として置いたとき、
//! 値の式が関数のローカル変数・引数・型引数を通常のRustの式と同じ意味で
//! 参照できることを固定する回帰試験 (PR #45、`docs/static_graph.md`
//! 「値の式の名前解決」節)。`in fn` は、値の式をlet束縛したクロージャで
//! 固定する形を選ぶための印であり、モジュール直下の項目位置
//! (`graph <名前>;`) とは異なるコード生成を選ぶ。
//!
//! あわせて、呼び出し位置 (構築マクロを呼ぶ位置) に同名のローカル変数が
//! あっても、値の式は宣言位置の束縛を指すことも確かめる
//! (macro_rules!のローカル変数の衛生規則による)。同名の関数によるすり替え
//! 耐性は `static_value_expr_resolves_at_declaration_site.rs` が項目位置で
//! 固定する。
//!
//! `#![deny(warnings)]` により、`in fn` の値供給がletとmacro_rules!だけを
//! 生成しimplを使わないため`non_local_definitions`が出ないことも固定する。

#![deny(warnings)]

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 関数内組織 {
    include!("generated/static_value_expr_captures_locals_in_fn_関数内組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_captures_locals_in_fn_関数内組織.rs";
    schema 関数内組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 関数内チーム {
    include!("generated/static_value_expr_captures_locals_in_fn_関数内チーム.rs");
}

// 引数・ローカル変数を値の式が直接参照する。呼び出し位置 (関数の末尾) に
// 同名のローカル変数・関数を置いても、値の式は宣言位置の束縛を指す
// (macro_rules!のローカル変数の衛生規則)。
fn 太郎を関数の中で組み立てる(部署名: &str) -> String {
    let 名前 = "たろう".to_string();
    let 開発部名 = 部署名.to_string();

    #[rustfmt::skip]
    関数内組織! {
        generated = "generated/static_value_expr_captures_locals_in_fn_関数内チーム.rs";
        graph 関数内チーム in fn;
        node 太郎 = 社員 { 名前: 名前.clone() };
        node 開発部 = 部署 { 名前: 開発部名.clone() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }

    // 呼び出し位置に同名のローカルを用意しても、既に宣言位置で固定された
    // 値の式には影響しない。
    let 名前 = "呼び出し位置の影".to_string();
    let _ = &名前;

    let nodes = 関数内チーム::construct::nodes!();
    let edges = 関数内チーム::construct::edges!(&nodes);
    let g = 関数内チーム::Graph::new(&edges);
    g.node_refs().太郎().太郎の所属().team().entity().名前.clone()
}

#[test]
fn 関数の中の値の式は引数とローカル変数を参照でき呼び出し位置の同名識別子に影響されない() {
    assert_eq!(太郎を関数の中で組み立てる("開発部"), "開発部");
}
