//! instanceを関数の中 (`graph <名前> in fn;`) に置き、値の式がローカル変数を
//! 借りるだけ (`.clone()`で読むだけ) のとき、instance宣言の後ろでも同じ
//! ローカル変数を使い続けられることを固定する回帰試験
//! (`docs/static_graph.md`「値の式の名前解決」節)。値の式を束縛する
//! クロージャは`move`を付けないため、参照だけを必要とする式では
//! ローカル変数を消費しない (通常のRustのクロージャ捕捉推論と同じ)。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 借用組織 {
    include!("generated/static_value_expr_borrows_local_leaves_it_usable_借用組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_borrows_local_leaves_it_usable_借用組織.rs";
    schema 借用組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 借用チーム {
    include!("generated/static_value_expr_borrows_local_leaves_it_usable_借用チーム.rs");
}

fn 宣言の後でローカルを使う() -> (String, usize) {
    let ローカル = String::from("たろう");

    #[rustfmt::skip]
    借用組織! {
        generated = "generated/static_value_expr_borrows_local_leaves_it_usable_借用チーム.rs";
        graph 借用チーム in fn;
        node 太郎 = 社員 { 名前: ローカル.clone() };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }

    // 値の式は`ローカル`を借りるだけ (`.clone()`) なので、instance宣言の
    // 後ろでも`ローカル`をそのまま使い続けられる。
    let 長さ = ローカル.len();

    let g = 借用チーム::construct!();
    (g.node_refs().太郎().entity().名前.clone(), 長さ)
}

#[test]
fn 借りるだけの値の式は宣言の後でもローカル変数を消費しない() {
    let (名前, 長さ) = 宣言の後でローカルを使う();
    assert_eq!(名前, "たろう");
    assert_eq!(長さ, "たろう".len());
}
