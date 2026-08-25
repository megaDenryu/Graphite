fn main() {
    struct LocalNode;

    graphite::__graph_schema_inline_for_test! {
        schema LocalSchema {
            node LocalNode;
        }
    }
}
