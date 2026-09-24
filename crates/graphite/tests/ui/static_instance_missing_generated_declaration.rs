// `generated = "...";` を書かずに静的グラフのinstanceマクロを呼んだ場合の
// 診断を固定する (issue #41 段階4)。`missing_generated_declaration.rs`
// (動的グラフ) と対になる静的グラフ版。

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

組織! {
    graph 開発チーム;
    node 太郎: 社員 = 社員を作る();
}

fn main() {}
