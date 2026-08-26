//! 再計算エンジン — 「グラフによる再定式化」の核心部分。
//!
//! [`Engine`] は不変な依存グラフ ([`crate::schema::Sheet`]、`graph_schema!`
//! が生成した型) と、可変な「今の値」(`HashMap<CellId, f64>`) を分けて
//! 持つ (`../Bullet/docs/graph_design_sketches.md` 決定2)。[`Engine::set_input`] が
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
//! セル自身の `Formula` は「どの演算か」だけを持ち、演算対象はこのセルを
//! 終点とするエッジをその都度絞り込んで求める (`evaluation.rs`) —
//! `Formula` とグラフの両方に同じ依存情報を複製する二重管理を
//! `docs/modeling_guide.md` §5 の適用で解消している (`README.md`
//! 「モデリングガイド§5の適用例」節参照)。

mod dependency_projection;
mod evaluation;
mod formula_wiring;
mod recompute_step;

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};

use graphite::{CycleError, Graph};

use crate::schema::{CellId, Formula, Sheet};

pub use recompute_step::RecomputeStep;

/// 依存グラフ (不変) + 現在値ストア (可変) を束ねた再計算エンジン。
pub struct Engine {
    graph: Sheet::Graph,
    /// `Feeds`/`Lhs`/`Rhs` エッジを1つに射影した汎用グラフ。
    /// `reachable_from`/`topological_sort` はここに1回だけ委譲する
    /// (`graphite::Graph` が既に持つ水準1アルゴリズムを再実装しない)。
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
    /// 本数が一致しない場合 (`formula_wiring.rs` 参照)。
    pub fn new(graph: Sheet::Graph) -> Result<Self, CycleError<CellId>> {
        formula_wiring::validate_formula_wiring(&graph);

        let dependency_graph = dependency_projection::project_dependency_graph(&graph);
        let topo_order: Vec<CellId> = dependency_graph
            .topological_sort()?
            .into_iter()
            .cloned()
            .collect();
        let values: HashMap<CellId, f64> = graph.cell_ids().map(|id| (id.clone(), 0.0)).collect();

        Ok(Self {
            graph,
            dependency_graph,
            topo_order,
            values,
        })
    }

    /// 依存グラフそのもの (schema/graph! が生成した不変な `Sheet`) への
    /// 参照。`main.rs` がセル一覧や式を読むために使う。
    pub fn graph(&self) -> &Sheet::Graph {
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
    /// (呼び出し規約違反。`docs/development/design_principles.md` 原則2 — graphite
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
    /// (README「グリッチ不在の証明」節、`tests/recomputation.rs` 参照)。
    ///
    /// # Panics
    /// - `id` が `Sheet` に存在しないキーの場合。
    /// - `id` が入力セル (`Formula::Input`) ではない場合 (計算セルへの
    ///   直接代入は契約違反 — 式を経由せず値を書き換えると依存グラフと
    ///   値ストアが不整合になるため)。
    pub fn set_input(&mut self, id: &CellId, value: f64) -> Vec<RecomputeStep> {
        let cell = self
            .graph
            .cell_by_id(id)
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
            let formula = self
                .graph
                .cell_by_id(cell_id)
                .expect("topo_orderに含まれるキーはcell_by_id()に必ず存在する")
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
}
