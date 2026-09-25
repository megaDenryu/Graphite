// schema 宣言の前に書ける属性は `#[derive(Clone)]` だけであり、他の導出は拒否する (issue #50)。

pub struct Person {
    pub name: String,
}

fn main() {
    graphite::__dynamic_graph_schema_inline_for_test! {
        #[derive(Clone, Debug)]
        schema Broken {
            node Person;
        }
    }
}
