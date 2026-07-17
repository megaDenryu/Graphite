// スプライス項 `..式` の式が `IntoIterator<Item = (K, T)>` を満たさない場合の
// エラーを検証する (`docs/graph_splice.md` §4)。マクロ自身は式の型を一切
// パースせず、脱糖後の `__graphite_b.extend(not_pairs)` 呼び出しが素の rustc
// 型検査 (トレイト境界違反) に落ちることで検出される。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

graphite::graph_schema! {
    schema SpliceBad {
        node Person;
    }
}

fn main() {
    let not_pairs: Vec<i32> = vec![1, 2, 3];
    let _ = graphite::graph!(SpliceBad {
        ..not_pairs,
    });
}
