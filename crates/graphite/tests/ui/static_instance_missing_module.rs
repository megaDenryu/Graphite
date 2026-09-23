// instanceの `generated = "...";` は書いたが、instance module
// (`mod 開発チーム { include!(..); }`) の宣言そのものを忘れた場合の診断を
// 固定する (issue #41)。`開発チーム::..` を参照するコード全てが解決に
// 失敗し、2件のE0433になる。
//
// 1件目は指紋照合コードが参照する `開発チーム::__GRAPHITE_STATIC_INSTANCE_FINGERPRINT`。
// このパスの先頭識別子だけ `generated = "..."` リテラルのspanで組み立てる
// (`instance_entry.rs`) ため、`generated` の行を指す。
// 2件目はDSLトークンの型参照が参照する `開発チーム::太郎Ref` 等。
// instance自身のトークン (`graph 開発チーム;` のグラフ名トークン) のspanを
// そのまま使う設計 (`docs/static_graph.md` 「追跡の契約」) のため、
// `graph 開発チーム;` の行を指す。`Nodes`・`Edges`型はconstruct macro
// (`construct::nodes!`/`construct::edges!`) を実際に呼んで初めて参照される
// ため、このテストのように呼ばない場合は
// `開発チーム::Nodes`・`開発チーム::Edges` へのE0433も、petgraphの同名
// structへの無関係なimport提案も出ない。

struct 社員;

#[allow(non_snake_case)]
mod 組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
        10319720238056268413,
        1420965213717410858,
        810849268007507131,
        13845130453493871279,
    ];
}

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
    }
}

fn 社員を作る() -> 社員 {
    社員
}

// `mod 開発チーム { .. }` の宣言を書き忘れた状態を再現する。
組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 太郎: 社員 = 社員を作る();
}

fn main() {}
