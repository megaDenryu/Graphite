// `Graph`・`NodeRefs`・`EdgeRefs` の配線フィールド (個体名・辺名を直接持つ
// フィールド、`graph`) が非公開であり、利用者が構造体リテラルで直接作れ
// ないことを固定する回帰試験。非公開にする前は、
// `Graph { 太郎: g1.node_refs().太郎().entity()..、.. }` のように、2つの
// グラフの部品を構造体リテラルで混ぜた不整合な `Graph` を組み立てられた。
// schemaの宣言は`static_multi_module.rs`と同じ内容にし、fingerprintが
// 一致する既存の生成ファイルをそのまま`include!`する。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 組織 {
    include!("../generated/static_multi_module_組織.rs");
}

#[rustfmt::skip]
graphite::static_graph_schema! {
    generated = "generated/static_multi_module_組織.rs";
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 開発チーム {
    include!("../generated/static_multi_module_開発チーム.rs");
}

#[rustfmt::skip]
組織! {
    generated = "generated/static_multi_module_開発チーム.rs";
    graph 開発チーム;
    node 太郎 = 社員 { 名前: "太郎".into() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

fn main() {
    let _g = 開発チーム::construct!();

    // 構造体リテラルで直接`Graph`を組み立てようとする迂回はコンパイル
    // エラーになる (`Graph`のフィールドが個体を直接持ち、非公開のため)。
    let _迂回 = 開発チーム::Graph {
        太郎: 社員 { 名前: "差し替え".into() },
        開発部: 部署 { 名前: "差し替え".into() },
    };
}
