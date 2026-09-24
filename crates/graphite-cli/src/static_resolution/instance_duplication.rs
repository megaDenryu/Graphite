//! 同じCargo targetの中で、同じグラフ名・同じ`generated`文字列を持つ
//! instanceの重複を検出する。
//!
//! instanceの値マクロ・`construct!`が内部で呼ぶ値マクロの名前
//! (`__graphite_values_{グラフ名}_{instance印}!`等) は、`generated = "..."`
//! 文字列だけから計算する`instance印`(`graphite_codegen`の
//! `naming::internal_names`) とグラフ名の組で決まる。パッケージ内の2つの
//! instanceが同じグラフ名で同じ`generated`文字列を選ぶと、この名前が完全に
//! 一致し、`construct!`を呼び出し位置のテキスト順スコープにある別instance
//! (同じ名前を持つ側) が無言で使われてしまう (実測: `instance_duplication`の
//! `tests`)。コンパイル時のマクロ (`static_graph_schema!`・`{schema名}!`) は
//! 自分が書かれた宣言元ファイルのパッケージ相対パスをstable Rustから知る
//! 手段を持たないため、この重複はマクロ自身では検出できない。パッケージ
//! 全体を1回ずつ構文解析して回る生成器 (`graphite-cli`) だけが、この一意性
//! を機械的に保証できる立場にある (`docs/static_graph.md`「制約」節)。

use std::collections::BTreeMap;
use std::error::Error;

use graphite_codegen::DeclarationSite;

use crate::cargo_target::CargoTarget;

#[derive(Default)]
pub(crate) struct 静的instance重複検査器 {
    登録済み: BTreeMap<(CargoTarget, String, String), String>,
}

impl 静的instance重複検査器 {
    // 同じtarget・同じグラフ名・同じ`generated`文字列の組が既に登録済み
    // なら、両方の宣言元を示すエラーを返す。初回はサイトの表示を記録して
    // `Ok`を返す。
    pub(crate) fn 検査する(
        &mut self,
        target: &CargoTarget,
        グラフ名: &str,
        generated_path: &str,
        site: &DeclarationSite,
    ) -> Result<(), Box<dyn Error>> {
        let key = (target.clone(), グラフ名.to_string(), generated_path.to_string());
        if let Some(既存site表示) = self.登録済み.get(&key) {
            return Err(format!(
                "静的グラフのinstance `{グラフ名}` はCargo target `{}` の中で `generated` 文字列 `{generated_path}` が重複しています: {既存site表示} と {}。値マクロの名前は `generated` 文字列だけから計算するため、どちらかの `generated` を別の文字列に変えてください",
                target.表示(),
                site.display()
            )
            .into());
        }
        self.登録済み.insert(key, site.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests;
