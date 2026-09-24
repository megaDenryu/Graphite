// 意味カードの箇条書き1行 (`- 見出し: 値` の形)。design_principles.md の
// 意味カード書式 (issue #41 §5.2) が要求する「意味の箇条」を組み立てる。

use std::fmt::Display;

pub(crate) struct 意味項目 {
    テキスト: String,
}

impl 意味項目 {
    pub(crate) fn new(見出し: &str, 値: impl Display) -> Self {
        Self { テキスト: format!("{見出し}: `{値}`") }
    }

    // 値の後ろへ丸括弧付きの補足を添える版 (「検証制約」項目のような、
    // 制約が検証済みであることの注記に使う)。
    pub(crate) fn 補足付き(見出し: &str, 値: impl Display, 補足: &str) -> Self {
        Self { テキスト: format!("{見出し}: `{値}` ({補足})") }
    }

    // 値をこちらで包まず、呼び出し側が組み立て済みのテキストをそのまま
    // 使う版。列挙する名前のそれぞれを個別に `` で囲みたい場合
    // (「instance 宣言の右辺式から作る個体: `太郎`・`次郎`・`一郎`」のよう
    // に、列挙全体を1つの `` で囲まない場合) に使う。
    pub(crate) fn 組み立て済みの値で(見出し: &str, 組み立て済みの値: &str) -> Self {
        Self { テキスト: format!("{見出し}: {組み立て済みの値}") }
    }

    pub(crate) fn テキスト(&self) -> &str {
        &self.テキスト
    }
}
