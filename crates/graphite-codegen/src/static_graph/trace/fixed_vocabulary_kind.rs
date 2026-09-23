// Graphiteが定義する固定語彙のカタログ (issue #41 のB分類)。利用者のDSLに
// 同名の宣言トークンが存在しない生成名の一覧と、識別子文字列への変換だけを
// 持つ (由来としての使い方は `origin::名前の由来::GraphiteLanguage` が担う)。

#[derive(Clone, Copy)]
pub(crate) enum 固定語彙 {
    Nodes,
    Edges,
    NodeRefs,
    EdgeRefs,
    Graph,
    // `Graph::new` だけの固定語彙。issue #41 当初は`Nodes`/`Edges`/
    // `NodeRefs`/`EdgeRefs`も対象だったが、PR #45レビューAでそれらの`new`を
    // C分類の内部専用構築子 (`naming::internal_names::内部構築子名` 等) へ
    // 降格したため、`Graph::new`だけが公開契約として残った。
    GraphNew,
    Entity,
    NodeRefsフィールド,
    EdgeRefsフィールド,
    // instance展開が呼び出し位置から辿れる構築の入口 (PR #45レビューA・D)。
    // `{instance名}::construct::nodes!`/`{instance名}::construct::edges!`と
    // いう修飾パスの、それぞれの区間の固定語彙。
    ConstructModule,
    ConstructNodes,
    ConstructEdges,
}

impl 固定語彙 {
    pub(crate) fn 識別子文字列(self) -> &'static str {
        match self {
            Self::Nodes => "Nodes",
            Self::Edges => "Edges",
            Self::NodeRefs => "NodeRefs",
            Self::EdgeRefs => "EdgeRefs",
            Self::Graph => "Graph",
            Self::GraphNew => "new",
            Self::Entity => "entity",
            Self::NodeRefsフィールド => "node_refs",
            Self::EdgeRefsフィールド => "edge_refs",
            Self::ConstructModule => "construct",
            Self::ConstructNodes => "nodes",
            Self::ConstructEdges => "edges",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 固定語彙の識別子文字列を返す() {
        assert_eq!(固定語彙::GraphNew.識別子文字列(), "new");
        assert_eq!(固定語彙::Entity.識別子文字列(), "entity");
        assert_eq!(固定語彙::NodeRefsフィールド.識別子文字列(), "node_refs");
        assert_eq!(固定語彙::EdgeRefsフィールド.識別子文字列(), "edge_refs");
        assert_eq!(固定語彙::ConstructModule.識別子文字列(), "construct");
        assert_eq!(固定語彙::ConstructNodes.識別子文字列(), "nodes");
        assert_eq!(固定語彙::ConstructEdges.識別子文字列(), "edges");
    }
}
