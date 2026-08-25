//! 確保契約 (ヒープ確保0回) の機械検証。
//!
//! `docs/desugaring_reference.md` の計算量・確保契約の表が主張する
//! 「参照の生成・役割クエリ・端点対検索・走査はヒープを確保しない」を、
//! 確保回数を数えるグローバル割り当て器で実測して固定する。時間を計る
//! ベンチマークは計測ぶれで偽陽性を出すため置かない。
//!
//! 統合テストは1ファイルが1つのcrate rootなので、`#[global_allocator]` の
//! 差し替えはこのテストバイナリの中だけに閉じ、他のテストへ影響しない。
//! 数える先をスレッドローカルにしているのは、`cargo test` が同じバイナリ内の
//! テストを並行実行するため、他スレッドの確保が測定区間へ混入するのを防ぐ
//! ためである。

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;

thread_local! {
    /// この呼び出しスレッドで発生したヒープ確保の累計回数。
    ///
    /// 注意: グローバル割り当て器は言語仕様上グローバルな1点でしか差し替え
    /// られないため、この累計を読み書きするのは下記の割り当て器と
    /// [`確保回数の計測`] の2つだけに閉じる。`Cell<usize>` は破棄処理を
    /// 持たないため、`const` 初期化されたスレッドローカルへの参照自体が
    /// ヒープを確保することはない。
    static ヒープ確保回数: Cell<usize> = const { Cell::new(0) };
}

/// 確保回数を数えてからシステム割り当て器へ委譲する割り当て器。
struct 確保回数を数える割り当て器;

impl 確保回数を数える割り当て器 {
    /// スレッドの終了処理中は累計へ到達できないため、その場合は数えない。
    fn 確保を1回数える(&self) {
        let _ = ヒープ確保回数.try_with(|累計| 累計.set(累計.get() + 1));
    }
}

unsafe impl GlobalAlloc for 確保回数を数える割り当て器 {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.確保を1回数える();
        System.alloc(layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.確保を1回数える();
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(
        &self, 領域: *mut u8, layout: Layout, 新しい大きさ: usize
    ) -> *mut u8 {
        self.確保を1回数える();
        System.realloc(領域, layout, 新しい大きさ)
    }

    unsafe fn dealloc(&self, 領域: *mut u8, layout: Layout) {
        System.dealloc(領域, layout)
    }
}

#[global_allocator]
static 数える割り当て器: 確保回数を数える割り当て器 = 確保回数を数える割り当て器;

/// ヒープ確保回数の計測区間。`開始する` が区間の始点を控え、
/// `区間内の確保回数` が増分を確定して区間を閉じる。
#[must_use]
struct 確保回数の計測 {
    開始時点の累計: usize,
}

impl 確保回数の計測 {
    fn 開始する() -> Self {
        Self {
            開始時点の累計: Self::現在の累計確保回数(),
        }
    }

    fn 区間内の確保回数(self) -> usize {
        Self::現在の累計確保回数() - self.開始時点の累計
    }

    fn 現在の累計確保回数() -> usize {
        ヒープ確保回数.try_with(Cell::get).unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct 人物 {
    pub 名前: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 商品 {
    pub 名前: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 取引情報 {
    pub 金額: u64,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod 確保契約 {
    include!("generated/allocation_contract_確保契約.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/allocation_contract_確保契約.rs";
    schema 確保契約 {
        node 人物;
        node 商品;

        edge 購入 = (購入者: 人物) -[取引: 取引情報]-> (対象商品: 商品) where unique pair;
        edge 閲覧 = (閲覧者: 人物) -> (対象商品: 商品);
        edge 推薦 = (推薦者: 人物) -> (対象商品: 商品) where each 推薦者: 0..1;
        edge 常用 = (常用者: 人物) -> (対象商品: 商品) where each 常用者: 1;
        edge 友人 = 人物 -- 人物 where unique pair;
    }
}

use 確保契約::{人物Id, 商品Id, 購入Id};

#[rustfmt::skip]
fn 測定用グラフを構築する() -> 確保契約::Graph {
    graphite::graph!(確保契約 {
        太郎 = 人物 { 名前: "太郎".into() },
        花子 = 人物 { 名前: "花子".into() },
        次郎 = 人物 { 名前: "次郎".into() },
        本 = 商品 { 名前: "本".into() },
        筆 = 商品 { 名前: "筆".into() },

        太郎の本購入 = 購入(太郎 -[取引情報 { 金額: 1200 }]-> 本),
        太郎の筆購入 = 購入(太郎 -[取引情報 { 金額: 300 }]-> 筆),
        花子の本購入 = 購入(花子 -[取引情報 { 金額: 900 }]-> 本),

        太郎の本閲覧1 = 閲覧(太郎 -> 本),
        太郎の本閲覧2 = 閲覧(太郎 -> 本),

        太郎の推薦 = 推薦(太郎 -> 本),

        太郎の常用 = 常用(太郎 -> 本),
        花子の常用 = 常用(花子 -> 筆),
        次郎の常用 = 常用(次郎 -> 本),

        太郎と花子 = 友人(太郎 -- 花子),
        太郎と次郎 = 友人(太郎 -- 次郎),
    })
    .expect("確保契約の測定用グラフは制約を満たすはず")
    .into_graph()
}

fn 名前から人物参照を得る<'graph>(
    graph: &'graph 確保契約::Graph,
    名前: &str,
) -> 確保契約::人物Ref<'graph> {
    graph
        .人物_by_id(&人物Id(名前.to_string()))
        .expect("測定用グラフに存在する人物")
}

fn 名前から商品参照を得る<'graph>(
    graph: &'graph 確保契約::Graph,
    名前: &str,
) -> 確保契約::商品Ref<'graph> {
    graph
        .商品_by_id(&商品Id(名前.to_string()))
        .expect("測定用グラフに存在する商品")
}

#[test]
fn 計測器は実際の確保を検出できる() {
    let 計測区間 = 確保回数の計測::開始する();
    let 確保した領域 = black_box(vec![0u8; 64]);
    let 確保回数 = 計測区間.区間内の確保回数();
    drop(確保した領域);

    assert!(
        確保回数 >= 1,
        "計測器がヒープ確保を1回も検出できていない (確保0回の測定結果が意味を持たなくなる)"
    );
}

#[test]
fn 参照の生成はヒープを確保しない() {
    let graph = 測定用グラフを構築する();
    let 人物の公開id = 人物Id("太郎".to_string());
    let 辺の公開id = 購入Id("太郎の本購入".to_string());

    let 計測区間 = 確保回数の計測::開始する();
    let 人物参照 = graph.人物_by_id(&人物の公開id).expect("太郎は存在する");
    let 辺参照 = graph
        .購入_by_id(&辺の公開id)
        .expect("太郎の本購入は存在する");
    black_box((
        人物参照.id(),
        人物参照.value(),
        人物参照.名前.len(),
        辺参照.id(),
        辺参照.購入者(),
        辺参照.対象商品(),
        辺参照.from_id(),
        辺参照.to_id(),
        辺参照.payload(),
        辺参照.取引(),
    ));
    let 確保回数 = 計測区間.区間内の確保回数();

    assert_eq!(確保回数, 0, "参照の生成と端点・積み荷の読み出し");
}

#[test]
fn 静的アクセサはヒープを確保しない() {
    #[rustfmt::skip]
    let graph = graphite::graph!(確保契約 {
        太郎 = 人物 { 名前: "太郎".into() },
        本 = 商品 { 名前: "本".into() },
        太郎の常用 = 常用(太郎 -> 本),
    })
    .expect("最小の測定用グラフは制約を満たすはず");

    let 計測区間 = 確保回数の計測::開始する();
    black_box((graph.太郎().id(), graph.本().id(), graph.太郎の常用().id()));
    let 確保回数 = 計測区間.区間内の確保回数();

    assert_eq!(確保回数, 0, "名前付きラッパーの静的アクセサ");
}

#[test]
fn 役割クエリの開始と走査はヒープを確保しない() {
    let graph = 測定用グラフを構築する();
    let 太郎 = 名前から人物参照を得る(&graph, "太郎");
    let 本 = 名前から商品参照を得る(&graph, "本");

    let 計測区間 = 確保回数の計測::開始する();
    let mut 合計金額 = 0u64;
    for 辺 in 太郎.購入_as_購入者() {
        合計金額 += 辺.payload().金額;
    }
    let mut 閲覧本数 = 0usize;
    for 辺 in 本.閲覧_as_対象商品() {
        閲覧本数 += 辺.id().0.len();
    }
    let 高々1本の推薦 = 太郎.推薦_as_推薦者();
    let ちょうど1本の常用 = 太郎.常用_as_常用者();
    let mut 接続本数 = 0usize;
    for 辺 in 太郎.友人_incident() {
        let (第1端点, 第2端点) = 辺.endpoints();
        接続本数 += 第1端点.名前.len() + 第2端点.名前.len();
    }
    black_box((
        合計金額,
        閲覧本数,
        高々1本の推薦,
        ちょうど1本の常用,
        接続本数,
    ));
    let 確保回数 = 計測区間.区間内の確保回数();

    assert_eq!(確保回数, 0, "役割クエリと無向辺の接続クエリ");
}

#[test]
fn 端点対検索はヒープを確保しない() {
    let graph = 測定用グラフを構築する();
    let 太郎 = 名前から人物参照を得る(&graph, "太郎");
    let 花子 = 名前から人物参照を得る(&graph, "花子");
    let 本 = 名前から商品参照を得る(&graph, "本");

    let 計測区間 = 確保回数の計測::開始する();
    let 購入の一意な対 = 太郎.購入_between(本);
    let 購入の一意な対の非パニック版 = 太郎.購入_try_between(本).expect("同じGraphの参照");
    let mut 平行辺の本数 = 0usize;
    for 辺 in 太郎.閲覧_between(本) {
        平行辺の本数 += 辺.id().0.len();
    }
    let mut 非パニック版の本数 = 0usize;
    for 辺 in 太郎.閲覧_try_between(本).expect("同じGraphの参照") {
        非パニック版の本数 += 辺.id().0.len();
    }
    let 友人の順序なし対 = 太郎.友人_between(花子);
    black_box((
        購入の一意な対,
        購入の一意な対の非パニック版,
        平行辺の本数,
        非パニック版の本数,
        友人の順序なし対,
    ));
    let 確保回数 = 計測区間.区間内の確保回数();

    assert_eq!(確保回数, 0, "有向・無向の端点対検索とその非パニック版");
}

#[test]
fn 種別apiの走査はヒープを確保しない() {
    let graph = 測定用グラフを構築する();

    let 計測区間 = 確保回数の計測::開始する();
    let mut 文字数の合計 = 0usize;
    let mut 金額の合計 = 0u64;
    for 人 in graph.人物_iter() {
        文字数の合計 += 人.名前.len();
    }
    for 辺 in graph.購入_iter() {
        金額の合計 += 辺.payload().金額;
    }
    for 公開id in graph.人物_ids() {
        文字数の合計 += 公開id.0.len();
    }
    for 公開id in graph.購入_ids() {
        文字数の合計 += 公開id.0.len();
    }
    let 件数の合計 = graph.人物_len() + graph.購入_len() + graph.友人_len();
    black_box((文字数の合計, 金額の合計, 件数の合計));
    let 確保回数 = 計測区間.区間内の確保回数();

    assert_eq!(確保回数, 0, "種別APIの走査と件数取得");
}
