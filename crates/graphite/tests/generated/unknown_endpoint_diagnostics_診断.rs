// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/unknown_endpoint_diagnostics.rs:24
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    6972513464851901544u64, 16158424315542838713u64, 10146360922831636970u64,
    4012181187738977718u64,
];
/// `生成キーの地点` ノードの公開ID。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 生成キーの地点Id(pub String);
/// `生成キーの経路` 辺の公開ID。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 生成キーの経路Id(pub String);
/// `生成キーの連絡` 辺の公開ID。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 生成キーの連絡Id(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __生成キーの地点InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __宣言キーの地点InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __生成キーの経路InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __宣言キーの経路InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __生成キーの連絡InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __宣言キーの連絡InternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __生成キーの地点NamedPosition(
    __生成キーの地点InternalPosition,
    u64,
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __宣言キーの地点NamedPosition(
    __宣言キーの地点InternalPosition,
    u64,
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __生成キーの経路NamedPosition(
    __生成キーの経路InternalPosition,
    u64,
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __宣言キーの経路NamedPosition(
    __宣言キーの経路InternalPosition,
    u64,
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __生成キーの連絡NamedPosition(
    __生成キーの連絡InternalPosition,
    u64,
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __宣言キーの連絡NamedPosition(
    __宣言キーの連絡InternalPosition,
    u64,
);
/// 構築時に組み立てる `生成キーの経路` 辺の値。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
#[derive(Clone, PartialEq)]
pub struct 生成キーの経路 {
    /// この辺の始点ノードの公開ID。
    pub 始点: 生成キーの地点Id,
    /// この辺の終点ノードの公開ID。
    pub 終点: 生成キーの地点Id,
}
impl 生成キーの経路 {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn new(from: 生成キーの地点Id, to: 生成キーの地点Id) -> Self {
        Self { 始点: from, 終点: to }
    }
}
impl graphite::DirectedEdgeLiteral<生成キーの地点Id, 生成キーの地点Id, ()>
for 生成キーの経路 {
    fn from_graph_literal(
        from: 生成キーの地点Id,
        to: 生成キーの地点Id,
        (): (),
    ) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for 生成キーの経路 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(生成キーの経路))
            .field(&self.始点)
            .field(&self.終点)
            .finish()
    }
}
/// 構築時に組み立てる `宣言キーの経路` 辺の値。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
#[derive(Clone, PartialEq)]
pub struct 宣言キーの経路 {
    /// この辺の始点ノードの公開ID。
    pub 始点: 生成キーの地点Id,
    /// この辺の終点ノードの公開ID。
    pub 終点: 生成キーの地点Id,
}
impl 宣言キーの経路 {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn new(from: 生成キーの地点Id, to: 生成キーの地点Id) -> Self {
        Self { 始点: from, 終点: to }
    }
}
impl graphite::DirectedEdgeLiteral<生成キーの地点Id, 生成キーの地点Id, ()>
for 宣言キーの経路 {
    fn from_graph_literal(
        from: 生成キーの地点Id,
        to: 生成キーの地点Id,
        (): (),
    ) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for 宣言キーの経路 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(宣言キーの経路))
            .field(&self.始点)
            .field(&self.終点)
            .finish()
    }
}
/// 構築時に組み立てる `生成キーの連絡` 辺の値。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
#[derive(Clone, PartialEq)]
pub struct 生成キーの連絡 {
    /// この辺の始点ノードの公開ID。
    pub 始点: 利用者が宣言した地点キー,
    /// この辺の終点ノードの公開ID。
    pub 終点: 利用者が宣言した地点キー,
}
impl 生成キーの連絡 {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn new(
        from: 利用者が宣言した地点キー,
        to: 利用者が宣言した地点キー,
    ) -> Self {
        Self { 始点: from, 終点: to }
    }
}
impl graphite::DirectedEdgeLiteral<
    利用者が宣言した地点キー,
    利用者が宣言した地点キー,
    (),
> for 生成キーの連絡 {
    fn from_graph_literal(
        from: 利用者が宣言した地点キー,
        to: 利用者が宣言した地点キー,
        (): (),
    ) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for 生成キーの連絡 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(生成キーの連絡))
    }
}
/// 構築時に組み立てる `宣言キーの連絡` 辺の値。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
#[derive(Clone, PartialEq)]
pub struct 宣言キーの連絡 {
    /// この辺の始点ノードの公開ID。
    pub 始点: 利用者が宣言した地点キー,
    /// この辺の終点ノードの公開ID。
    pub 終点: 利用者が宣言した地点キー,
}
impl 宣言キーの連絡 {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn new(
        from: 利用者が宣言した地点キー,
        to: 利用者が宣言した地点キー,
    ) -> Self {
        Self { 始点: from, 終点: to }
    }
}
impl graphite::DirectedEdgeLiteral<
    利用者が宣言した地点キー,
    利用者が宣言した地点キー,
    (),
> for 宣言キーの連絡 {
    fn from_graph_literal(
        from: 利用者が宣言した地点キー,
        to: 利用者が宣言した地点キー,
        (): (),
    ) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for 宣言キーの連絡 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(宣言キーの連絡))
    }
}
#[allow(dead_code)]
struct __生成キーの経路Record {
    始点: __生成キーの地点InternalPosition,
    終点: __生成キーの地点InternalPosition,
}
#[allow(dead_code)]
struct __宣言キーの経路Record {
    始点: __生成キーの地点InternalPosition,
    終点: __生成キーの地点InternalPosition,
}
#[allow(dead_code)]
struct __生成キーの連絡Record {
    始点: __宣言キーの地点InternalPosition,
    終点: __宣言キーの地点InternalPosition,
}
#[allow(dead_code)]
struct __宣言キーの連絡Record {
    始点: __宣言キーの地点InternalPosition,
    終点: __宣言キーの地点InternalPosition,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `schema 診断`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    Duplicate生成キーの地点(生成キーの地点Id),
    /// このノード種別のキーが重複している。
    Duplicate宣言キーの地点(利用者が宣言した地点キー),
    /// このエッジ種別のキーが重複している。
    生成キーの経路DuplicateKey(生成キーの経路Id),
    /// このエッジが未知の始点キーを参照している。
    生成キーの経路UnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: 生成キーの経路Id,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: 生成キーの地点Id,
    },
    /// このエッジが未知の終点キーを参照している。
    生成キーの経路UnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: 生成キーの経路Id,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: 生成キーの地点Id,
    },
    /// このエッジ種別のキーが重複している。
    宣言キーの経路DuplicateKey(利用者が宣言した経路キー),
    /// このエッジが未知の始点キーを参照している。
    宣言キーの経路UnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: 利用者が宣言した経路キー,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: 生成キーの地点Id,
    },
    /// このエッジが未知の終点キーを参照している。
    宣言キーの経路UnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: 利用者が宣言した経路キー,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: 生成キーの地点Id,
    },
    /// このエッジ種別のキーが重複している。
    生成キーの連絡DuplicateKey(生成キーの連絡Id),
    /// このエッジが未知の始点キーを参照している。
    生成キーの連絡UnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: 生成キーの連絡Id,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: 利用者が宣言した地点キー,
    },
    /// このエッジが未知の終点キーを参照している。
    生成キーの連絡UnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: 生成キーの連絡Id,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: 利用者が宣言した地点キー,
    },
    /// このエッジ種別のキーが重複している。
    宣言キーの連絡DuplicateKey(利用者が宣言した経路キー),
    /// このエッジが未知の始点キーを参照している。
    宣言キーの連絡UnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: 利用者が宣言した経路キー,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: 利用者が宣言した地点キー,
    },
    /// このエッジが未知の終点キーを参照している。
    宣言キーの連絡UnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: 利用者が宣言した経路キー,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: 利用者が宣言した地点キー,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::Duplicate生成キーの地点(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}",
                    "生成キーの地点", id
                )
            }
            Violation::Duplicate宣言キーの地点(_) => {
                write!(f, "{}のキーが重複しています", "宣言キーの地点")
            }
            Violation::生成キーの経路DuplicateKey(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}",
                    "生成キーの経路", id
                )
            }
            Violation::生成キーの経路UnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    source, "生成キーの地点", "生成キーの経路", edge,
                    "始点"
                )
            }
            Violation::生成キーの経路UnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    target, "生成キーの地点", "生成キーの経路", edge,
                    "終点"
                )
            }
            Violation::宣言キーの経路DuplicateKey(_) => {
                write!(f, "{}のキーが重複しています", "宣言キーの経路")
            }
            Violation::宣言キーの経路UnknownSource { source, .. } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` の{})",
                    source, "生成キーの地点", "宣言キーの経路", "始点"
                )
            }
            Violation::宣言キーの経路UnknownTarget { target, .. } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` の{})",
                    target, "生成キーの地点", "宣言キーの経路", "終点"
                )
            }
            Violation::生成キーの連絡DuplicateKey(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}",
                    "生成キーの連絡", id
                )
            }
            Violation::生成キーの連絡UnknownSource { edge, .. } => {
                write!(
                    f,
                    "未知のキーが {} として解決できません (辺 `{}` {:?} の{})",
                    "宣言キーの地点", "生成キーの連絡", edge, "始点"
                )
            }
            Violation::生成キーの連絡UnknownTarget { edge, .. } => {
                write!(
                    f,
                    "未知のキーが {} として解決できません (辺 `{}` {:?} の{})",
                    "宣言キーの地点", "生成キーの連絡", edge, "終点"
                )
            }
            Violation::宣言キーの連絡DuplicateKey(_) => {
                write!(f, "{}のキーが重複しています", "宣言キーの連絡")
            }
            Violation::宣言キーの連絡UnknownSource { .. } => {
                write!(
                    f,
                    "未知のキーが {} として解決できません (辺 `{}` の{})",
                    "宣言キーの地点", "宣言キーの連絡", "始点"
                )
            }
            Violation::宣言キーの連絡UnknownTarget { .. } => {
                write!(
                    f,
                    "未知のキーが {} として解決できません (辺 `{}` の{})",
                    "宣言キーの地点", "宣言キーの連絡", "終点"
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
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `schema 診断`
pub struct Graph {
    __graphite_node_生成キーの地点: graphite::KeyedTable<
        生成キーの地点Id,
        super::生成キーの地点,
    >,
    __graphite_node_宣言キーの地点: graphite::KeyedTable<
        利用者が宣言した地点キー,
        super::宣言キーの地点,
    >,
    生成キーの経路: graphite::KeyedTable<
        生成キーの経路Id,
        __生成キーの経路Record,
    >,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    生成キーの経路_from_index: graphite::MultipleRoleIndex<
        __生成キーの経路InternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    生成キーの経路_to_index: graphite::MultipleRoleIndex<
        __生成キーの経路InternalPosition,
    >,
    __graphite_生成キーの経路_by_pair: std::collections::HashMap<
        (
            __生成キーの地点InternalPosition,
            __生成キーの地点InternalPosition,
        ),
        Vec<__生成キーの経路InternalPosition>,
    >,
    宣言キーの経路: graphite::KeyedTable<
        利用者が宣言した経路キー,
        __宣言キーの経路Record,
    >,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    宣言キーの経路_from_index: graphite::MultipleRoleIndex<
        __宣言キーの経路InternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    宣言キーの経路_to_index: graphite::MultipleRoleIndex<
        __宣言キーの経路InternalPosition,
    >,
    __graphite_宣言キーの経路_by_pair: std::collections::HashMap<
        (
            __生成キーの地点InternalPosition,
            __生成キーの地点InternalPosition,
        ),
        Vec<__宣言キーの経路InternalPosition>,
    >,
    生成キーの連絡: graphite::KeyedTable<
        生成キーの連絡Id,
        __生成キーの連絡Record,
    >,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    生成キーの連絡_from_index: graphite::MultipleRoleIndex<
        __生成キーの連絡InternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    生成キーの連絡_to_index: graphite::MultipleRoleIndex<
        __生成キーの連絡InternalPosition,
    >,
    __graphite_生成キーの連絡_by_pair: std::collections::HashMap<
        (
            __宣言キーの地点InternalPosition,
            __宣言キーの地点InternalPosition,
        ),
        Vec<__生成キーの連絡InternalPosition>,
    >,
    宣言キーの連絡: graphite::KeyedTable<
        利用者が宣言した経路キー,
        __宣言キーの連絡Record,
    >,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    宣言キーの連絡_from_index: graphite::MultipleRoleIndex<
        __宣言キーの連絡InternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    宣言キーの連絡_to_index: graphite::MultipleRoleIndex<
        __宣言キーの連絡InternalPosition,
    >,
    __graphite_宣言キーの連絡_by_pair: std::collections::HashMap<
        (
            __宣言キーの地点InternalPosition,
            __宣言キーの地点InternalPosition,
        ),
        Vec<__宣言キーの連絡InternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn 生成キーの地点_by_id<'graph>(
        &'graph self,
        id: &生成キーの地点Id,
    ) -> Option<生成キーの地点Ref<'graph>> {
        let internal_position = __生成キーの地点InternalPosition(
            self.__graphite_node_生成キーの地点.position(id)?,
        );
        Some(生成キーの地点Ref {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn 生成キーの地点_value_mut(
        &mut self,
        id: &生成キーの地点Id,
    ) -> Option<&mut super::生成キーの地点> {
        self.__graphite_node_生成キーの地点.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn 生成キーの地点_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph 生成キーの地点Id> {
        self.__graphite_node_生成キーの地点.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn 生成キーの地点_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 生成キーの地点Ref<'graph>> + 'graph {
        self.__graphite_node_生成キーの地点
            .positions()
            .map(move |position| 生成キーの地点Ref {
                graph: self,
                internal_position: __生成キーの地点InternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn 生成キーの地点_len(&self) -> usize {
        self.__graphite_node_生成キーの地点.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn 宣言キーの地点_by_id<'graph>(
        &'graph self,
        id: &利用者が宣言した地点キー,
    ) -> Option<宣言キーの地点Ref<'graph>> {
        let internal_position = __宣言キーの地点InternalPosition(
            self.__graphite_node_宣言キーの地点.position(id)?,
        );
        Some(宣言キーの地点Ref {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn 宣言キーの地点_value_mut(
        &mut self,
        id: &利用者が宣言した地点キー,
    ) -> Option<&mut super::宣言キーの地点> {
        self.__graphite_node_宣言キーの地点.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn 宣言キーの地点_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph 利用者が宣言した地点キー> {
        self.__graphite_node_宣言キーの地点.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn 宣言キーの地点_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 宣言キーの地点Ref<'graph>> + 'graph {
        self.__graphite_node_宣言キーの地点
            .positions()
            .map(move |position| 宣言キーの地点Ref {
                graph: self,
                internal_position: __宣言キーの地点InternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn 宣言キーの地点_len(&self) -> usize {
        self.__graphite_node_宣言キーの地点.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_by_id<'graph>(
        &'graph self,
        id: &生成キーの経路Id,
    ) -> Option<生成キーの経路Ref<'graph>> {
        Some(生成キーの経路Ref {
            graph: self,
            internal_position: __生成キーの経路InternalPosition(
                self.生成キーの経路.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph 生成キーの経路Id> {
        self.生成キーの経路.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 生成キーの経路Ref<'graph>> + 'graph {
        self.生成キーの経路
            .positions()
            .map(move |position| 生成キーの経路Ref {
                graph: self,
                internal_position: __生成キーの経路InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_len(&self) -> usize {
        self.生成キーの経路.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_by_id<'graph>(
        &'graph self,
        id: &利用者が宣言した経路キー,
    ) -> Option<宣言キーの経路Ref<'graph>> {
        Some(宣言キーの経路Ref {
            graph: self,
            internal_position: __宣言キーの経路InternalPosition(
                self.宣言キーの経路.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph 利用者が宣言した経路キー> {
        self.宣言キーの経路.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 宣言キーの経路Ref<'graph>> + 'graph {
        self.宣言キーの経路
            .positions()
            .map(move |position| 宣言キーの経路Ref {
                graph: self,
                internal_position: __宣言キーの経路InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_len(&self) -> usize {
        self.宣言キーの経路.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_by_id<'graph>(
        &'graph self,
        id: &生成キーの連絡Id,
    ) -> Option<生成キーの連絡Ref<'graph>> {
        Some(生成キーの連絡Ref {
            graph: self,
            internal_position: __生成キーの連絡InternalPosition(
                self.生成キーの連絡.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph 生成キーの連絡Id> {
        self.生成キーの連絡.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 生成キーの連絡Ref<'graph>> + 'graph {
        self.生成キーの連絡
            .positions()
            .map(move |position| 生成キーの連絡Ref {
                graph: self,
                internal_position: __生成キーの連絡InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_len(&self) -> usize {
        self.生成キーの連絡.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_by_id<'graph>(
        &'graph self,
        id: &利用者が宣言した経路キー,
    ) -> Option<宣言キーの連絡Ref<'graph>> {
        Some(宣言キーの連絡Ref {
            graph: self,
            internal_position: __宣言キーの連絡InternalPosition(
                self.宣言キーの連絡.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph 利用者が宣言した経路キー> {
        self.宣言キーの連絡.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 宣言キーの連絡Ref<'graph>> + 'graph {
        self.宣言キーの連絡
            .positions()
            .map(move |position| 宣言キーの連絡Ref {
                graph: self,
                internal_position: __宣言キーの連絡InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_len(&self) -> usize {
        self.宣言キーの連絡.len()
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
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
#[derive(Clone, Copy)]
pub struct 生成キーの経路Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __生成キーの経路InternalPosition,
}
impl<'graph> 生成キーの経路Ref<'graph> {
    fn record(self) -> &'graph __生成キーの経路Record {
        self.graph
            .生成キーの経路
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn id(self) -> &'graph 生成キーの経路Id {
        self.graph
            .生成キーの経路
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 始点(self) -> 生成キーの地点Ref<'graph> {
        生成キーの地点Ref {
            graph: self.graph,
            internal_position: __生成キーの地点InternalPosition(
                self.record().始点.0,
            ),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 終点(self) -> 生成キーの地点Ref<'graph> {
        生成キーの地点Ref {
            graph: self.graph,
            internal_position: __生成キーの地点InternalPosition(
                self.record().終点.0,
            ),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn from(self) -> 生成キーの地点Ref<'graph> {
        self.始点()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn to(self) -> 生成キーの地点Ref<'graph> {
        self.終点()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn from_id(self) -> &'graph 生成キーの地点Id {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn to_id(self) -> &'graph 生成キーの地点Id {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for 生成キーの経路Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(生成キーの経路Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
#[derive(Clone, Copy)]
pub struct 宣言キーの経路Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __宣言キーの経路InternalPosition,
}
impl<'graph> 宣言キーの経路Ref<'graph> {
    fn record(self) -> &'graph __宣言キーの経路Record {
        self.graph
            .宣言キーの経路
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn id(self) -> &'graph 利用者が宣言した経路キー {
        self.graph
            .宣言キーの経路
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 始点(self) -> 生成キーの地点Ref<'graph> {
        生成キーの地点Ref {
            graph: self.graph,
            internal_position: __生成キーの地点InternalPosition(
                self.record().始点.0,
            ),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 終点(self) -> 生成キーの地点Ref<'graph> {
        生成キーの地点Ref {
            graph: self.graph,
            internal_position: __生成キーの地点InternalPosition(
                self.record().終点.0,
            ),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn from(self) -> 生成キーの地点Ref<'graph> {
        self.始点()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn to(self) -> 生成キーの地点Ref<'graph> {
        self.終点()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn from_id(self) -> &'graph 生成キーの地点Id {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn to_id(self) -> &'graph 生成キーの地点Id {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for 宣言キーの経路Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(宣言キーの経路Ref))
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
#[derive(Clone, Copy)]
pub struct 生成キーの連絡Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __生成キーの連絡InternalPosition,
}
impl<'graph> 生成キーの連絡Ref<'graph> {
    fn record(self) -> &'graph __生成キーの連絡Record {
        self.graph
            .生成キーの連絡
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn id(self) -> &'graph 生成キーの連絡Id {
        self.graph
            .生成キーの連絡
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 始点(self) -> 宣言キーの地点Ref<'graph> {
        宣言キーの地点Ref {
            graph: self.graph,
            internal_position: __宣言キーの地点InternalPosition(
                self.record().始点.0,
            ),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 終点(self) -> 宣言キーの地点Ref<'graph> {
        宣言キーの地点Ref {
            graph: self.graph,
            internal_position: __宣言キーの地点InternalPosition(
                self.record().終点.0,
            ),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn from(self) -> 宣言キーの地点Ref<'graph> {
        self.始点()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn to(self) -> 宣言キーの地点Ref<'graph> {
        self.終点()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn from_id(self) -> &'graph 利用者が宣言した地点キー {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn to_id(self) -> &'graph 利用者が宣言した地点キー {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for 生成キーの連絡Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(生成キーの連絡Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
#[derive(Clone, Copy)]
pub struct 宣言キーの連絡Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __宣言キーの連絡InternalPosition,
}
impl<'graph> 宣言キーの連絡Ref<'graph> {
    fn record(self) -> &'graph __宣言キーの連絡Record {
        self.graph
            .宣言キーの連絡
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn id(self) -> &'graph 利用者が宣言した経路キー {
        self.graph
            .宣言キーの連絡
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 始点(self) -> 宣言キーの地点Ref<'graph> {
        宣言キーの地点Ref {
            graph: self.graph,
            internal_position: __宣言キーの地点InternalPosition(
                self.record().始点.0,
            ),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 終点(self) -> 宣言キーの地点Ref<'graph> {
        宣言キーの地点Ref {
            graph: self.graph,
            internal_position: __宣言キーの地点InternalPosition(
                self.record().終点.0,
            ),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn from(self) -> 宣言キーの地点Ref<'graph> {
        self.始点()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn to(self) -> 宣言キーの地点Ref<'graph> {
        self.終点()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn from_id(self) -> &'graph 利用者が宣言した地点キー {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn to_id(self) -> &'graph 利用者が宣言した地点キー {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for 宣言キーの連絡Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(宣言キーの連絡Ref))
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `schema 診断`
pub struct Builder {
    __graphite_node_生成キーの地点: Vec<
        (生成キーの地点Id, super::生成キーの地点),
    >,
    __graphite_node_宣言キーの地点: Vec<
        (利用者が宣言した地点キー, super::宣言キーの地点),
    >,
    生成キーの経路: Vec<(生成キーの経路Id, 生成キーの経路)>,
    宣言キーの経路: Vec<
        (利用者が宣言した経路キー, 宣言キーの経路),
    >,
    生成キーの連絡: Vec<(生成キーの連絡Id, 生成キーの連絡)>,
    宣言キーの連絡: Vec<
        (利用者が宣言した経路キー, 宣言キーの連絡),
    >,
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
pub trait 診断Insertable: Sized {
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
pub trait 診断DefaultId: 診断Insertable {
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
pub trait 診断Node: 診断Insertable {}
impl 診断Insertable for super::生成キーの地点 {
    type Id = 生成キーの地点Id;
    type NamedPosition = __生成キーの地点NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __生成キーの地点NamedPosition(
            __生成キーの地点InternalPosition(
                graphite::TablePosition::from_index(
                    b.__graphite_node_生成キーの地点.len(),
                ),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.生成キーの地点(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.生成キーの地点(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __生成キーの地点NamedPosition {
    type Reference<'graph> = 生成キーの地点Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        生成キーの地点Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 診断DefaultId for super::生成キーの地点 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        診断Insertable::insert_named_with_id(
            self,
            b,
            生成キーの地点Id(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        診断Insertable::insert_with_id(self, b, 生成キーの地点Id(binding))
    }
}
impl 診断Node for super::生成キーの地点 {}
/// 完成済みグラフ上の `生成キーの地点` ノード個体。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
#[derive(Clone, Copy)]
pub struct 生成キーの地点Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __生成キーの地点InternalPosition,
}
impl<'graph> 生成キーの地点Ref<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn id(self) -> &'graph 生成キーの地点Id {
        self.graph
            .__graphite_node_生成キーの地点
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn value(self) -> &'graph super::生成キーの地点 {
        self.graph
            .__graphite_node_生成キーの地点
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_as_始点(
        self,
    ) -> impl Iterator<Item = 生成キーの経路Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .生成キーの経路_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 生成キーの経路Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_as_終点(
        self,
    ) -> impl Iterator<Item = 生成キーの経路Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .生成キーの経路_to_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 生成キーの経路Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_try_between(
        self,
        other: 生成キーの地点Ref<'graph>,
    ) -> Result<
        impl Iterator<Item = 生成キーの経路Ref<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_生成キーの経路_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| 生成キーの経路Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::生成キーの経路_try_between`] を使う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路_between(
        self,
        other: 生成キーの地点Ref<'graph>,
    ) -> impl Iterator<Item = 生成キーの経路Ref<'graph>> + 'graph {
        self.生成キーの経路_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(生成キーの地点Ref),
                    stringify!(生成キーの経路_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_as_始点(
        self,
    ) -> impl Iterator<Item = 宣言キーの経路Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .宣言キーの経路_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 宣言キーの経路Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_as_終点(
        self,
    ) -> impl Iterator<Item = 宣言キーの経路Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .宣言キーの経路_to_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 宣言キーの経路Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_try_between(
        self,
        other: 生成キーの地点Ref<'graph>,
    ) -> Result<
        impl Iterator<Item = 宣言キーの経路Ref<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_宣言キーの経路_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| 宣言キーの経路Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::宣言キーの経路_try_between`] を使う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路_between(
        self,
        other: 生成キーの地点Ref<'graph>,
    ) -> impl Iterator<Item = 宣言キーの経路Ref<'graph>> + 'graph {
        self.宣言キーの経路_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(生成キーの地点Ref),
                    stringify!(宣言キーの経路_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for 生成キーの地点Ref<'graph> {
    type Target = super::生成キーの地点;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_生成キーの地点
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for 生成キーの地点Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(生成キーの地点Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl 診断Insertable for super::宣言キーの地点 {
    type Id = 利用者が宣言した地点キー;
    type NamedPosition = __宣言キーの地点NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __宣言キーの地点NamedPosition(
            __宣言キーの地点InternalPosition(
                graphite::TablePosition::from_index(
                    b.__graphite_node_宣言キーの地点.len(),
                ),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.宣言キーの地点(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.宣言キーの地点(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __宣言キーの地点NamedPosition {
    type Reference<'graph> = 宣言キーの地点Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        宣言キーの地点Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 診断Node for super::宣言キーの地点 {}
/// 完成済みグラフ上の `宣言キーの地点` ノード個体。
///
/// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
#[derive(Clone, Copy)]
pub struct 宣言キーの地点Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __宣言キーの地点InternalPosition,
}
impl<'graph> 宣言キーの地点Ref<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn id(self) -> &'graph 利用者が宣言した地点キー {
        self.graph
            .__graphite_node_宣言キーの地点
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn value(self) -> &'graph super::宣言キーの地点 {
        self.graph
            .__graphite_node_宣言キーの地点
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_as_始点(
        self,
    ) -> impl Iterator<Item = 生成キーの連絡Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .生成キーの連絡_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 生成キーの連絡Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_as_終点(
        self,
    ) -> impl Iterator<Item = 生成キーの連絡Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .生成キーの連絡_to_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 生成キーの連絡Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_try_between(
        self,
        other: 宣言キーの地点Ref<'graph>,
    ) -> Result<
        impl Iterator<Item = 生成キーの連絡Ref<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_生成キーの連絡_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| 生成キーの連絡Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::生成キーの連絡_try_between`] を使う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡_between(
        self,
        other: 宣言キーの地点Ref<'graph>,
    ) -> impl Iterator<Item = 生成キーの連絡Ref<'graph>> + 'graph {
        self.生成キーの連絡_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(宣言キーの地点Ref),
                    stringify!(生成キーの連絡_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_as_始点(
        self,
    ) -> impl Iterator<Item = 宣言キーの連絡Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .宣言キーの連絡_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 宣言キーの連絡Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_as_終点(
        self,
    ) -> impl Iterator<Item = 宣言キーの連絡Ref<'graph>> + 'graph {
        let positions = self
            .graph
            .宣言キーの連絡_to_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 宣言キーの連絡Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_try_between(
        self,
        other: 宣言キーの地点Ref<'graph>,
    ) -> Result<
        impl Iterator<Item = 宣言キーの連絡Ref<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_宣言キーの連絡_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| 宣言キーの連絡Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::宣言キーの連絡_try_between`] を使う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡_between(
        self,
        other: 宣言キーの地点Ref<'graph>,
    ) -> impl Iterator<Item = 宣言キーの連絡Ref<'graph>> + 'graph {
        self.宣言キーの連絡_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(宣言キーの地点Ref),
                    stringify!(宣言キーの連絡_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for 宣言キーの地点Ref<'graph> {
    type Target = super::宣言キーの地点;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_宣言キーの地点
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for 宣言キーの地点Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(宣言キーの地点Ref))
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait 診断Edge: 診断Insertable {}
impl 診断Insertable for 生成キーの経路 {
    type Id = 生成キーの経路Id;
    type NamedPosition = __生成キーの経路NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __生成キーの経路NamedPosition(
            __生成キーの経路InternalPosition(
                graphite::TablePosition::from_index(b.生成キーの経路.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.生成キーの経路(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.生成キーの経路(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __生成キーの経路NamedPosition {
    type Reference<'graph> = 生成キーの経路Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        生成キーの経路Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 診断DefaultId for 生成キーの経路 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        診断Insertable::insert_named_with_id(
            self,
            b,
            生成キーの経路Id(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        診断Insertable::insert_with_id(self, b, 生成キーの経路Id(binding))
    }
}
impl 診断Edge for 生成キーの経路 {}
impl 診断Insertable for 宣言キーの経路 {
    type Id = 利用者が宣言した経路キー;
    type NamedPosition = __宣言キーの経路NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __宣言キーの経路NamedPosition(
            __宣言キーの経路InternalPosition(
                graphite::TablePosition::from_index(b.宣言キーの経路.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.宣言キーの経路(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.宣言キーの経路(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __宣言キーの経路NamedPosition {
    type Reference<'graph> = 宣言キーの経路Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        宣言キーの経路Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 診断Edge for 宣言キーの経路 {}
impl 診断Insertable for 生成キーの連絡 {
    type Id = 生成キーの連絡Id;
    type NamedPosition = __生成キーの連絡NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __生成キーの連絡NamedPosition(
            __生成キーの連絡InternalPosition(
                graphite::TablePosition::from_index(b.生成キーの連絡.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.生成キーの連絡(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.生成キーの連絡(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __生成キーの連絡NamedPosition {
    type Reference<'graph> = 生成キーの連絡Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        生成キーの連絡Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 診断DefaultId for 生成キーの連絡 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        診断Insertable::insert_named_with_id(
            self,
            b,
            生成キーの連絡Id(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        診断Insertable::insert_with_id(self, b, 生成キーの連絡Id(binding))
    }
}
impl 診断Edge for 生成キーの連絡 {}
impl 診断Insertable for 宣言キーの連絡 {
    type Id = 利用者が宣言した経路キー;
    type NamedPosition = __宣言キーの連絡NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __宣言キーの連絡NamedPosition(
            __宣言キーの連絡InternalPosition(
                graphite::TablePosition::from_index(b.宣言キーの連絡.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.宣言キーの連絡(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.宣言キーの連絡(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __宣言キーの連絡NamedPosition {
    type Reference<'graph> = 宣言キーの連絡Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        宣言キーの連絡Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl 診断Edge for 宣言キーの連絡 {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_生成キーの地点: Vec::new(),
            __graphite_node_宣言キーの地点: Vec::new(),
            生成キーの経路: Vec::new(),
            宣言キーの経路: Vec::new(),
            生成キーの連絡: Vec::new(),
            宣言キーの連絡: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 生成キーの地点`
    pub fn 生成キーの地点(
        &mut self,
        id: 生成キーの地点Id,
        value: super::生成キーの地点,
    ) -> &mut Self {
        self.__graphite_node_生成キーの地点.push((id, value));
        self
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `node 宣言キーの地点(id: 利用者が宣言した地点キー)`
    pub fn 宣言キーの地点(
        &mut self,
        id: 利用者が宣言した地点キー,
        value: super::宣言キーの地点,
    ) -> &mut Self {
        self.__graphite_node_宣言キーの地点.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの経路 = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 生成キーの経路(
        &mut self,
        id: 生成キーの経路Id,
        value: 生成キーの経路,
    ) -> &mut Self {
        self.生成キーの経路.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの経路(id: 利用者が宣言した経路キー) = (始点: 生成キーの地点) -> (終点: 生成キーの地点)`
    pub fn 宣言キーの経路(
        &mut self,
        id: 利用者が宣言した経路キー,
        value: 宣言キーの経路,
    ) -> &mut Self {
        self.宣言キーの経路.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 生成キーの連絡 = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 生成キーの連絡(
        &mut self,
        id: 生成キーの連絡Id,
        value: 生成キーの連絡,
    ) -> &mut Self {
        self.生成キーの連絡.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/unknown_endpoint_diagnostics.rs` の `edge 宣言キーの連絡(id: 利用者が宣言した経路キー) = (始点: 宣言キーの地点) -> (終点: 宣言キーの地点)`
    pub fn 宣言キーの連絡(
        &mut self,
        id: 利用者が宣言した経路キー,
        value: 宣言キーの連絡,
    ) -> &mut Self {
        self.宣言キーの連絡.push((id, value));
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
        N: 診断Node + 診断DefaultId,
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
        N: 診断Node + 診断DefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: 診断Node>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: 診断Node>(
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
        E: 診断Edge + 診断DefaultId,
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
        E: 診断Edge + 診断DefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: 診断Edge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: 診断Edge>(
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
        T: 診断DefaultId,
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
        let mut __graphite_node_生成キーの地点: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_生成キーの地点 {
            if !__graphite_node_生成キーの地点.insert(id.clone(), value) {
                __violations.push(Violation::Duplicate生成キーの地点(id));
            }
        }
        let mut __graphite_node_宣言キーの地点: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_宣言キーの地点 {
            if !__graphite_node_宣言キーの地点.insert(id.clone(), value) {
                __violations.push(Violation::Duplicate宣言キーの地点(id));
            }
        }
        let mut __graphite_生成キーの経路: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 生成キーの経路_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut 生成キーの経路_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_生成キーの経路_by_pair: std::collections::HashMap<
            (
                __生成キーの地点InternalPosition,
                __生成キーの地点InternalPosition,
            ),
            Vec<__生成キーの経路InternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.生成キーの経路 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::生成キーの経路DuplicateKey(id));
                continue;
            }
            let 生成キーの経路 { 始点: from, 終点: to } = value;
            let from_position = __graphite_node_生成キーの地点
                .position(&from)
                .map(__生成キーの地点InternalPosition);
            let to_position = __graphite_node_生成キーの地点
                .position(&to)
                .map(__生成キーの地点InternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::生成キーの経路UnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::生成キーの経路UnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __生成キーの経路InternalPosition(
                    graphite::TablePosition::from_index(
                        __graphite_生成キーの経路.len(),
                    ),
                );
                __graphite_生成キーの経路_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                生成キーの経路_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                生成キーの経路_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_生成キーの経路
                    .insert(
                        id,
                        __生成キーの経路Record {
                            始点: from_position,
                            終点: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_宣言キーの経路: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 宣言キーの経路_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut 宣言キーの経路_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_宣言キーの経路_by_pair: std::collections::HashMap<
            (
                __生成キーの地点InternalPosition,
                __生成キーの地点InternalPosition,
            ),
            Vec<__宣言キーの経路InternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.宣言キーの経路 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::宣言キーの経路DuplicateKey(id));
                continue;
            }
            let 宣言キーの経路 { 始点: from, 終点: to } = value;
            let from_position = __graphite_node_生成キーの地点
                .position(&from)
                .map(__生成キーの地点InternalPosition);
            let to_position = __graphite_node_生成キーの地点
                .position(&to)
                .map(__生成キーの地点InternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::宣言キーの経路UnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::宣言キーの経路UnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __宣言キーの経路InternalPosition(
                    graphite::TablePosition::from_index(
                        __graphite_宣言キーの経路.len(),
                    ),
                );
                __graphite_宣言キーの経路_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                宣言キーの経路_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                宣言キーの経路_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_宣言キーの経路
                    .insert(
                        id,
                        __宣言キーの経路Record {
                            始点: from_position,
                            終点: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_生成キーの連絡: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 生成キーの連絡_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut 生成キーの連絡_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_生成キーの連絡_by_pair: std::collections::HashMap<
            (
                __宣言キーの地点InternalPosition,
                __宣言キーの地点InternalPosition,
            ),
            Vec<__生成キーの連絡InternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.生成キーの連絡 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::生成キーの連絡DuplicateKey(id));
                continue;
            }
            let 生成キーの連絡 { 始点: from, 終点: to } = value;
            let from_position = __graphite_node_宣言キーの地点
                .position(&from)
                .map(__宣言キーの地点InternalPosition);
            let to_position = __graphite_node_宣言キーの地点
                .position(&to)
                .map(__宣言キーの地点InternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::生成キーの連絡UnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::生成キーの連絡UnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __生成キーの連絡InternalPosition(
                    graphite::TablePosition::from_index(
                        __graphite_生成キーの連絡.len(),
                    ),
                );
                __graphite_生成キーの連絡_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                生成キーの連絡_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                生成キーの連絡_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_生成キーの連絡
                    .insert(
                        id,
                        __生成キーの連絡Record {
                            始点: from_position,
                            終点: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_宣言キーの連絡: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 宣言キーの連絡_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut 宣言キーの連絡_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_宣言キーの連絡_by_pair: std::collections::HashMap<
            (
                __宣言キーの地点InternalPosition,
                __宣言キーの地点InternalPosition,
            ),
            Vec<__宣言キーの連絡InternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.宣言キーの連絡 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::宣言キーの連絡DuplicateKey(id));
                continue;
            }
            let 宣言キーの連絡 { 始点: from, 終点: to } = value;
            let from_position = __graphite_node_宣言キーの地点
                .position(&from)
                .map(__宣言キーの地点InternalPosition);
            let to_position = __graphite_node_宣言キーの地点
                .position(&to)
                .map(__宣言キーの地点InternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::宣言キーの連絡UnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::宣言キーの連絡UnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __宣言キーの連絡InternalPosition(
                    graphite::TablePosition::from_index(
                        __graphite_宣言キーの連絡.len(),
                    ),
                );
                __graphite_宣言キーの連絡_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                宣言キーの連絡_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                宣言キーの連絡_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_宣言キーの連絡
                    .insert(
                        id,
                        __宣言キーの連絡Record {
                            始点: from_position,
                            終点: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let 生成キーの経路_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_生成キーの地点
                .positions()
                .map(|position| {
                    生成キーの経路_from_index
                        .remove(&__生成キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 生成キーの経路_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_生成キーの地点
                .positions()
                .map(|position| {
                    生成キーの経路_to_index
                        .remove(&__生成キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 宣言キーの経路_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_生成キーの地点
                .positions()
                .map(|position| {
                    宣言キーの経路_from_index
                        .remove(&__生成キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 宣言キーの経路_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_生成キーの地点
                .positions()
                .map(|position| {
                    宣言キーの経路_to_index
                        .remove(&__生成キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 生成キーの連絡_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_宣言キーの地点
                .positions()
                .map(|position| {
                    生成キーの連絡_from_index
                        .remove(&__宣言キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 生成キーの連絡_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_宣言キーの地点
                .positions()
                .map(|position| {
                    生成キーの連絡_to_index
                        .remove(&__宣言キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 宣言キーの連絡_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_宣言キーの地点
                .positions()
                .map(|position| {
                    宣言キーの連絡_from_index
                        .remove(&__宣言キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 宣言キーの連絡_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_宣言キーの地点
                .positions()
                .map(|position| {
                    宣言キーの連絡_to_index
                        .remove(&__宣言キーの地点InternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_生成キーの地点,
            __graphite_node_宣言キーの地点,
            生成キーの経路: __graphite_生成キーの経路,
            宣言キーの経路: __graphite_宣言キーの経路,
            生成キーの連絡: __graphite_生成キーの連絡,
            宣言キーの連絡: __graphite_宣言キーの連絡,
            生成キーの経路_from_index,
            生成キーの経路_to_index,
            __graphite_生成キーの経路_by_pair,
            宣言キーの経路_from_index,
            宣言キーの経路_to_index,
            __graphite_宣言キーの経路_by_pair,
            生成キーの連絡_from_index,
            生成キーの連絡_to_index,
            __graphite_生成キーの連絡_by_pair,
            宣言キーの連絡_from_index,
            宣言キーの連絡_to_index,
            __graphite_宣言キーの連絡_by_pair,
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
