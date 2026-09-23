use super::{貸出中の蔵書を1件持つ図書グラフを組み立てる, 読書会グラフの割り当てを求める};

#[test]
fn 外部crateから生成した公開apiを呼べる() {
    let graph = 貸出中の蔵書を1件持つ図書グラフを組み立てる();
    assert_eq!(graph.book_len(), 1);
    assert_eq!(graph.borrowed_len(), 1);
    let 貸出 = graph.borrowed_iter().next().expect("辺が1本ある");
    assert_eq!(貸出.loan().day, 1);
    assert_eq!(貸出.reader().name, "検証");
}

#[test]
fn 外部crateから生成した無向の積み荷ありの辺の公開apiを呼べる() {
    let graph = 貸出中の蔵書を1件持つ図書グラフを組み立てる();
    assert_eq!(graph.recommended_len(), 1);
    let 推薦 = graph.recommended_iter().next().expect("無向辺が1本ある");
    assert_eq!(推薦.note().text, "面白い");
}

#[test]
fn 外部crateからstatic_graph_schemaの公開apiを呼べる() {
    assert_eq!(読書会グラフの割り当てを求める(), ("型で守るグラフ".to_string(), "検証".to_string()));
}
