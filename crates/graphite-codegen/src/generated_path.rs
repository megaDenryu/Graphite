//! `generated = "..."` に書ける相対パスの形式検査。
//!
//! `graph_schema!` (コンパイル時) と `xtask` (生成時、`generated_target`
//! 経由でこの関数を呼ぶ) の両方が同じ形式を守らせる必要があるため、判定は
//! この純粋層に1箇所だけ置く。

use std::ffi::OsStr;
use std::path::{Component, Path};

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
