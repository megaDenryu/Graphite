// フェーズ4 項目5: `graph_schema!` が生成するハンドシェイク用マクロ
// (`__graphite_check_edge_OrgChart!`) により、存在しないエッジ種別の参照は
// まず親切な compile_error! で報告される。`graph!` はスキーマの中身を
// 知らないままなので、加えてビルダーに対する通常の Rust メソッド解決も
// 走り、`no method named ...` という rustc 標準のエラーも重ねて出る
// (両方が stderr に現れる)。

graphite::graph_schema! {
    schema OrgChart {
        node Employee { name: String, id: u32 }
        node Department { name: String }

        edge belongs_to: Employee -> Department (1);
    }
}

fn main() {
    #[rustfmt::skip]
    let _ = graphite::graph!(OrgChart {
        tanaka: Employee { name: "田中".into(), id: 1 },
        sales: Department { name: "営業".into() },

        tanaka -[not_a_real_edge]-> sales,
    });
}
