// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/main.rs:114
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    17859109688511163860u64, 16942206122062835165u64, 11853613963298907214u64,
    3209493204181396826u64,
];
/// `Person` ノードの公開ID。
///
/// 宣言: `src/main.rs` の `node Person`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
/// `Team` ノードの公開ID。
///
/// 宣言: `src/main.rs` の `node Team`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TeamId(pub String);
/// `BelongsTo` 辺の公開ID。
///
/// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BelongsToId(pub String);
/// `Boss` 辺の公開ID。
///
/// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BossId(pub String);
/// `Reports` 辺の公開ID。
///
/// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportsId(pub String);
/// `ReviewedBy` 辺の公開ID。
///
/// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReviewedById(pub String);
/// `Friends` 辺の公開ID。
///
/// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FriendsId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __TeamInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __BelongsToInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __BossInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ReportsInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ReviewedByInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __FriendsInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PersonNamedPosition(__PersonInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __TeamNamedPosition(__TeamInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __BelongsToNamedPosition(__BelongsToInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __BossNamedPosition(__BossInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ReportsNamedPosition(__ReportsInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ReviewedByNamedPosition(__ReviewedByInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __FriendsNamedPosition(__FriendsInternalPosition, u64);
/// 構築時に組み立てる `BelongsTo` 辺の値。
///
/// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
#[derive(Clone, PartialEq)]
pub struct BelongsTo {
    /// この辺の始点ノードの公開ID。
    pub member: PersonId,
    /// この辺の終点ノードの公開ID。
    pub team: TeamId,
}
impl BelongsTo {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn new(from: PersonId, to: TeamId) -> Self {
        Self { member: from, team: to }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, TeamId, ()> for BelongsTo {
    fn from_graph_literal(from: PersonId, to: TeamId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for BelongsTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(BelongsTo))
            .field(&self.member)
            .field(&self.team)
            .finish()
    }
}
/// 構築時に組み立てる `Boss` 辺の値。
///
/// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
#[derive(Clone)]
pub struct Boss {
    /// この辺の始点ノードの公開ID。
    pub subordinate: PersonId,
    /// この辺の終点ノードの公開ID。
    pub superior: PersonId,
    /// この辺が運ぶ積み荷。
    pub appointment: BossEdge,
}
impl Boss {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn new(from: PersonId, to: PersonId, payload: BossEdge) -> Self {
        Self {
            subordinate: from,
            superior: to,
            appointment: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn payload(&self) -> &BossEdge {
        &self.appointment
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, PersonId, BossEdge> for Boss {
    fn from_graph_literal(from: PersonId, to: PersonId, payload: BossEdge) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Boss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Boss))
    }
}
/// 構築時に組み立てる `Reports` 辺の値。
///
/// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
#[derive(Clone, PartialEq)]
pub struct Reports {
    /// この辺の始点ノードの公開ID。
    pub reporter: PersonId,
    /// この辺の終点ノードの公開ID。
    pub recipient: PersonId,
}
impl Reports {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn new(from: PersonId, to: PersonId) -> Self {
        Self {
            reporter: from,
            recipient: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, PersonId, ()> for Reports {
    fn from_graph_literal(from: PersonId, to: PersonId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Reports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Reports))
            .field(&self.reporter)
            .field(&self.recipient)
            .finish()
    }
}
/// 構築時に組み立てる `ReviewedBy` 辺の値。
///
/// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
#[derive(Clone)]
pub struct ReviewedBy {
    /// この辺の始点ノードの公開ID。
    pub reviewee: PersonId,
    /// この辺の終点ノードの公開ID。
    pub reviewer: PersonId,
    /// この辺が運ぶ積み荷。
    pub review: ReviewEdge,
}
impl ReviewedBy {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn new(from: PersonId, to: PersonId, payload: ReviewEdge) -> Self {
        Self {
            reviewee: from,
            reviewer: to,
            review: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn payload(&self) -> &ReviewEdge {
        &self.review
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, PersonId, ReviewEdge> for ReviewedBy {
    fn from_graph_literal(from: PersonId, to: PersonId, payload: ReviewEdge) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for ReviewedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ReviewedBy))
    }
}
/// 構築時に組み立てる `Friends` 辺の値。
///
/// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
#[derive(Clone, PartialEq)]
pub struct Friends {
    endpoints: graphite::UnorderedPair<PersonId>,
}
impl Friends {
    /// 両端の公開IDから構築用の辺値を作る。両端の順序は保たない。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn new(a: PersonId, b: PersonId) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
        }
    }
    /// この辺値の両端の公開IDを順序なし対として借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn endpoints(&self) -> (&PersonId, &PersonId) {
        self.endpoints.endpoints()
    }
}
impl graphite::UndirectedEdgeLiteral<PersonId, ()> for Friends {
    fn from_graph_literal(a: PersonId, b: PersonId, (): ()) -> Self {
        Self::new(a, b)
    }
}
impl std::fmt::Debug for Friends {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Friends))
            .field(&self.endpoints().0)
            .field(&self.endpoints().1)
            .finish()
    }
}
#[allow(dead_code)]
struct __BelongsToRecord {
    member: __PersonInternalPosition,
    team: __TeamInternalPosition,
}
#[allow(dead_code)]
struct __BossRecord {
    subordinate: __PersonInternalPosition,
    superior: __PersonInternalPosition,
    appointment: BossEdge,
}
#[allow(dead_code)]
struct __ReportsRecord {
    reporter: __PersonInternalPosition,
    recipient: __PersonInternalPosition,
}
#[allow(dead_code)]
struct __ReviewedByRecord {
    reviewee: __PersonInternalPosition,
    reviewer: __PersonInternalPosition,
    review: ReviewEdge,
}
#[allow(dead_code)]
struct __FriendsRecord {
    endpoints: graphite::UnorderedPair<__PersonInternalPosition>,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `src/main.rs` の `schema Org`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    DuplicatePerson(PersonId),
    /// このノード種別のキーが重複している。
    DuplicateTeam(TeamId),
    /// このエッジ種別のキーが重複している。
    BelongsToDuplicateKey(BelongsToId),
    /// このエッジが未知の始点キーを参照している。
    BelongsToUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: BelongsToId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: PersonId,
    },
    /// このエッジが未知の終点キーを参照している。
    BelongsToUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: BelongsToId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: TeamId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    BelongsToMemberEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: PersonId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
    /// このエッジ種別のキーが重複している。
    BossDuplicateKey(BossId),
    /// このエッジが未知の始点キーを参照している。
    BossUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: BossId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: PersonId,
    },
    /// このエッジが未知の終点キーを参照している。
    BossUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: BossId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: PersonId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    BossSubordinateEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: PersonId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
    /// このエッジ種別のキーが重複している。
    ReportsDuplicateKey(ReportsId),
    /// このエッジが未知の始点キーを参照している。
    ReportsUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: ReportsId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: PersonId,
    },
    /// このエッジが未知の終点キーを参照している。
    ReportsUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: ReportsId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: PersonId,
    },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    ReportsUniquePairViolation {
        /// 2本目の辺が張られた対の始点ノードの公開ID。
        source: PersonId,
        /// 2本目の辺が張られた対の終点ノードの公開ID。
        target: PersonId,
    },
    /// このエッジ種別のキーが重複している。
    ReviewedByDuplicateKey(ReviewedById),
    /// このエッジが未知の始点キーを参照している。
    ReviewedByUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: ReviewedById,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: PersonId,
    },
    /// このエッジが未知の終点キーを参照している。
    ReviewedByUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: ReviewedById,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: PersonId,
    },
    /// このエッジ種別のキーが重複している。
    FriendsDuplicateKey(FriendsId),
    /// このエッジが未知の端点キーを参照している (無向のため位置の
    /// 区別は無い)。
    FriendsUnknownEndpoint {
        /// 未知のキーを参照した辺の公開ID。
        edge: FriendsId,
        /// この辺が端点として参照した、対応するノードが存在しないキー。
        endpoint: PersonId,
    },
    /// このエッジ種別の `unique pair` 違反 (無向のため
    /// 順序を無視した対で判定)。
    FriendsUniquePairViolation {
        /// 2本目の辺が張られた対の一方の端点の公開ID。
        a: PersonId,
        /// 2本目の辺が張られた対のもう一方の端点の公開ID。
        b: PersonId,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicatePerson(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Person", id)
            }
            Violation::DuplicateTeam(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Team", id)
            }
            Violation::BelongsToDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "BelongsTo", id)
            }
            Violation::BelongsToUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    source, "Person", "BelongsTo", edge, "始点"
                )
            }
            Violation::BelongsToUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    target, "Team", "BelongsTo", edge, "終点"
                )
            }
            Violation::BelongsToMemberEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "BelongsTo", "Person", source, "ちょうど1", count
                )
            }
            Violation::BossDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Boss", id)
            }
            Violation::BossUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    source, "Person", "Boss", edge, "始点"
                )
            }
            Violation::BossUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    target, "Person", "Boss", edge, "終点"
                )
            }
            Violation::BossSubordinateEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Boss", "Person", source, "0..1", count
                )
            }
            Violation::ReportsDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Reports", id)
            }
            Violation::ReportsUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    source, "Person", "Reports", edge, "始点"
                )
            }
            Violation::ReportsUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    target, "Person", "Reports", edge, "終点"
                )
            }
            Violation::ReportsUniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "Reports", source, target
                )
            }
            Violation::ReviewedByDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "ReviewedBy", id)
            }
            Violation::ReviewedByUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    source, "Person", "ReviewedBy", edge, "始点"
                )
            }
            Violation::ReviewedByUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    target, "Person", "ReviewedBy", edge, "終点"
                )
            }
            Violation::FriendsDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Friends", id)
            }
            Violation::FriendsUnknownEndpoint { edge, endpoint } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    endpoint, "Person", "Friends", edge, "端点"
                )
            }
            Violation::FriendsUniquePairViolation { a, b } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {{{:?}, {:?}}} の対に既に辺が存在します",
                    "Friends", a, b
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
/// 宣言: `src/main.rs` の `schema Org`
pub struct Graph {
    __graphite_node_person: graphite::KeyedTable<PersonId, super::Person>,
    __graphite_node_team: graphite::KeyedTable<TeamId, super::Team>,
    belongs_to: graphite::KeyedTable<BelongsToId, __BelongsToRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    belongs_to_from_index: graphite::ExactlyOneRoleIndex<__BelongsToInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    belongs_to_to_index: graphite::MultipleRoleIndex<__BelongsToInternalPosition>,
    __graphite_belongs_to_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __TeamInternalPosition),
        Vec<__BelongsToInternalPosition>,
    >,
    boss: graphite::KeyedTable<BossId, __BossRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    boss_from_index: graphite::OptionalRoleIndex<__BossInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    boss_to_index: graphite::MultipleRoleIndex<__BossInternalPosition>,
    __graphite_boss_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __PersonInternalPosition),
        Vec<__BossInternalPosition>,
    >,
    reports: graphite::KeyedTable<ReportsId, __ReportsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    reports_from_index: graphite::MultipleRoleIndex<__ReportsInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    reports_to_index: graphite::MultipleRoleIndex<__ReportsInternalPosition>,
    __graphite_reports_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __PersonInternalPosition),
        __ReportsInternalPosition,
    >,
    reviewed_by: graphite::KeyedTable<ReviewedById, __ReviewedByRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    reviewed_by_from_index: graphite::MultipleRoleIndex<__ReviewedByInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    reviewed_by_to_index: graphite::MultipleRoleIndex<__ReviewedByInternalPosition>,
    __graphite_reviewed_by_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __PersonInternalPosition),
        Vec<__ReviewedByInternalPosition>,
    >,
    friends: graphite::KeyedTable<FriendsId, __FriendsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    friends_index: graphite::MultipleRoleIndex<__FriendsInternalPosition>,
    __graphite_friends_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__PersonInternalPosition>,
        __FriendsInternalPosition,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn person_by_id<'graph>(
        &'graph self,
        id: &PersonId,
    ) -> Option<PersonRef<'graph>> {
        let internal_position = __PersonInternalPosition(
            self.__graphite_node_person.position(id)?,
        );
        Some(PersonRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn person_value_mut(&mut self, id: &PersonId) -> Option<&mut super::Person> {
        self.__graphite_node_person.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn person_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph PersonId> {
        self.__graphite_node_person.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn person_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = PersonRef<'graph>> + 'graph {
        self.__graphite_node_person
            .positions()
            .map(move |position| PersonRef {
                graph: self,
                internal_position: __PersonInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn person_len(&self) -> usize {
        self.__graphite_node_person.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn team_by_id<'graph>(&'graph self, id: &TeamId) -> Option<TeamRef<'graph>> {
        let internal_position = __TeamInternalPosition(
            self.__graphite_node_team.position(id)?,
        );
        Some(TeamRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn team_value_mut(&mut self, id: &TeamId) -> Option<&mut super::Team> {
        self.__graphite_node_team.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn team_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph TeamId> {
        self.__graphite_node_team.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn team_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = TeamRef<'graph>> + 'graph {
        self.__graphite_node_team
            .positions()
            .map(move |position| TeamRef {
                graph: self,
                internal_position: __TeamInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn team_len(&self) -> usize {
        self.__graphite_node_team.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_by_id<'graph>(
        &'graph self,
        id: &BelongsToId,
    ) -> Option<BelongsToRef<'graph>> {
        Some(BelongsToRef {
            graph: self,
            internal_position: __BelongsToInternalPosition(self.belongs_to.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph BelongsToId> {
        self.belongs_to.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = BelongsToRef<'graph>> + 'graph {
        self.belongs_to
            .positions()
            .map(move |position| BelongsToRef {
                graph: self,
                internal_position: __BelongsToInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_len(&self) -> usize {
        self.belongs_to.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_by_id<'graph>(&'graph self, id: &BossId) -> Option<BossRef<'graph>> {
        Some(BossRef {
            graph: self,
            internal_position: __BossInternalPosition(self.boss.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_payload_mut(&mut self, id: &BossId) -> Option<&mut BossEdge> {
        self.boss.get_mut(id).map(|record: &mut __BossRecord| &mut record.appointment)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph BossId> {
        self.boss.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = BossRef<'graph>> + 'graph {
        self.boss
            .positions()
            .map(move |position| BossRef {
                graph: self,
                internal_position: __BossInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_len(&self) -> usize {
        self.boss.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_by_id<'graph>(
        &'graph self,
        id: &ReportsId,
    ) -> Option<ReportsRef<'graph>> {
        Some(ReportsRef {
            graph: self,
            internal_position: __ReportsInternalPosition(self.reports.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ReportsId> {
        self.reports.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ReportsRef<'graph>> + 'graph {
        self.reports
            .positions()
            .map(move |position| ReportsRef {
                graph: self,
                internal_position: __ReportsInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_len(&self) -> usize {
        self.reports.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_by_id<'graph>(
        &'graph self,
        id: &ReviewedById,
    ) -> Option<ReviewedByRef<'graph>> {
        Some(ReviewedByRef {
            graph: self,
            internal_position: __ReviewedByInternalPosition(
                self.reviewed_by.position(id)?,
            ),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_payload_mut(
        &mut self,
        id: &ReviewedById,
    ) -> Option<&mut ReviewEdge> {
        self.reviewed_by
            .get_mut(id)
            .map(|record: &mut __ReviewedByRecord| &mut record.review)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ReviewedById> {
        self.reviewed_by.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ReviewedByRef<'graph>> + 'graph {
        self.reviewed_by
            .positions()
            .map(move |position| ReviewedByRef {
                graph: self,
                internal_position: __ReviewedByInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_len(&self) -> usize {
        self.reviewed_by.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends_by_id<'graph>(
        &'graph self,
        id: &FriendsId,
    ) -> Option<FriendsRef<'graph>> {
        Some(FriendsRef {
            graph: self,
            internal_position: __FriendsInternalPosition(self.friends.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph FriendsId> {
        self.friends.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = FriendsRef<'graph>> + 'graph {
        self.friends
            .positions()
            .map(move |position| FriendsRef {
                graph: self,
                internal_position: __FriendsInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends_len(&self) -> usize {
        self.friends.len()
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
/// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
#[derive(Clone, Copy)]
pub struct BelongsToRef<'graph> {
    graph: &'graph Graph,
    internal_position: __BelongsToInternalPosition,
}
impl<'graph> BelongsToRef<'graph> {
    fn record(self) -> &'graph __BelongsToRecord {
        self.graph
            .belongs_to
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn id(self) -> &'graph BelongsToId {
        self.graph
            .belongs_to
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn member(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().member.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn team(self) -> TeamRef<'graph> {
        TeamRef {
            graph: self.graph,
            internal_position: __TeamInternalPosition(self.record().team.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn from(self) -> PersonRef<'graph> {
        self.member()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn to(self) -> TeamRef<'graph> {
        self.team()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn to_id(self) -> &'graph TeamId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for BelongsToRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(BelongsToRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
#[derive(Clone, Copy)]
pub struct BossRef<'graph> {
    graph: &'graph Graph,
    internal_position: __BossInternalPosition,
}
impl<'graph> BossRef<'graph> {
    fn record(self) -> &'graph __BossRecord {
        self.graph
            .boss
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn id(self) -> &'graph BossId {
        self.graph
            .boss
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn subordinate(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().subordinate.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn superior(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().superior.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn from(self) -> PersonRef<'graph> {
        self.subordinate()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn to(self) -> PersonRef<'graph> {
        self.superior()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn to_id(self) -> &'graph PersonId {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn appointment(self) -> &'graph BossEdge {
        &self.record().appointment
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn payload(self) -> &'graph BossEdge {
        &self.record().appointment
    }
}
impl<'graph> std::fmt::Debug for BossRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(BossRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
#[derive(Clone, Copy)]
pub struct ReportsRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ReportsInternalPosition,
}
impl<'graph> ReportsRef<'graph> {
    fn record(self) -> &'graph __ReportsRecord {
        self.graph
            .reports
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn id(self) -> &'graph ReportsId {
        self.graph
            .reports
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reporter(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().reporter.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn recipient(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().recipient.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn from(self) -> PersonRef<'graph> {
        self.reporter()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn to(self) -> PersonRef<'graph> {
        self.recipient()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn to_id(self) -> &'graph PersonId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for ReportsRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ReportsRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
#[derive(Clone, Copy)]
pub struct ReviewedByRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ReviewedByInternalPosition,
}
impl<'graph> ReviewedByRef<'graph> {
    fn record(self) -> &'graph __ReviewedByRecord {
        self.graph
            .reviewed_by
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn id(self) -> &'graph ReviewedById {
        self.graph
            .reviewed_by
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewee(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().reviewee.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewer(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().reviewer.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn from(self) -> PersonRef<'graph> {
        self.reviewee()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn to(self) -> PersonRef<'graph> {
        self.reviewer()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn to_id(self) -> &'graph PersonId {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn review(self) -> &'graph ReviewEdge {
        &self.record().review
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn payload(self) -> &'graph ReviewEdge {
        &self.record().review
    }
}
impl<'graph> std::fmt::Debug for ReviewedByRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ReviewedByRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の無向辺個体。
///
/// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
#[derive(Clone, Copy)]
pub struct FriendsRef<'graph> {
    graph: &'graph Graph,
    internal_position: __FriendsInternalPosition,
}
impl<'graph> FriendsRef<'graph> {
    fn record(self) -> &'graph __FriendsRecord {
        self.graph
            .friends
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn id(self) -> &'graph FriendsId {
        self.graph
            .friends
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の両端を順序なし対として返す。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn endpoints(self) -> (PersonRef<'graph>, PersonRef<'graph>) {
        let (first, second) = self.record().endpoints.endpoints();
        (
            PersonRef {
                graph: self.graph,
                internal_position: __PersonInternalPosition(first.0),
            },
            PersonRef {
                graph: self.graph,
                internal_position: __PersonInternalPosition(second.0),
            },
        )
    }
}
impl<'graph> std::fmt::Debug for FriendsRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(FriendsRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `src/main.rs` の `schema Org`
pub struct Builder {
    __graphite_node_person: Vec<(PersonId, super::Person)>,
    __graphite_node_team: Vec<(TeamId, super::Team)>,
    belongs_to: Vec<(BelongsToId, BelongsTo)>,
    boss: Vec<(BossId, Boss)>,
    reports: Vec<(ReportsId, Reports)>,
    reviewed_by: Vec<(ReviewedById, ReviewedBy)>,
    friends: Vec<(FriendsId, Friends)>,
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
pub trait OrgInsertable: Sized {
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
pub trait OrgDefaultId: OrgInsertable {
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
pub trait OrgNode: OrgInsertable {}
impl OrgInsertable for super::Person {
    type Id = PersonId;
    type NamedPosition = __PersonNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __PersonNamedPosition(
            __PersonInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_person.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.person(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.person(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __PersonNamedPosition {
    type Reference<'graph> = PersonRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        PersonRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgDefaultId for super::Person {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgInsertable::insert_named_with_id(self, b, PersonId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgInsertable::insert_with_id(self, b, PersonId(binding))
    }
}
impl OrgNode for super::Person {}
/// 完成済みグラフ上の `Person` ノード個体。
///
/// 宣言: `src/main.rs` の `node Person`
#[derive(Clone, Copy)]
pub struct PersonRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PersonInternalPosition,
}
impl<'graph> PersonRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn id(self) -> &'graph PersonId {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn value(self) -> &'graph super::Person {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_as_member(self) -> BelongsToRef<'graph> {
        BelongsToRef {
            graph: self.graph,
            internal_position: *self
                .graph
                .belongs_to_from_index
                .get(self.internal_position.0),
        }
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_try_between(
        self,
        other: TeamRef<'graph>,
    ) -> Result<
        impl Iterator<Item = BelongsToRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_belongs_to_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| BelongsToRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::belongs_to_try_between`] を使う。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_between(
        self,
        other: TeamRef<'graph>,
    ) -> impl Iterator<Item = BelongsToRef<'graph>> + 'graph {
        self.belongs_to_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(belongs_to_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_as_subordinate(self) -> Option<BossRef<'graph>> {
        self.graph
            .boss_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| BossRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_as_superior(self) -> impl Iterator<Item = BossRef<'graph>> + 'graph {
        let positions = self.graph.boss_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| BossRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<
        impl Iterator<Item = BossRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_boss_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| BossRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::boss_try_between`] を使う。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss_between(
        self,
        other: PersonRef<'graph>,
    ) -> impl Iterator<Item = BossRef<'graph>> + 'graph {
        self.boss_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(boss_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_as_reporter(
        self,
    ) -> impl Iterator<Item = ReportsRef<'graph>> + 'graph {
        let positions = self.graph.reports_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ReportsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_as_recipient(
        self,
    ) -> impl Iterator<Item = ReportsRef<'graph>> + 'graph {
        let positions = self.graph.reports_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ReportsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<Option<ReportsRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_reports_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| ReportsRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::reports_try_between`] を使う。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports_between(
        self,
        other: PersonRef<'graph>,
    ) -> Option<ReportsRef<'graph>> {
        self.reports_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(reports_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_as_reviewee(
        self,
    ) -> impl Iterator<Item = ReviewedByRef<'graph>> + 'graph {
        let positions = self.graph.reviewed_by_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ReviewedByRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_as_reviewer(
        self,
    ) -> impl Iterator<Item = ReviewedByRef<'graph>> + 'graph {
        let positions = self.graph.reviewed_by_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ReviewedByRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<
        impl Iterator<Item = ReviewedByRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_reviewed_by_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| ReviewedByRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::reviewed_by_try_between`] を使う。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by_between(
        self,
        other: PersonRef<'graph>,
    ) -> impl Iterator<Item = ReviewedByRef<'graph>> + 'graph {
        self.reviewed_by_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(reviewed_by_between)
                )
            })
    }
    /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends_incident(self) -> impl Iterator<Item = FriendsRef<'graph>> + 'graph {
        let positions = self.graph.friends_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| FriendsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序なし端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<Option<FriendsRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_friends_by_pair
            .get(
                &graphite::UnorderedPair::new(
                    self.internal_position,
                    other.internal_position,
                ),
            )
            .copied();
        Ok(
            found
                .map(|internal_position| FriendsRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::friends_try_between`] を使う。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends_between(
        self,
        other: PersonRef<'graph>,
    ) -> Option<FriendsRef<'graph>> {
        self.friends_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(friends_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for PersonRef<'graph> {
    type Target = super::Person;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for PersonRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(PersonRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl OrgInsertable for super::Team {
    type Id = TeamId;
    type NamedPosition = __TeamNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __TeamNamedPosition(
            __TeamInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_team.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.team(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.team(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __TeamNamedPosition {
    type Reference<'graph> = TeamRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        TeamRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgDefaultId for super::Team {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgInsertable::insert_named_with_id(self, b, TeamId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgInsertable::insert_with_id(self, b, TeamId(binding))
    }
}
impl OrgNode for super::Team {}
/// 完成済みグラフ上の `Team` ノード個体。
///
/// 宣言: `src/main.rs` の `node Team`
#[derive(Clone, Copy)]
pub struct TeamRef<'graph> {
    graph: &'graph Graph,
    internal_position: __TeamInternalPosition,
}
impl<'graph> TeamRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn id(self) -> &'graph TeamId {
        self.graph
            .__graphite_node_team
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn value(self) -> &'graph super::Team {
        self.graph
            .__graphite_node_team
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to_as_team(
        self,
    ) -> impl Iterator<Item = BelongsToRef<'graph>> + 'graph {
        let positions = self.graph.belongs_to_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| BelongsToRef {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for TeamRef<'graph> {
    type Target = super::Team;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_team
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for TeamRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(TeamRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait OrgEdge: OrgInsertable {}
impl OrgInsertable for BelongsTo {
    type Id = BelongsToId;
    type NamedPosition = __BelongsToNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __BelongsToNamedPosition(
            __BelongsToInternalPosition(
                graphite::TablePosition::from_index(b.belongs_to.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.belongs_to(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.belongs_to(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __BelongsToNamedPosition {
    type Reference<'graph> = BelongsToRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        BelongsToRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgDefaultId for BelongsTo {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgInsertable::insert_named_with_id(self, b, BelongsToId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgInsertable::insert_with_id(self, b, BelongsToId(binding))
    }
}
impl OrgEdge for BelongsTo {}
impl OrgInsertable for Boss {
    type Id = BossId;
    type NamedPosition = __BossNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __BossNamedPosition(
            __BossInternalPosition(graphite::TablePosition::from_index(b.boss.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.boss(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.boss(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __BossNamedPosition {
    type Reference<'graph> = BossRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        BossRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgDefaultId for Boss {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgInsertable::insert_named_with_id(self, b, BossId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgInsertable::insert_with_id(self, b, BossId(binding))
    }
}
impl OrgEdge for Boss {}
impl OrgInsertable for Reports {
    type Id = ReportsId;
    type NamedPosition = __ReportsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ReportsNamedPosition(
            __ReportsInternalPosition(
                graphite::TablePosition::from_index(b.reports.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.reports(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.reports(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ReportsNamedPosition {
    type Reference<'graph> = ReportsRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ReportsRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgDefaultId for Reports {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgInsertable::insert_named_with_id(self, b, ReportsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgInsertable::insert_with_id(self, b, ReportsId(binding))
    }
}
impl OrgEdge for Reports {}
impl OrgInsertable for ReviewedBy {
    type Id = ReviewedById;
    type NamedPosition = __ReviewedByNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ReviewedByNamedPosition(
            __ReviewedByInternalPosition(
                graphite::TablePosition::from_index(b.reviewed_by.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.reviewed_by(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.reviewed_by(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ReviewedByNamedPosition {
    type Reference<'graph> = ReviewedByRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ReviewedByRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgDefaultId for ReviewedBy {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgInsertable::insert_named_with_id(self, b, ReviewedById(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgInsertable::insert_with_id(self, b, ReviewedById(binding))
    }
}
impl OrgEdge for ReviewedBy {}
impl OrgInsertable for Friends {
    type Id = FriendsId;
    type NamedPosition = __FriendsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __FriendsNamedPosition(
            __FriendsInternalPosition(
                graphite::TablePosition::from_index(b.friends.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.friends(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.friends(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __FriendsNamedPosition {
    type Reference<'graph> = FriendsRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        FriendsRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgDefaultId for Friends {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgInsertable::insert_named_with_id(self, b, FriendsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgInsertable::insert_with_id(self, b, FriendsId(binding))
    }
}
impl OrgEdge for Friends {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_person: Vec::new(),
            __graphite_node_team: Vec::new(),
            belongs_to: Vec::new(),
            boss: Vec::new(),
            reports: Vec::new(),
            reviewed_by: Vec::new(),
            friends: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/main.rs` の `node Person`
    pub fn person(&mut self, id: PersonId, value: super::Person) -> &mut Self {
        self.__graphite_node_person.push((id, value));
        self
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/main.rs` の `node Team`
    pub fn team(&mut self, id: TeamId, value: super::Team) -> &mut Self {
        self.__graphite_node_team.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/main.rs` の `edge BelongsTo = (member: Person) -> (team: Team) where each member: 1`
    pub fn belongs_to(&mut self, id: BelongsToId, value: BelongsTo) -> &mut Self {
        self.belongs_to.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/main.rs` の `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`
    pub fn boss(&mut self, id: BossId, value: Boss) -> &mut Self {
        self.boss.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/main.rs` の `edge Reports = (reporter: Person) -> (recipient: Person) where unique pair`
    pub fn reports(&mut self, id: ReportsId, value: Reports) -> &mut Self {
        self.reports.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/main.rs` の `edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person)`
    pub fn reviewed_by(&mut self, id: ReviewedById, value: ReviewedBy) -> &mut Self {
        self.reviewed_by.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/main.rs` の `edge Friends = Person -- Person where unique pair`
    pub fn friends(&mut self, id: FriendsId, value: Friends) -> &mut Self {
        self.friends.push((id, value));
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
        N: OrgNode + OrgDefaultId,
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
        N: OrgNode + OrgDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: OrgNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: OrgNode>(
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
        E: OrgEdge + OrgDefaultId,
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
        E: OrgEdge + OrgDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: OrgEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: OrgEdge>(
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
        T: OrgDefaultId,
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
        let mut __graphite_node_person: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_person {
            if !__graphite_node_person.insert(id.clone(), value) {
                __violations.push(Violation::DuplicatePerson(id));
            }
        }
        let mut __graphite_node_team: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_team {
            if !__graphite_node_team.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateTeam(id));
            }
        }
        let mut __graphite_belongs_to: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut belongs_to_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut belongs_to_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_belongs_to_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __TeamInternalPosition),
            Vec<__BelongsToInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.belongs_to {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::BelongsToDuplicateKey(id));
                continue;
            }
            let BelongsTo { member: from, team: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_team
                .position(&to)
                .map(__TeamInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::BelongsToUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::BelongsToUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __BelongsToInternalPosition(
                    graphite::TablePosition::from_index(__graphite_belongs_to.len()),
                );
                __graphite_belongs_to_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                belongs_to_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                belongs_to_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_belongs_to
                    .insert(
                        id,
                        __BelongsToRecord {
                            member: from_position,
                            team: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&BelongsTo) = |edge| {
            let _ = &edge.member;
        };
        for position in __graphite_node_person.positions() {
            let internal_position = __PersonInternalPosition(position);
            let key = __graphite_node_person
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = belongs_to_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if count != 1usize {
                __violations
                    .push(Violation::BelongsToMemberEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_boss: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut boss_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut boss_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_boss_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __PersonInternalPosition),
            Vec<__BossInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.boss {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::BossDuplicateKey(id));
                continue;
            }
            let Boss { subordinate: from, superior: to, appointment } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_person
                .position(&to)
                .map(__PersonInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::BossUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::BossUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __BossInternalPosition(
                    graphite::TablePosition::from_index(__graphite_boss.len()),
                );
                __graphite_boss_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                boss_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                boss_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_boss
                    .insert(
                        id,
                        __BossRecord {
                            subordinate: from_position,
                            superior: to_position,
                            appointment,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Boss) = |edge| {
            let _ = &edge.subordinate;
        };
        for position in __graphite_node_person.positions() {
            let internal_position = __PersonInternalPosition(position);
            let key = __graphite_node_person
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = boss_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::BossSubordinateEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_reports: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut reports_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut reports_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_reports_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __PersonInternalPosition),
            __ReportsInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.reports {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ReportsDuplicateKey(id));
                continue;
            }
            let Reports { reporter: from, recipient: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_person
                .position(&to)
                .map(__PersonInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ReportsUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ReportsUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_reports_by_pair.contains_key(&(from_position, to_position))
                {
                    __violations
                        .push(Violation::ReportsUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __ReportsInternalPosition(
                    graphite::TablePosition::from_index(__graphite_reports.len()),
                );
                __graphite_reports_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                reports_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                reports_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_reports
                    .insert(
                        id,
                        __ReportsRecord {
                            reporter: from_position,
                            recipient: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_reviewed_by: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut reviewed_by_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut reviewed_by_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_reviewed_by_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __PersonInternalPosition),
            Vec<__ReviewedByInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.reviewed_by {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ReviewedByDuplicateKey(id));
                continue;
            }
            let ReviewedBy { reviewee: from, reviewer: to, review } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_person
                .position(&to)
                .map(__PersonInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ReviewedByUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ReviewedByUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __ReviewedByInternalPosition(
                    graphite::TablePosition::from_index(__graphite_reviewed_by.len()),
                );
                __graphite_reviewed_by_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                reviewed_by_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                reviewed_by_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_reviewed_by
                    .insert(
                        id,
                        __ReviewedByRecord {
                            reviewee: from_position,
                            reviewer: to_position,
                            review,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_friends: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut friends_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_friends_by_pair: std::collections::HashMap<
            graphite::UnorderedPair<__PersonInternalPosition>,
            __FriendsInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.friends {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::FriendsDuplicateKey(id));
                continue;
            }
            let Friends { endpoints } = value;
            let (p0, p1) = endpoints.endpoints();
            let p0 = p0.clone();
            let p1 = p1.clone();
            let first_position = __graphite_node_person
                .position(&p0)
                .map(__PersonInternalPosition);
            let second_position = __graphite_node_person
                .position(&p1)
                .map(__PersonInternalPosition);
            if first_position.is_none() {
                __violations
                    .push(Violation::FriendsUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p0.clone(),
                    });
            }
            if p1 != p0 && second_position.is_none() {
                __violations
                    .push(Violation::FriendsUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p1.clone(),
                    });
            }
            if let (Some(first_position), Some(second_position)) = (
                first_position,
                second_position,
            ) {
                if __graphite_friends_by_pair
                    .contains_key(
                        &graphite::UnorderedPair::new(first_position, second_position),
                    )
                {
                    __violations
                        .push(Violation::FriendsUniquePairViolation {
                            a: p0.clone(),
                            b: p1.clone(),
                        });
                }
                let internal_edge_position = __FriendsInternalPosition(
                    graphite::TablePosition::from_index(__graphite_friends.len()),
                );
                __graphite_friends_by_pair
                    .insert(
                        graphite::UnorderedPair::new(first_position, second_position),
                        internal_edge_position,
                    );
                friends_index
                    .entry(first_position)
                    .or_default()
                    .push(internal_edge_position);
                if second_position != first_position {
                    friends_index
                        .entry(second_position)
                        .or_default()
                        .push(internal_edge_position);
                }
                let inserted = __graphite_friends
                    .insert(
                        id,
                        __FriendsRecord {
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
        let belongs_to_from_index = graphite::ExactlyOneRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    belongs_to_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let belongs_to_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_team
                .positions()
                .map(|position| {
                    belongs_to_to_index
                        .remove(&__TeamInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let boss_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    boss_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let boss_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    boss_to_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let reports_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    reports_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let reports_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    reports_to_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let reviewed_by_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    reviewed_by_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let reviewed_by_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    reviewed_by_to_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let friends_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    friends_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_person,
            __graphite_node_team,
            belongs_to: __graphite_belongs_to,
            boss: __graphite_boss,
            reports: __graphite_reports,
            reviewed_by: __graphite_reviewed_by,
            friends: __graphite_friends,
            belongs_to_from_index,
            belongs_to_to_index,
            __graphite_belongs_to_by_pair,
            boss_from_index,
            boss_to_index,
            __graphite_boss_by_pair,
            reports_from_index,
            reports_to_index,
            __graphite_reports_by_pair,
            reviewed_by_from_index,
            reviewed_by_to_index,
            __graphite_reviewed_by_by_pair,
            friends_index,
            __graphite_friends_by_pair,
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
