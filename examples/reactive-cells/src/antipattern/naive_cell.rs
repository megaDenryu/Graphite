//! observer パターン (コールバック購読) で書いたナイーブなリアクティブセル。

use std::cell::RefCell;
use std::rc::Rc;

// 値を保持し、値が変わったら購読者へ登録順に同期的に通知するだけの
// 素朴なリアクティブセル。
//
// Graphite の `crate::engine::Engine` と対比してほしい違いは1点だけ:
// 「どのセルがどのセルに依存するか」という情報が、このセル自身の中にも
// 呼び出し側のどこにも、静的なデータとして存在しないこと。存在するのは
// `subscribers` という「後で呼ばれるクロージャの列」だけであり、
// 全体の依存構造は実行してみるまで分からない。
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

    // 値を更新し、購読者へ登録順に通知する。通知は同期的 — つまり
    // 1人目の購読者のコールバックが (別のセルを更新して) さらに孫の
    // 通知を引き起こす場合、その孫の通知は2人目の購読者が呼ばれる
    // 前に完了する。これが (a) グリッチの直接原因になる。
    pub fn set(&self, value: f64) {
        *self.value.borrow_mut() = value;
        self.notify();
    }

    // `f` を購読者として登録する。登録順序がそのまま通知順序になる —
    // これが (c) 非決定性の直接原因になる (「非決定」とは「登録順を
    // 見なければ予測できない」という意味。同じ登録順なら結果は再現する
    // が、依存構造から読み取れる情報ではなく、コードの書き方＝登録順
    // という無関係な要因に結果が左右される)。
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
