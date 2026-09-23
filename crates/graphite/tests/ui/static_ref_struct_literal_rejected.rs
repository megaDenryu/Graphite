// `{個体名}Ref`/`{辺名}Ref` の配線フィールド (entity/nodes/edges) が非公開
// であり、利用者が構造体リテラルで直接作れないことを固定する回帰試験。
// 非公開にする前は、親moduleから構造体リテラルで別の`Nodes`を混ぜた
// 不整合な参照を組み立てられた (issue #41)。schemaの宣言は
// `static_multi_module.rs`と同じ内容にし、fingerprintが一致する既存の
// 生成ファイルをそのまま`include!`する。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 組織 {
    include!("../generated/static_multi_module_組織.rs");
}

#[rustfmt::skip]
graphite::static_graph_schema! {
    generated = "generated/static_multi_module_組織.rs";
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 開発チーム {
    include!("../generated/static_multi_module_開発チーム.rs");
}

#[rustfmt::skip]
組織! {
    generated = "generated/static_multi_module_開発チーム.rs";
    graph 開発チーム;
    node 太郎 = 社員 { 名前: "太郎".into() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

fn main() {
    let nodes = 開発チーム::construct::nodes!();
    let edges = 開発チーム::construct::edges!(&nodes);
    let g = 開発チーム::Graph::new(&edges);

    // 別の由来の`Nodes`/`Edges`を、構造体リテラルで直接組み合わせようと
    // する迂回はコンパイルエラーになる (フィールドが非公開のため)。
    let _迂回 = 開発チーム::太郎Ref { entity: g.node_refs().太郎().entity(), nodes: &nodes, edges: &edges };
}
