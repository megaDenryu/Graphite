fn main() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct LocalNodeId(String);

    struct LocalNode;

    graphite::graph_schema! {
        schema LocalSchema {
            node LocalNode;
        }
    }
}
