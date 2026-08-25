//! Graphite — グラフ指向データ構造のランタイムライブラリ。
//!
//! このクレートは利用者が唯一 depend するクレートであり、
//! `graphite-macros` (proc-macro クレート) の内容を re-export する
//! (serde/serde_derive と同じ 2 クレート構成)。
//!
//! 水準1相当のジェネリックグラフ [`Graph`] (フェーズ2) に加え、フェーズ3で
//! 水準2相当の図式グラフスキーマを宣言する `graph_schema!` と、インスタンス
//! リテラル `graph!` を実装した (`graphite-macros` から re-export)。
//! `flow!` (`docs/flow_macro.md`) はこれらとは独立した別レイヤで、データの辺
//! (宣言) とは対照的な「関数の辺」(即時実行) を文位置マクロとして提供する。
//! 設計の一次資料:
//! - `../../../Bullet/docs/rust_graph_extension_sketch.md`
//! - `../../../Bullet/docs/graph_design_sketches.md`

mod compute;
mod graph;
mod keyed_table;
mod unordered_pair;

use std::sync::atomic::{AtomicU64, Ordering};
use std::{fmt, ops::Range};

pub use compute::{ComputeGraph, ComputeGraphBuilder, ComputeGraphError};
pub use graph::{CycleError, Graph, GraphBuilder, GraphError};
pub use keyed_table::KeyedTable;
pub use unordered_pair::UnorderedPair;

/// 異なる `Graph` から得た参照を1つの検索へ渡したことを表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphMismatch;

impl fmt::Display for GraphMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("異なる Graph の値から得た参照は組み合わせられません。同じ graph! または同じ Graph の値から得た参照だけを渡してください")
    }
}

impl std::error::Error for GraphMismatch {}

/// 多重度制約のない役割索引を、役割ごとの範囲と連続した辺位置列で保持する。
#[doc(hidden)]
pub struct MultipleRoleIndex<P> {
    ranges: Vec<Range<usize>>,
    positions: Vec<P>,
}

#[doc(hidden)]
pub struct ExactlyOneRoleIndex<P>(Vec<P>);

impl<P> ExactlyOneRoleIndex<P> {
    #[doc(hidden)]
    pub fn from_buckets(buckets: Vec<Vec<P>>) -> Self {
        Self(
            buckets
                .into_iter()
                .map(|mut bucket| {
                    assert_eq!(
                        bucket.len(),
                        1,
                        "多重度1の役割索引には各ノードの辺位置が1つ必要です"
                    );
                    bucket.pop().expect("長さを検査済みです")
                })
                .collect(),
        )
    }

    #[doc(hidden)]
    pub fn get(&self, position: usize) -> &P {
        &self.0[position]
    }
}

#[doc(hidden)]
pub struct OptionalRoleIndex<P>(Vec<Option<P>>);

impl<P> OptionalRoleIndex<P> {
    #[doc(hidden)]
    pub fn from_buckets(buckets: Vec<Vec<P>>) -> Self {
        Self(
            buckets
                .into_iter()
                .map(|mut bucket| {
                    assert!(
                        bucket.len() <= 1,
                        "多重度0..1の役割索引には辺位置を高々1つだけ格納できます"
                    );
                    bucket.pop()
                })
                .collect(),
        )
    }

    #[doc(hidden)]
    pub fn get(&self, position: usize) -> Option<&P> {
        self.0.get(position).and_then(Option::as_ref)
    }
}

impl<P> MultipleRoleIndex<P> {
    #[doc(hidden)]
    pub fn from_buckets(buckets: Vec<Vec<P>>) -> Self {
        let mut positions = Vec::with_capacity(buckets.iter().map(Vec::len).sum());
        let ranges = buckets
            .into_iter()
            .map(|bucket| {
                let start = positions.len();
                positions.extend(bucket);
                start..positions.len()
            })
            .collect();
        Self { ranges, positions }
    }

    #[doc(hidden)]
    pub fn get(&self, position: usize) -> &[P] {
        self.ranges
            .get(position)
            .map(|range| &self.positions[range.clone()])
            .unwrap_or(&[])
    }
}

/// `graph!` が有向の柄から辺値を構築するための内部契約。
#[doc(hidden)]
pub trait DirectedEdgeLiteral<From, To, Payload>: Sized {
    fn from_graph_literal(from: From, to: To, payload: Payload) -> Self;
}

/// `graph!` が無向の柄から辺値を構築するための内部契約。
#[doc(hidden)]
pub trait UndirectedEdgeLiteral<Endpoint, Payload>: Sized {
    fn from_graph_literal(first: Endpoint, second: Endpoint, payload: Payload) -> Self;
}

/// `graph!` が名前付き要素の内部位置から `Graph` の借用に束縛された参照値
/// (`NodeRef`/`EdgeRef`) を直接構築するための内部契約。公開 ID の索引は
/// 経由しない。
#[doc(hidden)]
pub trait NamedGraphElement<G> {
    type Reference<'graph>
    where
        G: 'graph;

    fn bind<'graph>(&self, graph: &'graph G) -> Self::Reference<'graph>;
}

/// `graph!` の1回の構築を識別する印を採番するための、クレート全体で
/// 唯一のカウンタ。値そのものに意味はなく、二度と同じ値を発行しないことだけ
/// が要件なので、採番の受け渡しに順序保証は要らない。
static 構築印カウンタ: AtomicU64 = AtomicU64::new(0);

/// `graph!` の1回の構築を識別する印を新しく1つ発行する。名前付き位置が
/// 生成元と異なる `Graph` へ [`NamedGraphElement::bind`] されるのを実行時に
/// 検出するために使う。`graph_schema!` が生成する `Builder::new()` がこれを
/// 呼び、同じ builder から生まれる `Graph` と全ての名前付き位置へ同じ値を
/// 埋め込む。`bind` はこの値を照合し、一致しなければ契約違反として
/// `panic!` する (`# Panics` の考え方は `docs/design_principles.md` 原則2 —
/// この違反は builder/Graph の取り違えという呼び出し規約違反であり、通常の
/// ドメインエラーではない)。採番の順序に意味はなく重複しなければよいため
/// `Relaxed` で十分。加算は `checked_add` で行い、カウンタが `u64` の上限に
/// 達して次の値を発行できない場合は無言で一周させず `panic!` する
/// (到達したらバグの様式。実運用で `graph!` を `u64::MAX` 回呼ぶことは
/// 想定していない)。
#[doc(hidden)]
pub fn 次の構築印を発行する() -> u64 {
    構築印カウンタ
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |現在の値| {
            現在の値.checked_add(1)
        })
        .expect("構築印が u64 を使い切りました")
}

/// 名前付き要素の内部位置 (`{Schema}::{Type}NamedPosition`) を builder へ
/// 積む操作の許可証。
///
/// フィールドが非公開なため、この型の値はこのクレート内でしか作れない。
/// 公開の構築経路は [`build_named_graph`] だけで、`{Schema}::Graph::create`
/// のクロージャ (`FnOnce(&mut Builder)`) には署名上そもそも許可証が渡らない
/// ため、`insert_named`/`add_named` 系の呼び出しへ到達できない。これにより
/// 「`create` のクロージャで名前付き要素の内部位置 (`NamedPosition`、`Copy`)
/// を外の変数へ退避し、別の `Graph` の `bind` へ渡すと無言で別要素を指して
/// しまう」という取り違えの経路を、型で塞ぐ。
///
/// この許可証が塞ぐのは「`{Schema}::Graph::create` の通常経路」からの偶発的
/// 誤用だけである。`create_named` 自体は `#[doc(hidden)]` の `pub fn` であり、
/// 呼び出し規約を無視して直接呼べば許可証はクロージャへ渡ってくるため、
/// 許可証だけでは名前付き位置の持ち出しそのものは封鎖できない。持ち出した
/// 名前付き位置を別の `Graph` へ渡す誤用の検出は、上記の構築印の照合が担う。
#[doc(hidden)]
pub struct NamedInsertPermit {
    _private: (),
}

/// `graph_schema!` が生成する builder が実装する、凍結操作の内部契約。
/// [`build_named_graph`] が `Graph`/`Violation` の具体型を知らずに `freeze()`
/// を呼べるようにするためだけの橋渡しであり、利用者が直接実装することは
/// 想定しない。
#[doc(hidden)]
pub trait FreezableBuilder {
    type Graph;
    type Violation;

    fn freeze_into_graph(self) -> Result<Self::Graph, Self::Violation>;
}

/// 名前付き要素の内部位置を凍結境界の外まで運ぶ、唯一の構築経路。
/// `{Schema}::Graph::create_named` の生成コードはこの関数へ薄く委譲するだけで、
/// [`NamedInsertPermit`] はここでしか作らない。クロージャ `f` は
/// `&mut Builder` に加えて `&NamedInsertPermit` を受け取るため、
/// `insert_named`/`add_named` 系メソッドをこのクロージャの中でだけ呼べる。
#[doc(hidden)]
pub fn build_named_graph<B, F, N>(
    new_builder: impl FnOnce() -> B,
    f: F,
) -> Result<(B::Graph, N), B::Violation>
where
    B: FreezableBuilder,
    F: for<'b> FnOnce(&'b mut B, &'b NamedInsertPermit) -> N,
{
    let mut builder = new_builder();
    let permit = NamedInsertPermit { _private: () };
    let named_positions = f(&mut builder, &permit);
    builder
        .freeze_into_graph()
        .map(|graph| (graph, named_positions))
}

pub use graphite_macros::{flow, graph, graph_schema};
