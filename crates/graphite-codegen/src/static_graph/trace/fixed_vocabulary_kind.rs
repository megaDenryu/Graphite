// Graphiteが定義する固定語彙のカタログ (issue #41 のB分類)。利用者のDSLに
// 同名の宣言トークンが存在しない生成名の一覧と、識別子文字列への変換だけを
// 持つ (由来としての使い方は `origin::名前の由来::GraphiteLanguage` が担う)。

// `構築する(所有者)` が指す、生成する `new` がどの型に属するか。
#[derive(Clone, Copy)]
pub(crate) enum 固定語彙の所有者 {
    Nodes,
    Edges,
    NodeRefs,
    EdgeRefs,
    Graph,
}

impl 固定語彙の所有者 {
    pub(crate) fn 型名(self) -> &'static str {
        match self {
            Self::Nodes => "Nodes",
            Self::Edges => "Edges",
            Self::NodeRefs => "NodeRefs",
            Self::EdgeRefs => "EdgeRefs",
            Self::Graph => "Graph",
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum 固定語彙 {
    Nodes,
    Edges,
    NodeRefs,
    EdgeRefs,
    Graph,
    // `naming::fixed_vocabulary::固定語彙の宣言表示` が内側の所有者を読み、
    // 意味カードの「固定語彙:」段落に `Edges::new` のような所有者付きの
    // 表示を作る (issue #41 是正10)。
    構築する(固定語彙の所有者),
    Entity,
    NodeRefsフィールド,
    EdgeRefsフィールド,
}

impl 固定語彙 {
    pub(crate) fn 識別子文字列(self) -> &'static str {
        match self {
            Self::Nodes => "Nodes",
            Self::Edges => "Edges",
            Self::NodeRefs => "NodeRefs",
            Self::EdgeRefs => "EdgeRefs",
            Self::Graph => "Graph",
            Self::構築する(_) => "new",
            Self::Entity => "entity",
            Self::NodeRefsフィールド => "node_refs",
            Self::EdgeRefsフィールド => "edge_refs",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 固定語彙の所有者の型名を返す() {
        assert_eq!(固定語彙の所有者::Nodes.型名(), "Nodes");
        assert_eq!(固定語彙の所有者::Edges.型名(), "Edges");
        assert_eq!(固定語彙の所有者::NodeRefs.型名(), "NodeRefs");
        assert_eq!(固定語彙の所有者::EdgeRefs.型名(), "EdgeRefs");
        assert_eq!(固定語彙の所有者::Graph.型名(), "Graph");
    }

    #[test]
    fn 固定語彙の識別子文字列を返す() {
        assert_eq!(固定語彙::構築する(固定語彙の所有者::Nodes).識別子文字列(), "new");
        assert_eq!(固定語彙::Entity.識別子文字列(), "entity");
        assert_eq!(固定語彙::NodeRefsフィールド.識別子文字列(), "node_refs");
        assert_eq!(固定語彙::EdgeRefsフィールド.識別子文字列(), "edge_refs");
    }
}
