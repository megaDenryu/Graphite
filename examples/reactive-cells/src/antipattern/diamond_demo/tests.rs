//! グリッチ (1回目の観測が矛盾すること) と、登録順によってその内容が
//! 入れ替わることを固定する。

use super::build_diamond_demo;

#[test]
fn ダイヤモンド依存はdを2回再計算し1回目は矛盾した中間状態になる() {
    let demo = build_diamond_demo(false);
    demo.trigger(5.0);

    let log = demo.d_log.borrow();
    assert_eq!(
        log.len(),
        2,
        "dはbからの通知とcからの通知の両方で再計算される (2回)"
    );

    // 1回目: bは新しい値(10)だがcはまだ古い値(0)のまま (矛盾した中間状態)。
    let (b1, c1, d1) = log[0];
    assert_eq!(b1, 10.0); // a*2 = 5*2
    assert_eq!(c1, 0.0); // まだ更新されていない
    assert_eq!(d1, 10.0); // b+c = 10+0 (本来あるべき最終値115とは異なる)

    // 2回目: cが更新され、ようやく正しい最終値になる。
    let (b2, c2, d2) = log[1];
    assert_eq!(b2, 10.0);
    assert_eq!(c2, 105.0); // a+100 = 5+100
    assert_eq!(d2, 115.0);

    assert_eq!(
        demo.d.get(),
        115.0,
        "最終的には正しい値に収束する (グリッチは過程の問題)"
    );
}

#[test]
fn 購読登録順を入れ替えるとグリッチの内容が入れ替わる() {
    let normal = build_diamond_demo(false);
    normal.trigger(5.0);
    let swapped = build_diamond_demo(true);
    swapped.trigger(5.0);

    // 最終値はどちらも同じ (115) だが、1回目の観測 (=どちらが古い
    // ままか) は登録順に依存して入れ替わる。
    assert_eq!(normal.d.get(), swapped.d.get());

    let normal_first = normal.d_log.borrow()[0];
    let swapped_first = swapped.d_log.borrow()[0];
    assert_ne!(
        normal_first, swapped_first,
        "登録順が違えば1回目のグリッチ内容も変わるはず"
    );
    // 入れ替えた方はcが先に更新されbが古いままのグリッチになる。
    let (b1, c1, _d1) = swapped_first;
    assert_eq!(c1, 105.0);
    assert_eq!(b1, 0.0);
}
