//! 個体の値の式が、`construct::nodes!` を呼ぶたびに (キャッシュされず) 1回
//! だけ評価されることを固定する回帰試験。値マクロ
//! (`crates/graphite-codegen/src/static_graph/inline/value_supply.rs`) は
//! 呼ぶたびに毎回展開されるマクロ呼び出しであることに変わりはなく、値が
//! どこかにキャッシュされて使い回されることはない。同名個体を持つ複数
//! instanceの衝突回避の検証は `static_same_individual_name_multiple_instances.rs`
//! が別に持つ。

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証評価回数組織 {
    include!("generated/static_individual_value_evaluated_once_per_assembly_検証評価回数組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_individual_value_evaluated_once_per_assembly_検証評価回数組織.rs";
    schema 検証評価回数組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署);
    }
}

#[allow(non_upper_case_globals)]
static 太郎の評価回数: AtomicUsize = AtomicUsize::new(0);

fn 太郎の値を求める() -> 社員 {
    太郎の評価回数.fetch_add(1, Ordering::SeqCst);
    社員 { 名前: "太郎".into() }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 検証評価回数チーム {
    include!("generated/static_individual_value_evaluated_once_per_assembly_検証評価回数チーム.rs");
}

検証評価回数組織! {
    generated = "generated/static_individual_value_evaluated_once_per_assembly_検証評価回数チーム.rs";
    graph 検証評価回数チーム;
    node 太郎: 社員 = 太郎の値を求める();
    node 開発部 = 部署 { 名前: "開発部".into() };
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}

#[test]
fn 個体の値の式はnodesを組み立てるたびに1回だけ評価される() {
    太郎の評価回数.store(0, Ordering::SeqCst);

    let 組み立て結果1回目 = 検証評価回数チーム::construct::nodes!();
    assert_eq!(太郎の評価回数.load(Ordering::SeqCst), 1, "1回目の組み立てで1回評価される");
    let edges1回目 = 検証評価回数チーム::construct::edges!(&組み立て結果1回目);
    let g1回目 = 検証評価回数チーム::Graph::new(&edges1回目);
    assert_eq!(g1回目.node_refs().太郎().entity().名前, "太郎");

    let _組み立て結果2回目 = 検証評価回数チーム::construct::nodes!();
    assert_eq!(
        太郎の評価回数.load(Ordering::SeqCst),
        2,
        "2回目の組み立てでも改めて1回評価される (キャッシュされて使い回されない)"
    );
}
