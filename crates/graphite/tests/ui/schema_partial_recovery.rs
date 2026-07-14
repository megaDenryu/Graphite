// G4a: schema 内の1つのノード宣言だけ構文が壊れていて (型名の後に `;` を
// 期待する位置に余分なトークンがある)、もう一方の Department 宣言は正常に
// パースできるケース。
//
// 宣言単位のエラー回復 (部分生成) が効いていれば:
// - 壊れた Employee 宣言由来の compile_error! が1件だけ出る
// - 正常にパースできた Department は普通にグラフ機械 (ID型・アクセサ等) が
//   生成され続け、それを使う fn main のコードに「cannot find type」等の
//   二次エラーは出ない。
//
// もし部分生成が効いていなければ、スキーマ全体の生成が丸ごと消え、
// `DepartmentId` も未定義になって fn main 側に無関係な二次エラーが大量に
// 出るはずである。

pub struct Department {
    pub name: String,
}

graphite::graph_schema! {
    schema Broken {
        node Employee extra_token;

        node Department;
    }
}

fn main() {
    let _id = DepartmentId("営業部".to_string());
}
