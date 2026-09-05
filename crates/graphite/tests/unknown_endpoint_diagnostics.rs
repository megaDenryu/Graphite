//! 未知の端点キーの参照と端点対の重複について、生成された診断文を検証する。
//!
//! 端点のキーと辺のIDは、生成ID型か利用者が宣言したID型かで表示できるかが
//! 別々に決まる。このファイルは4つの位置 (有向辺の始点・終点、無向辺の端点、
//! `unique pair` の端点対) をすべて4通りの組み合わせで試験できる schema を
//! 1つ宣言し、位置ごとの試験は `unknown_endpoint_positions` モジュールが持つ。
//! 各位置で、表示できる綴りだけが載り、省いた綴りには省いた理由が添うことを
//! 固定する (issue #26)。

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

        edge 生成キーの交友 = 生成キーの地点 -- 生成キーの地点;
        edge 宣言キーの交友(id: 利用者が宣言した経路キー) = 生成キーの地点 -- 生成キーの地点;
        edge 生成キーの親交 = 宣言キーの地点 -- 宣言キーの地点;
        edge 宣言キーの親交(id: 利用者が宣言した経路キー) = 宣言キーの地点 -- 宣言キーの地点;

        edge 両端が生成キーの専有経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点) where unique pair;
        edge 終点が宣言キーの専有経路 = (始点: 生成キーの地点) -> (終点: 宣言キーの地点) where unique pair;
        edge 始点が宣言キーの専有経路 = (始点: 宣言キーの地点) -> (終点: 生成キーの地点) where unique pair;
        edge 両端が宣言キーの専有経路 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点) where unique pair;
    }
}

mod unknown_endpoint_positions;

fn 生成キーの地点のキーを作る(綴り: &str) -> 診断::生成キーの地点Id {
    診断::生成キーの地点Id(綴り.into())
}

// builderへ積んだ内容を凍結させ、検査が拒否した違反を1件受け取る。
fn 構築に失敗させる(組み立てる: impl FnOnce(&mut 診断::Builder)) -> 診断::Violation {
    match 診断::Graph::create(組み立てる) {
        Err(違反) => 違反,
        Ok(_) => panic!("凍結時の検査は違反を検出するはず"),
    }
}
