// 内部構築子 (`__graphite_internal_new`) を利用者のコードから直接呼ぶと
// `#[deprecated]`警告になり、`#![deny(warnings)]`の下ではエラーになる
// ことを固定する回帰試験。`pub(crate)`はクレート内のどこからでも呼べて
// しまうため、stable Rustの可視性だけでは「呼べるのは
// `construct::nodes!`だけ」という主張を強制できない (`node_entities.rs`
// 冒頭コメント参照)。schemaの宣言は`static_multi_module.rs`と同じ内容に
// し、fingerprintが一致する既存の生成ファイルをそのまま`include!`する。

#![deny(warnings)]

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
    let nodes = 開発チーム::construct::nodes!();
    let _edges = 開発チーム::construct::edges!(&nodes);
    let _差し替え = 開発チーム::Nodes::__graphite_internal_new(
        社員 { 名前: "差し替え".into() },
        部署 { 名前: "差し替え".into() },
    );
}
