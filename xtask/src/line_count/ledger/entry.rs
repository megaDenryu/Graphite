//! 台帳の1件が持つ、区分と根拠。
//!
//! 区分の綴りの読み書きと、冒頭コメントが書くべき定型の文の組み立てをこの1箇所へ集める。
//! 台帳の読み取り (`ledger.rs`) が綴りを持つと、区分を足すたびに綴りが2箇所へ散る。

// 台帳が定める、超過を許す区分。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ExceptionCategory {
    Consolidated,
    DeclarativeData,
    AwaitingRedesign,
}

impl ExceptionCategory {
    fn from_cell(text: &str) -> Option<Self> {
        match text {
            "統合による超過" => Some(Self::Consolidated),
            "宣言的データリテラル" => Some(Self::DeclarativeData),
            "再設計待ち" => Some(Self::AwaitingRedesign),
            _ => None,
        }
    }

    fn cell_text(self) -> &'static str {
        match self {
            Self::Consolidated => "統合による超過",
            Self::DeclarativeData => "宣言的データリテラル",
            Self::AwaitingRedesign => "再設計待ち",
        }
    }

    // 宣言的データリテラルだけは150行の上限を適用しない (規約の別枠)。
    pub(crate) fn applies_upper_limit(self) -> bool {
        !matches!(self, Self::DeclarativeData)
    }

    pub(crate) fn awaits_redesign(self) -> bool {
        matches!(self, Self::AwaitingRedesign)
    }
}

// 台帳の1件。区分と根拠を持ち、冒頭コメントが書くべき定型の文を組み立てる。
pub(crate) struct LedgerEntry {
    category: ExceptionCategory,
    rationale: String,
}

impl LedgerEntry {
    // 区分の枠と根拠の枠から1件を組み立てる。区分の綴りが3語のどれでもなければ組み立てない。
    pub(super) fn from_cells(category_cell: &str, rationale: &str) -> Option<Self> {
        Some(Self {
            category: ExceptionCategory::from_cell(category_cell)?,
            rationale: rationale.to_string(),
        })
    }

    pub(crate) fn category(&self) -> ExceptionCategory {
        self.category
    }

    // 規約が冒頭コメントへ要求する定型の文。台帳の区分と根拠から組み立てる。
    pub(crate) fn declaration_sentences(&self) -> String {
        format!(
            "このファイルは1ファイル100行の原則の例外である (区分: {})。{}。             超過を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。",
            self.category.cell_text(),
            self.rationale
        )
    }
}
