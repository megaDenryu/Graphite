//! `static_schema!` の外で宣言する実体型 (issue #24)。ノード型・積み荷型は
//! 普通の struct であり、`static_schema!` はこれらを直接参照するだけで
//! 生成しない (`graph_schema!` と同じ方針)。

pub(crate) trait 名前持ち {
    fn 名前(&self) -> &str;
}

pub(crate) struct 社員 {
    pub(crate) 名前: String,
}
impl 名前持ち for 社員 {
    fn 名前(&self) -> &str {
        &self.名前
    }
}

pub(crate) struct 部署 {
    pub(crate) 名前: String,
}
impl 名前持ち for 部署 {
    fn 名前(&self) -> &str {
        &self.名前
    }
}

pub(crate) struct 任命記録 {
    pub(crate) 任命日: i32,
}

pub(crate) struct 経緯記録 {
    pub(crate) 経緯: String,
}

// `node 名前: 型 = 式;` (型注釈+任意式) が構造体リテラルに限らない任意の式を
// 受理することを示すための、構造体リテラルではない式 (関数呼び出し)。
pub(crate) fn 社員を作る(名前: &str) -> 社員 {
    社員 { 名前: 名前.into() }
}
