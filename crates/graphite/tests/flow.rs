//! `flow!` の意味論を確認するテスト (`docs/flow_macro.md` 参照)。
//!
//! trybuild (コンパイルエラーになるべきケース: 重複束縛名・項単位の回復)
//! は `crates/graphite/tests/compile_fail.rs` から `tests/ui/*.rs` を通して
//! 実行される。

// --- 直線 (1本の矢印) ---

#[test]
fn 直線1本の矢印は関数適用のletに脱糖する() {
    fn double(x: i32) -> i32 {
        x * 2
    }

    #[rustfmt::skip]
    graphite::flow! {
        3 -[double]-> doubled,
    };
    assert_eq!(doubled, 6);
}

// --- チェーン形 ---

#[test]
fn チェーン形は複数の矢印文の糖衣になる() {
    fn parse(s: &str) -> i32 {
        s.parse().unwrap()
    }
    fn double(x: i32) -> i32 {
        x * 2
    }
    fn to_string(x: i32) -> String {
        x.to_string()
    }

    #[rustfmt::skip]
    graphite::flow! {
        "21" -[parse]-> parsed -[double]-> doubled -[to_string]-> rendered,
    };
    assert_eq!(parsed, 21);
    assert_eq!(doubled, 42);
    assert_eq!(rendered, "42");
}

// --- fan-out (1つの値を複数の矢印に流す) ---

#[test]
fn fan_outは同じ始点を複数の矢印文に書けば実現できる() {
    fn double(x: i32) -> i32 {
        x * 2
    }
    fn negate(x: i32) -> i32 {
        -x
    }

    let parsed = 10; // i32 は Copy なので、そのまま2本の矢印に流せる。
    #[rustfmt::skip]
    graphite::flow! {
        parsed -[double]-> doubled,
        parsed -[negate]-> negated,
    };
    assert_eq!(doubled, 20);
    assert_eq!(negated, -10);
}

#[test]
fn fan_outは借用元startpointを介して非copy型でも共有できる() {
    fn len(s: &String) -> usize {
        s.len()
    }
    fn shout(s: &String) -> String {
        format!("{}!!", s)
    }

    let source = String::from("hello");
    #[rustfmt::skip]
    graphite::flow! {
        &source -[len]-> length,
        &source -[shout]-> shouted,
    };
    assert_eq!(length, 5);
    assert_eq!(shouted, "hello!!");
    // 始点を借用で流したので、flow! の後も source は使える。
    assert_eq!(source, "hello");
}

// --- fan-in (タプル始点) ---

#[test]
fn fan_inはタプル始点で多引数呼び出しに脱糖する() {
    fn merge(a: i32, b: i32) -> i32 {
        a + b
    }

    let valid = 10;
    let report = 32;
    #[rustfmt::skip]
    graphite::flow! {
        (valid, report) -[merge]-> out,
    };
    assert_eq!(out, 42);
}

#[test]
fn fan_inは3引数以上でも同様に脱糖する() {
    fn sum3(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }

    let x = 1;
    let y = 2;
    let z = 3;
    #[rustfmt::skip]
    graphite::flow! {
        (x, y, z) -[sum3]-> total,
    };
    assert_eq!(total, 6);
}

// --- クロージャ関数式 ---

#[test]
fn 関数式にはクロージャ変数も直接書ける() {
    let add_one = |x: i32| x + 1;

    #[rustfmt::skip]
    graphite::flow! {
        41 -[add_one]-> answer,
    };
    assert_eq!(answer, 42);
}

#[test]
fn 関数式にはインラインクロージャリテラルも書ける() {
    #[rustfmt::skip]
    graphite::flow! {
        20 -[|x: i32| x * 2]-> doubled,
    };
    assert_eq!(doubled, 40);
}

// --- 束縛が flow! の後で見えること (文位置マクロ + call-site スパン) ---

#[test]
fn 束縛はflowの後で普通のローカル変数として見える() {
    fn parse(s: &str) -> i32 {
        s.parse().unwrap()
    }
    fn validate(x: i32) -> bool {
        x > 0
    }
    fn stats(x: i32) -> i32 {
        x * x
    }

    #[rustfmt::skip]
    graphite::flow! {
        "6" -[parse]-> parsed,
        parsed -[validate]-> valid,
        parsed -[stats]-> report,
    };

    // parsed/valid/report はすべて flow! の呼び出しの外側 (このテスト関数の
    // スコープ) で普通の let 束縛として見える。
    let combined: i32 = if valid { report } else { 0 };
    assert_eq!(parsed, 6);
    assert!(valid);
    assert_eq!(report, 36);
    assert_eq!(combined, 36);
}

// --- fan-out + fan-in を組み合わせた仕様どおりの例 ---

struct Report {
    summary: String,
}

#[test]
fn 仕様のfan_out_fan_in例が動く() {
    fn parse(s: &str) -> i32 {
        s.parse().unwrap()
    }
    fn validate(x: i32) -> bool {
        x >= 0
    }
    fn stats(x: i32) -> i32 {
        x * 2
    }
    fn merge(valid: bool, report: i32) -> Report {
        Report {
            summary: format!("valid={valid} report={report}"),
        }
    }

    #[rustfmt::skip]
    graphite::flow! {
        "21" -[parse]-> parsed,
        parsed -[validate]-> valid,
        parsed -[stats]-> report,
        (valid, report) -[merge]-> out,
    };
    assert_eq!(out.summary, "valid=true report=42");
}
