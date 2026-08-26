//! `Engine::new` が構築時に決めること — 循環の拒否、トポロジカル順序、
//! 初期値、`Formula` とエッジ本数の整合性検査。

use std::collections::HashSet;

use super::id;
use crate::engine::Engine;
use crate::fixtures::{cyclic_demo_sheet, default_sheet};
use crate::schema::{Cell, CellId, Formula, Sheet};

#[test]
fn engine_newは循環がなければ成功しトポロジカル順序を持つ() {
    let engine = Engine::new(default_sheet().unwrap()).expect("循環が無いので成功するはず");
    assert_eq!(engine.topological_order().len(), 10);
    // unit_priceはsubtotalより前に来るはず (依存元が依存先より前)。
    let order = engine.topological_order();
    let pos = |k: &str| order.iter().position(|c| c.0 == k).unwrap();
    assert!(pos("unit_price") < pos("subtotal"));
    assert!(pos("subtotal") < pos("discount_amount"));
    assert!(pos("discount_amount") < pos("adjustment"));
    assert!(pos("tax") < pos("adjustment"));
    assert!(pos("adjustment") < pos("grand_total"));
}

#[test]
fn engine_newは循環があるとcycleerrorで失敗する() {
    // `Engine`はDebugを実装しない (`Sheet`自体がgraph_schema!の生成物として
    // Debugを持たないため) ので、`expect_err`/`unwrap_err` (Ok型にDebugを
    // 要求する) ではなくmatchで直接取り出す。
    let err = match Engine::new(cyclic_demo_sheet().unwrap()) {
        Err(err) => err,
        Ok(_) => panic!("循環があるので失敗するはず"),
    };
    let members: HashSet<CellId> = err.cycle.iter().cloned().collect();
    assert_eq!(members, HashSet::from([id("a"), id("b"), id("c")]));
    assert_eq!(err.cycle.len(), 3);
}

#[test]
fn 初期値は全セル0である() {
    let engine = Engine::new(default_sheet().unwrap()).unwrap();
    assert_eq!(engine.value(&id("grand_total")), 0.0);
    assert_eq!(engine.value(&id("unit_price")), 0.0);
}

#[test]
#[should_panic(expected = "Lhsエッジがちょうど1本必要です")]
fn engine_newはsubセルにlhsエッジが無いとパニックする() {
    #[rustfmt::skip]
    let broken = graphite::graph!(Sheet {
        tax             = Cell { formula: Formula::Input },
        discount_amount = Cell { formula: Formula::Input },
        adjustment      = Cell { formula: Formula::Sub },

        r_discount_amount_adjustment = Rhs(discount_amount -> adjustment),
    })
    .expect("構造としては正常に構築できる (Lhs不足は検証対象外)")
    .into_graph();
    let _ = Engine::new(broken);
}

#[test]
#[should_panic(expected = "Feedsエッジが1本以上必要です")]
fn engine_newはmulセルにfeedsエッジが無いとパニックする() {
    #[rustfmt::skip]
    let broken = graphite::graph!(Sheet {
        lonely = Cell { formula: Formula::Mul },
    })
    .expect("構造としては正常に構築できる (Feeds不足は検証対象外)")
    .into_graph();
    let _ = Engine::new(broken);
}
