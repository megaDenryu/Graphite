//! 関数の中の同じスコープに、同名の値あり個体「太郎」・同名の積み荷あり辺
//! 「太郎の所属」を持つinstanceを2つ置いても供給関数名が衝突しないことを
//! 固定する回帰試験 (`crates/graphite-codegen/src/static_graph/inline/assembly.rs`
//! 参照)。最上位に置いた場合の同じ検証は
//! `static_same_individual_name_multiple_instances.rs` が別に持つ。

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
mod 検証組織内 {
    include!("generated/static_same_individual_name_inside_function_検証組織内.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_same_individual_name_inside_function_検証組織内.rs";
    schema 検証組織内 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署);
    }
}

fn 関数内で同名個体を持つ2つのinstanceを組み立てる() -> (String, String) {
    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    mod 検証チームc {
        include!("generated/static_same_individual_name_inside_function_検証チームc.rs");
    }

    #[rustfmt::skip]
    検証組織内! {
        generated = "generated/static_same_individual_name_inside_function_検証チームc.rs";
        graph 検証チームc;
        node 太郎 = 社員 { 名前: "太郎(C)".into() };
        node 開発部 = 部署 { 名前: "開発部C".into() };
        edge 太郎の所属 = 所属(太郎 -[任命記録 { 任命日: 2022 }]-> 開発部);
    }

    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    mod 検証チームd {
        include!("generated/static_same_individual_name_inside_function_検証チームd.rs");
    }

    #[rustfmt::skip]
    検証組織内! {
        generated = "generated/static_same_individual_name_inside_function_検証チームd.rs";
        graph 検証チームd;
        node 太郎 = 社員 { 名前: "太郎(D)".into() };
        node 総務部 = 部署 { 名前: "総務部D".into() };
        edge 太郎の所属 = 所属(太郎 -[任命記録 { 任命日: 2023 }]-> 総務部);
    }

    let nodes_c = 検証チームcの個体を組み立てる();
    let edges_c = 検証チームcの辺を組み立てる(&nodes_c);
    let g_c = 検証チームc::Graph::new(&nodes_c, &edges_c);

    let nodes_d = 検証チームdの個体を組み立てる();
    let edges_d = 検証チームdの辺を組み立てる(&nodes_d);
    let g_d = 検証チームd::Graph::new(&nodes_d, &edges_d);

    (
        g_c.node_refs.太郎.太郎の所属().team().entity().名前.clone(),
        g_d.node_refs.太郎.太郎の所属().team().entity().名前.clone(),
    )
}

#[test]
fn 関数の中に同名個体を持つ複数instanceを置いてもビルドでき値は独立している() {
    let (所属c, 所属d) = 関数内で同名個体を持つ2つのinstanceを組み立てる();
    assert_eq!(所属c, "開発部C");
    assert_eq!(所属d, "総務部D");
}
