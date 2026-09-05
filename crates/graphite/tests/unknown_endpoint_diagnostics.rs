//! 未知の端点キーを参照した違反の診断文を検証する。
//!
//! 辺のIDと端点のキーは、生成ID型か利用者が宣言したID型かで表示できるかが
//! 別々に決まる。4通りの組み合わせで、表示できる綴りだけが載り、解決できな
//! かった綴りが文頭へ出ることを固定する (issue #26)。

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct 利用者が宣言した地点キー(pub &'static str);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct 利用者が宣言した経路キー(pub &'static str);

pub struct 生成キーの地点;
pub struct 宣言キーの地点;

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod 診断 {
    include!("generated/unknown_endpoint_diagnostics_診断.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/unknown_endpoint_diagnostics_診断.rs";
    schema 診断 {
        node 生成キーの地点;
        node 宣言キーの地点(id: 利用者が宣言した地点キー);

        edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点);
        edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点);
        edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点);
        edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点);
    }
}

fn 生成キーの地点のキー(綴り: &str) -> 診断::生成キーの地点Id {
    診断::生成キーの地点Id(綴り.into())
}

// 存在しない始点を参照する辺を1本だけ持つグラフを組み、その違反を返す。
fn 構築に失敗させる(
    組み立てる: impl FnOnce(&mut 診断::Builder)
) -> 診断::Violation {
    match 診断::Graph::create(組み立てる) {
        Err(違反) => 違反,
        Ok(_) => panic!("未知の始点を参照した辺は拒否されるはず"),
    }
}

#[test]
fn 端点も辺idも生成id型なら両方の綴りを載せる() {
    let 違反 = 構築に失敗させる(|builder| {
        builder.生成キーの地点(生成キーの地点のキー("到達先"), 生成キーの地点);
        builder.生成キーの経路(
            診断::生成キーの経路Id("経路0".into()),
            診断::生成キーの経路::new(
                生成キーの地点のキー("ミオ"),
                生成キーの地点のキー("到達先"),
            ),
        );
    });
    assert_eq!(
        違反.to_string(),
        "未知のキー 生成キーの地点Id(\"ミオ\") が 生成キーの地点 として解決できません \
         (辺 `生成キーの経路` 生成キーの経路Id(\"経路0\") の始点)"
    );
    assert_eq!(format!("{違反:?}"), 違反.to_string());
}

#[test]
fn 辺idだけが利用者の宣言型なら端点の綴りだけを載せる() {
    let 違反 = 構築に失敗させる(|builder| {
        builder.生成キーの地点(生成キーの地点のキー("到達先"), 生成キーの地点);
        builder.宣言キーの経路(
            利用者が宣言した経路キー("経路0"),
            診断::宣言キーの経路::new(
                生成キーの地点のキー("ミオ"),
                生成キーの地点のキー("到達先"),
            ),
        );
    });
    assert_eq!(
        違反.to_string(),
        "未知のキー 生成キーの地点Id(\"ミオ\") が 生成キーの地点 として解決できません \
         (辺 `宣言キーの経路` の始点)"
    );
}

#[test]
fn 端点だけが利用者の宣言型なら辺の綴りだけを載せる() {
    let 違反 = 構築に失敗させる(|builder| {
        builder.宣言キーの地点(利用者が宣言した地点キー("到達先"), 宣言キーの地点);
        builder.生成キーの連絡(
            診断::生成キーの連絡Id("経路0".into()),
            診断::生成キーの連絡::new(
                利用者が宣言した地点キー("ミオ"),
                利用者が宣言した地点キー("到達先"),
            ),
        );
    });
    assert_eq!(
        違反.to_string(),
        "未知のキーが 宣言キーの地点 として解決できません \
         (辺 `生成キーの連絡` 生成キーの連絡Id(\"経路0\") の始点)"
    );
}

#[test]
fn 端点も辺idも利用者の宣言型なら綴りを載せない() {
    let 違反 = 構築に失敗させる(|builder| {
        builder.宣言キーの地点(利用者が宣言した地点キー("到達先"), 宣言キーの地点);
        builder.宣言キーの連絡(
            利用者が宣言した経路キー("経路0"),
            診断::宣言キーの連絡::new(
                利用者が宣言した地点キー("ミオ"),
                利用者が宣言した地点キー("到達先"),
            ),
        );
    });
    assert_eq!(
        違反.to_string(),
        "未知のキーが 宣言キーの地点 として解決できません (辺 `宣言キーの連絡` の始点)"
    );
}
