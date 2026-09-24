//! instanceの値の式が、instance宣言を書いた位置のRust式として意味が決まる
//! ことを固定する回帰試験 (`docs/static_graph.md`「値の式の名前
//! 解決」節)。この試験は、instance宣言の位置と構築マクロの呼び出し位置に
//! 同名の識別子があっても、instance宣言の位置の意味を保つことを確かめる。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 宣言位置組織 {
    include!("generated/static_value_expr_resolves_at_declaration_site_宣言位置組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_resolves_at_declaration_site_宣言位置組織.rs";
    schema 宣言位置組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

// instanceの値の式が呼ぶ、モジュール直下の関数。呼び出し位置に同名の別の
// 項目があっても、値の式は常にこの関数を指す。
fn 名前を作る() -> String {
    "宣言位置".into()
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 宣言位置チーム {
    include!("generated/static_value_expr_resolves_at_declaration_site_宣言位置チーム.rs");
}

#[rustfmt::skip]
宣言位置組織! {
    generated = "generated/static_value_expr_resolves_at_declaration_site_宣言位置チーム.rs";
    graph 宣言位置チーム;
    node 太郎 = 社員 { 名前: 名前を作る() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

// instanceの後ろに書いた、instanceと同じmodule直下の関数。この関数の中に
// 同名の別の関数を定義してから構築マクロを呼んでも、値の式の`名前を作る()`
// は宣言位置 (モジュール直下) の意味を保つ。
fn 呼び出し位置に同名の関数があっても宣言位置のままである() -> String {
    // すり替わらないことを示すためだけに置く影の関数。呼ばれないことが
    // この試験の主張そのものである。
    #[allow(dead_code)]
    fn 名前を作る() -> String {
        "呼び出し位置の影".into()
    }
    let g = 宣言位置チーム::construct!();
    g.node_refs().太郎().entity().名前.clone()
}

#[test]
fn 呼び出し位置に同名の関数があっても値の式は宣言位置を指す() {
    assert_eq!(呼び出し位置に同名の関数があっても宣言位置のままである(), "宣言位置");
}

#[test]
fn 同じmoduleの別の関数から呼んでも宣言位置を指す() {
    let g = 宣言位置チーム::construct!();
    assert_eq!(g.node_refs().太郎().entity().名前, "宣言位置");
}
