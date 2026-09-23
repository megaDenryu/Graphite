//! `mod`を関数の外、instance宣言 (`配置検証組織! { .. }`) を関数の中に置く配置で
//! 警告0件であることを固定する回帰試験。
//! `#![deny(warnings)]` により、instance展開がimplを使っていたら出るはずの
//! `non_local_definitions` 等の警告をコンパイルエラーとして検出する。

#![deny(warnings)]

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 配置検証組織 {
    include!("generated/static_mod_outside_instance_inside_fn_配置検証組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_mod_outside_instance_inside_fn_配置検証組織.rs";
    schema 配置検証組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

// このmoduleは最上位 (関数の外) にある一方、instance宣言
// (`配置検証組織! { .. }`) は下の`太郎の所属先を求める`関数の中にある。
// instance展開がimplを使わないため`non_local_definitions`は出ない
// (`docs/static_graph.md` 「追跡の契約」参照)。
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 配置検証チーム {
    include!("generated/static_mod_outside_instance_inside_fn_配置検証チーム.rs");
}

fn 太郎の所属先を求める() -> String {
    #[rustfmt::skip]
    配置検証組織! {
        generated = "generated/static_mod_outside_instance_inside_fn_配置検証チーム.rs";
        graph 配置検証チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }

    let nodes = 配置検証チーム::construct::nodes!();
    let edges = 配置検証チーム::construct::edges!(&nodes);
    let g = 配置検証チーム::Graph::new(&edges);
    g.node_refs.太郎.太郎の所属().team().entity().名前.clone()
}

#[test]
fn mod外_instance関数内の配置で警告0件のまま辿れる() {
    assert_eq!(太郎の所属先を求める(), "開発部");
}
