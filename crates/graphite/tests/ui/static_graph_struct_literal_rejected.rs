// `Graph`・`NodeRefs`・`EdgeRefs` の配線フィールド (node_refs/edge_refs、
// 個体名・辺名のフィールド) が非公開であり、利用者が構造体リテラルで
// 直接作れないことを固定する回帰試験。非公開にする前は、
// `Graph { node_refs: NodeRefs { 戊: g1.node_refs.戊, .. }, edge_refs:
// g2.edge_refs }` のように、2つのグラフの部品を構造体リテラルで混ぜた
// 不整合な `Graph` を組み立てられた。schemaの宣言は
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
    let nodes1 = 開発チーム::construct::nodes!();
    let edges1 = 開発チーム::construct::edges!(&nodes1);
    let g1 = 開発チーム::Graph::new(&edges1);
    let nodes2 = 開発チーム::construct::nodes!();
    let edges2 = 開発チーム::construct::edges!(&nodes2);
    let g2 = 開発チーム::Graph::new(&edges2);

    // 2つの由来が異なる`g1`・`g2`の部品を構造体リテラルで混ぜようとする
    // 迂回はコンパイルエラーになる (`Graph`・`NodeRefs`・`EdgeRefs`の
    // フィールドが非公開のため)。
    let _迂回 = 開発チーム::Graph {
        node_refs: 開発チーム::NodeRefs { 太郎: g1.node_refs().太郎(), 開発部: g2.node_refs().開発部() },
        edge_refs: 開発チーム::EdgeRefs { 太郎の所属: g2.edge_refs().太郎の所属() },
    };
}
