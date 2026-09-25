//! `#[derive(Clone)]` を選んだ schema の完成済みグラフの複製と、複製と構築印と参照の関係を固定する (issue #50)。
//!
//! 3つの役割索引 (多重度1・0..1・制約なし) と無向辺の端点対索引を全部持つ
//! schema にし、`Graph` の導出が要する索引の複製を1つ残らず通す。

#[derive(Debug, Clone, PartialEq)]
pub struct 人物 {
    pub 名前: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 商品 {
    pub 名前: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 取引情報 {
    pub 金額: u64,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod 複製世界 {
    include!("generated/graph_clone_複製世界.rs");
}

#[rustfmt::skip]
graphite::dynamic_graph_schema! {
    generated = "generated/graph_clone_複製世界.rs";
    #[derive(Clone)]
    schema 複製世界 {
        node 人物;
        node 商品;
        edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair;
        edge 担当 = (担当者: 人物) -> (担当商品: 商品) where each 担当商品: 1, each 担当者: 0..1;
        edge 友人 = 人物 -- 人物 where unique pair;
    }
}

use 複製世界::{人物Id, 購入Id};

macro_rules! 世界を構築する {
    () => {
        graphite::graph!(複製世界 {
            太郎 = 人物 { 名前: "太郎".into() },
            花子 = 人物 { 名前: "花子".into() },
            本 = 商品 { 名前: "本".into() },
            太郎の購入 = 購入(太郎 -[取引情報 { 金額: 1200 }]-> 本),
            本の担当 = 担当(花子 -> 本),
            友人関係 = 友人(太郎 -- 花子),
        })
        .expect("複製世界を構築できるはず")
    };
}

#[test]
fn 複製したグラフは元と同じ個体と辺と索引を持ち元から独立している() {
    let 元 = 世界を構築する!().into_graph();
    let mut 複製 = 元.clone();

    let 太郎 = 複製.人物_by_id(&人物Id("太郎".into())).expect("太郎がいるはず");
    let 本 = 太郎.購入_between(複製.商品_iter().next().expect("本があるはず"));
    assert_eq!(本.expect("購入が複製されているはず").取引().金額, 1200);
    assert_eq!(複製.商品_iter().next().expect("本").担当_as_担当商品().担当者().名前, "花子");
    assert_eq!(太郎.友人_incident().count(), 1);

    複製.購入_payload_mut(&購入Id("太郎の購入".into())).expect("購入").金額 = 1;
    複製.人物_value_mut(&人物Id("太郎".into())).expect("太郎").名前 = "複製の太郎".into();
    assert_eq!(元.購入_by_id(&購入Id("太郎の購入".into())).expect("購入").取引().金額, 1200);
    assert_eq!(元.人物_by_id(&人物Id("太郎".into())).expect("太郎").名前, "太郎");
}

#[test]
fn 名前付きラッパーの複製は静的アクセサで複製したグラフを指す() {
    let 元 = 世界を構築する!();
    let mut 複製 = 元.clone();
    複製.購入_payload_mut(&購入Id("太郎の購入".into())).expect("購入").金額 = 1;

    assert_eq!(複製.太郎の購入().取引().金額, 1);
    assert_eq!(元.太郎の購入().取引().金額, 1200);
    assert_eq!(複製.本の担当().担当者().id(), 複製.花子().id());
}

#[test]
fn 複製は構築印を引き継ぐので元と複製の参照の組は不一致にならず受け手のグラフに束縛される() {
    let 元 = 世界を構築する!();
    let mut 複製 = 元.clone();
    複製.購入_payload_mut(&購入Id("太郎の購入".into())).expect("購入").金額 = 1;

    let 辺 = 元
        .太郎()
        .購入_try_between(複製.本())
        .expect("構築印が同じなので不一致にならないはず")
        .expect("購入があるはず");
    assert_eq!(辺.取引().金額, 1200);
}

#[test]
fn 同じ内容を別に構築したグラフは構築印が異なり参照の組を拒否する() {
    let 一つ目 = 世界を構築する!();
    let 二つ目 = 世界を構築する!();

    assert!(一つ目.太郎().購入_try_between(二つ目.本()).is_err());
}
