// G4a: schema 内の1つのノード宣言だけ構文が壊れていて (フィールド名と型の
// 区切りの `:` が無い)、もう一方の Department 宣言は正常にパースできる
// ケース。
//
// 宣言単位のエラー回復 (部分生成) が効いていれば:
// - 壊れた Employee 宣言由来の compile_error! が1件だけ出る
// - 正常にパースできた Department は普通に型として生成され続け、それを
//   使う fn main のコードに「cannot find type」等の二次エラーは出ない。
//
// もし部分生成が効いていなければ (旧挙動)、スキーマ全体の生成が丸ごと
// 消え、Department も未定義になって fn main 側に無関係な二次エラーが
// 大量に出るはずである。

graphite::graph_schema! {
    schema Broken {
        node Employee { name String }

        node Department { name: String }
    }
}

fn main() {
    let _d = Department {
        name: "営業".to_string(),
    };
}
