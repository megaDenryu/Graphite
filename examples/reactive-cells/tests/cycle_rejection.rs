//! 統合テスト: 循環が、Graphite 側では構築時に拒否され、observer パターン側
//! では自然に止まらないことを確認する。

use reactive_cells::antipattern::build_infinite_loop_demo;
use reactive_cells::engine::Engine;
use reactive_cells::fixtures::cyclic_demo_sheet;

#[test]
fn 循環する依存グラフはengine_newの時点でcycleerrorになる() {
    let sheet = cyclic_demo_sheet().expect("構造検証自体は循環でも通る");
    // `Engine`はDebugを実装しないため`expect_err`ではなくmatchで取り出す。
    let err = match Engine::new(sheet) {
        Err(err) => err,
        Ok(_) => panic!("循環があるのでEngine::newは失敗するはず"),
    };
    assert_eq!(err.cycle.len(), 3);
    // 循環パスが実際に閉路になっている (cycle[i] -> cycle[i+1] が
    // feedsエッジとして存在する) ことまでは、CycleError自体の保証
    // (`crates/graphite/src/graph/cycle_error.rs`のドキュメント参照) に委ねる。
}

#[test]
fn 循環購読の無限notifyは安全弁なしでは自然に止まらない() {
    let cap = 500;
    let count = build_infinite_loop_demo(cap);
    assert_eq!(
        count, cap,
        "capに到達する = 循環があれば自然には止まらないことの証拠"
    );
}
