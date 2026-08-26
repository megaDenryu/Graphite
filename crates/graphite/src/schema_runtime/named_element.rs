//! 名前付き要素の内部位置を `Graph` の借用へ束縛して参照値へ変える契約を所有する。

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
