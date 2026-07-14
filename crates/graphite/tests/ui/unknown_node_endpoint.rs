// エッジの端点が未宣言のノード型を指しているケース。
// `Department` は node 宣言されていないため、コンパイルエラーになるはず。
// この検証は `node` 宣言一覧の中に `Department` という識別子があるかどうか
// だけを見るので、`Department` という Rust 型が実在するかは無関係
// (このケースは validate エラーで止まり、コード生成自体が行われないため
// 型解決までは進まない)。

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Employee;

            edge Employee -[belongs_to]-> Department (1);
        }
    }
}
