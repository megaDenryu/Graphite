// v4: `graph!` 内の識別子はノード・エッジを跨いで単一の平坦な名前空間
// (`docs/schema_v4.md` §0 規則1: 名前は常にキーの束縛)。同じ識別子を2回
// 宣言するとコンパイルエラーになるはず。
//
// このケースでは `instance_codegen::generate` がコード生成前 (トークン
// 列を1つも返す前) に `syn::Error` を返すため、マクロ呼び出し全体が
// `compile_error!(..)` だけに置き換わる。`let _ = graphite::graph!(..);`
// のような式位置で使うとその1行が有効な式にならず無関係な二次エラーが
// 大量に出てしまうため、あえて文(statement)位置で呼び出している
// (`graph_unknown_edge_label.rs` は生成が成功する経路なので式位置のまま)。

pub struct Employee {
    pub name: String,
    pub id: u32,
}

pub struct Department {
    pub name: String,
}

graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;

        edge BelongsTo = Employee -> Department where each Employee: 1;
    }
}

fn main() {
    #[rustfmt::skip]
    graphite::graph!(OrgChart {
        tanaka = Employee { name: "田中".into(), id: 1 },
        sales = Department { name: "営業".into() },
        tanaka = Employee { name: "田中2".into(), id: 2 },

        bt = BelongsTo(tanaka -> sales),
    });
}
