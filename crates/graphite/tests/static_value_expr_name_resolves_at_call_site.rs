//! instanceの値の式の中の、ローカル変数以外の名前 (関数名等) が
//! `construct::nodes!`/`construct::edges!`を呼んだ位置を起点に解決される
//! ことを固定する回帰試験 (`docs/static_graph.md`「制約」節)。instance宣言
//! と同じテキスト順スコープの範囲内でも、呼び出し位置に同じ名前の別の
//! 項目が見えていれば、値の式が指す先はそちらへすり替わる。この試験は、
//! 対照群 (instanceの外の呼び出しがそのまま外側の関数を呼ぶ場合) に加え、
//! instanceの後ろに書いた関数の中から呼ぶ場合と、instanceの後ろに書いた
//! インラインの子moduleから呼ぶ場合の両方ですり替わることを、実際に
//! すり替わった値を読んで確かめる。この挙動が将来変わったらこの試験が
//! 知らせる。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 名前解決組織 {
    include!("generated/static_value_expr_name_resolves_at_call_site_名前解決組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_name_resolves_at_call_site_名前解決組織.rs";
    schema 名前解決組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

// instanceの値の式が呼ぶ、モジュール直下の関数。呼び出し位置に同名の別の
// 項目が無ければ、値の式はこの関数を呼ぶ。
fn 名前を作る() -> String {
    "外側".into()
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 名前解決チームトップレベル {
    include!("generated/static_value_expr_name_resolves_at_call_site_名前解決チームトップレベル.rs");
}

// instanceと同じmodule直下 (関数の外) からそのまま`構築!`を呼ぶ、対照群。
// 呼び出し位置を覆う同名の項目が無いため、値の式は素直にモジュール直下の
// `名前を作る()`を呼ぶ。
#[rustfmt::skip]
名前解決組織! {
    generated = "generated/static_value_expr_name_resolves_at_call_site_名前解決チームトップレベル.rs";
    graph 名前解決チームトップレベル;
    node 太郎 = 社員 { 名前: 名前を作る() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

fn トップレベルでは外側の同名関数のままである() -> String {
    let nodes = 名前解決チームトップレベル::construct::nodes!();
    let edges = 名前解決チームトップレベル::construct::edges!(&nodes);
    let g = 名前解決チームトップレベル::Graph::new(&edges);
    g.node_refs().太郎().entity().名前.clone()
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 名前解決チーム関数内 {
    include!("generated/static_value_expr_name_resolves_at_call_site_名前解決チーム関数内.rs");
}

#[rustfmt::skip]
名前解決組織! {
    generated = "generated/static_value_expr_name_resolves_at_call_site_名前解決チーム関数内.rs";
    graph 名前解決チーム関数内;
    node 太郎 = 社員 { 名前: 名前を作る() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

// instanceの後ろに書いた、instanceと同じmodule直下の関数。この関数の中で
// `構築!`を呼ぶと、値の式の`名前を作る()`はこの関数の中のローカル関数
// (同じ名前で外側を覆う) を指す。呼び出し位置がこの関数の中である以上、
// instance自身がこの関数の外にあっても結果は変わる。
fn 関数の中のローカルな同名関数へすり替わる() -> String {
    fn 名前を作る() -> String {
        "関数内の影".into()
    }
    let nodes = 名前解決チーム関数内::construct::nodes!();
    let edges = 名前解決チーム関数内::construct::edges!(&nodes);
    let g = 名前解決チーム関数内::Graph::new(&edges);
    g.node_refs().太郎().entity().名前.clone()
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 名前解決チーム子モジュール内 {
    include!("generated/static_value_expr_name_resolves_at_call_site_名前解決チーム子モジュール内.rs");
}

#[rustfmt::skip]
名前解決組織! {
    generated = "generated/static_value_expr_name_resolves_at_call_site_名前解決チーム子モジュール内.rs";
    graph 名前解決チーム子モジュール内;
    node 太郎 = 社員 { 名前: 名前を作る() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

// instanceの後ろに書いた、instanceと同じファイルのインラインの子module。
// `macro_rules!`のテキスト順スコープにより値マクロが見えるため、ここから
// `構築!`を呼べてしまう (`docs/static_graph.md`「制約」節「残る穴」)。
// この中の同名関数が値の式の`名前を作る()`をすり替える。
mod 子モジュール {
    use super::{名前解決チーム子モジュール内, 社員, 部署};

    fn 名前を作る() -> String {
        "子moduleの影".into()
    }

    pub(super) fn 子moduleのローカルな同名関数へすり替わる() -> String {
        let nodes = 名前解決チーム子モジュール内::construct::nodes!();
        let edges = 名前解決チーム子モジュール内::construct::edges!(&nodes);
        let g = 名前解決チーム子モジュール内::Graph::new(&edges);
        g.node_refs().太郎().entity().名前.clone()
    }
}

#[test]
fn トップレベルから呼ぶと外側の同名関数のままである() {
    assert_eq!(トップレベルでは外側の同名関数のままである(), "外側");
}

#[test]
fn 関数の中から呼ぶと関数内のローカルな同名関数へすり替わる() {
    assert_eq!(関数の中のローカルな同名関数へすり替わる(), "関数内の影");
}

#[test]
fn 子moduleから呼ぶと子module内のローカルな同名関数へすり替わる() {
    assert_eq!(子モジュール::子moduleのローカルな同名関数へすり替わる(), "子moduleの影");
}
