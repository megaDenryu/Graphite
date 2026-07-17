//! `graph!` リテラルの脱糖はノード項ごとに
//! `let key = __graphite_b.insert("key", 式);` という `let` 束縛を生成する
//! (`crates/graphite-macros/src/instance_codegen.rs` 項目G1)。この束縛は
//! そのノードが以後のエッジ項で参照されて初めて読まれるため、どのエッジにも
//! 使われない**孤立ノード**では rustc の `unused variable` 警告が出る。
//! 孤立ノードはグラフとして正当 (ノード種別の宣言・多重度検査のいずれにも
//! 違反しない) なので、この警告はユーザーのグラフ設計の問題ではなく
//! マクロの実装詳細に起因するノイズである。
//!
//! `instance_codegen.rs` はノード項の `let` 束縛に
//! `#[allow(unused_variables)]` を付与してこれを抑制している。このファイルは
//! ファイル全体を `#![deny(unused_variables)]` にした上でエッジを1本も
//! 張らない `graph!` を書くことで、抑制が機能していることを保証する
//! 回帰テスト。抑制が外れれば `unused variable` 警告が `deny` によって
//! コンパイルエラーへ格上げされ、このファイルはコンパイルに失敗する。

#![deny(unused_variables)]

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ
/// (`docs/edge_syntax_v2.md`)。
#[derive(Debug, Clone, PartialEq)]
pub struct Widget {
    pub name: String,
}

// エッジを1本も持たないスキーマ。孤立ノードのみで完結するグラフを
// 素直に表現するために、あえてエッジ無しにしてある (`///` ではなく `//`
// にしているのは、マクロ呼び出し直前の doc comment は rustdoc に
// 展開してもらえず `unused_doc_comments` 警告の原因になるため)。
#[rustfmt::skip]
graphite::graph_schema! {
    schema WidgetGraph {
        node Widget;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn 孤立ノードだけのgraphリテラルは警告なしで通る() {
        // a, b はいずれもどのエッジにも使われない孤立ノード。
        // `#![deny(unused_variables)]` のもとでこのファイルがコンパイル
        // できていること自体が、instance_codegen.rs の
        // `#[allow(unused_variables)]` が効いている証拠になる。
        let g = graphite::graph!(WidgetGraph {
            a = Widget { name: "A".into() },
            b = Widget { name: "B".into() },
        })
        .expect("エッジの無い graph! も構築に成功するはず");

        assert_eq!(
            Widget::get(&g, &WidgetId("a".to_string())).unwrap().name,
            "A"
        );
        assert_eq!(
            Widget::get(&g, &WidgetId("b".to_string())).unwrap().name,
            "B"
        );
    }
}
