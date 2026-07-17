// v4: 旧多重度注釈 `(1)`/`(0..1)`/`(0..*)` は字面ごと廃止された。where 節の
// `each <型>: <spec>` の spec は `1` / `0..1` のいずれかのみサポートする。
// それ以外 (`2..5` 等) はコンパイルエラーになるはず。

pub struct Employee {
    pub name: String,
}

pub struct Department {
    pub name: String,
}

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Employee;
            node Department;

            edge BelongsTo = Employee -> Department where each Employee: 2..5;
        }
    }
}
