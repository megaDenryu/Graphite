use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::repository_root::RepositoryRoot;

/// 全schema宣言から集めた「生成先とその期待内容」の一覧。
///
/// 生成先が重複していれば、書き出す前に検出する。
pub struct GenerationPlan {
    expected: BTreeMap<PathBuf, String>,
}

impl GenerationPlan {
    pub fn new() -> Self {
        Self {
            expected: BTreeMap::new(),
        }
    }

    pub fn add(
        &mut self,
        root: &RepositoryRoot,
        target: PathBuf,
        content: String,
    ) -> Result<(), Box<dyn Error>> {
        if self.expected.insert(target.clone(), content).is_some() {
            return Err(
                format!("生成先が重複しています: {}", root.relative_display(&target)).into(),
            );
        }
        Ok(())
    }

    /// 作業ツリーと異なる生成先を書き換え、書き換えた分を表示する。
    pub fn write_stale_files(&self, root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
        for (target, content) in self.stale_files() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            // 生成の途中で落ちても半端な内容を残さないよう、一時ファイルを
            // 作ってから置き換える。
            let temporary = target.with_extension("rs.graphite-tmp");
            fs::write(&temporary, content)?;
            if target.exists() {
                fs::remove_file(target)?;
            }
            fs::rename(&temporary, target)?;
            println!("生成: {}", root.relative_display(target));
        }
        Ok(())
    }

    /// 作業ツリーが期待内容と一致するかを検査し、一致しなければエラーにする。
    pub fn verify(&self, root: &RepositoryRoot) -> Result<(), Box<dyn Error>> {
        let stale = self.stale_files();
        if stale.is_empty() {
            return Ok(());
        }
        let paths = stale
            .iter()
            .map(|(target, _)| root.relative_display(target))
            .collect::<Vec<_>>()
            .join("\n");
        Err(format!(
            "生成ファイルが古いか存在しません。リポジトリルートで `cargo xtask generate` を実行してください:\n{paths}"
        )
        .into())
    }

    fn stale_files(&self) -> Vec<(&PathBuf, &String)> {
        self.expected
            .iter()
            .filter(|(target, content)| {
                fs::read_to_string(target).ok().as_deref() != Some(content.as_str())
            })
            .collect()
    }
}
