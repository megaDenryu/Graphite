// GraphiteTS (../../GraphiteTS、issue #23) の設計を、マクロ無しの通常の Rust で
// この1ファイルに写す練習用ファイル。本文は自分で書く。
//
// TS 版との対応表 (書くときの道しるべ):
//   TS: type OrgNode = Person | Team (untagged union)
//     → Rust: enum OrgNode { Person(Person), Team(Team) } (tagged な直和)。
//       呼び出し側の書き味は impl From<Person> for OrgNode +
//       fn add_node(&mut self, n: impl Into<OrgNode>) で近づけられる
//   TS: Node<NodeId extends string | number> (ライブラリの能力契約)
//     → Rust: trait NodeId: Eq + Hash (+ Clone) のような能力トレイト、
//       または各ノードが fn id(&self) -> &str を持つだけでもよい
//   TS: graph.edgesFrom(node, Boss) -> Boss | undefined (instanceof で絞る)
//     → Rust: fn edges_from<E: EdgeKind>(...) の形。OrgEdge から &Boss を
//       取り出す変換 (TryFrom や accessor) を辺の種類ごとに手書きする。
//       ここが graph_schema! の消している定型文の本体
//   TS: TraversalResult<E> (条件型で E / E|undefined / readonly E[])
//     → Rust: trait EdgeKind { type TraversalOut; } の関連型。
//       ExactlyOne は E、ZeroOrOne は Option<E>、Many は Vec<E>
//   TS: freeze() の実行時検証 (多重度の下限が検出できなかった)
//     → Rust: enum は閉じているので、辺の種類を列挙して下限検査まで書ける
//       (TS 版で「proc macro が本質的に必要」だった部分が enum で埋まるかの確認)
//
// 実行場所: C:\devs\Graphite\examples\graphitets-by-hand で
//   cargo run
//   cargo test
//
// このファイルは1ファイル100行の原則の例外である (区分: 再設計待ち)。この
// ファイルは150行を超える。このファイルは GraphiteTS の動的グラフ版を通常の
// Rust へ写す練習用ファイルである。扱いの判定は issue #28 のやること4 が行
// う。超過を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。

use std::collections::HashMap;
use std::hash::Hash;

trait NodeId: Eq + Hash + Clone {}

trait Node {
    type Id: NodeId;
    fn id(&self) -> Self::Id;
}

trait EdgeId: Eq + Hash + Clone {}

trait Edge {
    type Id: EdgeId;
    type StartId: NodeId;
    type EndId: NodeId;
    fn id(&self) -> Self::Id;
    fn start_id(&self) -> Self::StartId;
    fn end_id(&self) -> Self::EndId;
}

trait Graph
where
    <Self::Edge as Edge>::StartId: Into<<Self::Node as Node>::Id>,
    <Self::Edge as Edge>::EndId: Into<<Self::Node as Node>::Id>,
{
    type Node: Node;
    type Edge: Edge;
}

struct 単純ノード<Id> {
    id: Id,
}
impl<I: NodeId> Node for 単純ノード<I> {
    type Id = I;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

struct 単純辺<Id, StartId, EndId> {
    id: Id,
    start: StartId,
    end: EndId,
}
impl<I: EdgeId, S: NodeId, E: NodeId> Edge for 単純辺<I, S, E> {
    type Id = I;
    type StartId = S;
    type EndId = E;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn start_id(&self) -> Self::StartId {
        self.start.clone()
    }
    fn end_id(&self) -> Self::EndId {
        self.end.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct 男Id(String);
impl NodeId for 男Id {}
type 男 = 単純ノード<男Id>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct 女Id(String);
impl NodeId for 女Id {}
type 女 = 単純ノード<女Id>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct 恋人関係Id(String);
impl EdgeId for 恋人関係Id {}
type 恋人関係 = 単純辺<恋人関係Id, 男Id, 女Id>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct 友達関係Id(String);
impl EdgeId for 友達関係Id {}
type 男友達関係 = 単純辺<友達関係Id, 男Id, 男Id>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct 女友達関係Id(String);
impl EdgeId for 女友達関係Id {}
type 女友達関係 = 単純辺<女友達関係Id, 女Id, 女Id>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct 男女友達関係Id(String);
impl EdgeId for 男女友達関係Id {}
type 男女友達関係 = 単純辺<男女友達関係Id, 男Id, 女Id>;

#[derive(Clone, PartialEq, Eq, Hash)]
enum 人物Id {
    男(男Id),
    女(女Id),
}
impl NodeId for 人物Id {}
impl From<男Id> for 人物Id {
    fn from(x: 男Id) -> Self {
        人物Id::男(x)
    }
}
impl From<女Id> for 人物Id {
    fn from(x: 女Id) -> Self {
        人物Id::女(x)
    }
}

enum 人物 {
    男(男),
    女(女),
}
impl Node for 人物 {
    type Id = 人物Id;
    fn id(&self) -> Self::Id {
        match self {
            人物::男(x) => 人物Id::男(x.id()),
            人物::女(x) => 人物Id::女(x.id()),
        }
    }
}
impl From<男> for 人物 {
    fn from(x: 男) -> Self {
        人物::男(x)
    }
}
impl From<女> for 人物 {
    fn from(x: 女) -> Self {
        人物::女(x)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum 関係Id {
    恋人(恋人関係Id),
    男友達(友達関係Id),
    女友達(女友達関係Id),
    男女友達(男女友達関係Id),
}
impl EdgeId for 関係Id {}

enum 関係 {
    恋人(恋人関係),
    男友達(男友達関係),
    女友達(女友達関係),
    男女友達(男女友達関係),
}
impl Edge for 関係 {
    type Id = 関係Id;
    type StartId = 人物Id;
    type EndId = 人物Id;
    fn id(&self) -> Self::Id {
        match self {
            関係::恋人(x) => 関係Id::恋人(x.id()),
            関係::男友達(x) => 関係Id::男友達(x.id()),
            関係::女友達(x) => 関係Id::女友達(x.id()),
            関係::男女友達(x) => 関係Id::男女友達(x.id()),
        }
    }
    fn start_id(&self) -> Self::StartId {
        match self {
            関係::恋人(x) => x.start_id().into(),
            関係::男友達(x) => x.start_id().into(),
            関係::女友達(x) => x.start_id().into(),
            関係::男女友達(x) => x.start_id().into(),
        }
    }
    fn end_id(&self) -> Self::EndId {
        match self {
            関係::恋人(x) => x.end_id().into(),
            関係::男友達(x) => x.end_id().into(),
            関係::女友達(x) => x.end_id().into(),
            関係::男女友達(x) => x.end_id().into(),
        }
    }
}

// 汎用グラフ: HashMap による一元管理・freeze検証・辿りを1回だけ書けば、
// どの schema (N, E の組) でも使い回せる。ビルダーは独立した型として
// 持たず、new() の中で一時的な HashMap に集めてから検証する。
struct 汎用グラフ<N: Node, E: Edge<StartId = N::Id, EndId = N::Id>> {
    ノード一覧: HashMap<N::Id, N>,
    辺一覧: HashMap<E::Id, E>,
}
impl<N: Node, E: Edge<StartId = N::Id, EndId = N::Id>> 汎用グラフ<N, E> {
    fn new(
        nodes: impl IntoIterator<Item = N>,
        edges: impl IntoIterator<Item = E>,
    ) -> Result<Self, String> {
        let mut ノード一覧 = HashMap::new();
        for n in nodes {
            ノード一覧.insert(n.id(), n);
        }
        let mut 辺一覧 = HashMap::new();
        for e in edges {
            辺一覧.insert(e.id(), e);
        }
        for e in 辺一覧.values() {
            if !ノード一覧.contains_key(&e.start_id()) {
                return Err("start ノードが存在しない".to_string());
            }
            if !ノード一覧.contains_key(&e.end_id()) {
                return Err("end ノードが存在しない".to_string());
            }
        }
        Ok(Self { ノード一覧, 辺一覧 })
    }
    fn ノード(&self, id: &N::Id) -> Option<&N> {
        self.ノード一覧.get(id)
    }
    fn 辺(&self, id: &E::Id) -> Option<&E> {
        self.辺一覧.get(id)
    }
    fn 辺の全件(&self) -> impl Iterator<Item = &E> {
        self.辺一覧.values()
    }
    fn 始点(&self, e: &E) -> &N {
        self.ノード一覧
            .get(&e.start_id())
            .expect("frozen graph は endpoint が必ず存在する")
    }
    fn 終点(&self, e: &E) -> &N {
        self.ノード一覧
            .get(&e.end_id())
            .expect("frozen graph は endpoint が必ず存在する")
    }
}

type 人間関係グラフ = 汎用グラフ<人物, 関係>;
impl Graph for 人間関係グラフ {
    type Node = 人物;
    type Edge = 関係;
}

impl From<恋人関係> for 関係 {
    fn from(x: 恋人関係) -> Self {
        関係::恋人(x)
    }
}
impl From<男友達関係> for 関係 {
    fn from(x: 男友達関係) -> Self {
        関係::男友達(x)
    }
}
impl From<女友達関係> for 関係 {
    fn from(x: 女友達関係) -> Self {
        関係::女友達(x)
    }
}
impl From<男女友達関係> for 関係 {
    fn from(x: 男女友達関係) -> Self {
        関係::男女友達(x)
    }
}

// Edge と同じ名前の関連型を持つ trait が仮にあったら、という実験。
// わざとエラーが出たままにしてある (E0221 が実際に見える)。
trait HasCache {
    type StartId; // Edge と同じ名前
}

fn ambiguous_demo<T: Edge + HasCache>() -> <T as Edge>::StartId {
    todo!()
}

fn main() {
    let 太郎id = 男Id("太郎".into());
    let 花子id = 女Id("花子".into());

    let graph = 人間関係グラフ::new(
        [
            男 { id: 太郎id.clone() }.into(),
            女 { id: 花子id.clone() }.into(),
        ],
        [恋人関係 {
            id: 恋人関係Id("r1".into()),
            start: 太郎id.clone(),
            end: 花子id.clone(),
        }
        .into()],
    )
    .unwrap();

    let rel = graph.辺(&関係Id::恋人(恋人関係Id("r1".into()))).unwrap();
    let 始点 = graph.始点(rel);
    let もう一度 = graph.ノード(&人物Id::男(太郎id)).unwrap();
    assert!(std::ptr::eq(始点, もう一度));

    for e in graph.辺の全件() {
        let _ = graph.始点(e);
        let _ = graph.終点(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 始点と同じidのノード参照は同一インスタンスである() {
        let id = 男Id("a".into());
        let graph = 人間関係グラフ::new(
            [
                男 { id: id.clone() }.into(),
                女 {
                    id: 女Id("b".into()),
                }
                .into(),
            ],
            [恋人関係 {
                id: 恋人関係Id("r".into()),
                start: id.clone(),
                end: 女Id("b".into()),
            }
            .into()],
        )
        .unwrap();

        let e = graph
            .辺(&関係Id::恋人(恋人関係Id("r".into())))
            .unwrap();
        let 始点 = graph.始点(e);
        let 直接引き = graph.ノード(&人物Id::男(id)).unwrap();
        assert!(std::ptr::eq(始点, 直接引き));
    }

    #[test]
    fn 存在しない端点は_freeze_で失敗する() {
        let result = 人間関係グラフ::new(
            [],
            [恋人関係 {
                id: 恋人関係Id("r".into()),
                start: 男Id("no-one".into()),
                end: 女Id("no-one-2".into()),
            }
            .into()],
        );
        assert!(result.is_err());
    }
}
