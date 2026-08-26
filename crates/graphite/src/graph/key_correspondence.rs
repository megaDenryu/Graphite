//! キーと内部位置の対応表 — 正引きと逆引きが互いの逆であることを所有する。
//!
//! 正引き (キー → 位置) と逆引き (位置 → キー) を同時に更新する経路を
//! [`キー対応表::対応を登録する`] 1つに閉じることで、両者が食い違う状態を作れなくする。

use std::collections::HashMap;
use std::hash::Hash;

use super::topology::ノード位置;

/// ユーザーキー `K` と内部位置の相互対応。
#[derive(Debug)]
pub(in crate::graph) struct キー対応表<K> {
    正引き: HashMap<K, ノード位置>,
    逆引き: HashMap<ノード位置, K>,
}

impl<K> キー対応表<K> {
    pub(in crate::graph) fn 空の対応表を生成する() -> Self {
        Self {
            正引き: HashMap::new(),
            逆引き: HashMap::new(),
        }
    }
}

impl<K> キー対応表<K>
where
    K: Hash + Eq + Clone,
{
    /// 正引きと逆引きの両方へ同時に書き込む、この表の唯一の更新経路。
    pub(in crate::graph) fn 対応を登録する(&mut self, キー: K, 位置: ノード位置) {
        self.逆引き.insert(位置, キー.clone());
        self.正引き.insert(キー, 位置);
    }

    pub(in crate::graph) fn キーが登録済みか(&self, キー: &K) -> bool {
        self.正引き.contains_key(キー)
    }

    pub(in crate::graph) fn 位置(&self, キー: &K) -> Option<ノード位置> {
        self.正引き.get(キー).copied()
    }

    /// 位置に対応するキー。位置は同じグラフのトポロジー由来であることが前提で、
    /// 対応が無ければ呼び出し側の取り違えなので添字と同じく `panic!` する。
    pub(in crate::graph) fn キー(&self, 位置: ノード位置) -> &K {
        &self.逆引き[&位置]
    }

    /// 全キー (順序は未規定)。
    pub(in crate::graph) fn キーの一覧(&self) -> impl Iterator<Item = &K> {
        self.正引き.keys()
    }

    /// 全キーと対応する位置の対 (順序は未規定)。
    pub(in crate::graph) fn キーと位置の一覧(
        &self,
    ) -> impl Iterator<Item = (&K, ノード位置)> {
        self.正引き.iter().map(|(キー, &位置)| (キー, 位置))
    }
}
