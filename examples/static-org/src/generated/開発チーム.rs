// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/main.rs:64
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    9142047691600016164u64, 3399734374293553053u64, 3768875613529231478u64,
    14684438191211809226u64,
];
/// Graphite 静的グラフの個体実体の所有者 `Nodes` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `Nodes` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Nodes {
    太郎: 社員,
    次郎: 社員,
    一郎: 社員,
    開発部: 部署,
}
impl Nodes {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct::nodes!/construct::edges! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(
        太郎: 社員,
        次郎: 社員,
        一郎: 社員,
        開発部: 部署,
    ) -> Self {
        Self {
            太郎,
            次郎,
            一郎,
            開発部,
        }
    }
}
/// Graphite 静的グラフの辺実体の所有者 `Edges` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `Edges` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Edges<'a> {
    __graphite_nodes: &'a Nodes,
    太郎の所属: 組織::所属Edge<'a>,
    次郎の所属: 組織::所属Edge<'a>,
    一郎の所属: 組織::所属Edge<'a>,
    太郎の上司: 組織::上司Edge<'a>,
    太郎と次郎: 組織::友人Edge<'a>,
    太郎と一郎の同僚: 組織::同僚Edge<'a>,
}
impl<'a> Edges<'a> {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct::nodes!/construct::edges! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(
        nodes: &'a Nodes,
        太郎の上司: 任命記録,
        太郎と一郎の同僚: 経緯記録,
    ) -> Self {
        Self {
            __graphite_nodes: nodes,
            太郎の所属: 組織::所属Edge {
                member: &nodes.太郎,
                team: &nodes.開発部,
            },
            次郎の所属: 組織::所属Edge {
                member: &nodes.次郎,
                team: &nodes.開発部,
            },
            一郎の所属: 組織::所属Edge {
                member: &nodes.一郎,
                team: &nodes.開発部,
            },
            太郎の上司: 組織::上司Edge {
                subordinate: &nodes.太郎,
                superior: &nodes.次郎,
                任命: 太郎の上司,
            },
            太郎と次郎: 組織::友人Edge {
                甲: &nodes.太郎,
                乙: &nodes.次郎,
            },
            太郎と一郎の同僚: 組織::同僚Edge {
                甲: &nodes.太郎,
                乙: &nodes.一郎,
                経緯: 太郎と一郎の同僚,
            },
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `開発チーム`
/// - 個体: `太郎`
/// - 実体型: `社員`
///
/// 宣言: `src/main.rs` の `node 太郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 太郎Ref<'a> {
    entity: &'a 社員,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 太郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `member`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            entity: &self.edges.太郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎の上司`
    /// - 辺種別: `上司`
    /// - この個体の役割: `subordinate`
    /// - 戻り値: `太郎の上司Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    pub fn 太郎の上司(&self) -> 太郎の上司Ref<'a> {
        太郎の上司Ref {
            entity: &self.edges.太郎の上司,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎と次郎`
    /// - 辺種別: `友人`
    /// - この個体の役割: `甲`
    /// - 戻り値: `太郎と次郎Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎と次郎 = 友人(太郎 -- 次郎)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair`
    pub fn 太郎と次郎(&self) -> 太郎と次郎Ref<'a> {
        太郎と次郎Ref {
            entity: &self.edges.太郎と次郎,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎と一郎の同僚`
    /// - 辺種別: `同僚`
    /// - この個体の役割: `甲`
    /// - 戻り値: `太郎と一郎の同僚Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎と一郎の同僚 = 同僚(太郎 -[..]- 一郎)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員)`
    pub fn 太郎と一郎の同僚(&self) -> 太郎と一郎の同僚Ref<'a> {
        太郎と一郎の同僚Ref {
            entity: &self.edges.太郎と一郎の同僚,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `開発チーム`
/// - 個体: `次郎`
/// - 実体型: `社員`
///
/// 宣言: `src/main.rs` の `node 次郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 次郎Ref<'a> {
    entity: &'a 社員,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 次郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `次郎`
    /// - 具体辺: `次郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `member`
    /// - 戻り値: `次郎の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 次郎の所属 = 所属(次郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 次郎の所属(&self) -> 次郎の所属Ref<'a> {
        次郎の所属Ref {
            entity: &self.edges.次郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `次郎`
    /// - 具体辺: `太郎の上司`
    /// - 辺種別: `上司`
    /// - この個体の役割: `superior`
    /// - 戻り値: `太郎の上司Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    pub fn 太郎の上司(&self) -> 太郎の上司Ref<'a> {
        太郎の上司Ref {
            entity: &self.edges.太郎の上司,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `次郎`
    /// - 具体辺: `太郎と次郎`
    /// - 辺種別: `友人`
    /// - この個体の役割: `乙`
    /// - 戻り値: `太郎と次郎Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎と次郎 = 友人(太郎 -- 次郎)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair`
    pub fn 太郎と次郎(&self) -> 太郎と次郎Ref<'a> {
        太郎と次郎Ref {
            entity: &self.edges.太郎と次郎,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `開発チーム`
/// - 個体: `一郎`
/// - 実体型: `社員`
///
/// 宣言: `src/main.rs` の `node 一郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 一郎Ref<'a> {
    entity: &'a 社員,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 一郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `一郎`
    /// - 具体辺: `一郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `member`
    /// - 戻り値: `一郎の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 一郎の所属 = 所属(一郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 一郎の所属(&self) -> 一郎の所属Ref<'a> {
        一郎の所属Ref {
            entity: &self.edges.一郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `一郎`
    /// - 具体辺: `太郎と一郎の同僚`
    /// - 辺種別: `同僚`
    /// - この個体の役割: `乙`
    /// - 戻り値: `太郎と一郎の同僚Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎と一郎の同僚 = 同僚(太郎 -[..]- 一郎)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員)`
    pub fn 太郎と一郎の同僚(&self) -> 太郎と一郎の同僚Ref<'a> {
        太郎と一郎の同僚Ref {
            entity: &self.edges.太郎と一郎の同僚,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `開発チーム`
/// - 個体: `開発部`
/// - 実体型: `部署`
///
/// 宣言: `src/main.rs` の `node 開発部: 部署`
#[derive(Clone, Copy)]
pub struct 開発部Ref<'a> {
    entity: &'a 部署,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 開発部Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 部署 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `開発部`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `team`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            entity: &self.edges.太郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `開発部`
    /// - 具体辺: `次郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `team`
    /// - 戻り値: `次郎の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 次郎の所属 = 所属(次郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 次郎の所属(&self) -> 次郎の所属Ref<'a> {
        次郎の所属Ref {
            entity: &self.edges.次郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `開発部`
    /// - 具体辺: `一郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `team`
    /// - 戻り値: `一郎の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 一郎の所属 = 所属(一郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 一郎の所属(&self) -> 一郎の所属Ref<'a> {
        一郎の所属Ref {
            entity: &self.edges.一郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `開発チーム`
/// - 具体辺: `太郎の所属`
/// - 辺種別: `所属`
///
/// 宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
///
/// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
#[derive(Clone, Copy)]
pub struct 太郎の所属Ref<'a> {
    entity: &'a 組織::所属Edge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 太郎の所属Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `member: 社員`
    /// - 具体辺: `太郎の所属`
    /// - 具体端点: `太郎`
    /// - 戻り値: `太郎Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    pub fn member(&self) -> 太郎Ref<'a> {
        太郎Ref {
            entity: self.entity.member,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `team: 部署`
    /// - 具体辺: `太郎の所属`
    /// - 具体端点: `開発部`
    /// - 戻り値: `開発部Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    pub fn team(&self) -> 開発部Ref<'a> {
        開発部Ref {
            entity: self.entity.team,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `開発チーム`
/// - 具体辺: `次郎の所属`
/// - 辺種別: `所属`
///
/// 宣言: `src/main.rs` の `edge 次郎の所属 = 所属(次郎 -> 開発部)`
///
/// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
#[derive(Clone, Copy)]
pub struct 次郎の所属Ref<'a> {
    entity: &'a 組織::所属Edge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 次郎の所属Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `member: 社員`
    /// - 具体辺: `次郎の所属`
    /// - 具体端点: `次郎`
    /// - 戻り値: `次郎Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 次郎の所属 = 所属(次郎 -> 開発部)`
    pub fn member(&self) -> 次郎Ref<'a> {
        次郎Ref {
            entity: self.entity.member,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `team: 部署`
    /// - 具体辺: `次郎の所属`
    /// - 具体端点: `開発部`
    /// - 戻り値: `開発部Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 次郎の所属 = 所属(次郎 -> 開発部)`
    pub fn team(&self) -> 開発部Ref<'a> {
        開発部Ref {
            entity: self.entity.team,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `開発チーム`
/// - 具体辺: `一郎の所属`
/// - 辺種別: `所属`
///
/// 宣言: `src/main.rs` の `edge 一郎の所属 = 所属(一郎 -> 開発部)`
///
/// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
#[derive(Clone, Copy)]
pub struct 一郎の所属Ref<'a> {
    entity: &'a 組織::所属Edge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 一郎の所属Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `member: 社員`
    /// - 具体辺: `一郎の所属`
    /// - 具体端点: `一郎`
    /// - 戻り値: `一郎Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 一郎の所属 = 所属(一郎 -> 開発部)`
    pub fn member(&self) -> 一郎Ref<'a> {
        一郎Ref {
            entity: self.entity.member,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `team: 部署`
    /// - 具体辺: `一郎の所属`
    /// - 具体端点: `開発部`
    /// - 戻り値: `開発部Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 一郎の所属 = 所属(一郎 -> 開発部)`
    pub fn team(&self) -> 開発部Ref<'a> {
        開発部Ref {
            entity: self.entity.team,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `開発チーム`
/// - 具体辺: `太郎の上司`
/// - 辺種別: `上司`
///
/// 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
///
/// 関係する schema 宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
#[derive(Clone, Copy)]
pub struct 太郎の上司Ref<'a> {
    entity: &'a 組織::上司Edge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 太郎の上司Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `上司`
    /// - 役割: `subordinate: 社員`
    /// - 具体辺: `太郎の上司`
    /// - 具体端点: `太郎`
    /// - 戻り値: `太郎Ref`
    /// - 検証制約: `each subordinate: 0..1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn subordinate(&self) -> 太郎Ref<'a> {
        太郎Ref {
            entity: self.entity.subordinate,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `上司`
    /// - 役割: `superior: 社員`
    /// - 具体辺: `太郎の上司`
    /// - 具体端点: `次郎`
    /// - 戻り値: `次郎Ref`
    /// - 検証制約: `each subordinate: 0..1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn superior(&self) -> 次郎Ref<'a> {
        次郎Ref {
            entity: self.entity.superior,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの積み荷アクセサ。
    ///
    /// - 辺種別: `上司`
    /// - 積み荷: `任命: 任命記録`
    /// - 具体辺: `太郎の上司`
    ///
    /// 宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn 任命(&self) -> &'a 任命記録 {
        &self.entity.任命
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `開発チーム`
/// - 具体辺: `太郎と次郎`
/// - 辺種別: `友人`
///
/// 宣言: `src/main.rs` の `edge 太郎と次郎 = 友人(太郎 -- 次郎)`
///
/// 関係する schema 宣言: `src/main.rs` の `edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair`
#[derive(Clone, Copy)]
pub struct 太郎と次郎Ref<'a> {
    entity: &'a 組織::友人Edge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 太郎と次郎Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `友人`
    /// - 役割: `甲: 社員`
    /// - 具体辺: `太郎と次郎`
    /// - 具体端点: `太郎`
    /// - 戻り値: `太郎Ref`
    /// - 検証制約: `unique pair` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎と次郎 = 友人(太郎 -- 次郎)`
    pub fn 甲(&self) -> 太郎Ref<'a> {
        太郎Ref {
            entity: self.entity.甲,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `友人`
    /// - 役割: `乙: 社員`
    /// - 具体辺: `太郎と次郎`
    /// - 具体端点: `次郎`
    /// - 戻り値: `次郎Ref`
    /// - 検証制約: `unique pair` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎と次郎 = 友人(太郎 -- 次郎)`
    pub fn 乙(&self) -> 次郎Ref<'a> {
        次郎Ref {
            entity: self.entity.乙,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `開発チーム`
/// - 具体辺: `太郎と一郎の同僚`
/// - 辺種別: `同僚`
///
/// 宣言: `src/main.rs` の `edge 太郎と一郎の同僚 = 同僚(太郎 -[..]- 一郎)`
///
/// 関係する schema 宣言: `src/main.rs` の `edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員)`
#[derive(Clone, Copy)]
pub struct 太郎と一郎の同僚Ref<'a> {
    entity: &'a 組織::同僚Edge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 太郎と一郎の同僚Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `同僚`
    /// - 役割: `甲: 社員`
    /// - 具体辺: `太郎と一郎の同僚`
    /// - 具体端点: `太郎`
    /// - 戻り値: `太郎Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員)`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎と一郎の同僚 = 同僚(太郎 -[..]- 一郎)`
    pub fn 甲(&self) -> 太郎Ref<'a> {
        太郎Ref {
            entity: self.entity.甲,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `同僚`
    /// - 役割: `乙: 社員`
    /// - 具体辺: `太郎と一郎の同僚`
    /// - 具体端点: `一郎`
    /// - 戻り値: `一郎Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員)`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎と一郎の同僚 = 同僚(太郎 -[..]- 一郎)`
    pub fn 乙(&self) -> 一郎Ref<'a> {
        一郎Ref {
            entity: self.entity.乙,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの積み荷アクセサ。
    ///
    /// - 辺種別: `同僚`
    /// - 積み荷: `経緯: 経緯記録`
    /// - 具体辺: `太郎と一郎の同僚`
    ///
    /// 宣言: `src/main.rs` の `edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員)`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 太郎と一郎の同僚 = 同僚(太郎 -[..]- 一郎)`
    pub fn 経緯(&self) -> &'a 経緯記録 {
        &self.entity.経緯
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    太郎: 太郎Ref<'a>,
    次郎: 次郎Ref<'a>,
    一郎: 一郎Ref<'a>,
    開発部: 開発部Ref<'a>,
}
impl<'a> NodeRefs<'a> {
    fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            太郎: 太郎Ref {
                entity: &nodes.太郎,
                nodes,
                edges,
            },
            次郎: 次郎Ref {
                entity: &nodes.次郎,
                nodes,
                edges,
            },
            一郎: 一郎Ref {
                entity: &nodes.一郎,
                nodes,
                edges,
            },
            開発部: 開発部Ref {
                entity: &nodes.開発部,
                nodes,
                edges,
            },
        }
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `太郎`
    ///
    /// 宣言: `src/main.rs` の `node 太郎: 社員 = ..`
    pub fn 太郎(&self) -> 太郎Ref<'a> {
        self.太郎
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `次郎`
    ///
    /// 宣言: `src/main.rs` の `node 次郎: 社員 = ..`
    pub fn 次郎(&self) -> 次郎Ref<'a> {
        self.次郎
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `一郎`
    ///
    /// 宣言: `src/main.rs` の `node 一郎: 社員 = ..`
    pub fn 一郎(&self) -> 一郎Ref<'a> {
        self.一郎
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `開発部`
    ///
    /// 宣言: `src/main.rs` の `node 開発部: 部署`
    pub fn 開発部(&self) -> 開発部Ref<'a> {
        self.開発部
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    太郎の所属: 太郎の所属Ref<'a>,
    次郎の所属: 次郎の所属Ref<'a>,
    一郎の所属: 一郎の所属Ref<'a>,
    太郎の上司: 太郎の上司Ref<'a>,
    太郎と次郎: 太郎と次郎Ref<'a>,
    太郎と一郎の同僚: 太郎と一郎の同僚Ref<'a>,
}
impl<'a> EdgeRefs<'a> {
    fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            太郎の所属: 太郎の所属Ref {
                entity: &edges.太郎の所属,
                nodes,
                edges,
            },
            次郎の所属: 次郎の所属Ref {
                entity: &edges.次郎の所属,
                nodes,
                edges,
            },
            一郎の所属: 一郎の所属Ref {
                entity: &edges.一郎の所属,
                nodes,
                edges,
            },
            太郎の上司: 太郎の上司Ref {
                entity: &edges.太郎の上司,
                nodes,
                edges,
            },
            太郎と次郎: 太郎と次郎Ref {
                entity: &edges.太郎と次郎,
                nodes,
                edges,
            },
            太郎と一郎の同僚: 太郎と一郎の同僚Ref {
                entity: &edges.太郎と一郎の同僚,
                nodes,
                edges,
            },
        }
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 具体辺: `太郎の所属`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        self.太郎の所属
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 具体辺: `次郎の所属`
    ///
    /// 宣言: `src/main.rs` の `edge 次郎の所属 = 所属(次郎 -> 開発部)`
    pub fn 次郎の所属(&self) -> 次郎の所属Ref<'a> {
        self.次郎の所属
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 具体辺: `一郎の所属`
    ///
    /// 宣言: `src/main.rs` の `edge 一郎の所属 = 所属(一郎 -> 開発部)`
    pub fn 一郎の所属(&self) -> 一郎の所属Ref<'a> {
        self.一郎の所属
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 具体辺: `太郎の上司`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn 太郎の上司(&self) -> 太郎の上司Ref<'a> {
        self.太郎の上司
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 具体辺: `太郎と次郎`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎と次郎 = 友人(太郎 -- 次郎)`
    pub fn 太郎と次郎(&self) -> 太郎と次郎Ref<'a> {
        self.太郎と次郎
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 具体辺: `太郎と一郎の同僚`
    ///
    /// 宣言: `src/main.rs` の `edge 太郎と一郎の同僚 = 同僚(太郎 -[..]- 一郎)`
    pub fn 太郎と一郎の同僚(&self) -> 太郎と一郎の同僚Ref<'a> {
        self.太郎と一郎の同僚
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph<'a> {
    node_refs: NodeRefs<'a>,
    edge_refs: EdgeRefs<'a>,
}
impl<'a> Graph<'a> {
    /// Graphite 静的グラフの `Graph` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `Graph::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(edges: &'a Edges<'a>) -> Self {
        let nodes = edges.__graphite_nodes;
        Self {
            node_refs: NodeRefs::new(nodes, edges),
            edge_refs: EdgeRefs::new(nodes, edges),
        }
    }
    /// Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> &NodeRefs<'a> {
        &self.node_refs
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> &EdgeRefs<'a> {
        &self.edge_refs
    }
}
/// Graphite 静的グラフの構築の入口をまとめるmodule `construct` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `construct` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub mod construct {
    /// Graphite 静的グラフの個体実体の所有者 `Nodes` を構築するマクロ `nodes` (Graphite の固定語彙)。値ありの個体はinstance宣言の式からこのマクロが計算し、値なしの個体だけを引数で受け取る。
    ///
    /// - graph: `開発チーム`
    /// - 実行時に渡す個体 (宣言順): `開発部: 部署`
    /// - 戻り値: `Nodes`
    ///
    /// 固定語彙: `construct::nodes!` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `src/main.rs` の `graph 開発チーム`
    macro_rules! nodes {
        ($開発部:expr) => {
            { let (太郎, 次郎, 一郎,) = __graphite_values_開発チーム!();
            #[allow(deprecated)] let __graphite_nodes =
            開発チーム::Nodes::__graphite_internal_new(太郎, 次郎, 一郎,
            $開発部); __graphite_nodes }
        };
    }
    pub(crate) use nodes;
    /// Graphite 静的グラフの辺実体の所有者 `Edges` を構築するマクロ `edges` (Graphite の固定語彙)。積み荷ありの具体辺はすべてinstance宣言の式からこのマクロが計算する。
    ///
    /// - graph: `開発チーム`
    /// - 引数: `nodes: &Nodes`
    /// - 戻り値: `Edges`
    ///
    /// 固定語彙: `construct::edges!` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `src/main.rs` の `graph 開発チーム`
    macro_rules! edges {
        ($nodes:expr) => {
            { let (太郎の上司, 太郎と一郎の同僚,) =
            __graphite_payloads_開発チーム!(); #[allow(deprecated)] let
            __graphite_edges = 開発チーム::Edges::__graphite_internal_new($nodes,
            太郎の上司, 太郎と一郎の同僚); __graphite_edges }
        };
    }
    pub(crate) use edges;
}
