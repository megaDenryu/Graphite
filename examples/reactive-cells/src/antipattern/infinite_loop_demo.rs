//! (b) 無限ループを、循環購読 (`x→y→x→..`) で再現するデモ。

use std::cell::RefCell;
use std::rc::Rc;

use super::naive_cell::NaiveCell;

/// (b) 無限ループ。`x`/`y` が互いに「相手が変わったら自分を更新する」を
/// 購読し合う (誰も循環に気づかない、という状況を再現するため循環検出
/// ロジックは一切書かない)。実際に無限に回すとスタックオーバーフロー
/// するため、`cap` 回で強制停止する安全弁だけを入れてある。
///
/// 戻り値は実際に実行された通知の回数。`cap` にちょうど達していれば
/// 「安全弁が無ければ止まらなかった」ことの証拠になる
/// (README「無限ループの実演」節、`tests/cycle_rejection.rs` 参照)。
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
    fn 循環購読は安全弁が無ければ止まらないことをcap到達で示す() {
        let count = build_infinite_loop_demo(200);
        assert_eq!(
            count, 200,
            "安全弁のcapにちょうど到達する = 自然には止まらないことの証拠"
        );
    }
}
