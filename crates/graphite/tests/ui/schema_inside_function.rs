fn main() {
    struct LocalNode;

    graphite::graph_schema! {
        schema LocalSchema {
            node LocalNode;
        }
    }
}
