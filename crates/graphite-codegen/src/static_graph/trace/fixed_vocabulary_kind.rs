// Graphiteが定義する固定語彙のカタログ (issue #41 のB分類)。利用者のDSLに
// 同名の宣言トークンが存在しない生成名の一覧と、識別子文字列への変換だけを
// 持つ (由来としての使い方は `origin::名前の由来::GraphiteLanguage` が担う)。
//
// `Entity`の生の綴りは`literal`(instanceの構文検証) も読むため、層の向き
// (`literal`は`trace`に依存しない) を保ったまま両者より下位の
// `reserved_words`へ置いてある。

use crate::static_graph::reserved_words::実体アクセサ名;

#[derive(Clone, Copy)]
pub(crate) enum 固定語彙 {
    NodeRefs,
    EdgeRefs,
    Graph,
    Entity,
    NodeRefsMethod,
    EdgeRefsMethod,
    // instance展開が呼び出し位置から辿れる構築の唯一の入口
    // `{instance名}::construct!`。個体実体・積み荷の所有者 (旧`Nodes`/
    // `Edges`) は`Graph`自身のフィールドへ統合したため、この1語だけが
    // 構築の固定語彙になる。
    Construct,
}

impl 固定語彙 {
    pub(crate) fn 識別子文字列(self) -> &'static str {
        match self {
            Self::NodeRefs => "NodeRefs",
            Self::EdgeRefs => "EdgeRefs",
            Self::Graph => "Graph",
            Self::Entity => 実体アクセサ名,
            Self::NodeRefsMethod => "node_refs",
            Self::EdgeRefsMethod => "edge_refs",
            Self::Construct => "construct",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 固定語彙の識別子文字列を返す() {
        assert_eq!(固定語彙::Entity.識別子文字列(), "entity");
        assert_eq!(固定語彙::NodeRefsMethod.識別子文字列(), "node_refs");
        assert_eq!(固定語彙::EdgeRefsMethod.識別子文字列(), "edge_refs");
        assert_eq!(固定語彙::Construct.識別子文字列(), "construct");
    }
}
