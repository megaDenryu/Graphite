//! 最上位 (このファイルのモジュール直下) の同じスコープに、同名の値あり
//! 個体「太郎」・同名の積み荷あり辺「太郎の所属」を持つinstanceを2つ置いても
//! 供給関数名が衝突しないことを固定する回帰試験
//! (`crates/graphite-codegen/src/static_graph/inline/assembly.rs` 参照)。
//! 供給関数 (`__graphite_initial_value_{名前}`等) を組み立て関数の本体へ
//! 入れ子にする前は、呼び出し位置に並ぶ自由関数としてinstanceを跨いで
//! 同じ名前を生成しており、この配置は
//! `error[E0428]: the name '__graphite_initial_value_太郎' is defined
//! multiple times` になっていた。関数の中に置いた場合の同じ検証は
//! `static_same_individual_name_inside_function.rs`、個体の値の式の
//! 評価回数の検証は `static_individual_value_evaluated_once_per_assembly.rs`
//! が別に持つ。

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
    include!("generated/static_same_individual_name_multiple_instances_検証組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_same_individual_name_multiple_instances_検証組織.rs";
    schema 検証組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署);
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証チームa {
    include!("generated/static_same_individual_name_multiple_instances_検証チームa.rs");
}

#[rustfmt::skip]
検証組織! {
    generated = "generated/static_same_individual_name_multiple_instances_検証チームa.rs";
    graph 検証チームa;
    node 太郎 = 社員 { 名前: "太郎(A)".into() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -[任命記録 { 任命日: 2020 }]-> 開発部);
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証チームb {
    include!("generated/static_same_individual_name_multiple_instances_検証チームb.rs");
}

#[rustfmt::skip]
検証組織! {
    generated = "generated/static_same_individual_name_multiple_instances_検証チームb.rs";
    graph 検証チームb;
    node 太郎 = 社員 { 名前: "太郎(B)".into() };
    node 総務部 = 部署 { 名前: "総務部".into() };
    edge 太郎の所属 = 所属(太郎 -[任命記録 { 任命日: 2021 }]-> 総務部);
}

#[test]
fn 最上位に同名個体を持つ複数instanceを置いてもビルドでき値は独立している() {
    let nodes_a = 検証チームaの個体を組み立てる();
    let edges_a = 検証チームaの辺を組み立てる(&nodes_a);
    let g_a = 検証チームa::Graph::new(&nodes_a, &edges_a);

    let nodes_b = 検証チームbの個体を組み立てる();
    let edges_b = 検証チームbの辺を組み立てる(&nodes_b);
    let g_b = 検証チームb::Graph::new(&nodes_b, &edges_b);

    assert_eq!(g_a.node_refs.太郎.entity().名前, "太郎(A)");
    assert_eq!(g_b.node_refs.太郎.entity().名前, "太郎(B)");
    assert_eq!(g_a.node_refs.太郎.太郎の所属().team().entity().名前, "開発部");
    assert_eq!(g_b.node_refs.太郎.太郎の所属().team().entity().名前, "総務部");
}
