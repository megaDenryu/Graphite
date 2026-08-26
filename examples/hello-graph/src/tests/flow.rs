//! `flow!` が宣言ではなくその場の関数呼び出しへ脱糖することのテスト。

#[test]
fn flowはfan_outとfan_inを組み合わせた関数の辺として動く() {
    // §5 のデモと同じ形。graph! の宣言される辺と対照的に、flow! は
    // その場で関数を呼ぶだけの脱糖であることをアサーションで確認する。
    fn parse(s: &str) -> i32 {
        s.parse().unwrap()
    }
    fn validate(x: i32) -> bool {
        x >= 0
    }
    fn double(x: i32) -> i32 {
        x * 2
    }
    fn merge(valid: bool, doubled: i32) -> String {
        format!("valid={valid} doubled={doubled}")
    }

    #[rustfmt::skip]
    graphite::flow! {
        "21" -[parse]-> parsed,
        parsed -[validate]-> valid,
        parsed -[double]-> doubled,
        (valid, doubled) -[merge]-> summary,
    };
    assert_eq!(parsed, 21);
    assert!(valid);
    assert_eq!(doubled, 42);
    assert_eq!(summary, "valid=true doubled=42");
}
