// `generated = "...";` を書かずに `graph_schema!` を呼んだ場合の診断を固定する。
// 実際の追跡形式は最初のキーが必ず `generated` でなければならない。

struct Person;

graphite::graph_schema! {
    schema Missing {
        node Person;
    }
}

fn main() {}
