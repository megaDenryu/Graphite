//! 生成計画1つの所有と、その計画表への操作。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。こ
//! のファイルは生成計画1つを所有する。このファイルは、その計画表への4つの操作
//!  (追加・古いファイルの書き出し・差分検査・孤児の検出) を統合している。この
//! 4つは同じ計画表の同じ不変条件を触るため、分けると流れの一片になる。超過を
//! 許す根拠の台帳は `docs/development/line_count_ledger.md` にある。

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

use crate::generated_target_path::GeneratedTargetPath;
use crate::io_context::with_path_context;
use crate::generation_tree::GenerationTree;

// 全schema宣言から集めた「生成先とその期待内容」の一覧。
//
// 生成先が重複していれば、書き出す前に検出する。
#[derive(Default)]
pub struct GenerationPlan {
    expected: BTreeMap<GeneratedTargetPath, String>,
}

impl GenerationPlan {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        tree: &GenerationTree,
        target: GeneratedTargetPath,
        content: String,
    ) -> Result<(), Box<dyn Error>> {
        let display = tree.relative_display(target.as_path());
        if self.expected.insert(target, content).is_some() {
            return Err(format!(
                "生成先が重複しています: {display}\n各宣言の generated パスを別の名前にしてください。"
            )
            .into());
        }
        Ok(())
    }

    // 計画に載っている生成先の数。schema宣言1件につき1つである。
    pub fn declaration_count(&self) -> usize {
        self.expected.len()
    }

    // 作業ツリーと異なる生成先を書き換え、書き換えた件数を返す。
    //
    // 書き換えた分はその場で1行ずつ表示する。件数を返すのは、呼び出し側が
    // 「宣言は何件あって、そのうち何件を書いたか」の要約を出すためである。
    pub fn write_stale_files(&self, tree: &GenerationTree) -> Result<usize, Box<dyn Error>> {
        let mut written = 0;
        for (target, content) in self.stale_files() {
            let target = target.as_path();
            if let Some(parent) = target.parent() {
                with_path_context(fs::create_dir_all(parent), &tree.relative_display(parent))?;
            }
            // 生成の途中で落ちても半端な内容を残さないよう、一時ファイルを
            // 作ってから置き換える。`fs::rename` は Windows でも既存の
            // 置き換え先ファイルをアトミックに上書きできるため、事前の
            // `remove_file` は不要であり、削除と作成の間に別プロセスが
            // 割り込む窓を作るだけむしろ危険である。
            let temporary = target.with_extension("rs.graphite-tmp");
            with_path_context(
                fs::write(&temporary, content),
                &tree.relative_display(&temporary),
            )?;
            with_path_context(
                fs::rename(&temporary, target),
                &format!(
                    "{} -> {}",
                    tree.relative_display(&temporary),
                    tree.relative_display(target)
                ),
            )?;
            println!("生成: {}", tree.relative_display(target));
            written += 1;
        }
        Ok(written)
    }

    // 作業ツリーが期待内容と一致するかを検査し、一致しなければエラーにする。
    //
    // 期待に対して古い/存在しないファイルに加えて、`generated/` 配下に
    // 実在するのに期待集合に無いファイル (schema宣言の削除・移動で
    // 取り残された孤児) も検出する。孤児は自動削除せず、一覧をエラーで
    // 報告するだけにとどめる (手編集されていないと機械的には断定できない
    // ため)。
    pub fn verify(&self, tree: &GenerationTree) -> Result<(), Box<dyn Error>> {
        let mut sections = Vec::new();

        let stale = self.stale_files();
        if !stale.is_empty() {
            let paths = stale
                .iter()
                .map(|(target, _)| tree.relative_display(target.as_path()))
                .collect::<Vec<_>>()
                .join("\n");
            sections.push(format!(
                "生成ファイルが古いか存在しません。パッケージのディレクトリで `cargo graphite generate` を実行してください (Graphite リポジトリ自身の開発では `cargo xtask generate`):\n{paths}"
            ));
        }

        let orphans = self.orphan_files(tree)?;
        if !orphans.is_empty() {
            let paths = orphans
                .iter()
                .map(|target| tree.relative_display(target.as_path()))
                .collect::<Vec<_>>()
                .join("\n");
            sections.push(format!(
                "宣言の無い生成ファイルが残っています(schemaの削除・移動で取り残された可能性があります。内容を確認して手動で削除してください):\n{paths}"
            ));
        }

        if sections.is_empty() {
            return Ok(());
        }
        Err(sections.join("\n\n").into())
    }

    fn stale_files(&self) -> Vec<(&GeneratedTargetPath, &String)> {
        self.expected
            .iter()
            .filter(|(target, content)| {
                fs::read_to_string(target.as_path()).ok().as_deref() != Some(content.as_str())
            })
            .collect()
    }

    fn orphan_files(
        &self,
        tree: &GenerationTree,
    ) -> Result<Vec<GeneratedTargetPath>, Box<dyn Error>> {
        Ok(tree
            .existing_generated_files()?
            .into_iter()
            .filter(|path| !self.expected.contains_key(path))
            .collect())
    }
}
