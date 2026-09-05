// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/graph_refs.rs:24
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    14445986729217011174u64, 9351231871506818459u64, 7227283738532981452u64,
    14248187126226662416u64,
];
/// `人物` ノードの公開ID。
///
/// 宣言: `tests/graph_refs.rs` の `node 人物`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 人物Id(pub String);
/// `商品` ノードの公開ID。
///
/// 宣言: `tests/graph_refs.rs` の `node 商品`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 商品Id(pub String);
/// `購入` 辺の公開ID。
///
/// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 購入Id(pub String);
/// `友人` 辺の公開ID。
///
/// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 友人Id(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __人物InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __商品InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __購入InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __友人InternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __人物NamedPosition(__人物InternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __商品NamedPosition(__商品InternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __購入NamedPosition(__購入InternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __友人NamedPosition(__友人InternalPosition, u64);
/// 構築時に組み立てる `購入` 辺の値。
///
/// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
#[derive(Clone)]
pub struct 購入 {
    /// この辺の始点ノードの公開ID。
    pub 購入者: 人物Id,
    /// この辺の終点ノードの公開ID。
    pub 対象商品: 商品Id,
    /// この辺が運ぶ積み荷。
    pub 取引: 取引情報,
}
impl 購入 {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn new(from: 人物Id, to: 商品Id, payload: 取引情報) -> Self {
        Self {
            購入者: from,
            対象商品: to,
            取引: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn payload(&self) -> &取引情報 {
        &self.取引
    }
}
impl graphite::DirectedEdgeLiteral<人物Id, 商品Id, 取引情報> for 購入 {
    fn from_graph_literal(from: 人物Id, to: 商品Id, payload: 取引情報) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for 購入 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(購入))
    }
}
/// 構築時に組み立てる `友人` 辺の値。
///
/// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
#[derive(Clone, PartialEq)]
pub struct 友人 {
    endpoints: graphite::UnorderedPair<人物Id>,
}
impl 友人 {
    /// 両端の公開IDから構築用の辺値を作る。両端の順序は保たない。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn new(a: 人物Id, b: 人物Id) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
        }
    }
    /// この辺値の両端の公開IDを順序なし対として借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn endpoints(&self) -> (&人物Id, &人物Id) {
        self.endpoints.endpoints()
    }
}
impl graphite::UndirectedEdgeLiteral<人物Id, ()> for 友人 {
    fn from_graph_literal(a: 人物Id, b: 人物Id, (): ()) -> Self {
        Self::new(a, b)
    }
}
impl std::fmt::Debug for 友人 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(友人))
            .field(&self.endpoints().0)
            .field(&self.endpoints().1)
            .finish()
    }
}
#[allow(dead_code)]
struct __購入Record {
    購入者: __人物InternalPosition,
    対象商品: __商品InternalPosition,
    取引: 取引情報,
}
#[allow(dead_code)]
struct __友人Record {
    endpoints: graphite::UnorderedPair<__人物InternalPosition>,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `tests/graph_refs.rs` の `schema 世界`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    Duplicate人物(人物Id),
    /// このノード種別のキーが重複している。
    Duplicate商品(商品Id),
    /// このエッジ種別のキーが重複している。
    購入DuplicateKey(購入Id),
    /// このエッジが未知の始点キーを参照している。
    購入UnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: 購入Id,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: 人物Id,
    },
    /// このエッジが未知の終点キーを参照している。
    購入UnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: 購入Id,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: 商品Id,
    },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    購入UniquePairViolation {
        /// 2本目の辺が張られた対の始点ノードの公開ID。
        source: 人物Id,
        /// 2本目の辺が張られた対の終点ノードの公開ID。
        target: 商品Id,
    },
    /// このエッジ種別のキーが重複している。
    友人DuplicateKey(友人Id),
    /// このエッジが未知の端点キーを参照している (無向のため位置の
    /// 区別は無い)。
    友人UnknownEndpoint {
        /// 未知のキーを参照した辺の公開ID。
        edge: 友人Id,
        /// この辺が端点として参照した、対応するノードが存在しないキー。
        endpoint: 人物Id,
    },
    /// このエッジ種別の `unique pair` 違反 (無向のため
    /// 順序を無視した対で判定)。
    友人UniquePairViolation {
        /// 2本目の辺が張られた対の一方の端点の公開ID。
        a: 人物Id,
        /// 2本目の辺が張られた対のもう一方の端点の公開ID。
        b: 人物Id,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::Duplicate人物(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "人物", id)
            }
            Violation::Duplicate商品(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "商品", id)
            }
            Violation::購入DuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "購入", id)
            }
            Violation::購入UnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "購入", edge, "人物", source
                )
            }
            Violation::購入UnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "購入", edge, "商品", target
                )
            }
            Violation::購入UniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "購入", source, target
                )
            }
            Violation::友人DuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "友人", id)
            }
            Violation::友人UnknownEndpoint { edge, endpoint } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の端点, {}): {:?}",
                    "友人", edge, "人物", endpoint
                )
            }
            Violation::友人UniquePairViolation { a, b } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {{{:?}, {:?}}} の対に既に辺が存在します",
                    "友人", a, b
                )
            }
        }
    }
}
impl std::fmt::Debug for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl std::error::Error for Violation {}
/// 凍結済み図式グラフ。構築後の構造は不変で、ノード値と辺の積み荷だけを
/// `&mut Graph` を要求する種別APIから更新できる。
///
/// 宣言: `tests/graph_refs.rs` の `schema 世界`
pub struct Graph {
    __graphite_node_人物: graphite::KeyedTable<人物Id, super::人物>,
    __graphite_node_商品: graphite::KeyedTable<商品Id, super::商品>,
    購入: graphite::KeyedTable<購入Id, __購入Record>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    購入_from_index: graphite::MultipleRoleIndex<__購入InternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    購入_to_index: graphite::MultipleRoleIndex<__購入InternalPosition>,
    __graphite_購入_by_pair: std::collections::HashMap<
        (__人物InternalPosition, __商品InternalPosition),
        __購入InternalPosition,
    >,
    友人: graphite::KeyedTable<友人Id, __友人Record>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    友人_index: graphite::MultipleRoleIndex<__友人InternalPosition>,
    __graphite_友人_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__人物InternalPosition>,
        __友人InternalPosition,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn 人物_by_id<'graph>(
        &'graph self,
        id: &人物Id,
    ) -> Option<人物Ref<'graph>> {
        let internal_position = __人物InternalPosition(
            self.__graphite_node_人物.position(id)?,
        );
        Some(人物Ref {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn 人物_value_mut(&mut self, id: &人物Id) -> Option<&mut super::人物> {
        self.__graphite_node_人物.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn 人物_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph 人物Id> {
        self.__graphite_node_人物.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn 人物_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 人物Ref<'graph>> + 'graph {
        self.__graphite_node_人物
            .positions()
            .map(move |position| 人物Ref {
                graph: self,
                internal_position: __人物InternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn 人物_len(&self) -> usize {
        self.__graphite_node_人物.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn 商品_by_id<'graph>(
        &'graph self,
        id: &商品Id,
    ) -> Option<商品Ref<'graph>> {
        let internal_position = __商品InternalPosition(
            self.__graphite_node_商品.position(id)?,
        );
        Some(商品Ref {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn 商品_value_mut(&mut self, id: &商品Id) -> Option<&mut super::商品> {
        self.__graphite_node_商品.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn 商品_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph 商品Id> {
        self.__graphite_node_商品.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn 商品_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 商品Ref<'graph>> + 'graph {
        self.__graphite_node_商品
            .positions()
            .map(move |position| 商品Ref {
                graph: self,
                internal_position: __商品InternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn 商品_len(&self) -> usize {
        self.__graphite_node_商品.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_by_id<'graph>(
        &'graph self,
        id: &購入Id,
    ) -> Option<購入Ref<'graph>> {
        Some(購入Ref {
            graph: self,
            internal_position: __購入InternalPosition(self.購入.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_payload_mut(&mut self, id: &購入Id) -> Option<&mut 取引情報> {
        self.購入.get_mut(id).map(|record: &mut __購入Record| &mut record.取引)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph 購入Id> {
        self.購入.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 購入Ref<'graph>> + 'graph {
        self.購入
            .positions()
            .map(move |position| 購入Ref {
                graph: self,
                internal_position: __購入InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_len(&self) -> usize {
        self.購入.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人_by_id<'graph>(
        &'graph self,
        id: &友人Id,
    ) -> Option<友人Ref<'graph>> {
        Some(友人Ref {
            graph: self,
            internal_position: __友人InternalPosition(self.友人.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph 友人Id> {
        self.友人.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 友人Ref<'graph>> + 'graph {
        self.友人
            .positions()
            .map(move |position| 友人Ref {
                graph: self,
                internal_position: __友人InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人_len(&self) -> usize {
        self.友人.len()
    }
    /// builder をクロージャに貸し出し、戻ったら凍結して図式適合
    /// (端点種別・where 制約) を一括検査する。最初の1件の違反で
    /// `Err` になる (複数の違反を全件見たい場合は
    /// [`Self::create_collecting`] を使う)。
    pub fn create<F>(f: F) -> Result<Self, Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze()
    }
    /// `graph!` が名前付き要素の名前付き位置を凍結境界の外へ運ぶための
    /// 内部構築経路。`Graph` の凍結に成功した場合だけ名前付き位置を返す。
    /// [`graphite::build_named_graph`] へ薄く委譲するだけで、
    /// [`graphite::NamedInsertPermit`] はそちらでしか作らない
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn create_named<F, N>(f: F) -> Result<(Self, N), Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder, &'b graphite::NamedInsertPermit) -> N,
    {
        graphite::build_named_graph(Builder::new, f)
    }
    /// [`Self::create`] の複数違反収集版。builder をクロージャに
    /// 貸し出し、戻ったら凍結して図式適合を検査する点は `create` と
    /// 同じだが、最初の1件で打ち切らず全違反を `Vec` に集めて返す。
    pub fn create_collecting<F>(f: F) -> Result<Self, Vec<Violation>>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze_collecting()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
#[derive(Clone, Copy)]
pub struct 購入Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __購入InternalPosition,
}
impl<'graph> 購入Ref<'graph> {
    fn record(self) -> &'graph __購入Record {
        self.graph
            .購入
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn id(self) -> &'graph 購入Id {
        self.graph
            .購入
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入者(self) -> 人物Ref<'graph> {
        人物Ref {
            graph: self.graph,
            internal_position: __人物InternalPosition(self.record().購入者.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 対象商品(self) -> 商品Ref<'graph> {
        商品Ref {
            graph: self.graph,
            internal_position: __商品InternalPosition(self.record().対象商品.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn from(self) -> 人物Ref<'graph> {
        self.購入者()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn to(self) -> 商品Ref<'graph> {
        self.対象商品()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn from_id(self) -> &'graph 人物Id {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn to_id(self) -> &'graph 商品Id {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 取引(self) -> &'graph 取引情報 {
        &self.record().取引
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn payload(self) -> &'graph 取引情報 {
        &self.record().取引
    }
}
impl<'graph> std::fmt::Debug for 購入Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(購入Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の無向辺個体。
///
/// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
#[derive(Clone, Copy)]
pub struct 友人Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __友人InternalPosition,
}
impl<'graph> 友人Ref<'graph> {
    fn record(self) -> &'graph __友人Record {
        self.graph
            .友人
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn id(self) -> &'graph 友人Id {
        self.graph
            .友人
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の両端を順序なし対として返す。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn endpoints(self) -> (人物Ref<'graph>, 人物Ref<'graph>) {
        let (first, second) = self.record().endpoints.endpoints();
        (
            人物Ref {
                graph: self.graph,
                internal_position: __人物InternalPosition(first.0),
            },
            人物Ref {
                graph: self.graph,
                internal_position: __人物InternalPosition(second.0),
            },
        )
    }
}
impl<'graph> std::fmt::Debug for 友人Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(友人Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `tests/graph_refs.rs` の `schema 世界`
pub struct Builder {
    __graphite_node_人物: Vec<(人物Id, super::人物)>,
    __graphite_node_商品: Vec<(商品Id, super::商品)>,
    購入: Vec<(購入Id, 購入)>,
    友人: Vec<(友人Id, 友人)>,
    /// この構築を識別する構築印。`Builder::new()` が発行し、この
    /// `Builder` から挿入する全ての名前付き位置と、凍結成功後の
    /// `Graph` へ同じ値を刻む。
    __graphite_construction_stamp: u64,
}
/// 型付き ID を受け取るノード・エッジ共通の挿入トレイト。
///
/// 署名が `insert_with_id(self, b, id)` と、挿入される値を receiver に
/// して `Builder` を引数で受ける向きなのは、`graph!` がノード項の値の
/// 型を解析せず、正しい内部ストレージへの振り分けを値の型の trait
/// ディスパッチに頼るためである。利用者向けの公開入口は
/// `Builder::insert`/`Builder::add` の側にある。
///
/// `insert_named_with_id` は [`graphite::NamedInsertPermit`] を要求する
/// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
/// `insert_with_id` (許可証不要、名前付き位置を返さない) は独立した
/// 実装を持ち、`insert_named_with_id` を経由しない
/// (`create` のクロージャから許可証なしで呼べる必要があるため)。
pub trait 世界Insertable: Sized {
    /// この要素を挿入したときに受け取る公開ID型。
    type Id;
    #[doc(hidden)]
    type NamedPosition;
    #[doc(hidden)]
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    /// 型付きの公開IDを指定して、この要素を `Builder` へ挿入する。
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id;
}
/// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
/// 実装する。明示ID型には実装せず、文字列変換を要求しない。
pub trait 世界DefaultId: 世界Insertable {
    #[doc(hidden)]
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    /// 束縛名の文字列から既定IDを作り、この要素を `Builder` へ挿入する。
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id;
}
/// ノード挿入で使うトレイト境界。読み取りは `Graph` の種別メソッドと
/// `NodeRef` のメソッドが提供する。利用者がこのトレイトのメソッドを
/// 直接呼ぶことは想定しない。
pub trait 世界Node: 世界Insertable {}
impl 世界Insertable for super::人物 {
    type Id = 人物Id;
    type NamedPosition = __人物NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __人物NamedPosition(
            __人物InternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_人物.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.人物(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.人物(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __人物NamedPosition {
    type Reference<'graph> = 人物Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        人物Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 世界DefaultId for super::人物 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        世界Insertable::insert_named_with_id(self, b, 人物Id(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        世界Insertable::insert_with_id(self, b, 人物Id(binding))
    }
}
impl 世界Node for super::人物 {}
/// 完成済みグラフ上の `人物` ノード個体。
///
/// 宣言: `tests/graph_refs.rs` の `node 人物`
#[derive(Clone, Copy)]
pub struct 人物Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __人物InternalPosition,
}
impl<'graph> 人物Ref<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn id(self) -> &'graph 人物Id {
        self.graph
            .__graphite_node_人物
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn value(self) -> &'graph super::人物 {
        self.graph
            .__graphite_node_人物
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_as_購入者(
        self,
    ) -> impl Iterator<Item = 購入Ref<'graph>> + 'graph {
        let positions = self.graph.購入_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 購入Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_try_between(
        self,
        other: 商品Ref<'graph>,
    ) -> Result<Option<購入Ref<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_購入_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| 購入Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::購入_try_between`] を使う。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_between(self, other: 商品Ref<'graph>) -> Option<購入Ref<'graph>> {
        self.購入_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(人物Ref), stringify!(購入_between)
                )
            })
    }
    /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人_incident(self) -> impl Iterator<Item = 友人Ref<'graph>> + 'graph {
        let positions = self.graph.友人_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 友人Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序なし端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人_try_between(
        self,
        other: 人物Ref<'graph>,
    ) -> Result<Option<友人Ref<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_友人_by_pair
            .get(
                &graphite::UnorderedPair::new(
                    self.internal_position,
                    other.internal_position,
                ),
            )
            .copied();
        Ok(
            found
                .map(|internal_position| 友人Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::友人_try_between`] を使う。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人_between(self, other: 人物Ref<'graph>) -> Option<友人Ref<'graph>> {
        self.友人_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(人物Ref), stringify!(友人_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for 人物Ref<'graph> {
    type Target = super::人物;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_人物
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for 人物Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(人物Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl 世界Insertable for super::商品 {
    type Id = 商品Id;
    type NamedPosition = __商品NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __商品NamedPosition(
            __商品InternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_商品.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.商品(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.商品(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __商品NamedPosition {
    type Reference<'graph> = 商品Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        商品Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 世界DefaultId for super::商品 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        世界Insertable::insert_named_with_id(self, b, 商品Id(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        世界Insertable::insert_with_id(self, b, 商品Id(binding))
    }
}
impl 世界Node for super::商品 {}
/// 完成済みグラフ上の `商品` ノード個体。
///
/// 宣言: `tests/graph_refs.rs` の `node 商品`
#[derive(Clone, Copy)]
pub struct 商品Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __商品InternalPosition,
}
impl<'graph> 商品Ref<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn id(self) -> &'graph 商品Id {
        self.graph
            .__graphite_node_商品
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn value(self) -> &'graph super::商品 {
        self.graph
            .__graphite_node_商品
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入_as_対象商品(
        self,
    ) -> impl Iterator<Item = 購入Ref<'graph>> + 'graph {
        let positions = self.graph.購入_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 購入Ref {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for 商品Ref<'graph> {
    type Target = super::商品;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_商品
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for 商品Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(商品Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait 世界Edge: 世界Insertable {}
impl 世界Insertable for 購入 {
    type Id = 購入Id;
    type NamedPosition = __購入NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __購入NamedPosition(
            __購入InternalPosition(
                graphite::TablePosition::from_index(b.購入.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.購入(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.購入(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __購入NamedPosition {
    type Reference<'graph> = 購入Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        購入Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 世界DefaultId for 購入 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        世界Insertable::insert_named_with_id(self, b, 購入Id(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        世界Insertable::insert_with_id(self, b, 購入Id(binding))
    }
}
impl 世界Edge for 購入 {}
impl 世界Insertable for 友人 {
    type Id = 友人Id;
    type NamedPosition = __友人NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __友人NamedPosition(
            __友人InternalPosition(
                graphite::TablePosition::from_index(b.友人.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.友人(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.友人(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __友人NamedPosition {
    type Reference<'graph> = 友人Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        友人Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 世界DefaultId for 友人 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        世界Insertable::insert_named_with_id(self, b, 友人Id(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        世界Insertable::insert_with_id(self, b, 友人Id(binding))
    }
}
impl 世界Edge for 友人 {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_人物: Vec::new(),
            __graphite_node_商品: Vec::new(),
            購入: Vec::new(),
            友人: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 人物`
    pub fn 人物(&mut self, id: 人物Id, value: super::人物) -> &mut Self {
        self.__graphite_node_人物.push((id, value));
        self
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/graph_refs.rs` の `node 商品`
    pub fn 商品(&mut self, id: 商品Id, value: super::商品) -> &mut Self {
        self.__graphite_node_商品.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair`
    pub fn 購入(&mut self, id: 購入Id, value: 購入) -> &mut Self {
        self.購入.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/graph_refs.rs` の `edge 友人 = 人物 -- 人物 where unique pair`
    pub fn 友人(&mut self, id: 友人Id, value: 友人) -> &mut Self {
        self.友人.push((id, value));
        self
    }
    /// 型名付きメソッド (`b.#accessor(id, value)` 群、上記
    /// `#node_methods`) の総称版。`graph!` の左辺名付きノード項は
    /// 下記 `insert_named` (名前付き位置を返す許可証付き経路) へ
    /// 脱糖するため、このメソッド自体は `graph!` を経由しない。
    /// 値の型を手書きで組み立てる場合 (プログラム的構築など) に使う。
    /// `graph!` はノード項の値の型を一切パースしないため
    /// (`key = 式` の「式」でしかない)、値の型 (`N: #node_trait_ident`)
    /// から正しい内部ストレージへの振り分けを rustc の型推論任せに
    /// する点は `insert_named` と共通。命名判断・trait の形は
    /// `gen_node_trait_and_impls` のドキュメントコメント参照。
    pub fn insert<N>(&mut self, key: impl Into<String>, value: N) -> N::Id
    where
        N: 世界Node + 世界DefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き要素の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named<N>(
        &mut self,
        key: impl Into<String>,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition)
    where
        N: 世界Node + 世界DefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: 世界Node>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: 世界Node>(
        &mut self,
        id: N::Id,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition) {
        value.insert_named_with_id(self, id, permit)
    }
    /// `insert` のエッジ版。`graph!` の辺行 `key = Kind(from -> to)`
    /// は名前付きフィールドの辺値型を関連コンストラクタで構築したあと、
    /// 下記 `add_named` へ脱糖する (`docs/schema_v4.md` §2/§3.2)。
    /// このメソッド自体は値の型から内部ストレージへ振り分ける総称
    /// ディスパッチを提供する手書き用APIで、`graph!` を直接経由しない。
    pub fn add<E>(&mut self, key: impl Into<String>, value: E) -> E::Id
    where
        E: 世界Edge + 世界DefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き辺の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named<E>(
        &mut self,
        key: impl Into<String>,
        value: E,
        permit: &graphite::NamedInsertPermit,
    ) -> (E::Id, E::NamedPosition)
    where
        E: 世界Edge + 世界DefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: 世界Edge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: 世界Edge>(
        &mut self,
        id: E::Id,
        value: E,
        permit: &graphite::NamedInsertPermit,
    ) -> (E::Id, E::NamedPosition) {
        value.insert_named_with_id(self, id, permit)
    }
    /// `insert`/`add` のイテレータ版 (`docs/bulk_construction.md`、
    /// `docs/graph_splice.md` §2)。実行時データからの構築で for
    /// ループが構築コードに残るのを避けるため、要素単位 API の反復に
    /// 完全に一致する意味論 (挿入順保持・検証は凍結時) をまとめて
    /// 提供する。ノード用・エッジ用の呼び分けが要らない単一の総称
    /// メソッドに統一している (v4 破壊的変更、旧 `extend_nodes`/
    /// `extend_edges` は廃止): 値の型が既定IDを生成できれば
    /// ノードでもエッジでもよい (どちらになるかは rustc の
    /// 型推論任せ)。`graph!` のスプライス項 (`..式`) もこのメソッドへ
    /// 脱糖する。`insert`/`add` と同じ理由 (トレイトが schema ごとに
    /// 名前が異なる) で、graphite ランタイム側の共通機構ではなく
    /// ここに生成する。
    pub fn extend<K, T>(&mut self, items: impl IntoIterator<Item = (K, T)>) -> Vec<T::Id>
    where
        K: Into<String>,
        T: 世界DefaultId,
    {
        items.into_iter().map(|(k, v)| v.insert_with_binding(self, k.into())).collect()
    }
    /// 検証ロジックの実体。最初の1件で打ち切らず全違反を `Vec` に
    /// 集めて返す。`freeze()` (単一エラー版) はこちらに委譲し先頭の1件を
    /// 取り出すだけの薄いラッパーにすることで、検証ロジックが二重実装に
    /// ならないようにしている。
    fn freeze_collecting(self) -> Result<Graph, Vec<Violation>> {
        let mut __violations: Vec<Violation> = Vec::new();
        let __graphite_construction_stamp = self.__graphite_construction_stamp;
        let mut __graphite_node_人物: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_人物 {
            if !__graphite_node_人物.insert(id.clone(), value) {
                __violations.push(Violation::Duplicate人物(id));
            }
        }
        let mut __graphite_node_商品: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_商品 {
            if !__graphite_node_商品.insert(id.clone(), value) {
                __violations.push(Violation::Duplicate商品(id));
            }
        }
        let mut __graphite_購入: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 購入_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut 購入_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_購入_by_pair: std::collections::HashMap<
            (__人物InternalPosition, __商品InternalPosition),
            __購入InternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.購入 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::購入DuplicateKey(id));
                continue;
            }
            let 購入 { 購入者: from, 対象商品: to, 取引 } = value;
            let from_position = __graphite_node_人物
                .position(&from)
                .map(__人物InternalPosition);
            let to_position = __graphite_node_商品
                .position(&to)
                .map(__商品InternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::購入UnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::購入UnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_購入_by_pair.contains_key(&(from_position, to_position))
                {
                    __violations
                        .push(Violation::購入UniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __購入InternalPosition(
                    graphite::TablePosition::from_index(__graphite_購入.len()),
                );
                __graphite_購入_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                購入_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                購入_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_購入
                    .insert(
                        id,
                        __購入Record {
                            購入者: from_position,
                            対象商品: to_position,
                            取引,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_友人: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 友人_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_友人_by_pair: std::collections::HashMap<
            graphite::UnorderedPair<__人物InternalPosition>,
            __友人InternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.友人 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::友人DuplicateKey(id));
                continue;
            }
            let 友人 { endpoints } = value;
            let (p0, p1) = endpoints.endpoints();
            let p0 = p0.clone();
            let p1 = p1.clone();
            let first_position = __graphite_node_人物
                .position(&p0)
                .map(__人物InternalPosition);
            let second_position = __graphite_node_人物
                .position(&p1)
                .map(__人物InternalPosition);
            if first_position.is_none() {
                __violations
                    .push(Violation::友人UnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p0.clone(),
                    });
            }
            if p1 != p0 && second_position.is_none() {
                __violations
                    .push(Violation::友人UnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p1.clone(),
                    });
            }
            if let (Some(first_position), Some(second_position)) = (
                first_position,
                second_position,
            ) {
                if __graphite_友人_by_pair
                    .contains_key(
                        &graphite::UnorderedPair::new(first_position, second_position),
                    )
                {
                    __violations
                        .push(Violation::友人UniquePairViolation {
                            a: p0.clone(),
                            b: p1.clone(),
                        });
                }
                let internal_edge_position = __友人InternalPosition(
                    graphite::TablePosition::from_index(__graphite_友人.len()),
                );
                __graphite_友人_by_pair
                    .insert(
                        graphite::UnorderedPair::new(first_position, second_position),
                        internal_edge_position,
                    );
                友人_index
                    .entry(first_position)
                    .or_default()
                    .push(internal_edge_position);
                if second_position != first_position {
                    友人_index
                        .entry(second_position)
                        .or_default()
                        .push(internal_edge_position);
                }
                let inserted = __graphite_友人
                    .insert(
                        id,
                        __友人Record {
                            endpoints: graphite::UnorderedPair::new(
                                first_position,
                                second_position,
                            ),
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let 購入_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_人物
                .positions()
                .map(|position| {
                    購入_from_index
                        .remove(&__人物InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 購入_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_商品
                .positions()
                .map(|position| {
                    購入_to_index
                        .remove(&__商品InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 友人_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_人物
                .positions()
                .map(|position| {
                    友人_index
                        .remove(&__人物InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_人物,
            __graphite_node_商品,
            購入: __graphite_購入,
            友人: __graphite_友人,
            購入_from_index,
            購入_to_index,
            __graphite_購入_by_pair,
            友人_index,
            __graphite_友人_by_pair,
            __graphite_construction_stamp,
        })
    }
    /// 最初の1件の違反で `Err` になる版。実装は
    /// `freeze_collecting` に委譲する。
    fn freeze(self) -> Result<Graph, Violation> {
        self.freeze_collecting().map_err(|mut violations| violations.remove(0))
    }
}
/// [`graphite::build_named_graph`] が `#schema_name`/`#violation_ident`
/// の具体型を知らずに凍結を呼べるようにするための橋渡し。
/// `freeze_into_graph` は既存の私有 `freeze()` (上記) へそのまま委譲する。
impl graphite::FreezableBuilder for Builder {
    type Graph = Graph;
    type Violation = Violation;
    fn freeze_into_graph(self) -> Result<Self::Graph, Self::Violation> {
        self.freeze()
    }
}
