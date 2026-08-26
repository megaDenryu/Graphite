//! `KeyedTable<K, V>` は、ノード表・辺表で共有するランタイム機構を提供する。
//!
//! `docs/schema_v4.md` §0/§3.1 の決定「基盤は多重グラフであり、辺もノードと
//! 同様にキーによる同一性を持つ」により、`graph_schema!` が生成するノード表
//! (`{Node}Id -> {Node}`) と辺表 (`{Kind}Id -> {Kind}`) はどちらも「型付き
//! ID → 値」の単純な写像であり、走査・検索の語彙
//! (`get`/`ids`/`iter`/`len`/`is_empty`) を共有する。
//!
//! rustdoc はここに集約する (`.claude/skills/proc-macro-dev/SKILL.md` の
//! 方針通り、生成コード自体には多重度・属性の有無以上のドキュメントを書かない)。

use std::collections::HashMap;
use std::hash::Hash;

/// キー付き要素表。内部は「挿入順の本体 `Vec<(K, V)>`」+「キー→添字の
/// `HashMap<K, usize>`」の組。
///
/// **順序保証 (仕様):** [`Self::ids`]/[`Self::iter`] は挿入順 (`insert` を
/// 呼んだ順) で走査する。`graph_schema!` が生成する `{Kind}::of`/`iter` 等は
/// この保証の上に「格納順を保持する」と約束している。
/// `get`/`contains_key` は `HashMap<K, usize>` 経由の O(1) のまま。
///
/// `graph_schema!` の生成コードが使う想定であり、利用者がこれを直接構築する
/// ことは想定しない (schema struct の生成フィールド型として使われる)。
#[derive(Debug, Clone)]
pub struct KeyedTable<K, V> {
    /// 挿入順の本体。走査 (`ids`/`iter`) はここを順に辿るだけで順序保証を
    /// 満たす。
    entries: Vec<(K, V)>,
    /// キー → `entries` の添字。`get`/`contains_key` の O(1) 化用。
    index: HashMap<K, usize>,
}

impl<K, V> KeyedTable<K, V> {
    /// 空の表を作る。
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: HashMap::new(),
        }
    }
}

impl<K, V> Default for KeyedTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> KeyedTable<K, V>
where
    K: Eq + Hash + Clone,
{
    /// `key` が既に存在すれば挿入せず `false` を返す (呼び出し側が重複キー
    /// 違反として扱えるように)。存在しなければ末尾に追加して `true` を返す
    /// (挿入順保証はこの「末尾に追加」によって成り立つ)。
    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self.index.contains_key(&key) {
            return false;
        }
        let idx = self.entries.len();
        self.index.insert(key.clone(), idx);
        self.entries.push((key, value));
        true
    }

    /// キーがこの表に存在するか。
    pub fn contains_key(&self, key: &K) -> bool {
        self.index.contains_key(key)
    }

    /// キーから値を読み出す。
    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = *self.index.get(key)?;
        self.entries.get(idx).map(|(_, v)| v)
    }

    /// キーから挿入順の内部位置を求める。`graph_schema!` の生成コードが
    /// 凍結済みグラフの薄い参照値を構築するために使う。
    #[doc(hidden)]
    pub fn position(&self, key: &K) -> Option<usize> {
        self.index.get(key).copied()
    }

    /// 内部位置からキーと値を読み出す。内部位置は表の構造を変更しない間だけ
    /// 安定するため、凍結済みグラフの生成コードだけが使う。
    #[doc(hidden)]
    pub fn get_at(&self, position: usize) -> Option<(&K, &V)> {
        self.entries.get(position).map(|(key, value)| (key, value))
    }

    /// キーから値を可変借用する。構造を変更せず値だけを更新する。
    #[doc(hidden)]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let position = *self.index.get(key)?;
        self.entries.get_mut(position).map(|(_, value)| value)
    }

    /// 内部位置を挿入順に列挙する。
    #[doc(hidden)]
    pub fn positions(&self) -> std::ops::Range<usize> {
        0..self.entries.len()
    }

    /// 全キーを走査するイテレータ。挿入順を保持する (仕様、上記構造体
    /// doc 参照)。
    pub fn ids(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(k, _)| k)
    }

    /// 全要素を `(キー, 値)` で走査するイテレータ。挿入順を保持する
    /// (仕様、上記構造体 doc 参照)。
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }

    /// 表に含まれる要素数。
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 要素が1つも無いか。
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
