//! グラフアルゴリズムによる FSM 設計検査 (README の「売り」)。
//!
//! `schema`+`graph!` で遷移表を書いただけでは、以下は誰も検査してくれない:
//!
//! - 「初期状態からどのイベント列を試しても絶対に辿り着けない状態」が
//!   紛れ込んでいないか (デッドコードのグラフ版)。
//! - 「終端でないのに、そこから先へ進む辺が1本も無い状態」(定義漏れ) が
//!   無いか。
//!
//! ここでは6種のイベントエッジ (`submit`/`pay`/`ship`/`deliver`/`cancel`/
//! `refund`) を全部束ねて汎用 `graphite::Graph<(), (), OrderStateId>` に
//! 射影し、[`Graph::reachable_from`]/[`Graph::out_neighbors`] という
//! ラベルを問わない汎用アルゴリズムだけで両方を機械的に検出する。

use std::collections::HashSet;

use graphite::Graph;

use crate::schema::{OrderFsm, OrderStateId};

/// 検査結果。両方が空なら「設計として健全」ということ ([`ValidationReport::is_ok`])。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidationReport {
    /// 初期状態からどのイベント列でも到達できない状態
    /// (呼ばれないデッドコードに相当)。
    pub unreachable: Vec<OrderStateId>,
    /// 終端状態のセットに含まれないのに、出て行く辺が1本も無い状態
    /// (定義漏れの疑いが強い)。
    pub dead_ends: Vec<OrderStateId>,
}

impl ValidationReport {
    pub fn is_ok(&self) -> bool {
        self.unreachable.is_empty() && self.dead_ends.is_empty()
    }
}

/// 6種のイベントエッジを全部束ねて、ラベルの区別を捨てた汎用グラフへ
/// 射影する。属性つきエッジ (`cancel`/`refund`) は `iter()` が3つ組
/// (始点, 終点, 属性) を返すが、ここでは到達可否の構造しか見ないので
/// 属性は捨てる。
fn project(fsm: &OrderFsm) -> Graph<(), (), OrderStateId> {
    let nodes: Vec<OrderStateId> = fsm.order_state_ids().cloned().collect();

    let mut edges: Vec<(OrderStateId, OrderStateId)> = Vec::new();
    edges.extend(fsm.submit().iter().map(|(from, to)| (from.clone(), to.clone())));
    edges.extend(fsm.pay().iter().map(|(from, to)| (from.clone(), to.clone())));
    edges.extend(fsm.ship().iter().map(|(from, to)| (from.clone(), to.clone())));
    edges.extend(fsm.deliver().iter().map(|(from, to)| (from.clone(), to.clone())));
    edges.extend(
        fsm.cancel()
            .iter()
            .map(|(from, to, _attrs)| (from.clone(), to.clone())),
    );
    edges.extend(
        fsm.refund()
            .iter()
            .map(|(from, to, _attrs)| (from.clone(), to.clone())),
    );

    Graph::from_edges(nodes, edges)
        .expect("OrderFsmのノードキー・6種のエッジの端点キーは常に整合しているはず")
}

/// `initial` を初期状態、`terminal` を終端状態集合として、到達不能状態と
/// 行き止まり状態を検出する。
pub fn validate(fsm: &OrderFsm, initial: &OrderStateId, terminal: &[OrderStateId]) -> ValidationReport {
    let graph = project(fsm);
    let terminal_set: HashSet<&OrderStateId> = terminal.iter().collect();

    // (a) 到達不能検出: reachable_from は `initial` 自身も含む反射的な
    // 到達可能性を返すので、グラフの全キーからその差集合を取れば
    // 「initial からは絶対に到達できない状態」が残る。
    let reachable: HashSet<&OrderStateId> = graph.reachable_from(initial).into_iter().collect();
    let mut unreachable: Vec<OrderStateId> = graph
        .keys()
        .filter(|key| !reachable.contains(key))
        .cloned()
        .collect();
    unreachable.sort();

    // (b) 行き止まり検出: 終端状態でないのに out_neighbors が空ならバグ。
    let mut dead_ends: Vec<OrderStateId> = graph
        .keys()
        .filter(|key| !terminal_set.contains(key) && graph.out_neighbors(key).is_empty())
        .cloned()
        .collect();
    dead_ends.sort();

    ValidationReport {
        unreachable,
        dead_ends,
    }
}
