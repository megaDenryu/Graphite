// G4b: graph! 内の1項目 (tanaka のノード宣言) だけ構文が壊れていて
// (フィールド間の `,` が無い)、他の項目 (sales のノード宣言、および
// belongs_to エッジ) は正常にパースできるケース。
//
// 項目単位のエラー回復 (部分生成) が効いていれば:
// - 壊れた tanaka 宣言由来の compile_error! が1件だけ出る
// - tanaka を端点に取る belongs_to エッジは、tanaka がこの graph! 呼び出し
//   内でノードとして宣言されていないため (G4b の二次エラー抑制) 黙って
//   生成対象から除外され、「ノードとして宣言されていません」という二次
//   エラーは出ない
// - sales (Department) は正常に生成され続ける

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

        edge Employee -[belongs_to]-> Department (1);
    }
}

fn main() {
    #[rustfmt::skip]
    let _ = graphite::graph!(OrgChart {
        tanaka: Employee { name: "田中".into() id: 1 },
        sales: Department { name: "営業".into() },

        tanaka -[belongs_to]-> sales,
    });
}
