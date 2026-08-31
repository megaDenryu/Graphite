// static-org のアサーション (issue #24 段階2)。役割アクセサでの複数段の辿り・
// 積み荷アクセス (有向・無向の両方)・無向辺の端点アクセス・同一インスタンス
// 性・生成された `{個体名}Ref` struct への後付けimpl (公開契約、
// docs/static_graph.md 参照) を固定する。

use super::*;

#[test]
fn 役割アクセサで複数段辿れる() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);
    assert_eq!(g.node_refs.太郎.太郎の上司().superior().次郎の所属().team().entity().名前(), "開発部");
}

#[test]
fn 有向辺の積み荷へ役割名のアクセサでアクセスできる() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);
    assert_eq!(g.edge_refs.太郎の上司.任命().任命日, 2020);
}

#[test]
fn 無向辺の積み荷へ役割名のアクセサでアクセスできる() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);
    assert_eq!(g.edge_refs.太郎と一郎の同僚.経緯().経緯, "同期入社");
}

#[test]
fn 無向辺は宣言した役割名のアクセサで両端を返す() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);
    assert_eq!(g.edge_refs.太郎と次郎.甲().entity().名前(), "太郎");
    assert_eq!(g.edge_refs.太郎と次郎.乙().entity().名前(), "次郎");
}

#[test]
fn 辿った先は宣言された実体と同一インスタンスである() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);
    assert!(std::ptr::eq(g.node_refs.太郎.太郎の上司().superior().entity, &nodes.次郎));
    assert!(std::ptr::eq(g.edge_refs.太郎の上司.subordinate().entity, g.node_refs.太郎.entity));
}

#[test]
fn ノード参照が返す辺参照は辺参照達のものと同じ実体を指す() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);
    assert!(std::ptr::eq(g.node_refs.太郎.太郎の所属().entity, g.edge_refs.太郎の所属.entity));
}

#[test]
fn 個体参照へ後付けしたメソッドをチェーンの末尾で呼べる() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);
    assert_eq!(g.node_refs.太郎.あだ名(), "太郎くん");
}

// 同一schemaから `組織!` を2回目に呼んでも (経理チームの花子の所属先を求める 内で
// 2度目の 組織! 展開が起きる)、辺値struct群が重複定義エラーにならないことの
// 実証。ビルドが通ること自体が主な実証であり、このassertは辿りが正しく
// 動くことも合わせて確認する。
#[test]
fn 同一schemaから2つ目のグラフを宣言してもコンパイルと辿りが両方動く() {
    assert_eq!(経理チームの花子の所属先を求める(), "総務部");
}
