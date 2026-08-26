//! 計算グラフの構築用 builder — 凍結前の半端な宣言列を唯一所有する。

use std::collections::HashMap;

use super::dependency_structure::依存構造;
use super::error::ComputeGraphError;
use super::node_kind::{ノード種別, 値を求める関数};
use super::node_table::計算ノード表;
use super::ComputeGraph;

/// [`ComputeGraph::builder`] が返す構築用 builder。
///
/// `input`/`computed` でノードを積み、[`Self::freeze`] で凍結する
/// (`docs/graph_design_sketches.md` 決定2 — クロージャスコープではなく
/// 値としての builder → freeze だが、「構築中の型」と「構築後の型」を
/// 分けるという要点は同じ)。
pub struct ComputeGraphBuilder<V> {
    entries: Vec<(String, ノード種別<V>)>,
    input_values: HashMap<String, V>,
}

impl<V> ComputeGraphBuilder<V> {
    pub(in crate::compute) fn 空のbuilderから始める() -> Self {
        Self {
            entries: Vec::new(),
            input_values: HashMap::new(),
        }
    }

    /// 入力ノードを1つ積む。`key` が重複した場合のエラーは [`Self::freeze`]
    /// まで遅延する ([`ComputeGraphError::Graph`] の
    /// [`crate::GraphError::DuplicateKey`])。
    pub fn input(&mut self, key: impl Into<String>, value: V) -> &mut Self {
        let key = key.into();
        self.input_values.insert(key.clone(), value);
        self.entries.push((key, ノード種別::入力ノード));
        self
    }

    /// 計算ノードを1つ積む。`deps` は評価時に `f` へ渡される位置引数の並び
    /// そのもの (`args[0]` = `deps` の0番目)。`deps` が参照するキーが未宣言
    /// だった場合のエラーは [`Self::freeze`] まで遅延する
    /// ([`ComputeGraphError::Graph`] の [`crate::GraphError::UnknownEndpoint`])。
    pub fn computed<D, S>(
        &mut self,
        key: impl Into<String>,
        deps: D,
        f: impl Fn(&[&V]) -> V + 'static,
    ) -> &mut Self
    where
        D: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let 依存キー列: Vec<String> = deps.into_iter().map(Into::into).collect();
        self.entries.push((
            key.into(),
            ノード種別::計算ノード {
                依存キー列,
                値を求める: 値を求める関数::関数から生成する(f),
            },
        ));
        self
    }

    /// 凍結して [`ComputeGraph`] を作る。
    ///
    /// 検証順序: まずキー重複・未宣言依存、次に循環 (どちらも依存構造へ委譲)。
    /// 凍結後は全ての計算ノードを dirty (未計算) 状態で始める — 「遅延:
    /// [`ComputeGraph::get`] するまで何も計算しない」がこの初期状態そのもの。
    pub fn freeze(self) -> Result<ComputeGraph<V>, ComputeGraphError> {
        let Self {
            entries,
            input_values,
        } = self;
        let 依存構造 = 依存構造::宣言列を検証して確定する(&entries)?;
        let ノード表 = 計算ノード表::宣言列から生成する(entries);
        Ok(ComputeGraph::部品から組み立てる(
            ノード表,
            依存構造,
            input_values,
        ))
    }
}
