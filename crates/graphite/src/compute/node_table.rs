//! 計算ノード表 — キーからノード種別を引き、依存キー列と計算の実行を答える。

use std::collections::HashMap;

use super::node_kind::ノード種別;

/// 表に対してキーを照会した結果。未知・入力ノード・計算ノードの3通りしかない。
pub(in crate::compute) enum キーの照会結果 {
    未知のキー,
    入力ノード,
    計算ノード,
}

/// キーからノード種別を引く表。構築後は不変で、依存構造と評価の両方がここを参照する。
pub(in crate::compute) struct 計算ノード表<V> {
    種別: HashMap<String, ノード種別<V>>,
}

impl<V> 計算ノード表<V> {
    pub(in crate::compute) fn 宣言列から生成する(
        宣言列: Vec<(String, ノード種別<V>)>,
    ) -> Self {
        Self {
            種別: 宣言列.into_iter().collect(),
        }
    }

    pub(in crate::compute) fn キーを含むか(&self, キー: &str) -> bool {
        self.種別.contains_key(キー)
    }

    pub(in crate::compute) fn キーを照会する(&self, キー: &str) -> キーの照会結果 {
        match self.種別.get(キー) {
            None => キーの照会結果::未知のキー,
            Some(ノード種別::入力ノード) => キーの照会結果::入力ノード,
            Some(ノード種別::計算ノード { .. }) => キーの照会結果::計算ノード,
        }
    }

    /// 計算ノードの依存キー列 (宣言順)。入力ノードと未知のキーには空を返す。
    pub(in crate::compute) fn 依存キー列(&self, キー: &str) -> &[String] {
        match self.種別.get(キー) {
            Some(ノード種別::計算ノード {
                依存キー列, ..
            }) => 依存キー列,
            _ => &[],
        }
    }

    /// 計算ノードの値を求める。入力ノードと未知のキーには `None` を返す
    /// (入力ノードの値は書き込まれるものであって計算されるものではない)。
    pub(in crate::compute) fn 依存値から計算する(
        &self,
        キー: &str,
        依存値: &[&V],
    ) -> Option<V> {
        match self.種別.get(キー) {
            Some(ノード種別::計算ノード {
                値を求める, ..
            }) => Some(値を求める.依存値から求める(依存値)),
            _ => None,
        }
    }

    /// 計算ノードのキーだけを列挙する (順序は未規定)。
    pub(in crate::compute) fn 計算ノードのキー列(&self) -> impl Iterator<Item = &String> {
        self.種別
            .iter()
            .filter(|(_, 種別)| matches!(種別, ノード種別::計算ノード { .. }))
            .map(|(キー, _)| キー)
    }
}
