use std::error::Error;
use std::fs;

use graphite_cli::with_path_context;

use crate::document_reference::{DocumentReference, ReferenceOrigin, ReferenceTarget};
use crate::missing_references::MissingReferences;
use crate::excerpt_inspection::ExcerptInspection;
use crate::quoted_excerpt::{is_rust_fence_start, QuotedExcerpt};
use crate::repository_root::RepositoryRoot;
use crate::source_reference_check::{
    InvalidSourceReferences, SourceCodeReference, UnparsableSourceReference, Violation,
};

// 走査対象の全ファイルから抜き出した文書参照・ソース参照・引用の一覧。
//
// 実在の判定にリポジトリルートが要るため、走査時に受け取ったルートを保持する。
pub struct ReferenceScan<'root> {
    root: &'root RepositoryRoot,
    references: Vec<DocumentReference>,
    source_references: Vec<SourceCodeReference>,
    quoted_excerpts: Vec<QuotedExcerpt>,
    unparsable_source_references: Vec<UnparsableSourceReference>,
    external_reference_count: usize,
    rust_fence_count: usize, // 走査した Markdown にある Rust コードフェンスの総数
}

impl<'root> ReferenceScan<'root> {
    // 走査対象ファイルを1件ずつ読み、書かれている文書参照とソース参照を抜き出す。
    pub fn over(root: &'root RepositoryRoot) -> Result<Self, Box<dyn Error>> {
        let mut references = Vec::new();
        let mut source_references = Vec::new();
        let mut quoted_excerpts = Vec::new();
        let mut unparsable_source_references = Vec::new();
        let mut external_reference_count = 0;
        let mut rust_fence_count = 0;
        for path in root.document_reference_sources()? {
            let origin_file = root.relative_display(&path);
            let text = with_path_context(fs::read_to_string(&path), &origin_file)?;
            // 引用の照合は Markdown のファイルだけを対象にする。コードフェンスは
            // Markdown の構文であり、行頭が `///` で始まる Rust の doc コメントの
            // 中の同じ記法を同じ規則では扱えない。
            let quotable = origin_file.ends_with(".md");
            let lines: Vec<&str> = text.lines().collect();
            let mut fence_tracker = FenceTracker::new();
            for (offset, line) in lines.iter().enumerate() {
                let quotable_line = quotable && fence_tracker.outside_fence(line);
                if quotable && fence_tracker.opened_rust_fence(line) {
                    rust_fence_count += 1;
                }
                for token in tokens_in(line) {
                    let origin = || ReferenceOrigin::new(origin_file.clone(), offset + 1);
                    match ReferenceTarget::classify(token) {
                        Some(ReferenceTarget::RepositoryDocument(target)) => {
                            references.push(DocumentReference::new(origin(), target));
                        }
                        Some(ReferenceTarget::SourceCode(target)) => {
                            if quotable_line {
                                quoted_excerpts.extend(QuotedExcerpt::following_fence(
                                    &lines,
                                    offset,
                                    origin(),
                                    target.clone(),
                                ));
                            }
                            source_references.push(SourceCodeReference::new(origin(), target));
                        }
                        Some(ReferenceTarget::UnparsableSourceCode(token)) => {
                            unparsable_source_references
                                .push(UnparsableSourceReference::new(origin(), token));
                        }
                        Some(ReferenceTarget::ExternalDocument) => external_reference_count += 1,
                        None => {}
                    }
                }
            }
        }
        Ok(Self {
            root,
            references,
            source_references,
            quoted_excerpts,
            unparsable_source_references,
            external_reference_count,
            rust_fence_count,
        })
    }

    pub fn reference_count(&self) -> usize {
        self.references.len()
    }

    pub fn source_reference_count(&self) -> usize {
        self.source_references.len()
    }

    // 走査した Markdown にある Rust コードフェンスのうち、引用として取り込まな
    // かったものの件数。検査が届かなかった範囲を報告に出すために数える。
    pub fn unquoted_rust_fence_count(&self) -> usize {
        self.rust_fence_count - self.quoted_excerpts.len()
    }

    pub fn external_reference_count(&self) -> usize {
        self.external_reference_count
    }

    // 実在しない綴りを指している参照を全件返す。
    //
    // 1件目で打ち切らないのは、綴りの是正を1周で終えられるようにするためである。
    pub fn missing_targets(&self) -> MissingReferences<'_> {
        let references = self
            .references
            .iter()
            .filter(|reference| !self.root.document_exists(reference.target()))
            .collect();
        MissingReferences::new(references)
    }

    // 引用全件へ2つの判定 (行範囲の妥当性と引用の鮮度) を掛けた結果を返す。
    pub fn inspect_excerpts(&self) -> ExcerptInspection<'_> {
        ExcerptInspection::over(&self.quoted_excerpts, self.root)
    }

    // 実在しないか行数を超えるか解析できないソース参照を全件返す。
    pub fn invalid_source_references(&self) -> InvalidSourceReferences<'_> {
        let mut violations: Vec<Violation<'_>> = self
            .source_references
            .iter()
            .filter_map(|reference| reference.evaluate(self.root))
            .collect();
        violations.extend(self.unparsable_source_references.iter().map(Violation::Unparsable));
        InvalidSourceReferences::new(violations)
    }
}

// 文書の行を先頭から順に読み、その行がコードフェンスの中にあるかを追う。
//
// フェンスの中に書かれた行番号付きソース参照へ引用の照合を適用すると、
// フェンスを閉じるバッククォート3つをこの検査が引用の開始と読み違え、その
// 後ろの散文をフェンス本文として照合してしまう。閉じないフェンスがある場合も
// 同じ形で誤る。行をまたいで開閉を覚える必要があるため、状態を持つ型にする。
struct FenceTracker {
    inside: bool,
}

impl FenceTracker {
    fn new() -> Self {
        Self { inside: false }
    }

    // 直前に読んだ1行が Rust コードフェンスの開始行だったか。
    //
    // 開始と終了はどちらも同じ記号で書かれるため、`outside_fence` が覚えた開閉の
    // 状態と合わせて初めて開始行だと分かる。呼ぶのは `outside_fence` の直後である。
    fn opened_rust_fence(&self, line: &str) -> bool {
        self.inside && is_rust_fence_start(line)
    }

    // 次の1行を読み、その行が引用の照合を適用してよい位置 (フェンスの外) に
    // あるかを返す。フェンスの開始行と終了行はどちらも外とみなさない。
    fn outside_fence(&mut self, line: &str) -> bool {
        let delimiter = line.trim_start().starts_with("```");
        if delimiter {
            self.inside = !self.inside;
        }
        !self.inside && !delimiter
    }
}

// 1行から、バッククォートで囲まれた区間と `](...)` の中身を抜き出す。
//
// 注意: 抜き出すのは区間の全体であり、`docs/` を部分文字列として切り出さない。
// 切り出すと、別リポジトリを正しく指した綴りと、先頭が欠けたまま自リポジトリを
// 指してしまう綴りを区別できない。
pub(crate) fn tokens_in(line: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = line.split('`').skip(1).step_by(2).collect();
    let mut rest = line;
    while let Some(start) = rest.find("](") {
        let after = &rest[start + 2..];
        let Some(end) = after.find(')') else {
            break;
        };
        tokens.push(&after[..end]);
        rest = &after[end..];
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::{tokens_in, FenceTracker};

    // 各行を順に読ませ、引用の照合を適用してよい行だけを真にした一覧を得る。
    fn 行ごとに照合を適用してよいかを判定する(lines: &[&str]) -> Vec<bool> {
        let mut tracker = FenceTracker::new();
        lines.iter().map(|line| tracker.outside_fence(line)).collect()
    }

    #[test]
    fn コードフェンスの中に書かれた参照は照合の対象にならない() {
        let lines = ["散文", "```rust", "`xtask/src/lib.rs:1-5`", "```", "続く散文"];
        assert_eq!(行ごとに照合を適用してよいかを判定する(&lines), vec![true, false, false, false, true]);
    }

    #[test]
    fn 閉じないフェンスの後ろの散文も照合の対象にならない() {
        let lines = ["```rust", "`xtask/src/lib.rs:1-5`", "後ろの散文"];
        assert_eq!(行ごとに照合を適用してよいかを判定する(&lines), vec![false, false, false]);
    }

    #[test]
    fn バッククォート区間とマークダウンリンクの両方を抜き出す() {
        let line = "正本は `docs/schema_v4.md` と [脱糖](docs/desugaring_reference.md) である";
        assert_eq!(
            tokens_in(line),
            vec!["docs/schema_v4.md", "docs/desugaring_reference.md"]
        );
    }

    #[test]
    fn 区間の外側の語はトークンに含めない() {
        let line = "/// (`docs/graph_splice.md` §2)。実行時データからの構築";
        assert_eq!(tokens_in(line), vec!["docs/graph_splice.md"]);
    }
}
