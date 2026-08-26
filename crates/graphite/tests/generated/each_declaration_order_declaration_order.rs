// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: crates/graphite/tests/each_declaration_order.rs:39
// 再生成: リポジトリルートで `cargo xtask generate` を実行する。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    11790603187545708424u64, 16904477885548598967u64, 2600448347193125662u64,
    5111711321215140306u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthorId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArticleId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WroteId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __AuthorInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ArticleInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __WroteInternalPosition(usize);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __AuthorNamedPosition(__AuthorInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ArticleNamedPosition(__ArticleInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __WroteNamedPosition(__WroteInternalPosition, u64);
#[derive(Clone, PartialEq)]
pub struct Wrote {
    pub writer: AuthorId,
    pub article: ArticleId,
    pub byline: Byline,
}
impl Wrote {
    pub fn new(from: AuthorId, to: ArticleId, payload: Byline) -> Self {
        Self {
            writer: from,
            article: to,
            byline: payload,
        }
    }
    pub fn payload(&self) -> &Byline {
        &self.byline
    }
}
impl graphite::DirectedEdgeLiteral<AuthorId, ArticleId, Byline> for Wrote {
    fn from_graph_literal(from: AuthorId, to: ArticleId, payload: Byline) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Wrote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Wrote))
    }
}
#[allow(dead_code)]
struct __WroteRecord {
    writer: __AuthorInternalPosition,
    article: __ArticleInternalPosition,
    byline: Byline,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicateAuthor(AuthorId),
    DuplicateArticle(ArticleId),
    /// このエッジ種別のキーが重複している。
    WroteDuplicateKey(WroteId),
    /// このエッジが未知の始点キーを参照している。
    WroteUnknownSource { edge: WroteId, source: AuthorId },
    /// このエッジが未知の終点キーを参照している。
    WroteUnknownTarget { edge: WroteId, target: ArticleId },
    /// このエッジ種別の `each` 制約違反 (入次数)。
    WroteArticleEachViolation { target: ArticleId, count: usize },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    WroteWriterEachViolation { source: AuthorId, count: usize },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateAuthor(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Author", id)
            }
            Violation::DuplicateArticle(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Article", id)
            }
            Violation::WroteDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Wrote", id)
            }
            Violation::WroteUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Wrote", edge, "Author", source
                )
            }
            Violation::WroteUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Wrote", edge, "Article", target
                )
            }
            Violation::WroteArticleEachViolation { target, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について入次数 {} を期待しますが実際は {} 本です",
                    "Wrote", "Article", target, "ちょうど1", count
                )
            }
            Violation::WroteWriterEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Wrote", "Author", source, "0..1", count
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
pub struct Graph {
    __graphite_node_author: graphite::KeyedTable<AuthorId, super::Author>,
    __graphite_node_article: graphite::KeyedTable<ArticleId, super::Article>,
    wrote: graphite::KeyedTable<WroteId, __WroteRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    wrote_from_index: graphite::OptionalRoleIndex<__WroteInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    wrote_to_index: graphite::ExactlyOneRoleIndex<__WroteInternalPosition>,
    __graphite_wrote_by_pair: std::collections::HashMap<
        (__AuthorInternalPosition, __ArticleInternalPosition),
        Vec<__WroteInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn author_by_id<'graph>(
        &'graph self,
        id: &AuthorId,
    ) -> Option<AuthorRef<'graph>> {
        let internal_position = __AuthorInternalPosition(
            self.__graphite_node_author.position(id)?,
        );
        Some(AuthorRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn author_value_mut(&mut self, id: &AuthorId) -> Option<&mut super::Author> {
        self.__graphite_node_author.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn author_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph AuthorId> {
        self.__graphite_node_author.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn author_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = AuthorRef<'graph>> + 'graph {
        self.__graphite_node_author
            .positions()
            .map(move |position| AuthorRef {
                graph: self,
                internal_position: __AuthorInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn author_len(&self) -> usize {
        self.__graphite_node_author.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn article_by_id<'graph>(
        &'graph self,
        id: &ArticleId,
    ) -> Option<ArticleRef<'graph>> {
        let internal_position = __ArticleInternalPosition(
            self.__graphite_node_article.position(id)?,
        );
        Some(ArticleRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn article_value_mut(&mut self, id: &ArticleId) -> Option<&mut super::Article> {
        self.__graphite_node_article.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn article_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ArticleId> {
        self.__graphite_node_article.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn article_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ArticleRef<'graph>> + 'graph {
        self.__graphite_node_article
            .positions()
            .map(move |position| ArticleRef {
                graph: self,
                internal_position: __ArticleInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn article_len(&self) -> usize {
        self.__graphite_node_article.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn wrote_by_id<'graph>(&'graph self, id: &WroteId) -> Option<WroteRef<'graph>> {
        Some(WroteRef {
            graph: self,
            internal_position: __WroteInternalPosition(self.wrote.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    pub fn wrote_payload_mut(&mut self, id: &WroteId) -> Option<&mut Byline> {
        self.wrote.get_mut(id).map(|record: &mut __WroteRecord| &mut record.byline)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn wrote_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph WroteId> {
        self.wrote.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn wrote_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = WroteRef<'graph>> + 'graph {
        self.wrote
            .positions()
            .map(move |position| WroteRef {
                graph: self,
                internal_position: __WroteInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn wrote_len(&self) -> usize {
        self.wrote.len()
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
#[derive(Clone, Copy)]
pub struct WroteRef<'graph> {
    graph: &'graph Graph,
    internal_position: __WroteInternalPosition,
}
impl<'graph> WroteRef<'graph> {
    fn record(self) -> &'graph __WroteRecord {
        self.graph
            .wrote
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph WroteId {
        self.graph
            .wrote
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn writer(self) -> AuthorRef<'graph> {
        AuthorRef {
            graph: self.graph,
            internal_position: __AuthorInternalPosition(self.record().writer.0),
        }
    }
    pub fn article(self) -> ArticleRef<'graph> {
        ArticleRef {
            graph: self.graph,
            internal_position: __ArticleInternalPosition(self.record().article.0),
        }
    }
    pub fn from(self) -> AuthorRef<'graph> {
        self.writer()
    }
    pub fn to(self) -> ArticleRef<'graph> {
        self.article()
    }
    pub fn from_id(self) -> &'graph AuthorId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ArticleId {
        self.to().id()
    }
    pub fn byline(self) -> &'graph Byline {
        &self.record().byline
    }
    pub fn payload(self) -> &'graph Byline {
        &self.record().byline
    }
}
impl<'graph> std::fmt::Debug for WroteRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(WroteRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_author: Vec<(AuthorId, super::Author)>,
    __graphite_node_article: Vec<(ArticleId, super::Article)>,
    wrote: Vec<(WroteId, Wrote)>,
    /// この構築を識別する構築印。`Builder::new()` が発行し、この
    /// builder から挿入する全ての名前付き位置と、凍結成功後の
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
pub trait DeclarationOrderInsertable: Sized {
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
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id;
}
/// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
/// 実装する。明示ID型には実装せず、文字列変換を要求しない。
pub trait DeclarationOrderDefaultId: DeclarationOrderInsertable {
    #[doc(hidden)]
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id;
}
/// ノード挿入で使うトレイト境界。読み取りは `Graph` の種別メソッドと
/// `NodeRef` のメソッドが提供する。利用者がこのトレイトのメソッドを
/// 直接呼ぶことは想定しない。
pub trait DeclarationOrderNode: DeclarationOrderInsertable {}
impl DeclarationOrderInsertable for super::Author {
    type Id = AuthorId;
    type NamedPosition = __AuthorNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __AuthorNamedPosition(
            __AuthorInternalPosition(b.__graphite_node_author.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.author(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.author(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __AuthorNamedPosition {
    type Reference<'graph> = AuthorRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        AuthorRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl DeclarationOrderDefaultId for super::Author {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        DeclarationOrderInsertable::insert_named_with_id(
            self,
            b,
            AuthorId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        DeclarationOrderInsertable::insert_with_id(self, b, AuthorId(binding))
    }
}
impl DeclarationOrderNode for super::Author {}
///完成済みグラフ上の `Author` ノード個体。
#[derive(Clone, Copy)]
pub struct AuthorRef<'graph> {
    graph: &'graph Graph,
    internal_position: __AuthorInternalPosition,
}
impl<'graph> AuthorRef<'graph> {
    pub fn id(self) -> &'graph AuthorId {
        self.graph
            .__graphite_node_author
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Author {
        self.graph
            .__graphite_node_author
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    pub fn wrote_as_writer(self) -> Option<WroteRef<'graph>> {
        self.graph
            .wrote_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| WroteRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn wrote_try_between(
        self,
        other: ArticleRef<'graph>,
    ) -> Result<
        impl Iterator<Item = WroteRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_wrote_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| WroteRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::wrote_try_between`] を使う。
    pub fn wrote_between(
        self,
        other: ArticleRef<'graph>,
    ) -> impl Iterator<Item = WroteRef<'graph>> + 'graph {
        self.wrote_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(AuthorRef), stringify!(wrote_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for AuthorRef<'graph> {
    type Target = super::Author;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_author
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for AuthorRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(AuthorRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl DeclarationOrderInsertable for super::Article {
    type Id = ArticleId;
    type NamedPosition = __ArticleNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ArticleNamedPosition(
            __ArticleInternalPosition(b.__graphite_node_article.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.article(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.article(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ArticleNamedPosition {
    type Reference<'graph> = ArticleRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ArticleRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl DeclarationOrderDefaultId for super::Article {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        DeclarationOrderInsertable::insert_named_with_id(
            self,
            b,
            ArticleId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        DeclarationOrderInsertable::insert_with_id(self, b, ArticleId(binding))
    }
}
impl DeclarationOrderNode for super::Article {}
///完成済みグラフ上の `Article` ノード個体。
#[derive(Clone, Copy)]
pub struct ArticleRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ArticleInternalPosition,
}
impl<'graph> ArticleRef<'graph> {
    pub fn id(self) -> &'graph ArticleId {
        self.graph
            .__graphite_node_article
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Article {
        self.graph
            .__graphite_node_article
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
    pub fn wrote_as_article(self) -> WroteRef<'graph> {
        WroteRef {
            graph: self.graph,
            internal_position: *self.graph.wrote_to_index.get(self.internal_position.0),
        }
    }
}
impl<'graph> std::ops::Deref for ArticleRef<'graph> {
    type Target = super::Article;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_article
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for ArticleRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ArticleRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait DeclarationOrderEdge: DeclarationOrderInsertable {}
impl DeclarationOrderInsertable for Wrote {
    type Id = WroteId;
    type NamedPosition = __WroteNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __WroteNamedPosition(
            __WroteInternalPosition(b.wrote.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.wrote(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.wrote(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __WroteNamedPosition {
    type Reference<'graph> = WroteRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        WroteRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl DeclarationOrderDefaultId for Wrote {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        DeclarationOrderInsertable::insert_named_with_id(
            self,
            b,
            WroteId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        DeclarationOrderInsertable::insert_with_id(self, b, WroteId(binding))
    }
}
impl DeclarationOrderEdge for Wrote {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_author: Vec::new(),
            __graphite_node_article: Vec::new(),
            wrote: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn author(&mut self, id: AuthorId, value: super::Author) -> &mut Self {
        self.__graphite_node_author.push((id, value));
        self
    }
    pub fn article(&mut self, id: ArticleId, value: super::Article) -> &mut Self {
        self.__graphite_node_article.push((id, value));
        self
    }
    pub fn wrote(&mut self, id: WroteId, value: Wrote) -> &mut Self {
        self.wrote.push((id, value));
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
        N: DeclarationOrderNode + DeclarationOrderDefaultId,
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
        N: DeclarationOrderNode + DeclarationOrderDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: DeclarationOrderNode>(
        &mut self,
        id: N::Id,
        value: N,
    ) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: DeclarationOrderNode>(
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
        E: DeclarationOrderEdge + DeclarationOrderDefaultId,
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
        E: DeclarationOrderEdge + DeclarationOrderDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: DeclarationOrderEdge>(
        &mut self,
        id: E::Id,
        value: E,
    ) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: DeclarationOrderEdge>(
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
        T: DeclarationOrderDefaultId,
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
        let mut __graphite_node_author: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_author {
            if !__graphite_node_author.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateAuthor(id));
            }
        }
        let mut __graphite_node_article: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_article {
            if !__graphite_node_article.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateArticle(id));
            }
        }
        let mut __graphite_wrote: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut wrote_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut wrote_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_wrote_by_pair: std::collections::HashMap<
            (__AuthorInternalPosition, __ArticleInternalPosition),
            Vec<__WroteInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.wrote {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::WroteDuplicateKey(id));
                continue;
            }
            let Wrote { writer: from, article: to, byline } = value;
            let from_position = __graphite_node_author
                .position(&from)
                .map(__AuthorInternalPosition);
            let to_position = __graphite_node_article
                .position(&to)
                .map(__ArticleInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::WroteUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::WroteUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __WroteInternalPosition(
                    __graphite_wrote.len(),
                );
                __graphite_wrote_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                wrote_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                wrote_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_wrote
                    .insert(
                        id,
                        __WroteRecord {
                            writer: from_position,
                            article: to_position,
                            byline,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Wrote) = |edge| {
            let _ = &edge.article;
        };
        let _: fn(&Wrote) = |edge| {
            let _ = &edge.writer;
        };
        for position in __graphite_node_article.positions() {
            let internal_position = __ArticleInternalPosition(position);
            let key = __graphite_node_article
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = wrote_to_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if count != 1usize {
                __violations
                    .push(Violation::WroteArticleEachViolation {
                        target: key.clone(),
                        count,
                    });
            }
        }
        for position in __graphite_node_author.positions() {
            let internal_position = __AuthorInternalPosition(position);
            let key = __graphite_node_author
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = wrote_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::WroteWriterEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let wrote_from_index = graphite::OptionalRoleIndex::from_buckets(
            (0..__graphite_node_author.len())
                .map(|position| {
                    wrote_from_index
                        .remove(&__AuthorInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let wrote_to_index = graphite::ExactlyOneRoleIndex::from_buckets(
            (0..__graphite_node_article.len())
                .map(|position| {
                    wrote_to_index
                        .remove(&__ArticleInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_author,
            __graphite_node_article,
            wrote: __graphite_wrote,
            wrote_from_index,
            wrote_to_index,
            __graphite_wrote_by_pair,
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
