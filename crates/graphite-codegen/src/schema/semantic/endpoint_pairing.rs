//! 端点対索引のキーの形と、同じ対に持てる辺の本数を確定して持つ。

/// 同じ端点対に何本の辺を張れるか。`where unique pair` の有無で決まる。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum 端点対の重複可否 {
    対ごとに1本だけ許す,
    対ごとに何本でも許す,
}

impl 端点対の重複可否 {
    pub(super) fn unique_pair指定から作る(unique_pair指定がある: bool) -> Self {
        if unique_pair指定がある {
            Self::対ごとに1本だけ許す
        } else {
            Self::対ごとに何本でも許す
        }
    }

    pub fn 対ごとに1本だけか(self) -> bool {
        self == Self::対ごとに1本だけ許す
    }
}

/// 端点対索引のキーの形。有向辺は始点と終点の順序に意味があり、無向辺は無い。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum 端点対のキーの形 {
    順序付きの対,
    順序なしの対,
}
