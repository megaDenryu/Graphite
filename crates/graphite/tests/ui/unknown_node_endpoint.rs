// エッジの端点が未宣言のノード型を指しているケース。
// `Department` は node 宣言されていないため、コンパイルエラーになるはず。

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Employee { name: String }

            edge belongs_to: Employee -> Department (1);
        }
    }
}
