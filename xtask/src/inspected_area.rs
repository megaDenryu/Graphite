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
}
