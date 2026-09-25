// `static_graph_schema!` は `dynamic_graph_schema!` の `#[derive(Clone)]` を受理しない (issue #50)。
// 静的グラフで完成したグラフの複製を選ぶ機能は提供していない (`docs/static_graph.md`)。

struct 社員;

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    #[derive(Clone)]
    schema 組織 {
        node 社員;
    }
}

fn main() {}
