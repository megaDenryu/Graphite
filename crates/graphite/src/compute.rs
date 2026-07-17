//! `ComputeGraph<V>` — 遅延実行・差分再計算する計算グラフ (`docs/compute_graph.md`)。
//!
//! `flow!` (`docs/flow_macro.md`) が「書いた瞬間に実行される即時実行の脱糖」
//! であるのに対し、`ComputeGraph` は計算グラフを**実行時の値として持ち**、
//! pull 型で遅延評価・差分再計算するランタイムエンジンである
//! (`examples/reactive-cells` の `Engine` の一般化 — あちらは `f64` +
//! `Formula` enum に特化した example、こちらは汎用ライブラリ)。
//!
//! ```
//! use graphite::ComputeGraph;
//!
//! let mut b = ComputeGraph::builder();
//! b.input("price", 100.0);
//! b.input("qty", 3.0);
//! b.computed("subtotal", ["price", "qty"], |args| args[0] * args[1]);
//! b.computed("tax", ["subtotal"], |args| args[0] * 0.1);
//! b.computed("total", ["subtotal", "tax"], |args| args[0] + args[1]);
//! let mut g = b.freeze()?; // 循環は ComputeGraphError::Cycle (循環パスつき) で拒否
//!
//! assert_eq!(*g.get("total"), 330.0); // 遅延: ここで初めて必要分だけ計算
//! g.set_input("qty", 5.0); // 差分: 影響ノードだけ dirty に
//! assert_eq!(*g.get("total"), 550.0); // 再計算は影響分をトポロジカル順に各1回
//! # Ok::<(), graphite::ComputeGraphError>(())
//! ```
//!
//! ## 設計決定
//!
//! - **値型は単一のジェネリック `V`** (`ComputeGraph<V>`)。異種の値はユーザーが
//!   enum で表現する想定であり、`ComputeGraph` 自体は実行時リフレクションを
//!   持ち込まない (`reactive-cells` の `Engine` と同じ整理)。
//! - **関数は `Box<dyn Fn(&[&V]) -> V>`。** `docs/design_principles.md` 原則5
//!   (ゼロコスト志向) は「マクロが生成するコードは手書きコードと同形でなければ
//!   ならない」という**マクロ生成コードに対する**規律である。`ComputeGraph` は
//!   マクロではなく手書きのランタイムエンジンであり、ユーザーが任意個・任意型の
//!   計算ノードを実行時に組み立てられる以上、動的ディスパッチ (`dyn Fn`) は
//!   回避不可能かつ Rust の正道 (手書きでも同じ設計になる — `Vec<Box<dyn
//!   FnMut()>>` 等と同じ立ち位置)。原則5 が禁じるのは「マクロが生成する
//!   コードに手書きでは書かないリフレクション/動的ディスパッチを混入させる
//!   こと」であり、ランタイムエンジンそのものの設計判断とは別の階層にある。
//! - **依存は位置引数** (`args[0]` = 依存キー列の 0 番目)。非可換な演算
//!   (減算等) の左右は依存リストの並び順で表現する — `flow!` の fan-in
//!   タプルと同じ規則。`docs/modeling_guide.md` §5 の「役割は名前で」は
//!   **グラフデータ**の規律 (`reactive-cells` の `Lhs`/`Rhs` エッジ種別分離
//!   参照) であり、関数適用の引数そのものは Rust の関数呼び出しと同じ
//!   位置渡しが正道 — グラフ構造の設計判断とクロージャ引数の設計判断は
//!   別の関心事である。
//! - **キーは名前 (`String`)。**
//! - **pull 型の遅延 + 差分。** [`ComputeGraph::set_input`] は値の書き込みと
//!   dirty 伝播 (`reachable_from` 相当) のみを行い、実際の再計算は一切行わない。
//!   [`ComputeGraph::get`] が「dirty な祖先だけ」をトポロジカル順に各 1 回
//!   再計算する (glitch-free)。トポロジカル順序は [`ComputeGraphBuilder::freeze`]
//!   で 1 回だけ計算しキャッシュする (依存構造は構築後不変なので、更新の
//!   たびに再計算する必要はない)。
//!
//! ## 既存機構の再利用 vs 内製
//!
//! - **再利用: 循環検出・キー重複・未宣言依存の検証、トポロジカル順序の
//!   計算。** [`ComputeGraphBuilder::freeze`] は依存キー列と `(依存元, 依存先)`
//!   の辺列を [`crate::Graph::from_edges`] に渡すだけで、キー重複
//!   ([`crate::GraphError::DuplicateKey`])・未宣言依存への辺
//!   ([`crate::GraphError::UnknownEndpoint`]) の検証がそのまま手に入る。
//!   続けて [`crate::Graph::topological_sort`] を呼べば、循環検出
//!   ([`crate::CycleError`]、循環パスつき) とトポロジカル順序の計算を
//!   1 回で済ませられる。これは `reactive-cells` の `Engine::new` が
//!   `graphite::Graph` へ依存グラフを射影して同じ2操作に委譲しているのと
//!   全く同じパターンであり、車輪の再発明を避けられる。
//! - **内製: 依存元の値の保持・位置引数への変換・dirty 集合の管理。**
//!   `crate::Graph<N, E, K>` はノード値 `N` を1種類の型に固定する設計
//!   (`docs/graph_design_sketches.md` 決定1/決定2 をそのまま輸入したもの)
//!   であり、「入力ノードは値のみ・計算ノードは依存キー列+関数を持つ」という
//!   異種混合のノードは表現できない。加えて評価時に依存値を**宣言順の
//!   位置引数**として渡す必要があり、`Graph` の `in_neighbors`/`out_neighbors`
//!   は順序を保証しない (内部実装が `petgraph` の近傍イテレータに委譲して
//!   いるため)。そのため各計算ノードの依存キー列は `ComputeGraph` 側に
//!   `Vec<String>` として直接持たせ、dirty 集合の管理・評価も `ComputeGraph`
//!   が内製する。「再利用できる部分は再利用し、既存の型に無理に押し込むと
//!   歪みが生じる部分は内製する」という判断そのもの。

use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::fmt;

use crate::{CycleError, Graph, GraphError};

/// ノードの種別。入力ノードは値のみ、計算ノードは依存キー列と関数を持つ。
enum NodeKind<V> {
    /// 入力ノード。値は [`ComputeGraph::set_input`] で直接書き込む。
    Input,
    /// 計算ノード。`deps` は宣言順の依存キー列 (位置引数の並びそのもの)、
    /// `f` はその値列から自分の値を求める関数。
    Computed {
        deps: Vec<String>,
        f: Box<dyn Fn(&[&V]) -> V>,
    },
}

/// [`ComputeGraph::builder`] が返す構築用 builder。
///
/// `input`/`computed` でノードを積み、[`Self::freeze`] で凍結する
/// (`docs/graph_design_sketches.md` 決定2 — クロージャスコープではなく
/// 値としての builder → freeze だが、「構築中の型」と「構築後の型」を
/// 分けるという要点は同じ)。
pub struct ComputeGraphBuilder<V> {
    entries: Vec<(String, NodeKind<V>)>,
    input_values: HashMap<String, V>,
}

impl<V> ComputeGraphBuilder<V> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            input_values: HashMap::new(),
        }
    }

    /// 入力ノードを1つ積む。`key` が重複した場合のエラーは [`Self::freeze`]
    /// まで遅延する ([`ComputeGraphError::Graph`] の
    /// [`GraphError::DuplicateKey`])。
    pub fn input(&mut self, key: impl Into<String>, value: V) -> &mut Self {
        let key = key.into();
        self.input_values.insert(key.clone(), value);
        self.entries.push((key, NodeKind::Input));
        self
    }

    /// 計算ノードを1つ積む。`deps` は評価時に `f` へ渡される位置引数の並び
    /// そのもの (`args[0]` = `deps` の0番目)。`deps` が参照するキーが未宣言
    /// だった場合のエラーは [`Self::freeze`] まで遅延する
    /// ([`ComputeGraphError::Graph`] の [`GraphError::UnknownEndpoint`])。
    pub fn computed<D, S>(&mut self, key: impl Into<String>, deps: D, f: impl Fn(&[&V]) -> V + 'static) -> &mut Self
    where
        D: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let deps: Vec<String> = deps.into_iter().map(Into::into).collect();
        self.entries.push((
            key.into(),
            NodeKind::Computed {
                deps,
                f: Box::new(f),
            },
        ));
        self
    }

    /// 凍結して [`ComputeGraph`] を作る。
    ///
    /// 検証順序: まずキー重複・未宣言依存 ([`crate::Graph::from_edges`] へ
    /// 委譲)、次に循環 ([`crate::Graph::topological_sort`] へ委譲、パスつき
    /// `CycleError`)。凍結後は全ての計算ノードを dirty (未計算) 状態で始める
    /// — 「遅延: [`ComputeGraph::get`] するまで何も計算しない」がこの
    /// 初期状態そのもの。
    pub fn freeze(self) -> Result<ComputeGraph<V>, ComputeGraphError> {
        let Self {
            entries,
            input_values,
        } = self;

        let node_keys: Vec<String> = entries.iter().map(|(key, _)| key.clone()).collect();
        let edges: Vec<(String, String)> = entries
            .iter()
            .filter_map(|(key, kind)| match kind {
                NodeKind::Computed { deps, .. } => Some((key, deps)),
                NodeKind::Input => None,
            })
            .flat_map(|(key, deps)| deps.iter().map(move |dep| (dep.clone(), key.clone())))
            .collect();

        let dependency_graph: Graph<(), (), String> =
            Graph::from_edges(node_keys, edges).map_err(ComputeGraphError::Graph)?;

        let topo_order: Vec<String> = dependency_graph
            .topological_sort()
            .map_err(ComputeGraphError::Cycle)?
            .into_iter()
            .cloned()
            .collect();

        let topo_index: HashMap<String, usize> = topo_order
            .iter()
            .enumerate()
            .map(|(i, key)| (key.clone(), i))
            .collect();

        let mut kinds: HashMap<String, NodeKind<V>> = HashMap::new();
        let mut dirty: HashSet<String> = HashSet::new();
        for (key, kind) in entries {
            if matches!(kind, NodeKind::Computed { .. }) {
                dirty.insert(key.clone());
            }
            kinds.insert(key, kind);
        }

        Ok(ComputeGraph {
            kinds,
            values: input_values,
            dirty,
            dependency_graph,
            topo_index,
        })
    }
}

/// [`ComputeGraphBuilder::freeze`] が返しうる構築エラー。
///
/// - [`Self::Graph`] — キー重複・未宣言依存 ([`crate::GraphError`] をそのまま
///   運ぶ。stringly-typed な理由メッセージへ潰さず型付きのまま公開する、
///   `docs/design_principles.md` 原則1)。
/// - [`Self::Cycle`] — 循環依存 ([`crate::CycleError`] をそのまま運ぶ。
///   `cycle` フィールドに循環を構成するキー列がそのまま入っている)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeGraphError {
    Graph(GraphError<String>),
    Cycle(CycleError<String>),
}

impl fmt::Display for ComputeGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputeGraphError::Graph(e) => write!(f, "{e}"),
            ComputeGraphError::Cycle(e) => write!(f, "{e}"),
        }
    }
}

impl StdError for ComputeGraphError {}

/// 遅延実行・差分再計算する計算グラフ (モジュール doc 参照)。
///
/// 構築後は依存構造 (どのノードがどのノードに依存するか) は不変。可変なのは
/// 「今の値」と「dirty (未計算/古い) かどうか」の2つだけ (`reactive-cells`
/// の `Engine` が不変な依存グラフ + 可変な値ストアを分けるのと同じ整理、
/// `docs/graph_design_sketches.md` 決定2)。
pub struct ComputeGraph<V> {
    kinds: HashMap<String, NodeKind<V>>,
    /// 現在の値。入力ノードは常に存在する。計算ノードは一度でも評価されると
    /// エントリができ、以後 dirty になっても値は残ったまま (次に
    /// [`Self::get`] されるまでの間、古い値として参照可能だが `dirty` が
    /// `true` の間は [`Self::get`] 経由でしか読めないので古い値が漏れることは
    /// ない)。
    values: HashMap<String, V>,
    /// 計算ノードのうち、依存元の変更以降まだ再評価していないもの (未評価の
    /// 初期状態を含む)。入力ノードは決してこの集合に入らない
    /// (値は直接書き込まれるので「古い」という概念がない)。
    dirty: HashSet<String>,
    /// 依存構造そのもの ([`crate::Graph::reachable_from`] による dirty 伝播
    /// 専用。評価時の位置引数の並びは各ノードの `deps: Vec<String>`
    /// (`kinds`) 側が真実であり、こちらは使わない — モジュール doc
    /// 「既存機構の再利用 vs 内製」参照)。
    dependency_graph: Graph<(), (), String>,
    /// [`ComputeGraphBuilder::freeze`] で1回だけ計算したトポロジカル順序の
    /// 位置索引。依存構造は構築後不変なので、更新のたびに再計算する必要は
    /// ない。
    topo_index: HashMap<String, usize>,
}

impl<V> ComputeGraph<V> {
    /// 構築用 builder を作る。
    pub fn builder() -> ComputeGraphBuilder<V> {
        ComputeGraphBuilder::new()
    }

    /// `key` の現在値を返す。dirty な祖先 (`key` 自身を含む) だけを
    /// トポロジカル順に各1回再計算してから返す (pull 型の遅延評価)。
    ///
    /// # Panics
    /// `key` がこのグラフに存在しないキーの場合 (呼び出し規約違反。
    /// `docs/design_principles.md` 原則2)。
    pub fn get(&mut self, key: &str) -> &V {
        assert!(
            self.kinds.contains_key(key),
            "get: 未知のキーです: {key:?}"
        );
        self.recompute_if_needed(key);
        &self.values[key]
    }

    /// 入力ノード `key` に新しい値を書き込み、影響を受ける計算ノードを
    /// dirty にする (再計算そのものは行わない — 差分は「書き込み + dirty
    /// 伝播」のみで完結し、実際の再計算は次の [`Self::get`] まで遅延する)。
    ///
    /// # Panics
    /// - `key` がこのグラフに存在しないキーの場合。
    /// - `key` が入力ノードではない (計算ノードである) 場合 — 計算ノードの
    ///   値は依存元ノードの更新から常に自動的に決まるべきであり、直接代入は
    ///   依存構造と値ストアの不整合を招く契約違反 (`docs/design_principles.md`
    ///   原則2、`reactive-cells` の `Engine::set_input` と同じ整理)。
    pub fn set_input(&mut self, key: &str, value: V) {
        match self.kinds.get(key) {
            None => panic!("set_input: 未知のキーです: {key:?}"),
            Some(NodeKind::Computed { .. }) => panic!(
                "set_input: {key:?} は計算ノードであり入力ノードではありません。\
                 計算ノードの値は依存元ノードの更新から自動的に決まります。"
            ),
            Some(NodeKind::Input) => {}
        }

        // 影響範囲 (keyを含む) をreachable_fromで絞り、key自身を除いた
        // 影響先だけをdirtyにする (keyの新しい値は直接書き込むので
        // 「再計算が必要」ではない)。
        let affected: Vec<String> = self
            .dependency_graph
            .reachable_from(&key.to_string())
            .into_iter()
            .cloned()
            .collect();

        self.values.insert(key.to_string(), value);
        for affected_key in affected {
            if affected_key != key {
                self.dirty.insert(affected_key);
            }
        }
    }

    /// `key` が dirty なら、その dirty な祖先 (`key` 自身を含む) をすべて
    /// トポロジカル順に評価し、`dirty` から取り除く。`key` が既に clean なら
    /// 何もしない (このグラフの不変条件: clean なノードの依存は全て clean —
    /// [`Self::set_input`] が入力の変更と同時にその descendant 全体を dirty
    /// 化するので、ある依存が dirty になった瞬間そのノードも必ず一緒に
    /// dirty 化されている。よって clean なノードに触れた時点でその祖先を
    /// 遡る必要はない)。
    fn recompute_if_needed(&mut self, key: &str) {
        if !self.dirty.contains(key) {
            return;
        }

        let mut to_eval: Vec<String> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        self.collect_dirty_closure(key, &mut seen, &mut to_eval);

        to_eval.sort_by_key(|k| self.topo_index[k]);

        for k in to_eval {
            self.evaluate(&k);
        }
    }

    /// `key` から依存を遡って dirty な祖先集合 (`key` 自身を含む) を集める。
    /// `seen` は多重経路 (ダイヤモンド依存) での重複走査を防ぐメモ化集合。
    /// clean なノードに到達したら (上の不変条件により) それ以上遡らない。
    fn collect_dirty_closure(&self, key: &str, seen: &mut HashSet<String>, out: &mut Vec<String>) {
        if !seen.insert(key.to_string()) {
            return;
        }
        if !self.dirty.contains(key) {
            return;
        }
        if let NodeKind::Computed { deps, .. } = &self.kinds[key] {
            for dep in deps {
                self.collect_dirty_closure(dep, seen, out);
            }
        }
        out.push(key.to_string());
    }

    /// `key` (計算ノードであるはず) を評価し、値を書き込んで clean にする。
    fn evaluate(&mut self, key: &str) {
        let new_value = match &self.kinds[key] {
            NodeKind::Input => return,
            NodeKind::Computed { deps, f } => {
                let args: Vec<&V> = deps.iter().map(|dep| &self.values[dep]).collect();
                f(&args)
            }
        };
        self.values.insert(key.to_string(), new_value);
        self.dirty.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// `key` の評価回数を数えるカウンタ付きの計算ノードを積む。
    fn computed_counting<D, S>(
        b: &mut ComputeGraphBuilder<f64>,
        key: &str,
        deps: D,
        counter: Rc<RefCell<usize>>,
        f: impl Fn(&[&f64]) -> f64 + 'static,
    ) where
        D: IntoIterator<Item = S>,
        S: Into<String>,
    {
        b.computed(key, deps, move |args| {
            *counter.borrow_mut() += 1;
            f(args)
        });
    }

    #[test]
    fn ダイヤモンド依存でも各ノードちょうど1回だけ再計算される() {
        // price -> a -> c
        // price -> b -> c
        let mut b = ComputeGraph::builder();
        b.input("price", 10.0);

        let count_a = Rc::new(RefCell::new(0));
        let count_b = Rc::new(RefCell::new(0));
        let count_c = Rc::new(RefCell::new(0));

        computed_counting(&mut b, "a", ["price"], count_a.clone(), |args| args[0] * 2.0);
        computed_counting(&mut b, "b", ["price"], count_b.clone(), |args| args[0] + 100.0);
        computed_counting(&mut b, "c", ["a", "b"], count_c.clone(), |args| args[0] + args[1]);

        let mut g = b.freeze().expect("循環が無いので成功するはず");

        assert_eq!(*g.get("c"), 20.0 + 110.0);
        assert_eq!(*count_a.borrow(), 1, "aはちょうど1回だけ再計算されるはず");
        assert_eq!(*count_b.borrow(), 1, "bはちょうど1回だけ再計算されるはず");
        assert_eq!(*count_c.borrow(), 1, "cはちょうど1回だけ再計算されるはず");

        // 差分更新でも各ノードちょうど1回。
        g.set_input("price", 20.0);
        assert_eq!(*g.get("c"), 40.0 + 120.0);
        assert_eq!(*count_a.borrow(), 2);
        assert_eq!(*count_b.borrow(), 2);
        assert_eq!(*count_c.borrow(), 2);
    }

    #[test]
    fn 遅延評価はgetしていない枝を再計算しない() {
        let mut b = ComputeGraph::builder();
        b.input("x", 1.0);

        let count_y = Rc::new(RefCell::new(0));
        let count_z = Rc::new(RefCell::new(0));
        computed_counting(&mut b, "y", ["x"], count_y.clone(), |args| args[0] * 2.0);
        computed_counting(&mut b, "z", ["x"], count_z.clone(), |args| args[0] * 3.0);

        let mut g = b.freeze().unwrap();

        // set_inputだけでは計算が走らない (freeze直後、getを一度も呼んでいない)。
        assert_eq!(*count_y.borrow(), 0);
        assert_eq!(*count_z.borrow(), 0);

        // yだけgetする。zは一度も評価されない。
        assert_eq!(*g.get("y"), 2.0);
        assert_eq!(*count_y.borrow(), 1);
        assert_eq!(*count_z.borrow(), 0, "getしていないzは再計算されないはず");

        // 入力を書き換えてもgetしなければzは動かない。
        g.set_input("x", 5.0);
        assert_eq!(*count_y.borrow(), 1);
        assert_eq!(*count_z.borrow(), 0);
    }

    #[test]
    fn 差分更新は影響外のノードを再計算しない() {
        // a -> b (aはinput)
        // d -> e (dはinput、aとは無関係な別枝)
        let mut b = ComputeGraph::builder();
        b.input("a", 1.0);
        b.input("d", 100.0);

        let count_b = Rc::new(RefCell::new(0));
        let count_e = Rc::new(RefCell::new(0));
        computed_counting(&mut b, "b", ["a"], count_b.clone(), |args| args[0] * 2.0);
        computed_counting(&mut b, "e", ["d"], count_e.clone(), |args| args[0] + 1.0);

        let mut g = b.freeze().unwrap();

        // 両方一度getしてキャッシュ済みの状態を作る。
        assert_eq!(*g.get("b"), 2.0);
        assert_eq!(*g.get("e"), 101.0);
        assert_eq!(*count_b.borrow(), 1);
        assert_eq!(*count_e.borrow(), 1);

        // aだけ変更する -> 影響が及ぶのはbのみ、eは無関係。
        g.set_input("a", 10.0);
        assert_eq!(*g.get("b"), 20.0);
        assert_eq!(*count_b.borrow(), 2, "bは再計算されるはず");

        // eをgetしても再計算されない (dirtyになっていないため)。
        assert_eq!(*g.get("e"), 101.0);
        assert_eq!(*count_e.borrow(), 1, "eは影響を受けていないので再計算されないはず");
    }

    #[test]
    fn freezeは循環をパスつきcycleerrorで拒否する() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.computed("a", ["b"], |args| *args[0]);
        b.computed("b", ["a"], |args| *args[0]);

        let err = match b.freeze() {
            Err(err) => err,
            Ok(_) => panic!("循環があるので失敗するはず"),
        };

        match err {
            ComputeGraphError::Cycle(cycle_err) => {
                let members: HashSet<String> = cycle_err.cycle.into_iter().collect();
                assert_eq!(members, HashSet::from(["a".to_string(), "b".to_string()]));
            }
            other => panic!("Cycleエラーになるはずが: {other:?}"),
        }
    }

    #[test]
    fn freezeは未宣言依存をエラーで拒否する() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.input("x", 1.0);
        b.computed("y", ["z"], |args| *args[0]); // "z"は未宣言

        let err = match b.freeze() {
            Err(err) => err,
            Ok(_) => panic!("未宣言依存があるので失敗するはず"),
        };

        match err {
            ComputeGraphError::Graph(GraphError::UnknownEndpoint { missing, .. }) => {
                assert_eq!(missing, "z");
            }
            other => panic!("UnknownEndpointエラーになるはずが: {other:?}"),
        }
    }

    #[test]
    fn freezeはキー重複をエラーで拒否する() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.input("x", 1.0);
        b.input("x", 2.0); // 重複

        let err = match b.freeze() {
            Err(err) => err,
            Ok(_) => panic!("キー重複があるので失敗するはず"),
        };

        assert_eq!(err, ComputeGraphError::Graph(GraphError::DuplicateKey("x".to_string())));
    }

    #[test]
    #[should_panic(expected = "未知のキーです")]
    fn getは未知のキーでパニックする() {
        let b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        let mut g = b.freeze().unwrap();
        let _ = g.get("no_such_key");
    }

    #[test]
    #[should_panic(expected = "未知のキーです")]
    fn set_inputは未知のキーでパニックする() {
        let b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        let mut g = b.freeze().unwrap();
        g.set_input("no_such_key", 1.0);
    }

    #[test]
    #[should_panic(expected = "計算ノードであり入力ノードではありません")]
    fn set_inputは計算ノードに対してパニックする() {
        let mut b: ComputeGraphBuilder<f64> = ComputeGraph::builder();
        b.input("x", 1.0);
        b.computed("y", ["x"], |args| *args[0]);
        let mut g = b.freeze().unwrap();
        g.set_input("y", 999.0);
    }
}
