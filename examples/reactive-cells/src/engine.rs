//! 再計算エンジン — 「グラフによる再定式化」の核心部分。
//!
//! [`Engine`] は不変な依存グラフ ([`crate::schema::Sheet`]、`graph_schema!`
//! が生成した型) と、可変な「今の値」(`HashMap<CellId, f64>`) を分けて
//! 持つ (`docs/graph_design_sketches.md` 決定2)。[`Engine::set_input`] が
//! 1回呼ばれるたびに:
//!
//! 1. `graphite::Graph::reachable_from` で「この入力の変更で影響を受ける
//!    セル (自分自身を含む)」だけに範囲を絞る。
//! 2. あらかじめ計算済みのトポロジカル順序 (`graphite::Graph::topological_sort`)
//!    をその範囲でフィルタしながら辿り、影響を受けるセルを**それぞれ
//!    ちょうど1回だけ**再計算する。
//!
//! トポロジカル順序は「あるセルを計算する時点で、そのセルが依存する
//! 全セルは既に最新値になっている」ことを保証する順序そのものなので、
//! これが glitch (矛盾した中間状態の観測) が原理的に起きない理由になる
//! (README「なぜグラフで直るのか」節)。
//!
//! ## 演算対象はグラフから読む (`Formula` はCellIdを持たない)
//!
//! `Sheet` の依存エッジは `Feeds` (可換な `Mul`/`Sum` の被演算子)・`Lhs`/
//! `Rhs` (非可換な `Sub` の被減数/減数) の3種類 (`src/schema.rs` 参照)。
//! [`Engine::eval_formula`] はセル自身の `Formula` (「どの演算か」だけを
//! 持つ) と、このセルを終点とするエッジをその都度絞り込んで演算対象を
//! 求める — `Formula` とグラフの両方に同じ依存情報を複製する二重管理を
//! `docs/modeling_guide.md` §5 の適用で解消している (`README.md`
//! 「モデリングガイド§5の適用例」節参照)。

use std::collections::{HashMap, HashSet};

use graphite::{CycleError, Graph};

use crate::schema::{Cell, CellId, Feeds, Formula, Lhs, Rhs, Sheet, SheetNode};

/// [`Engine::set_input`] が1回の更新で行った再計算1件分の記録。
///
/// `main.rs`/テストはこの列を「どのセルがどの順で再計算されたか」の
/// 証拠として読む。
#[derive(Debug, Clone, PartialEq)]
pub struct RecomputeStep {
    pub id: CellId,
    pub value: f64,
}

/// 依存グラフ (不変) + 現在値ストア (可変) を束ねた再計算エンジン。
pub struct Engine {
    graph: Sheet,
    /// `Feeds`/`Lhs`/`Rhs` エッジを1つに射影した汎用グラフ。
    /// `reachable_from`/`topological_sort` はここに1回だけ委譲する
    /// (`graphite::Graph` が既に持つ水準1アルゴリズムを再実装しない)。
    /// 3種のエッジはいずれも「依存元→依存先」という同じ向きの意味を
    /// 持つ (`src/schema.rs` 参照) ので、単純に合流させて良い。
    dependency_graph: Graph<(), (), CellId>,
    /// 構築時に1回だけ計算したトポロジカル順序。依存構造
    /// (`dependency_graph`) は構築後不変なので、この順序も更新ごとに
    /// 再計算する必要はない。
    topo_order: Vec<CellId>,
    values: HashMap<CellId, f64>,
}

impl Engine {
    /// `graph` (構築済みの不変な依存グラフ) から再計算エンジンを作る。
    ///
    /// 全ての値は `0.0` で初期化される (入力セルの初期値も
    /// [`Self::set_input`] で明示的に設定するのがこのexampleの流儀 —
    /// `main.rs`/テストの「値変更→伝播」の物語がそのまま初期化の物語にも
    /// なる)。
    ///
    /// `graph` に循環があれば、この時点で `Err(CycleError)` になる —
    /// これが「循環する依存グラフの構築を拒否する」の実体
    /// (README「循環の拒否」節)。`CycleError::cycle` には循環を構成する
    /// `CellId` の列がそのまま入っているので、`{cycle_error}` で
    /// 具体的な循環パスを表示できる。
    ///
    /// # Panics
    /// `graph` 内のセルの `Formula` が要求するエッジ本数と実際のエッジ
    /// 本数が一致しない場合 (例: `Formula::Sub` のセルに `Lhs`/`Rhs` の
    /// どちらかが無い/2本以上ある)。これは `graph!`/`Sheet::create` の
    /// 検証対象外 (端点存在と `unique pair` だけを見る、`src/fixtures.rs`
    /// 参照) なので、`Formula` とエッジの整合性はドメイン側 (ここ) の
    /// 責務として構築時に検査する (呼び出し規約違反はパニック、
    /// `docs/design_principles.md` 原則2)。
    pub fn new(graph: Sheet) -> Result<Self, CycleError<CellId>> {
        Self::validate_formula_wiring(&graph);

        let dependency_graph: Graph<(), (), CellId> = Graph::from_edges(
            Cell::ids(&graph).cloned(),
            Feeds::iter(&graph)
                .map(|(_id, edge)| (edge.from().clone(), edge.to().clone()))
                .chain(Lhs::iter(&graph).map(|(_id, edge)| (edge.from().clone(), edge.to().clone())))
                .chain(Rhs::iter(&graph).map(|(_id, edge)| (edge.from().clone(), edge.to().clone()))),
        )
        .expect(
            "Cell::ids()とFeeds/Lhs/Rhs::iter()の端点整合はSheet::create/create_collectingの検証で\
             既に保証されているはず (未知キー・重複キーはここでは起こらない)",
        );

        let topo_order: Vec<CellId> = dependency_graph
            .topological_sort()?
            .into_iter()
            .cloned()
            .collect();

        let values: HashMap<CellId, f64> = Cell::ids(&graph).map(|id| (id.clone(), 0.0)).collect();

        Ok(Self {
            graph,
            dependency_graph,
            topo_order,
            values,
        })
    }

    /// `Formula` が要求するエッジ本数と実際のエッジ本数の整合性を検査する。
    ///
    /// - `Mul`/`Sum` — このセルを終点とする `Feeds` エッジが1本以上必要
    ///   (可換なので本数の上限は無い)。
    /// - `Sub` — このセルを終点とする `Lhs`/`Rhs` エッジがそれぞれ
    ///   ちょうど1本必要 (被減数/減数はどちらも一意でなければならない)。
    /// - `Input` — エッジ本数を問わない (値は `set_input` で直接与える)。
    fn validate_formula_wiring(graph: &Sheet) {
        for (cell_id, cell) in Cell::iter(graph) {
            match cell.formula {
                Formula::Input => {}
                Formula::Mul | Formula::Sum => {
                    let count = Feeds::iter(graph).filter(|(_id, edge)| edge.to() == cell_id).count();
                    assert!(
                        count >= 1,
                        "{cell_id:?}: {:?}セルには演算対象を表すFeedsエッジが1本以上必要です (実際: {count}本)",
                        cell.formula
                    );
                }
                Formula::Sub => {
                    let lhs_count = Lhs::iter(graph).filter(|(_id, edge)| edge.to() == cell_id).count();
                    let rhs_count = Rhs::iter(graph).filter(|(_id, edge)| edge.to() == cell_id).count();
                    assert_eq!(
                        lhs_count, 1,
                        "{cell_id:?}: Subセルには被減数を表すLhsエッジがちょうど1本必要です (実際: {lhs_count}本)"
                    );
                    assert_eq!(
                        rhs_count, 1,
                        "{cell_id:?}: Subセルには減数を表すRhsエッジがちょうど1本必要です (実際: {rhs_count}本)"
                    );
                }
            }
        }
    }

    /// 依存グラフそのもの (schema/graph! が生成した不変な `Sheet`) への
    /// 参照。`main.rs` がセル一覧や式を読むために使う。
    pub fn graph(&self) -> &Sheet {
        &self.graph
    }

    /// トポロジカル順序 (構築時に1回だけ計算したもの)。
    pub fn topological_order(&self) -> &[CellId] {
        &self.topo_order
    }

    /// セルの現在値。
    ///
    /// # Panics
    /// `id` がこのエンジンの `Sheet` に存在しないキーの場合パニックする
    /// (呼び出し規約違反。`docs/design_principles.md` 原則2 — graphite
    /// ランタイムのビュー `of` と同じ契約)。
    pub fn value(&self, id: &CellId) -> f64 {
        *self
            .values
            .get(id)
            .unwrap_or_else(|| panic!("value: 未知のセルキーです: {id:?}"))
    }

    /// 入力セル `id` に新しい値を設定し、影響を受けるセルをトポロジカル
    /// 順に再計算する。戻り値は再計算した順序そのもの (`id` 自身は
    /// 含まない — `id` は「式で求めた」のではなく「直接設定した」ため)。
    ///
    /// 各セルは影響範囲に含まれる限り**ちょうど1回**だけ再計算される
    /// (README「グリッチ不在の証明」節、`tests/integration.rs` 参照)。
    ///
    /// # Panics
    /// - `id` が `Sheet` に存在しないキーの場合。
    /// - `id` が入力セル (`Formula::Input`) ではない場合 (計算セルへの
    ///   直接代入は契約違反 — 式を経由せず値を書き換えると依存グラフと
    ///   値ストアが不整合になるため)。
    pub fn set_input(&mut self, id: &CellId, value: f64) -> Vec<RecomputeStep> {
        let cell = Cell::get(&self.graph, id)
            .unwrap_or_else(|| panic!("set_input: 未知のセルキーです: {id:?}"));
        assert!(
            matches!(cell.formula, Formula::Input),
            "set_input: {id:?} は計算セルであり入力セルではありません (formula: {:?})。\
             計算セルの値は依存元セルの更新から自動的に決まります。",
            cell.formula
        );

        // 影響範囲 (idを含む) をreachable_fromで絞る。
        let affected: HashSet<CellId> = self
            .dependency_graph
            .reachable_from(id)
            .into_iter()
            .cloned()
            .collect();

        self.values.insert(id.clone(), value);

        let mut steps = Vec::new();
        for cell_id in &self.topo_order {
            if cell_id == id || !affected.contains(cell_id) {
                continue;
            }
            let formula = Cell::get(&self.graph, cell_id)
                .expect("topo_orderに含まれるキーはCell::get()に必ず存在する")
                .formula;
            let new_value = self.eval_formula(cell_id, formula);
            self.values.insert(cell_id.clone(), new_value);
            steps.push(RecomputeStep {
                id: cell_id.clone(),
                value: new_value,
            });
        }
        steps
    }

    /// `cell_id` の `formula` を評価する。演算対象は `formula` 自身では
    /// なく、`cell_id` を終点とする `Feeds`/`Lhs`/`Rhs` エッジから読む
    /// (このファイル冒頭のモジュール doc 参照)。
    fn eval_formula(&self, cell_id: &CellId, formula: Formula) -> f64 {
        match formula {
            Formula::Input => {
                unreachable!("Inputセルはset_inputのトポロジカル走査で再計算対象にならない")
            }
            Formula::Mul => self.feeds_into(cell_id).product(),
            Formula::Sum => self.feeds_into(cell_id).sum(),
            Formula::Sub => self.lhs_value(cell_id) - self.rhs_value(cell_id),
        }
    }

    /// `cell_id` を終点とする `Feeds` エッジの起点セルの値を、挿入順
    /// (`docs/schema_v4.md` §3.2 の順序保証) で列挙する。
    fn feeds_into<'a>(&'a self, cell_id: &'a CellId) -> impl Iterator<Item = f64> + 'a {
        Feeds::iter(&self.graph)
            .filter(move |(_id, edge)| edge.to() == cell_id)
            .map(move |(_id, edge)| self.value(edge.from()))
    }

    /// `cell_id` を終点とする `Lhs` エッジの起点セルの値 (被減数)。
    ///
    /// # Panics
    /// `Lhs` エッジがちょうど1本であることは [`Self::validate_formula_wiring`]
    /// が `Engine::new` の時点で検査済みなので、ここに到達した時点で
    /// 見つからなければ実装の不整合 (バグ) である。
    fn lhs_value(&self, cell_id: &CellId) -> f64 {
        let (_id, edge) = Lhs::iter(&self.graph)
            .find(|(_id, edge)| edge.to() == cell_id)
            .expect("validate_formula_wiringで存在を検査済みのはず");
        self.value(edge.from())
    }

    /// `cell_id` を終点とする `Rhs` エッジの起点セルの値 (減数)。
    ///
    /// # Panics
    /// [`Self::lhs_value`] と同様、`Engine::new` の検査済み前提が破れて
    /// いる場合のみパニックする (実装の不整合)。
    fn rhs_value(&self, cell_id: &CellId) -> f64 {
        let (_id, edge) = Rhs::iter(&self.graph)
            .find(|(_id, edge)| edge.to() == cell_id)
            .expect("validate_formula_wiringで存在を検査済みのはず");
        self.value(edge.from())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::{cyclic_demo_sheet, default_sheet};

    fn id(s: &str) -> CellId {
        CellId(s.to_string())
    }

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
    fn 全入力を設定すると見積の数値が正しく伝播する() {
        let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
        engine.set_input(&id("unit_price"), 1000.0);
        engine.set_input(&id("quantity"), 3.0);
        engine.set_input(&id("tax_rate"), 0.1);
        engine.set_input(&id("discount_rate"), 0.05);
        engine.set_input(&id("shipping_fee"), 500.0);

        assert_eq!(engine.value(&id("subtotal")), 3000.0);
        assert_eq!(engine.value(&id("discount_amount")), 150.0);
        assert_eq!(engine.value(&id("tax")), 300.0);
        assert_eq!(engine.value(&id("adjustment")), 150.0);
        assert_eq!(engine.value(&id("grand_total")), 3650.0);
    }

    #[test]
    fn 影響のないセルはreachable_fromで絞られ再計算されない() {
        let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
        engine.set_input(&id("unit_price"), 1000.0);
        engine.set_input(&id("quantity"), 3.0);
        engine.set_input(&id("tax_rate"), 0.1);
        engine.set_input(&id("discount_rate"), 0.05);
        engine.set_input(&id("shipping_fee"), 500.0);

        // tax_rateだけを変える -> 影響が及ぶのはtax/adjustment/grand_totalのみ。
        // subtotal/discount_amount/他の入力は無関係なので再計算されない。
        let steps = engine.set_input(&id("tax_rate"), 0.2);
        let ids: HashSet<CellId> = steps.iter().map(|s| s.id.clone()).collect();
        assert_eq!(ids, HashSet::from([id("tax"), id("adjustment"), id("grand_total")]));
        assert_eq!(steps.len(), 3, "各セルはちょうど1回だけ再計算されるはず");

        // 新しい税額: subtotal(3000) * 0.2 = 600、adjustment = 600 - 150 = 450、
        // grand_total = 3000 + 450 + 500 = 3950。
        assert_eq!(engine.value(&id("tax")), 600.0);
        assert_eq!(engine.value(&id("adjustment")), 450.0);
        assert_eq!(engine.value(&id("grand_total")), 3950.0);
    }

    #[test]
    fn ダイヤモンド依存でもadjustmentはちょうど1回だけ再計算される() {
        let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
        engine.set_input(&id("tax_rate"), 0.1);
        engine.set_input(&id("discount_rate"), 0.05);
        engine.set_input(&id("shipping_fee"), 500.0);
        engine.set_input(&id("quantity"), 4.0);

        // unit_priceの変更はsubtotal(a) -> discount_amount(b)/tax(c) -> adjustment(d)
        // というダイヤモンド全体に伝播する。
        let steps = engine.set_input(&id("unit_price"), 2000.0);
        let ids: Vec<CellId> = steps.iter().map(|s| s.id.clone()).collect();

        // 重複が無い (=それぞれちょうど1回) ことを確認する。
        let unique: HashSet<CellId> = ids.iter().cloned().collect();
        assert_eq!(ids.len(), unique.len(), "各セルの再計算は重複してはならない");
        assert_eq!(
            unique,
            HashSet::from([
                id("subtotal"),
                id("discount_amount"),
                id("tax"),
                id("adjustment"),
                id("grand_total"),
            ])
        );

        // 順序もトポロジカル (subtotalが最初、adjustmentはb,cの後、
        // grand_totalが最後) であることを確認する。
        let pos = |k: &str| ids.iter().position(|c| c.0 == k).unwrap();
        assert!(pos("subtotal") < pos("discount_amount"));
        assert!(pos("subtotal") < pos("tax"));
        assert!(pos("discount_amount") < pos("adjustment"));
        assert!(pos("tax") < pos("adjustment"));
        assert!(pos("adjustment") < pos("grand_total"));

        // 具体的な数値でも矛盾がないことを確認する:
        // subtotal=2000*4=8000, discount_amount=8000*0.05=400,
        // tax=8000*0.1=800, adjustment=800-400=400, grand_total=8000+400+500=8900。
        // glitchが起きていれば (例えばadjustmentが古いdiscount_amount/taxの
        // どちらかを混ぜて計算していれば) これらの等式は成立しない。
        assert_eq!(engine.value(&id("subtotal")), 8000.0);
        assert_eq!(engine.value(&id("discount_amount")), 400.0);
        assert_eq!(engine.value(&id("tax")), 800.0);
        assert_eq!(engine.value(&id("adjustment")), 400.0);
        assert_eq!(engine.value(&id("grand_total")), 8900.0);
    }

    #[test]
    fn 複数回の入力変更が累積して正しく反映される() {
        let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
        engine.set_input(&id("unit_price"), 100.0);
        engine.set_input(&id("quantity"), 1.0);
        engine.set_input(&id("tax_rate"), 0.1);
        engine.set_input(&id("discount_rate"), 0.0);
        engine.set_input(&id("shipping_fee"), 0.0);
        assert_eq!(engine.value(&id("grand_total")), 110.0);

        engine.set_input(&id("quantity"), 2.0);
        assert_eq!(engine.value(&id("subtotal")), 200.0);
        assert_eq!(engine.value(&id("grand_total")), 220.0);

        engine.set_input(&id("discount_rate"), 0.1);
        assert_eq!(engine.value(&id("discount_amount")), 20.0);
        assert_eq!(engine.value(&id("adjustment")), 20.0 - 20.0); // tax(20) - discount(20)
        assert_eq!(engine.value(&id("grand_total")), 200.0 + 0.0 + 0.0);
    }

    #[test]
    #[should_panic(expected = "未知のセルキーです")]
    fn set_inputは未知のキーでパニックする() {
        let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
        engine.set_input(&id("no_such_cell"), 1.0);
    }

    #[test]
    #[should_panic(expected = "計算セルであり入力セルではありません")]
    fn set_inputは計算セルに対してパニックする() {
        let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
        engine.set_input(&id("subtotal"), 999.0);
    }

    #[test]
    #[should_panic(expected = "value: 未知のセルキーです")]
    fn valueは未知のキーでパニックする() {
        let engine = Engine::new(default_sheet().unwrap()).unwrap();
        let _ = engine.value(&id("no_such_cell"));
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
        .expect("構造としては正常に構築できる (Lhs不足は検証対象外)");
        let _ = Engine::new(broken);
    }

    #[test]
    #[should_panic(expected = "Feedsエッジが1本以上必要です")]
    fn engine_newはmulセルにfeedsエッジが無いとパニックする() {
        #[rustfmt::skip]
        let broken = graphite::graph!(Sheet {
            lonely = Cell { formula: Formula::Mul },
        })
        .expect("構造としては正常に構築できる (Feeds不足は検証対象外)");
        let _ = Engine::new(broken);
    }
}
