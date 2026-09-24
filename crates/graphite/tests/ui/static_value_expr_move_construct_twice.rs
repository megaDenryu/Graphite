// 関数の中 (`in fn`) の値の式がローカル変数をmoveするとき、構築は1回しか
// できないことを固定するcompile-fail回帰試験。値の式を束縛するクロージャは
// moveした値を返すためFnOnceにしかならず、束縛マクロを2回呼ぶと2回目は
// 「すでにmoveされた値」エラーになる。これは通常のRustのmoveの意味と
// 一致する挙動であり、Graphite独自の制限ではない
// (`docs/static_graph.md`「値の式の名前解決」節)。schema・instance宣言は
// `static_value_expr_move_constructs_once.rs`と同じ内容にし、fingerprintが
// 一致する既存の生成ファイルをそのまま`include!`する。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod move二回組織 {
    include!("../generated/static_value_expr_move_construct_twice_move二回組織.rs");
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
    include!("../generated/static_value_expr_move_construct_twice_move二回チーム.rs");
}

fn 二回構築を試みる() {
    let 名前 = "たろう".to_string();

    #[rustfmt::skip]
    move二回組織! {
        generated = "generated/static_value_expr_move_construct_twice_move二回チーム.rs";
        graph move二回チーム in fn;
        node 太郎 = 社員 { 名前 };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }

    let _1回目 = move二回チーム::construct!();
    let _2回目 = move二回チーム::construct!();
}

fn main() {}
