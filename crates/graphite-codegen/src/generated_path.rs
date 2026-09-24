//! `generated = "..."` に書ける相対パスの形式検査。
//!
//! `dynamic_graph_schema!` (コンパイル時) と `xtask` (生成時、`generated_target`
//! 経由でこの関数を呼ぶ) の両方が同じ形式を守らせる必要があるため、判定は
//! この純粋層に1箇所だけ置く。

use std::ffi::OsStr;
use std::path::{Component, Path};

use crate::fingerprint::fnv1a;

// instanceの `generated = "..."` 文字列を保持する役割の型。
//
// 静的グラフのinstance展開 (`static_graph::inline::value_supply`) とCLIの
// 生成 (`static_graph::file::instance_file::construct`) は、この文字列
// だけから値マクロの名前に混ぜる印を独立に計算する
// (`instance印を計算する`が両者で同じ値になることを保証する)。この印が
// instance全体で実際に一意であることまではこの型単体では保証しない。
// 同じCargo targetの中で同じグラフ名・同じ`generated`文字列を持つ
// instanceが無いことは、パッケージ全体を走査する生成器 (`graphite-cli`の
// `static_resolution::instance_duplication`) が生成時に拒否して保証する
// (`docs/static_graph.md`「制約」節)。
#[derive(Clone, Copy)]
pub(crate) struct 生成先パス<'a>(&'a str);

impl<'a> 生成先パス<'a> {
    pub(crate) fn new(value: &'a str) -> Self {
        Self(value)
    }

    // instanceの値マクロ名 (`__graphite_values_{グラフ名}_{印}!`等) に混ぜる
    // 短い印を計算する。この文字列だけから決まるため、instance展開とCLIの
    // 生成が独立に呼んでも同じ値になる (`naming::internal_names`参照)。
    pub(crate) fn instance印を計算する(&self) -> String {
        format!("{:016x}", fnv1a(self.0.as_bytes(), 0xcbf29ce484222325))
    }
}

// 生成先の相対パスが `generated/<名前>.rs` の形式を満たすかを検査する。
//
// 満たさなければ、そのまま利用者へ見せてよい理由の文を返す。絶対パスや
// `..` を許すと、宣言元ディレクトリの外へ書き込めてしまう。
pub fn validate_generated_relative_path(value: &str) -> Result<(), String> {
    let relative = Path::new(value);
    let mut components = relative.components();
    let starts_with_generated_directory = matches!(
        components.next(),
        Some(Component::Normal(first)) if first == OsStr::new("generated")
    );
    let remaining_components_are_plain_names =
        components.all(|component| matches!(component, Component::Normal(_)));
    let has_rs_extension = relative.extension() == Some(OsStr::new("rs"));

    if starts_with_generated_directory && remaining_components_are_plain_names && has_rs_extension {
        Ok(())
    } else {
        Err(format!(
            "生成先は `generated/<名前>.rs` の形式で指定してください (実際: `{value}`)"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正しい相対パスを受理する() {
        assert!(validate_generated_relative_path("generated/world.rs").is_ok());
        assert!(validate_generated_relative_path("generated/nested/world.rs").is_ok());
    }

    #[test]
    fn 上位ディレクトリへの脱出を拒否する() {
        assert!(validate_generated_relative_path("../evil.rs").is_err());
        assert!(validate_generated_relative_path("generated/../evil.rs").is_err());
    }

    #[test]
    fn generatedディレクトリ配下でなければ拒否する() {
        assert!(validate_generated_relative_path("world.rs").is_err());
        assert!(validate_generated_relative_path("src/world.rs").is_err());
    }

    #[test]
    fn 拡張子がrsでなければ拒否する() {
        assert!(validate_generated_relative_path("generated/world.txt").is_err());
    }
}
