use std::error::Error;
use std::fs;

use crate::document_reference::{DocumentReference, ReferenceOrigin, ReferenceTarget};
use crate::io_context::with_path_context;
use crate::repository_root::RepositoryRoot;

/// 走査対象の全ファイルから抜き出した文書参照の一覧。
///
/// 実在の判定にリポジトリルートが要るため、走査時に受け取ったルートを保持する。
pub struct ReferenceScan<'root> {
    root: &'root RepositoryRoot,
    references: Vec<DocumentReference>,
    external_reference_count: usize,
}

impl<'root> ReferenceScan<'root> {
    /// 走査対象ファイルを1件ずつ読み、書かれている文書参照を抜き出す。
    pub fn over(root: &'root RepositoryRoot) -> Result<Self, Box<dyn Error>> {
        let mut references = Vec::new();
        let mut external_reference_count = 0;
        for path in root.document_reference_sources()? {
            let origin_file = root.relative_display(&path);
            let text = with_path_context(fs::read_to_string(&path), &origin_file)?;
            for (offset, line) in text.lines().enumerate() {
                for token in tokens_in(line) {
                    match ReferenceTarget::classify(token) {
                        Some(ReferenceTarget::RepositoryDocument(target)) => {
                            let origin = ReferenceOrigin::new(origin_file.clone(), offset + 1);
                            references.push(DocumentReference::new(origin, target));
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
            external_reference_count,
        })
    }

    pub fn reference_count(&self) -> usize {
        self.references.len()
    }

    pub fn external_reference_count(&self) -> usize {
        self.external_reference_count
    }

    /// 実在しない綴りを指している参照を全件返す。
    ///
    /// 1件目で打ち切らないのは、綴りの是正を1周で終えられるようにするためである。
    pub fn missing_targets(&self) -> Vec<&DocumentReference> {
        self.references
            .iter()
            .filter(|reference| !self.root.document_exists(reference.target()))
            .collect()
    }
}

/// 1行から、バッククォートで囲まれた区間と `](...)` の中身を抜き出す。
///
/// 注意: 抜き出すのは区間の全体であり、`docs/` を部分文字列として切り出さない。
/// 切り出すと、別リポジトリを正しく指した綴りと、先頭が欠けたまま自リポジトリを
/// 指してしまう綴りを区別できない。
fn tokens_in(line: &str) -> Vec<&str> {
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
    use super::tokens_in;

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
