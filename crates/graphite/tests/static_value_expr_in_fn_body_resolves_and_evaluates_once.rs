//! instanceを関数の中 (文の位置) に置いた場合も、値の式が宣言位置 (この
//! 関数のブロックのうち、instanceを書いたテキスト位置) の意味で解決される
//! こと、および`construct!`を呼ぶたびに (キャッシュされず) 1回だけ評価
//! されることを固定する回帰試験。モジュール直下 (項目の位置) で同じ性質を
//! 固定する試験は`static_value_expr_resolves_at_declaration_site.rs`・
//! `static_individual_value_evaluated_once_per_assembly.rs`が持つ。instance
//! を関数の中に置いても、値の式は依然として宣言位置に置いた捕捉しない`fn`
//! の本体として生成器が固定するため (`docs/static_graph.md`「値の式の
//! 名前解決」節)、コード生成の経路はモジュール直下と同じだが、実際に
//! 関数の中に置いた配置でも同じ性質が成り立つことを別に確かめる。
//! `non_local_definitions`が出ないことの固定は
//! `static_mod_outside_instance_inside_fn.rs`が別に持つ。

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 関数内解決組織 {
    include!("generated/static_value_expr_in_fn_body_resolves_and_evaluates_once_関数内解決組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_in_fn_body_resolves_and_evaluates_once_関数内解決組織.rs";
    schema 関数内解決組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
    }
}

#[allow(non_upper_case_globals)]
static 太郎の評価回数: AtomicUsize = AtomicUsize::new(0);

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 関数内解決チーム {
    include!("generated/static_value_expr_in_fn_body_resolves_and_evaluates_once_関数内解決チーム.rs");
}

// `太郎の値を求める` は、instanceと同じ関数の最上位ブロックに置いた入れ子の
// 項目である。呼び出し位置 (`construct!`を呼ぶ入れ子のブロック) に同名の
// 影の項目を置いても、値の式はこのブロック (宣言位置) の項目を指したまま
// 変わらない。
fn 太郎を関数の中で組み立てる() -> 関数内解決チーム::Graph {
    fn 太郎の値を求める() -> 社員 {
        太郎の評価回数.fetch_add(1, Ordering::SeqCst);
        社員 { 名前: "太郎".into() }
    }

    #[rustfmt::skip]
    関数内解決組織! {
        generated = "generated/static_value_expr_in_fn_body_resolves_and_evaluates_once_関数内解決チーム.rs";
        graph 関数内解決チーム;
        node 太郎: 社員 = 太郎の値を求める();
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }

    // 呼び出し位置を包む入れ子のブロックに同名の影の項目を置いても、値の式は
    // 宣言位置 (このブロックの最上位) の `太郎の値を求める` を指したまま
    // 変わらない (ブロックの外の同名項目を、内側の同名項目が隠すRustの
    // 通常のスコープ規則があるため、影の項目が実際に呼ばれないことがこの
    // 試験の主張そのものである)。
    {
        #[allow(dead_code)]
        fn 太郎の値を求める() -> 社員 {
            社員 { 名前: "呼び出し位置の影".into() }
        }
        関数内解決チーム::construct!()
    }
}

#[test]
fn 関数の中のinstanceは同じブロックの入れ子の項目を宣言位置の意味で解決し1回だけ評価する() {
    太郎の評価回数.store(0, Ordering::SeqCst);

    let g1回目 = 太郎を関数の中で組み立てる();
    assert_eq!(太郎の評価回数.load(Ordering::SeqCst), 1, "1回目の組み立てで1回評価される");
    assert_eq!(g1回目.node_refs().太郎().entity().名前, "太郎", "呼び出し位置の影ではなく宣言位置の項目が使われる");

    let _g2回目 = 太郎を関数の中で組み立てる();
    assert_eq!(
        太郎の評価回数.load(Ordering::SeqCst),
        2,
        "2回目の組み立てでも改めて1回評価される (キャッシュされて使い回されない)"
    );
}
