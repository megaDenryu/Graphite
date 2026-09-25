//! 複製を選ばない schema の生成物が、利用者のノード値型と積み荷の型に `Clone` を要求しないことを固定する (issue #50)。
//!
//! 注意: 下の3つの型へ `Clone` を導出すると、この試験は何も確かめなくなる。
//! 3つの役割索引と端点対索引を全部持つ schema にしてあるのは、ランタイムの
//! 索引型へ足した `Clone` の導出が、複製を選ばない schema へ要求を漏らして
//! いないことを同時に確かめるためである。

pub struct 人物 {
    pub 名前: String,
}

pub struct 商品 {
    pub 名前: String,
}

pub struct 取引情報 {
    pub 金額: u64,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod 複製しない世界 {
    include!("generated/graph_clone_not_required_複製しない世界.rs");
}

#[rustfmt::skip]
graphite::dynamic_graph_schema! {
    generated = "generated/graph_clone_not_required_複製しない世界.rs";
    schema 複製しない世界 {
        node 人物;
        node 商品;
        edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair;
        edge 担当 = (担当者: 人物) -> (担当商品: 商品) where each 担当商品: 1, each 担当者: 0..1;
        edge 友人 = 人物 -- 人物 where unique pair;
    }
}

#[test]
fn 複製を選ばないschemaはcloneできない型のままグラフを構築して参照できる() {
    let グラフ = graphite::graph!(複製しない世界 {
        太郎 = 人物 { 名前: "太郎".into() },
        花子 = 人物 { 名前: "花子".into() },
        本 = 商品 { 名前: "本".into() },
        太郎の購入 = 購入(太郎 -[取引情報 { 金額: 1200 }]-> 本),
        本の担当 = 担当(花子 -> 本),
        友人関係 = 友人(太郎 -- 花子),
    })
    .expect("複製しない世界を構築できるはず");

    assert_eq!(グラフ.太郎の購入().取引().金額, 1200);
    assert_eq!(グラフ.本の担当().担当者().名前, "花子");
    assert_eq!(グラフ.太郎().友人_incident().count(), 1);
    assert_eq!(グラフ.本().名前, "本");
}
