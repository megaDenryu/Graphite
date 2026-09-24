// `Graph` が個体・積み荷を直接持つフィールドが非公開であることを固定する
// (issue #41、PR #45)。値ありの個体はinstance宣言の式からのみ供給され、
// 利用者がフィールドへ直接アクセスして迂回できない。手書きの模型ではなく、
// 実際の生成ファイル (`cargo xtask generate`が書いたもの) をそのまま
// `include!`して検査する (schema・instanceの宣言は`static_multi_module.rs`
// と同じ内容にし、fingerprintが一致する既存の生成ファイルを使う)。

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
    let g = 開発チーム::construct!();
    // フィールドへの直接アクセスはできない (private field)。
    let _ = g.太郎;
}
