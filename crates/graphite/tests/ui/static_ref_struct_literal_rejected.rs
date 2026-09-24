// `{個体名}Ref`/`{辺名}Ref` の配線フィールド (`graph`) が非公開であり、
// 利用者が構造体リテラルで直接作れないことを固定する回帰試験。非公開に
// する前は、親moduleから構造体リテラルで別の`Graph`を混ぜた不整合な参照を
// 組み立てられた (issue #41)。schemaの宣言は`static_multi_module.rs`と
// 同じ内容にし、fingerprintが一致する既存の生成ファイルをそのまま
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
    let g = 開発チーム::construct!();

    // 別の由来の`Graph`を、構造体リテラルで直接組み合わせようとする迂回は
    // コンパイルエラーになる (フィールドが非公開のため)。
    let _迂回 = 開発チーム::太郎Ref { graph: &g };
}
