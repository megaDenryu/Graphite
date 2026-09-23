// 追跡情報 (issue #41 §5.1)。公開生成物の名前に必ず添える、由来・意味カード
// の材料。段階1では意味カードの本文を組み立てられることを単体試験で確かめ
// るだけで、生成コードのdocへはまだ出さない (`static_graph/naming` 参照)。

use super::{
    declaration_ref::{関係宣言, 宣言への参照},
    origin::名前の由来,
    semantic_item::意味項目,
};
use crate::schema::codegen::宣言元ファイルの綴り;

enum 宣言段落 {
    宣言(宣言への参照),
    固定語彙(String),
}

pub(crate) struct 追跡情報 {
    由来: 名前の由来,
    概要文: String,
    意味項目列: Vec<意味項目>,
    宣言段落: Option<宣言段落>,
    関係宣言: 関係宣言,
}

impl 追跡情報 {
    // 単体試験 (`naming::tests`) がA/B分類 (利用者語彙由来かGraphiteの固定
    // 語彙か) の検査に使うだけで、生成コードのdocへは出さない
    // (`file::doc_render::doc属性を組み立てる` は `意味カード()` だけを読む)。
    #[allow(dead_code)]
    pub(crate) fn 由来(&self) -> &名前の由来 {
        &self.由来
    }

    // issue #41 §5.2 の意味カード本文 (段落を空行区切りで連ねたもの)。
    // `///` は付けない (doc属性へ変換する側の責務、段階3以降)。
    pub(crate) fn 意味カード(&self) -> String {
        let mut 段落達 = vec![self.概要文.clone()];
        if !self.意味項目列.is_empty() {
            let 箇条書き =
                self.意味項目列.iter().map(|項目| format!("- {}", 項目.テキスト())).collect::<Vec<_>>().join("\n");
            段落達.push(箇条書き);
        }
        if let Some(宣言段落) = &self.宣言段落 {
            段落達.push(match 宣言段落 {
                宣言段落::宣言(参照) => 参照.段落(),
                宣言段落::固定語彙(文) => format!("固定語彙: {文}"),
            });
        }
        match &self.関係宣言 {
            関係宣言::無し => {}
            関係宣言::Schema(参照) => 段落達.push(参照.関係schema段落()),
            関係宣言::Instance(参照) => 段落達.push(参照.関係instance段落()),
        }
        段落達.join("\n\n")
    }
}

pub(crate) struct 追跡情報構築器 {
    由来: 名前の由来,
    概要文: String,
    意味項目列: Vec<意味項目>,
    宣言段落: Option<宣言段落>,
    関係宣言: 関係宣言,
}

impl 追跡情報構築器 {
    pub(crate) fn new(由来: 名前の由来, 概要文: impl Into<String>) -> Self {
        Self {
            由来,
            概要文: 概要文.into(),
            意味項目列: Vec::new(),
            宣言段落: None,
            関係宣言: 関係宣言::無し,
        }
    }

    pub(crate) fn 意味項目を足す(mut self, 項目: 意味項目) -> Self {
        self.意味項目列.push(項目);
        self
    }

    pub(crate) fn 宣言を添える(mut self, 宣言元: &宣言元ファイルの綴り, 宣言の形: impl Into<String>) -> Self {
        self.宣言段落 = 宣言元.宣言への参照を試みる(宣言の形).map(宣言段落::宣言);
        self
    }

    pub(crate) fn 固定語彙の宣言を添える(mut self, 文: impl Into<String>) -> Self {
        self.宣言段落 = Some(宣言段落::固定語彙(文.into()));
        self
    }

    pub(crate) fn 関係schema宣言を添える(
        mut self,
        宣言元: &宣言元ファイルの綴り,
        宣言の形: impl Into<String>,
    ) -> Self {
        self.関係宣言 =
            宣言元.宣言への参照を試みる(宣言の形).map(関係宣言::Schema).unwrap_or(関係宣言::無し);
        self
    }

    pub(crate) fn 関係instance宣言を添える(
        mut self,
        宣言元: &宣言元ファイルの綴り,
        宣言の形: impl Into<String>,
    ) -> Self {
        self.関係宣言 =
            宣言元.宣言への参照を試みる(宣言の形).map(関係宣言::Instance).unwrap_or(関係宣言::無し);
        self
    }

    pub(crate) fn 完成する(self) -> 追跡情報 {
        追跡情報 {
            由来: self.由来,
            概要文: self.概要文,
            意味項目列: self.意味項目列,
            宣言段落: self.宣言段落,
            関係宣言: self.関係宣言,
        }
    }
}
