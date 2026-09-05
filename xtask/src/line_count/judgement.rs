//! 数え上げた1ファイルと台帳の区分から下す判定。
//!
//! 判定の規則をこの1箇所へ集める。集計と表示 (`report.rs`) が規則を持つと、
//! 報告の見出しを足すたびに規則が枝分かれする。

use super::code_line_count::CodeLineCount;
use super::ledger::ExceptionCategory;

// ファイル1件の判定。違反は種類ごとに分かれ、報告の見出しと1対1に対応する。
pub(crate) enum FileJudgement {
    Acceptable,
    Unregistered,
    Shrunk,
    AwaitingRedesign,
    UpperLimitExceeded,
}

impl FileJudgement {
    pub(crate) fn of(count: &CodeLineCount, category: Option<ExceptionCategory>) -> Self {
        let Some(category) = category else {
            if count.exceeds_principle() {
                return Self::Unregistered;
            }
            return Self::Acceptable;
        };
        if !count.exceeds_principle() {
            return Self::Shrunk;
        }
        if category.awaits_redesign() {
            return Self::AwaitingRedesign;
        }
        if category.applies_upper_limit() && count.exceeds_upper_limit() {
            return Self::UpperLimitExceeded;
        }
        Self::Acceptable
    }
}

#[cfg(test)]
mod tests {
    use super::super::code_line_count::CodeLineCount;
    use super::super::ledger::ExceptionCategory;
    use super::FileJudgement;

    fn 行数(count: usize) -> CodeLineCount {
        CodeLineCount::of_text(&"let x = 1;\n".repeat(count))
    }

    #[test]
    fn 台帳に無い超過は未登録として判定する() {
        let judgement = FileJudgement::of(&行数(101), None);
        assert!(matches!(judgement, FileJudgement::Unregistered));
    }

    #[test]
    fn 台帳にある100行以内は削除を求める判定になる() {
        let judgement = FileJudgement::of(&行数(100), Some(ExceptionCategory::Consolidated));
        assert!(matches!(judgement, FileJudgement::Shrunk));
    }

    #[test]
    fn 宣言的データリテラルには150行の上限を適用しない() {
        let judgement = FileJudgement::of(&行数(200), Some(ExceptionCategory::DeclarativeData));
        assert!(matches!(judgement, FileJudgement::Acceptable));
    }

    #[test]
    fn 統合による超過が150行を超えると違反になる() {
        let judgement = FileJudgement::of(&行数(151), Some(ExceptionCategory::Consolidated));
        assert!(matches!(judgement, FileJudgement::UpperLimitExceeded));
    }
}
