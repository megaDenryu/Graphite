//! 敵: observer パターン (コールバック購読) で書いたナイーブなリアクティブ
//! セル。
//!
//! [`NaiveCell`] は「値が変わったら購読者へ即座に通知する」だけの、
//! よくある実装 (Excel の再計算・フロントエンドの signal ライブラリの
//! 초期実装・自作の「観察可能な値」でしばしば見る形)。依存関係は
//! 「誰が誰を `subscribe` したか」という**実行時のコールバック登録**
//! としてしか存在せず、静的な全体像 (どのセルがどのセルに依存するか)
//! はどこにも書かれていない。
//!
//! このモジュールは README「敵の紹介」節で説明する3つの問題を実際に
//! 動くコードで再現する:
//!
//! - [`build_diamond_demo`] — (a) グリッチ。ダイヤモンド依存
//!   (`a→b, a→c, b→d, c→d`) で `d` が2回再計算され、1回目は
//!   矛盾した中間状態を観測する。
//! - [`build_infinite_loop_demo`] — (b) 無限ループ。循環購読
//!   (`x→y→x→..`) に誰も気づかず notify が回り続ける (実際に無限に
//!   回すとスタックオーバーフローするため、デモでは `cap` で強制停止
//!   させ「本来なら止まらない」ことを回数で示す)。
//! - [`build_diamond_demo`] (引数 `swap_registration_order`) — (c) 更新
//!   順序が購読登録順に依存して非決定になる。

use std::cell::RefCell;
use std::rc::Rc;

/// 値を保持し、値が変わったら購読者へ**登録順に**同期的に通知するだけの
/// 素朴なリアクティブセル。
///
/// Graphite の [`crate::engine::Engine`] と対比してほしい違いは1点だけ:
/// 「どのセルがどのセルに依存するか」という情報が、このセル自身の中にも
/// 呼び出し側のどこにも、静的なデータとして存在しないこと。存在するのは
/// `subscribers` という「後で呼ばれるクロージャの列」だけであり、
/// 全体の依存構造は実行してみるまで分からない。
pub struct NaiveCell {
    value: RefCell<f64>,
    subscribers: RefCell<Vec<Rc<dyn Fn(f64)>>>,
}

impl NaiveCell {
    pub fn new(initial: f64) -> Rc<Self> {
        Rc::new(Self {
            value: RefCell::new(initial),
            subscribers: RefCell::new(Vec::new()),
        })
    }

    pub fn get(&self) -> f64 {
        *self.value.borrow()
    }

    /// 値を更新し、購読者へ**登録順に**通知する。通知は同期的 — つまり
    /// 1人目の購読者のコールバックが (別のセルを更新して) さらに孫の
    /// 通知を引き起こす場合、その孫の通知は2人目の購読者が呼ばれる
    /// **前に**完了する。これが (a) グリッチの直接原因になる。
    pub fn set(&self, value: f64) {
        *self.value.borrow_mut() = value;
        self.notify();
    }

    /// `f` を購読者として登録する。登録順序がそのまま通知順序になる —
    /// これが (c) 非決定性の直接原因になる (「非決定」とは「登録順を
    /// 見なければ予測できない」という意味。同じ登録順なら結果は再現する
    /// が、依存構造から読み取れる情報ではなく、コードの書き方＝登録順
    /// という無関係な要因に結果が左右される)。
    pub fn subscribe(&self, f: impl Fn(f64) + 'static) {
        self.subscribers.borrow_mut().push(Rc::new(f));
    }

    fn notify(&self) {
        let value = self.get();
        // 先に `Vec<Rc<dyn Fn(f64)>>` へ複製 (`Rc`の参照カウントを
        // 増やすだけで中身のクロージャ自体はコピーしない、安価な操作)
        // してからループするのは、コールバック内で新たな `subscribe` が
        // 呼ばれるケース (このデモでは使わないが素朴な実装では起こり
        // うる) でも `RefCell` の二重借用パニックを避けるため。
        let subscribers: Vec<Rc<dyn Fn(f64)>> = self.subscribers.borrow().iter().cloned().collect();
        for sub in subscribers {
            sub(value);
        }
    }
}

/// (a)(c) ダイヤモンド依存 `a→b, a→c, b→d, c→d` を observer パターンで
/// 組んだデモ。`b = a * 2`・`c = a + 100`・`d = b + c` という
/// `crate::fixtures::default_sheet` の `subtotal`/`discount_amount`/
/// `tax`/`adjustment` と同じ形の依存構造。
pub struct DiamondDemo {
    pub a: Rc<NaiveCell>,
    pub b: Rc<NaiveCell>,
    pub c: Rc<NaiveCell>,
    pub d: Rc<NaiveCell>,
    /// `d` が再計算されるたびの `(その時点のb, その時点のc, 新しいd)`。
    /// グリッチが起きていれば1件目の `b`/`c` が矛盾した組み合わせになる
    /// (README「グリッチの実演」節参照)。
    pub d_log: Rc<RefCell<Vec<(f64, f64, f64)>>>,
}

/// `swap_registration_order` が `false` なら `a` への購読を「`b`の更新」→
/// 「`c`の更新」の順で登録する (`true` なら逆順)。どちらでも最終値は同じ
/// だが、`d_log` の1件目 (=グリッチの内容) が入れ替わる — これが
/// (c) 更新順序が購読登録順に依存する、の実演。
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
    /// `a` に新しい値を設定し、購読チェーンによる同期的な伝播を1回
    /// 走らせる。
    pub fn trigger(&self, a_value: f64) {
        self.a.set(a_value);
    }
}

/// (b) 無限ループ。`x`/`y` が互いに「相手が変わったら自分を更新する」を
/// 購読し合う (誰も循環に気づかない、という状況を再現するため循環検出
/// ロジックは一切書かない)。実際に無限に回すとスタックオーバーフロー
/// するため、`cap` 回で強制停止する安全弁だけを入れてある。
///
/// 戻り値は実際に実行された通知の回数。`cap` にちょうど達していれば
/// 「安全弁が無ければ止まらなかった」ことの証拠になる
/// (README「無限ループの実演」節、`tests/integration.rs` 参照)。
pub fn build_infinite_loop_demo(cap: usize) -> usize {
    let x = NaiveCell::new(1.0);
    let y = NaiveCell::new(1.0);
    let notify_count = Rc::new(RefCell::new(0usize));

    {
        let y = y.clone();
        let notify_count = notify_count.clone();
        x.subscribe(move |x_val| {
            let mut count = notify_count.borrow_mut();
            if *count >= cap {
                return; // 安全弁 (本来のnotifyパターンにはこれが無い)。
            }
            *count += 1;
            drop(count);
            y.set(x_val + 1.0);
        });
    }
    {
        let x = x.clone();
        let notify_count = notify_count.clone();
        y.subscribe(move |y_val| {
            let mut count = notify_count.borrow_mut();
            if *count >= cap {
                return;
            }
            *count += 1;
            drop(count);
            x.set(y_val + 1.0);
        });
    }

    x.set(2.0); // 循環購読の連鎖を起動する。

    let final_count = *notify_count.borrow();
    final_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ダイヤモンド依存はdを2回再計算し1回目は矛盾した中間状態になる() {
        let demo = build_diamond_demo(false);
        demo.trigger(5.0);

        let log = demo.d_log.borrow();
        assert_eq!(log.len(), 2, "dはbからの通知とcからの通知の両方で再計算される (2回)");

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

        assert_eq!(demo.d.get(), 115.0, "最終的には正しい値に収束する (グリッチは過程の問題)");
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

    #[test]
    fn 循環購読は安全弁が無ければ止まらないことをcap到達で示す() {
        let count = build_infinite_loop_demo(200);
        assert_eq!(count, 200, "安全弁のcapにちょうど到達する = 自然には止まらないことの証拠");
    }
}
