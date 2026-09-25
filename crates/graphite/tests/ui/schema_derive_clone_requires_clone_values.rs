// `#[derive(Clone)]` を選んだ schema は、利用者のノード値型に `Clone` を要求する (issue #50)。

pub struct Person {
    pub name: String,
}

fn main() {
    graphite::__dynamic_graph_schema_inline_for_test! {
        #[derive(Clone)]
        schema Cloneable {
            node Person;
        }
    }
}
