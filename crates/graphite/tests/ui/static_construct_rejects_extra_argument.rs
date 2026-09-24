// `construct!` が、値なし宣言 (`node <名前>: <型>;`) の個体の数を超える
// 引数を渡すと拒否することを固定する回帰試験。この生成物の全ての個体は
// instance宣言の式から値を持つため (`static_multi_module.rs`と同じ内容)、
// `construct!` は引数を1つも取らない。単一の構築入口へ統合した後も、
// 利用者が値ありの個体の実体をこの入口から差し替えられないことを
// macro_rules!の展開失敗として確かめる (由来の異なる`Nodes`/`Edges`を
// 組み合わせる旧来の懸念は、`Nodes`/`Edges`という型自体が公開契約から
// 消えたことで構造的に成立しなくなった。詳細は`docs/static_graph.md`
// 「生成される名前の公開契約」参照)。schemaの宣言は`static_multi_module.rs`
// と同じ内容にし、fingerprintが一致する既存の生成ファイルをそのまま
// `include!`する。

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
    // 値なし宣言が0件なので `construct!` は引数を取らない。
    let _g = 開発チーム::construct!(社員 { 名前: "差し替え".into() });
}
