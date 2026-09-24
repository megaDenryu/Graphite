//! 関数の中 (`in fn`) の値の式がローカル変数をmoveするとき、構築は1回だけ
//! 成功することを固定する回帰試験 (`docs/static_graph.md`「値の式の名前
//! 解決」節)。値の式を束縛するクロージャはmoveした値を返すためFnOnceにしか
//! ならない。2回目の構築がコンパイルエラーになる側の固定は
//! `crates/graphite/tests/ui/static_value_expr_move_construct_twice.rs`
//! (このファイルと同じschema・instance宣言を再利用する) が持つ。
pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod move二回組織 {
    include!("generated/static_value_expr_move_construct_twice_move二回組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_move_construct_twice_move二回組織.rs";
    schema move二回組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod move二回チーム {
    include!("generated/static_value_expr_move_construct_twice_move二回チーム.rs");
}

fn 一回だけ構築する() -> String {
    let 名前 = "たろう".to_string();

    #[rustfmt::skip]
    move二回組織! {
        generated = "generated/static_value_expr_move_construct_twice_move二回チーム.rs";
        graph move二回チーム in fn;
        node 太郎 = 社員 { 名前 };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }

    let g = move二回チーム::construct!();
    g.node_refs().太郎().entity().名前.clone()
}

#[test]
fn moveする値の式は1回だけの構築なら成功する() {
    assert_eq!(一回だけ構築する(), "たろう");
}
