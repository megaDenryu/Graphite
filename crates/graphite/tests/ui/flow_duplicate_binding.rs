// flow! 内の束縛名は (graph! のキーと同様) 単一の平坦な名前空間。同じ束縛名
// を2回宣言するとコンパイルエラーになるはず (`docs/flow_macro.md`:
// 「重複束縛名の診断 (graph! のキー重複と同じ親切さ: 最初の宣言位置併記)」)。

fn double(x: i32) -> i32 {
    x * 2
}

fn negate(x: i32) -> i32 {
    -x
}

fn main() {
    #[rustfmt::skip]
    graphite::flow! {
        1 -[double]-> doubled,
        2 -[negate]-> doubled,
    };
}
