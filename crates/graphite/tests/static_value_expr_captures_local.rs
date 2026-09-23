//! instanceの値の式が、instanceを置いた関数のローカル変数・関数の引数・
//! ジェネリックの型引数を、通常のRust式と同じように参照できることを固定する
//! 回帰試験 (issue #46)。値マクロ
//! (`crates/graphite-codegen/src/static_graph/inline/value_supply.rs`) は
//! `macro_rules!` として呼び出し位置に展開されるため、入れ子の`fn`とは異なり
//! 外側のスコープを捕捉できる。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

pub struct 任命記録 {
    pub 任命日: u32,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証組織 {
    include!("generated/static_value_expr_captures_local_検証組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_captures_local_検証組織.rs";
    schema 検証組織 {
        node 社員;
        node 部署;
        edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1;
    }
}

// 関数の引数 (`名前`・`日付`) を、値ありの個体・積み荷の式が直接参照する。
// 入れ子の`fn`は外側の引数を捕捉できないが、値マクロは`macro_rules!`として
// 呼び出し位置に展開されるため参照できる。
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証チーム引数 {
    include!("generated/static_value_expr_captures_local_検証チーム引数.rs");
}

fn 関数の引数を個体の値の式が参照する(名前: String, 日付: u32) -> (String, u32) {
    #[rustfmt::skip]
    検証組織! {
        generated = "generated/static_value_expr_captures_local_検証チーム引数.rs";
        graph 検証チーム引数;
        node 太郎 = 社員 { 名前: 名前.clone() };
        node 次郎 = 社員 { 名前: "次郎".into() };
        edge 太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 日付 }]-> 次郎);
    }

    let nodes = 検証チーム引数::construct::nodes!();
    let edges = 検証チーム引数::construct::edges!(&nodes);
    let g = 検証チーム引数::Graph::new(&edges);
    (
        g.node_refs().太郎().entity().名前.clone(),
        g.node_refs().太郎().太郎の上司().任命().任命日,
    )
}

// 関数のローカル変数 (`let`束縛) を値の式が参照する。
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証チームローカル {
    include!("generated/static_value_expr_captures_local_検証チームローカル.rs");
}

fn 関数のローカル変数を個体の値の式が参照する() -> String {
    let 部署名 = format!("{}部", "開発");

    #[rustfmt::skip]
    検証組織! {
        generated = "generated/static_value_expr_captures_local_検証チームローカル.rs";
        graph 検証チームローカル;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 花子 = 社員 { 名前: "花子".into() };
        edge 太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 2024 }]-> 花子);
    }

    let _ = 部署名.clone();
    let nodes = 検証チームローカル::construct::nodes!();
    let edges = 検証チームローカル::construct::edges!(&nodes);
    let g = 検証チームローカル::Graph::new(&edges);
    format!("{} in {}", g.node_refs().太郎().entity().名前, 部署名)
}

// ジェネリックの型引数 (`T`) を経由した値を、値の式が参照する。
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証チーム型引数 {
    include!("generated/static_value_expr_captures_local_検証チーム型引数.rs");
}

fn 型引数を経由した値を個体の値の式が参照する<T: std::fmt::Display>(印: T) -> String {
    #[rustfmt::skip]
    検証組織! {
        generated = "generated/static_value_expr_captures_local_検証チーム型引数.rs";
        graph 検証チーム型引数;
        node 太郎: 社員 = 社員 { 名前: format!("太郎-{印}") };
        node 次郎: 社員 = 社員 { 名前: "次郎".into() };
        edge 太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 0 }]-> 次郎);
    }

    let nodes = 検証チーム型引数::construct::nodes!();
    let edges = 検証チーム型引数::construct::edges!(&nodes);
    let g = 検証チーム型引数::Graph::new(&edges);
    g.node_refs().太郎().entity().名前.clone()
}

#[test]
fn 値の式は関数の引数を参照できる() {
    let (名前, 任命日) = 関数の引数を個体の値の式が参照する("太郎".to_string(), 2025);
    assert_eq!(名前, "太郎");
    assert_eq!(任命日, 2025);
}

#[test]
fn 値の式は関数のローカル変数を参照できる() {
    assert_eq!(関数のローカル変数を個体の値の式が参照する(), "太郎 in 開発部");
}

#[test]
fn 値の式はジェネリックの型引数を経由した値を参照できる() {
    assert_eq!(型引数を経由した値を個体の値の式が参照する(42), "太郎-42");
}
