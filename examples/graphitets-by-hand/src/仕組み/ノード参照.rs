use std::marker::PhantomData;

use super::台帳モジュール::台帳;
use super::ノードタグモジュール::ノードタグ;

// ノード参照の形。個体ごとの違いは印 (タグ) だけで、形はこの1本で全員分。
pub struct ノード参照<'a, ノード達型, 辺達型, タグ: ノードタグ> {
    pub 実体: &'a タグ::実体,
    pub 台帳: 台帳<'a, ノード達型, 辺達型>,
    _印: PhantomData<タグ>,
}
impl<'a, ノード達型, 辺達型, タグ: ノードタグ> ノード参照<'a, ノード達型, 辺達型, タグ> {
    pub fn new(実体: &'a タグ::実体, 台帳: 台帳<'a, ノード達型, 辺達型>) -> Self {
        Self { 実体, 台帳, _印: PhantomData }
    }
}
impl<'a, ノード達型, 辺達型, タグ: ノードタグ> Clone for ノード参照<'a, ノード達型, 辺達型, タグ> {
    fn clone(&self) -> Self { *self }
}
impl<'a, ノード達型, 辺達型, タグ: ノードタグ> Copy for ノード参照<'a, ノード達型, 辺達型, タグ> {}
