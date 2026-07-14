// 不正な多重度 (1) / (0..1) / (0..*) 以外はコンパイルエラーになるはず。

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Employee { name: String }
            node Department { name: String }

            edge belongs_to: Employee -> Department (2..5);
        }
    }
}
