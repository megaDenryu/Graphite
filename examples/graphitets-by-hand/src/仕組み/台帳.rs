// 実体プール2つへの参照の束。全参照がこれを持ち歩く。
pub struct 台帳<'a, ノード達型, 辺達型> {
    pub ノード達: &'a ノード達型,
    pub 辺達: &'a 辺達型,
}
impl<'a, ノード達型, 辺達型> Clone for 台帳<'a, ノード達型, 辺達型> {
    fn clone(&self) -> Self { *self }
}
impl<'a, ノード達型, 辺達型> Copy for 台帳<'a, ノード達型, 辺達型> {}
