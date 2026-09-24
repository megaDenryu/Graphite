//! 値の式は生成ファイルへ複製されないことを固定する回帰試験
//! (`docs/static_graph.md`「値の式の名前解決」節、オーナーの裁定が挙げる
//! 9条件の1つ)。値の式そのものは、instance展開がinstanceの宣言位置へ
//! 直接埋め込む値マクロ (`__graphite_values_*`) の中にしか現れず、
//! `cargo xtask generate`が書く生成ファイル (`construct!`の本体等) には
//! 一切写らない。値の式の中に、生成ファイルの他の部分とは絶対に混同
//! しない目印の文字列を書き、生成ファイルの実際のテキストにその目印が
//! 無いことを確かめる。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 複製検査組織 {
    include!("generated/static_value_expr_not_duplicated_into_generated_file_複製検査組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_not_duplicated_into_generated_file_複製検査組織.rs";
    schema 複製検査組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 複製検査チーム {
    include!("generated/static_value_expr_not_duplicated_into_generated_file_複製検査チーム.rs");
}

複製検査組織! {
    generated = "generated/static_value_expr_not_duplicated_into_generated_file_複製検査チーム.rs";
    graph 複製検査チーム;
    node 太郎 = 社員 { 名前: "この文字列__値式限定目印__は生成ファイルへ複製されてはならない".to_string() };
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

#[test]
fn 値の式の目印は生成ファイルに現れない() {
    let 生成ファイル本文 = include_str!("generated/static_value_expr_not_duplicated_into_generated_file_複製検査チーム.rs");
    assert!(
        !生成ファイル本文.contains("値式限定目印"),
        "生成ファイルに値の式の目印が写っている (値の式を複製している):\n{生成ファイル本文}"
    );
}

#[test]
fn 構築すると値の式が実際に評価される() {
    let g = 複製検査チーム::construct!();
    assert_eq!(g.node_refs().太郎().entity().名前, "この文字列__値式限定目印__は生成ファイルへ複製されてはならない");
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 複製検査関数内チーム {
    include!("generated/static_value_expr_not_duplicated_into_generated_file_複製検査関数内チーム.rs");
}

// `in fn`(関数の中の位置) のinstanceも同じ性質を持つことを確かめる。
// 項目の位置 (上の`複製検査チーム`) はクロージャを使わない`fn`本体だが、
// `in fn`は宣言位置で`let`束縛したクロージャを使う (`docs/static_graph.md`
// 「値の式の名前解決」節) ため、値の式が生成ファイルへ写らないという性質を
// 別に確かめる必要がある。
fn 複製検査関数内チームを構築する() -> 複製検査関数内チーム::Graph {
    複製検査組織! {
        generated = "generated/static_value_expr_not_duplicated_into_generated_file_複製検査関数内チーム.rs";
        graph 複製検査関数内チーム in fn;
        node 次郎 = 社員 { 名前: "この文字列__関数内目印__は生成ファイルへ複製されてはならない".to_string() };
        node 総務部 = 部署 { 名前: "総務部".into() };
        edge 次郎の所属 = 所属(次郎 -> 総務部);
    }
    複製検査関数内チーム::construct!()
}

#[test]
fn 関数内位置の値の式の目印は生成ファイルに現れない() {
    let 生成ファイル本文 = include_str!("generated/static_value_expr_not_duplicated_into_generated_file_複製検査関数内チーム.rs");
    assert!(
        !生成ファイル本文.contains("関数内目印"),
        "生成ファイルにin fnの値の式の目印が写っている (値の式を複製している):\n{生成ファイル本文}"
    );
}

#[test]
fn 関数内で構築すると値の式が実際に評価される() {
    let g = 複製検査関数内チームを構築する();
    assert_eq!(g.node_refs().次郎().entity().名前, "この文字列__関数内目印__は生成ファイルへ複製されてはならない");
}
