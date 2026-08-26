//! 名前付き要素の内部位置を凍結境界の外へ運ぶ唯一の構築経路と、その許可証を所有する。

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
/// 名前付き位置を別の `Graph` へ渡す誤用の検出は、構築印の照合
/// ([`crate::次の構築印を発行する`]) が担う。
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
