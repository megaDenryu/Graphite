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
pub mod 世界 {
    include!("generated/graph_refs_世界.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/graph_refs_世界.rs";
    schema 世界 {
        node 人物;
        node 商品;
        edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair;
        edge 友人 = 人物 -- 人物 where unique pair;
    }
}

fn copyである<T: Copy>() {}

fn 構築する() -> 世界::Graph {
    graphite::graph!(世界 {
        太郎 = 人物 { 名前: "太郎".into() },
        花子 = 人物 { 名前: "花子".into() },
        本 = 商品 { 名前: "本".into() },
        太郎の購入 = 購入(太郎 -[取引情報 { 金額: 1200 }]-> 本),
        友人関係 = 友人(太郎 -- 花子),
    })
    .unwrap()
    .into_graph()
}

#[test]
fn 日本語名のnoderefとedgerefを生成して完成済み個体を参照できる() {
    copyである::<世界::人物Ref<'_>>();
    copyである::<世界::購入Ref<'_>>();
    assert_eq!(
        std::mem::size_of::<世界::人物Ref<'_>>(),
        std::mem::size_of::<usize>() * 2
    );
    assert_eq!(
        std::mem::size_of::<世界::購入Ref<'_>>(),
        std::mem::size_of::<usize>() * 2
    );

    let graph = 構築する();
    let 太郎id = 世界::人物Id("太郎".into());
    let 太郎: 世界::人物Ref<'_> = 世界::人物::get(&graph, &太郎id).unwrap();
    assert_eq!(太郎.id(), &太郎id);
    assert_eq!(太郎.value().名前, "太郎");
    assert_eq!(太郎.名前, "太郎");

    let 購入: 世界::購入Ref<'_> =
        世界::購入::get(&graph, &世界::購入Id("太郎の購入".into())).unwrap();
    assert_eq!(購入.id(), &世界::購入Id("太郎の購入".into()));
    assert_eq!(購入.購入者().id(), &太郎id);
    assert_eq!(購入.from_id(), &太郎id);
    assert_eq!(購入.対象商品().名前, "本");
    assert_eq!(購入.to_id(), &世界::商品Id("本".into()));
    assert_eq!(購入.取引().金額, 1200);
    assert_eq!(購入.payload().金額, 1200);
}

#[test]
fn iterはid付き値タプルではなくgraphに束縛されたrefを返す() {
    let graph = 構築する();
    let 人物ids: Vec<_> = 世界::人物::iter(&graph)
        .map(|person| person.id().clone())
        .collect();
    let 購入ids: Vec<_> = 世界::購入::iter(&graph)
        .map(|purchase| purchase.id().clone())
        .collect();

    assert_eq!(
        人物ids,
        vec![世界::人物Id("太郎".into()), 世界::人物Id("花子".into())]
    );
    assert_eq!(購入ids, vec![世界::購入Id("太郎の購入".into())]);
}

#[test]
fn 無向edgerefは方向を持たず両端のnoderefを返す() {
    let graph = 構築する();
    let relation = 世界::友人::get(&graph, &世界::友人Id("友人関係".into())).unwrap();
    let (first, second) = relation.endpoints();

    assert_eq!(first.id(), &世界::人物Id("太郎".into()));
    assert_eq!(second.id(), &世界::人物Id("花子".into()));
}

#[test]
fn graphの可変借用からノード値と辺の積み荷だけを更新できる() {
    let mut graph = 構築する();
    世界::人物::get_mut(&mut graph, &世界::人物Id("太郎".into()))
        .unwrap()
        .名前 = "太郎改".into();
    世界::購入::payload_mut(&mut graph, &世界::購入Id("太郎の購入".into()))
        .unwrap()
        .金額 = 1500;

    assert_eq!(
        世界::人物::get(&graph, &世界::人物Id("太郎".into()))
            .unwrap()
            .名前,
        "太郎改"
    );
    assert_eq!(
        世界::購入::get(&graph, &世界::購入Id("太郎の購入".into()))
            .unwrap()
            .payload()
            .金額,
        1500
    );
}

#[test]
fn 辺値はgraphへ挿入する前の通常値として構築できる() {
    let value = 世界::購入 {
        購入者: 世界::人物Id("太郎".into()),
        対象商品: 世界::商品Id("本".into()),
        取引: 取引情報 { 金額: 1200 },
    };

    assert_eq!(value.購入者, 世界::人物Id("太郎".into()));
    assert_eq!(value.payload().金額, 1200);
}
