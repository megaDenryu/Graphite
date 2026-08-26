//! 依存構造 — 検証済みの依存グラフとキーごとのトポロジカル位置を所有する (凍結後は不変)。

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
