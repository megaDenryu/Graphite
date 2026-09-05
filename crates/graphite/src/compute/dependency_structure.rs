//! 依存構造 — 検証済みの依存グラフとキーごとのトポロジカル位置を所有する (凍結後は不変)。
//!
//! ## 既存機構の再利用 vs 内製
//!
//! - **再利用: 循環検出・キー重複・未宣言依存の検証、トポロジカル順序の計算。**
//!   依存キー列と `(依存元, 依存先)` の辺列を [`Graph::from_edges`] に渡すだけで、
//!   キー重複 ([`crate::GraphError::DuplicateKey`])・未宣言依存への辺
//!   ([`crate::GraphError::UnknownEndpoint`]) の検証がそのまま手に入る。続けて
//!   [`Graph::topological_sort`] を呼べば、循環検出 ([`crate::CycleError`]、
//!   循環パスつき) とトポロジカル順序の計算を1回で済ませられる。これは
//!   `examples/reactive-cells` の `Engine::new` が `graphite::Graph` へ依存グラフを
//!   射影して同じ2操作に委譲しているのと全く同じパターンであり、車輪の再発明を
//!   避けられる。
//! - **内製: 依存元の値の保持・位置引数への変換・未再計算集合の管理。**
//!   [`crate::Graph`] はノード値を1種類の型に固定する設計
//!   (`../Bullet/docs/graph_design_sketches.md` 決定1/決定2 をそのまま輸入したもの) であり、
//!   「入力ノードは値のみ・計算ノードは依存キー列+関数を持つ」という異種混合の
//!   ノードは表現できない。加えて評価時に依存値を**宣言順の位置引数**として渡す
//!   必要があり、`Graph` の `in_neighbors`/`out_neighbors` は順序を保証しない
//!   (内部実装が `petgraph` の近傍イテレータに委譲しているため)。そのため各計算
//!   ノードの依存キー列は計算ノード表 (`node_table.rs`) が直接持ち、値と未再計算集合
//!   の管理は評価状態 (`evaluation_state.rs`) が内製する。「再利用できる部分は
//!   再利用し、既存の型に無理に押し込むと歪みが生じる部分は内製する」という判断
//!   そのもの。

use std::collections::HashMap;

use crate::Graph;

use super::error::ComputeGraphError;
use super::node_kind::ノード種別;

/// どのノードがどのノードに依存するかという構造そのもの。凍結時に一度だけ
/// 検証して確定し、以後は影響範囲とトポロジカル位置の問い合わせにだけ答える。
pub(in crate::compute) struct 依存構造 {
    依存グラフ: Graph<(), (), String>,
    キーごとのトポロジカル位置: HashMap<String, usize>,
}

impl 依存構造 {
    /// ノードの宣言列を検証して確定する。検証順序はキー重複・未宣言依存
    /// ([`Graph::from_edges`] へ委譲) が先で、循環 ([`Graph::topological_sort`]
    /// へ委譲、パスつき) が後。
    pub(in crate::compute) fn 宣言列を検証して確定する<V>(
        宣言列: &[(String, ノード種別<V>)],
    ) -> Result<Self, ComputeGraphError> {
        let ノードキー列: Vec<String> = 宣言列.iter().map(|(キー, _)| キー.clone()).collect();
        let 辺列: Vec<(String, String)> = 宣言列
            .iter()
            .filter_map(|(キー, 種別)| match 種別 {
                ノード種別::計算ノード {
                    依存キー列, ..
                } => Some((キー, 依存キー列)),
                ノード種別::入力ノード => None,
            })
            .flat_map(|(キー, 依存キー列)| {
                依存キー列
                    .iter()
                    .map(move |依存| (依存.clone(), キー.clone()))
            })
            .collect();

        let 依存グラフ: Graph<(), (), String> =
            Graph::from_edges(ノードキー列, 辺列).map_err(ComputeGraphError::Graph)?;

        let トポロジカル順序: Vec<String> = 依存グラフ
            .topological_sort()
            .map_err(ComputeGraphError::Cycle)?
            .into_iter()
            .cloned()
            .collect();

        let キーごとのトポロジカル位置: HashMap<String, usize> = トポロジカル順序
            .iter()
            .enumerate()
            .map(|(位置, キー)| (キー.clone(), 位置))
            .collect();

        Ok(Self {
            依存グラフ,
            キーごとのトポロジカル位置,
        })
    }

    /// `キー` の変更が影響を及ぼすキー列 (`キー` 自身を含む反射的な到達可能性)。
    pub(in crate::compute) fn 影響を受けるキー列(&self, キー: &str) -> Vec<String> {
        self.依存グラフ
            .reachable_from(&キー.to_string())
            .into_iter()
            .cloned()
            .collect()
    }

    /// 凍結時に1回だけ計算したトポロジカル順序の中での位置。依存構造は構築後
    /// 不変なので、更新のたびに再計算する必要はない。
    pub(in crate::compute) fn トポロジカル位置(&self, キー: &str) -> usize {
        self.キーごとのトポロジカル位置[キー]
    }
}
