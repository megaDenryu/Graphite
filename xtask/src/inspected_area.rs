//! 検査が1つの単位として数える領域。
//!
//! この型は doc コメントの検査と行数の検査が共有する。どちらの検査も、走査の起点と
//! 報告の表示にこの綴りを使う。

// 領域の同一性はリポジトリルートからの相対の綴りであり、表示にもそのまま使う。
// 裸の綴りを持ち回ると、走査の起点と表示の綴りが呼び出し側ごとにずれる。
pub(crate) struct InspectedArea {
    spelling: String,
}

impl InspectedArea {
    pub(crate) fn at(spelling: &str) -> Self {
        Self {
            spelling: spelling.to_string(),
        }
    }

    pub(crate) fn spelling(&self) -> &str {
        &self.spelling
    }

    // 指定した綴りが、この領域の綴りそのもの、またはその配下にあるか。
    //
    // この関数は、境界を `/` の有無で判定する。`crates/graphite-cli` は
    // `crates/graphite-cli-extra` を含まない (前方一致だけでは区切りを誤る)。
    pub(crate) fn contains_spelling(&self, spelling: &str) -> bool {
        spelling
            .strip_prefix(&self.spelling)
            .is_some_and(|rest| rest.starts_with('/'))
    }
}
