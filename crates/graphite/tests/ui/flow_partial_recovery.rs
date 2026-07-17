// G4: flow! 内の1項だけ構文が壊れていて (関数式が空、`-[]->`)、他の項は
// 正常にパースできるケース。
//
// 項単位のエラー回復 (部分生成) が効いていれば:
// - 壊れた1項目由来の compile_error! が1件だけ出る
// - 正常にパースできた2項目・3項目は `let doubled = ..;`/`let negated =
//   ..;` として普通に生成され続け、それを使う fn main 側のコードに
//   「cannot find value」等の無関係な二次エラーは出ない。
//
// もし部分生成が効いていなければ、flow! 呼び出し全体の生成が丸ごと消え、
// `doubled`/`negated` も未定義になって使用箇所に無関係な二次エラーが
// 大量に出るはずである。

fn double(x: i32) -> i32 {
    x * 2
}

fn negate(x: i32) -> i32 {
    -x
}

fn main() {
    #[rustfmt::skip]
    graphite::flow! {
        1 -[]-> broken,
        2 -[double]-> doubled,
        3 -[negate]-> negated,
    };
    let _ = doubled;
    let _ = negated;
}
