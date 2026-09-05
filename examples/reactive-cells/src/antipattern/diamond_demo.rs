//! (a) グリッチと (c) 登録順依存の非決定性を、ダイヤモンド依存
//! (`a→b, a→c, b→d, c→d`) の購読チェーンで再現するデモ。

use std::cell::RefCell;
use std::rc::Rc;

use super::naive_cell::NaiveCell;

// (a)(c) ダイヤモンド依存 `a→b, a→c, b→d, c→d` を observer パターンで
// 組んだデモ。`b = a * 2`・`c = a + 100`・`d = b + c` という
// `crate::fixtures::default_sheet` の `subtotal`/`discount_amount`/
// `tax`/`adjustment` と同じ形の依存構造。
//
// `d_log` は `d` が再計算されるたびの `(その時点のb, その時点のc, 新しいd)` である。
// グリッチが起きていれば1件目の `b`/`c` が矛盾した組み合わせになる
// (README「グリッチの実演」節参照)。
pub struct DiamondDemo {
    pub a: Rc<NaiveCell>,
    pub b: Rc<NaiveCell>,
    pub c: Rc<NaiveCell>,
    pub d: Rc<NaiveCell>,
    pub d_log: Rc<RefCell<Vec<(f64, f64, f64)>>>,
}

// `swap_registration_order` が `false` なら `a` への購読を「`b`の更新」→
// 「`c`の更新」の順で登録する (`true` なら逆順)。どちらでも最終値は同じ
// だが、`d_log` の1件目 (=グリッチの内容) が入れ替わる — これが
// (c) 更新順序が購読登録順に依存する、の実演。
pub fn build_diamond_demo(swap_registration_order: bool) -> DiamondDemo {
    let a = NaiveCell::new(0.0);
    let b = NaiveCell::new(0.0);
    let c = NaiveCell::new(0.0);
    let d = NaiveCell::new(0.0);
    let d_log: Rc<RefCell<Vec<(f64, f64, f64)>>> = Rc::new(RefCell::new(Vec::new()));

    // dの再計算本体を1つのRc<dyn Fn()>にまとめ、b/cの両方から同じものを
    // 呼び出す (bかcのどちらが変わってもdは同じ式で再計算される、という
    // 「1つの計算ロジックを複数の購読で駆動する」形が実務でも典型的)。
    let recompute_d: Rc<dyn Fn()> = {
        let b = b.clone();
        let c = c.clone();
        let d = d.clone();
        let d_log = d_log.clone();
        Rc::new(move || {
            let b_val = b.get();
            let c_val = c.get();
            d.set(b_val + c_val);
            d_log.borrow_mut().push((b_val, c_val, d.get()));
        })
    };

    let subscribe_b_update = {
        let b = b.clone();
        move |a_val: f64| b.set(a_val * 2.0)
    };
    let subscribe_c_update = {
        let c = c.clone();
        move |a_val: f64| c.set(a_val + 100.0)
    };
    let subscribe_d_from_b = {
        let recompute_d = recompute_d.clone();
        move |_b_val: f64| recompute_d()
    };
    let subscribe_d_from_c = {
        let recompute_d = recompute_d.clone();
        move |_c_val: f64| recompute_d()
    };

    b.subscribe(subscribe_d_from_b);
    c.subscribe(subscribe_d_from_c);

    if swap_registration_order {
        a.subscribe(subscribe_c_update);
        a.subscribe(subscribe_b_update);
    } else {
        a.subscribe(subscribe_b_update);
        a.subscribe(subscribe_c_update);
    }

    DiamondDemo { a, b, c, d, d_log }
}

impl DiamondDemo {
    // `a` に新しい値を設定し、購読チェーンによる同期的な伝播を1回
    // 走らせる。
    pub fn trigger(&self, a_value: f64) {
        self.a.set(a_value);
    }
}

#[cfg(test)]
mod tests;
