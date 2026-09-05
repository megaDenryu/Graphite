// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/lib.rs:42
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    13942286671104915267u64, 8965300523475896168u64, 8491419155506059333u64,
    16335442955436624129u64,
];
/// `Book` ノードの公開ID。
///
/// 宣言: `src/lib.rs` の `node Book`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BookId(pub String);
/// `Reader` ノードの公開ID。
///
/// 宣言: `src/lib.rs` の `node Reader`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReaderId(pub String);
/// `Borrowed` 辺の公開ID。
///
/// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BorrowedId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __BookInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ReaderInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __BorrowedInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __BookNamedPosition(__BookInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ReaderNamedPosition(__ReaderInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __BorrowedNamedPosition(__BorrowedInternalPosition, u64);
/// 構築時に組み立てる `Borrowed` 辺の値。
///
/// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
#[derive(Clone, PartialEq)]
pub struct Borrowed {
    /// この辺の始点ノードの公開ID。
    pub book: BookId,
    /// この辺の終点ノードの公開ID。
    pub reader: ReaderId,
    /// この辺が運ぶ積み荷。
    pub loan: Loan,
}
impl Borrowed {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn new(from: BookId, to: ReaderId, payload: Loan) -> Self {
        Self {
            book: from,
            reader: to,
            loan: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn payload(&self) -> &Loan {
        &self.loan
    }
}
impl graphite::DirectedEdgeLiteral<BookId, ReaderId, Loan> for Borrowed {
    fn from_graph_literal(from: BookId, to: ReaderId, payload: Loan) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Borrowed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Borrowed))
    }
}
#[allow(dead_code)]
struct __BorrowedRecord {
    book: __BookInternalPosition,
    reader: __ReaderInternalPosition,
    loan: Loan,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `src/lib.rs` の `schema Library`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    DuplicateBook(BookId),
    /// このノード種別のキーが重複している。
    DuplicateReader(ReaderId),
    /// このエッジ種別のキーが重複している。
    BorrowedDuplicateKey(BorrowedId),
    /// このエッジが未知の始点キーを参照している。
    BorrowedUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: BorrowedId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: BookId,
    },
    /// このエッジが未知の終点キーを参照している。
    BorrowedUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: BorrowedId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: ReaderId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    BorrowedBookEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: BookId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateBook(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Book", id)
            }
            Violation::DuplicateReader(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Reader", id)
            }
            Violation::BorrowedDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Borrowed", id)
            }
            Violation::BorrowedUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Borrowed", edge, "Book", source
                )
            }
            Violation::BorrowedUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Borrowed", edge, "Reader", target
                )
            }
            Violation::BorrowedBookEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Borrowed", "Book", source, "0..1", count
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
/// 宣言: `src/lib.rs` の `schema Library`
pub struct Graph {
    __graphite_node_book: graphite::KeyedTable<BookId, super::Book>,
    __graphite_node_reader: graphite::KeyedTable<ReaderId, super::Reader>,
    borrowed: graphite::KeyedTable<BorrowedId, __BorrowedRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    borrowed_from_index: graphite::OptionalRoleIndex<__BorrowedInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    borrowed_to_index: graphite::MultipleRoleIndex<__BorrowedInternalPosition>,
    __graphite_borrowed_by_pair: std::collections::HashMap<
        (__BookInternalPosition, __ReaderInternalPosition),
        Vec<__BorrowedInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn book_by_id<'graph>(&'graph self, id: &BookId) -> Option<BookRef<'graph>> {
        let internal_position = __BookInternalPosition(
            self.__graphite_node_book.position(id)?,
        );
        Some(BookRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn book_value_mut(&mut self, id: &BookId) -> Option<&mut super::Book> {
        self.__graphite_node_book.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn book_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph BookId> {
        self.__graphite_node_book.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn book_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = BookRef<'graph>> + 'graph {
        self.__graphite_node_book
            .positions()
            .map(move |position| BookRef {
                graph: self,
                internal_position: __BookInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn book_len(&self) -> usize {
        self.__graphite_node_book.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn reader_by_id<'graph>(
        &'graph self,
        id: &ReaderId,
    ) -> Option<ReaderRef<'graph>> {
        let internal_position = __ReaderInternalPosition(
            self.__graphite_node_reader.position(id)?,
        );
        Some(ReaderRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn reader_value_mut(&mut self, id: &ReaderId) -> Option<&mut super::Reader> {
        self.__graphite_node_reader.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn reader_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ReaderId> {
        self.__graphite_node_reader.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn reader_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ReaderRef<'graph>> + 'graph {
        self.__graphite_node_reader
            .positions()
            .map(move |position| ReaderRef {
                graph: self,
                internal_position: __ReaderInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn reader_len(&self) -> usize {
        self.__graphite_node_reader.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_by_id<'graph>(
        &'graph self,
        id: &BorrowedId,
    ) -> Option<BorrowedRef<'graph>> {
        Some(BorrowedRef {
            graph: self,
            internal_position: __BorrowedInternalPosition(self.borrowed.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_payload_mut(&mut self, id: &BorrowedId) -> Option<&mut Loan> {
        self.borrowed.get_mut(id).map(|record: &mut __BorrowedRecord| &mut record.loan)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph BorrowedId> {
        self.borrowed.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = BorrowedRef<'graph>> + 'graph {
        self.borrowed
            .positions()
            .map(move |position| BorrowedRef {
                graph: self,
                internal_position: __BorrowedInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_len(&self) -> usize {
        self.borrowed.len()
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
/// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
#[derive(Clone, Copy)]
pub struct BorrowedRef<'graph> {
    graph: &'graph Graph,
    internal_position: __BorrowedInternalPosition,
}
impl<'graph> BorrowedRef<'graph> {
    fn record(self) -> &'graph __BorrowedRecord {
        self.graph
            .borrowed
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn id(self) -> &'graph BorrowedId {
        self.graph
            .borrowed
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn book(self) -> BookRef<'graph> {
        BookRef {
            graph: self.graph,
            internal_position: __BookInternalPosition(self.record().book.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn reader(self) -> ReaderRef<'graph> {
        ReaderRef {
            graph: self.graph,
            internal_position: __ReaderInternalPosition(self.record().reader.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn from(self) -> BookRef<'graph> {
        self.book()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn to(self) -> ReaderRef<'graph> {
        self.reader()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn from_id(self) -> &'graph BookId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn to_id(self) -> &'graph ReaderId {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn loan(self) -> &'graph Loan {
        &self.record().loan
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn payload(self) -> &'graph Loan {
        &self.record().loan
    }
}
impl<'graph> std::fmt::Debug for BorrowedRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(BorrowedRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `src/lib.rs` の `schema Library`
pub struct Builder {
    __graphite_node_book: Vec<(BookId, super::Book)>,
    __graphite_node_reader: Vec<(ReaderId, super::Reader)>,
    borrowed: Vec<(BorrowedId, Borrowed)>,
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
pub trait LibraryInsertable: Sized {
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
pub trait LibraryDefaultId: LibraryInsertable {
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
pub trait LibraryNode: LibraryInsertable {}
impl LibraryInsertable for super::Book {
    type Id = BookId;
    type NamedPosition = __BookNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __BookNamedPosition(
            __BookInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_book.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.book(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.book(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __BookNamedPosition {
    type Reference<'graph> = BookRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        BookRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl LibraryDefaultId for super::Book {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        LibraryInsertable::insert_named_with_id(self, b, BookId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        LibraryInsertable::insert_with_id(self, b, BookId(binding))
    }
}
impl LibraryNode for super::Book {}
/// 完成済みグラフ上の `Book` ノード個体。
///
/// 宣言: `src/lib.rs` の `node Book`
#[derive(Clone, Copy)]
pub struct BookRef<'graph> {
    graph: &'graph Graph,
    internal_position: __BookInternalPosition,
}
impl<'graph> BookRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn id(self) -> &'graph BookId {
        self.graph
            .__graphite_node_book
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn value(self) -> &'graph super::Book {
        self.graph
            .__graphite_node_book
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_as_book(self) -> Option<BorrowedRef<'graph>> {
        self.graph
            .borrowed_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| BorrowedRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_try_between(
        self,
        other: ReaderRef<'graph>,
    ) -> Result<
        impl Iterator<Item = BorrowedRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_borrowed_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| BorrowedRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::borrowed_try_between`] を使う。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_between(
        self,
        other: ReaderRef<'graph>,
    ) -> impl Iterator<Item = BorrowedRef<'graph>> + 'graph {
        self.borrowed_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(BookRef), stringify!(borrowed_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for BookRef<'graph> {
    type Target = super::Book;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_book
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for BookRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(BookRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl LibraryInsertable for super::Reader {
    type Id = ReaderId;
    type NamedPosition = __ReaderNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ReaderNamedPosition(
            __ReaderInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_reader.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.reader(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.reader(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ReaderNamedPosition {
    type Reference<'graph> = ReaderRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ReaderRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl LibraryDefaultId for super::Reader {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        LibraryInsertable::insert_named_with_id(self, b, ReaderId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        LibraryInsertable::insert_with_id(self, b, ReaderId(binding))
    }
}
impl LibraryNode for super::Reader {}
/// 完成済みグラフ上の `Reader` ノード個体。
///
/// 宣言: `src/lib.rs` の `node Reader`
#[derive(Clone, Copy)]
pub struct ReaderRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ReaderInternalPosition,
}
impl<'graph> ReaderRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn id(self) -> &'graph ReaderId {
        self.graph
            .__graphite_node_reader
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn value(self) -> &'graph super::Reader {
        self.graph
            .__graphite_node_reader
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed_as_reader(
        self,
    ) -> impl Iterator<Item = BorrowedRef<'graph>> + 'graph {
        let positions = self.graph.borrowed_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| BorrowedRef {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for ReaderRef<'graph> {
    type Target = super::Reader;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_reader
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for ReaderRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ReaderRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait LibraryEdge: LibraryInsertable {}
impl LibraryInsertable for Borrowed {
    type Id = BorrowedId;
    type NamedPosition = __BorrowedNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __BorrowedNamedPosition(
            __BorrowedInternalPosition(
                graphite::TablePosition::from_index(b.borrowed.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.borrowed(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.borrowed(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __BorrowedNamedPosition {
    type Reference<'graph> = BorrowedRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        BorrowedRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl LibraryDefaultId for Borrowed {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        LibraryInsertable::insert_named_with_id(self, b, BorrowedId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        LibraryInsertable::insert_with_id(self, b, BorrowedId(binding))
    }
}
impl LibraryEdge for Borrowed {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_book: Vec::new(),
            __graphite_node_reader: Vec::new(),
            borrowed: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/lib.rs` の `node Book`
    pub fn book(&mut self, id: BookId, value: super::Book) -> &mut Self {
        self.__graphite_node_book.push((id, value));
        self
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/lib.rs` の `node Reader`
    pub fn reader(&mut self, id: ReaderId, value: super::Reader) -> &mut Self {
        self.__graphite_node_reader.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/lib.rs` の `edge Borrowed = (book: Book) -[loan: Loan]-> (reader: Reader) where each book: 0..1`
    pub fn borrowed(&mut self, id: BorrowedId, value: Borrowed) -> &mut Self {
        self.borrowed.push((id, value));
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
        N: LibraryNode + LibraryDefaultId,
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
        N: LibraryNode + LibraryDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: LibraryNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: LibraryNode>(
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
        E: LibraryEdge + LibraryDefaultId,
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
        E: LibraryEdge + LibraryDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: LibraryEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: LibraryEdge>(
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
        T: LibraryDefaultId,
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
        let mut __graphite_node_book: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_book {
            if !__graphite_node_book.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateBook(id));
            }
        }
        let mut __graphite_node_reader: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_reader {
            if !__graphite_node_reader.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateReader(id));
            }
        }
        let mut __graphite_borrowed: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut borrowed_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut borrowed_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_borrowed_by_pair: std::collections::HashMap<
            (__BookInternalPosition, __ReaderInternalPosition),
            Vec<__BorrowedInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.borrowed {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::BorrowedDuplicateKey(id));
                continue;
            }
            let Borrowed { book: from, reader: to, loan } = value;
            let from_position = __graphite_node_book
                .position(&from)
                .map(__BookInternalPosition);
            let to_position = __graphite_node_reader
                .position(&to)
                .map(__ReaderInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::BorrowedUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::BorrowedUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __BorrowedInternalPosition(
                    graphite::TablePosition::from_index(__graphite_borrowed.len()),
                );
                __graphite_borrowed_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                borrowed_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                borrowed_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_borrowed
                    .insert(
                        id,
                        __BorrowedRecord {
                            book: from_position,
                            reader: to_position,
                            loan,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Borrowed) = |edge| {
            let _ = &edge.book;
        };
        for position in __graphite_node_book.positions() {
            let internal_position = __BookInternalPosition(position);
            let key = __graphite_node_book
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = borrowed_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::BorrowedBookEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let borrowed_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_book
                .positions()
                .map(|position| {
                    borrowed_from_index
                        .remove(&__BookInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let borrowed_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_reader
                .positions()
                .map(|position| {
                    borrowed_to_index
                        .remove(&__ReaderInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_book,
            __graphite_node_reader,
            borrowed: __graphite_borrowed,
            borrowed_from_index,
            borrowed_to_index,
            __graphite_borrowed_by_pair,
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
