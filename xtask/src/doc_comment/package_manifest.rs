//! Cargo.toml から読み取る、内部領域判定に使う事実 (issue #32)。
//!
//! この検査器は、内部領域かどうかを人の記憶やハードコードした一覧で決めず、
//! Cargo.toml の設定そのものから機械的に導出する。この検査器は、
//! `publish = false`(空配列 `publish = []` を含む)を持つパッケージと、
//! `[lib] proc-macro = true`(下線表記 `proc_macro` を含む)を持つパッケージを
//! 内部領域として扱う。前者は crates.io の対象レジストリを持たないという
//! 意思の表明であり、後者は proc-macro クレートが入口以外を公開できないという
//! Rust の技術的制約による。

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fs;
use std::path::Path;

use graphite_cli::with_path_context;

// パッケージ1件の Cargo.toml から読み取った、内部領域判定に必要な事実。
pub(super) struct PackageManifestFacts {
    publish: PublishSetting,
    proc_macro: bool,
}

// `[package] publish` が持ちうる形。Cargo はワークスペース継承
// (`publish.workspace = true`) も許すが、この検査器はワークスペース側の値を
// 辿らない。この検査器は、継承と、真偽値・配列のどちらでもない値を判定不能と
// して扱い、エラーで打ち切る。
enum PublishSetting {
    Absent,     // キーが無い。既定は true 相当であり、公開面である。
    True,       // publish = true。公開面である。
    False,      // publish = false。内部領域である。
    EmptyList,  // publish = []。公開先レジストリが無いため内部領域である。
    Registries, // publish = [..](非空)。特定レジストリへは公開するため公開面である。
}

impl PublishSetting {
    fn is_internal(&self) -> bool {
        matches!(self, Self::False | Self::EmptyList)
    }
}

impl PackageManifestFacts {
    // この関数は、指定した Cargo.toml を読み、事実を取り出す。
    //
    // この関数は、読めない・構文解析できない Cargo.toml を内部領域の判定から
    // 黙って外さず、エラーとして呼び出し元へ返す (解析できなかった入力を検査
    // 対象から取りこぼしてはならない)。
    pub(super) fn read(manifest_path: &Path) -> Result<Self, Box<dyn Error>> {
        let text = with_path_context(
            fs::read_to_string(manifest_path),
            &manifest_path.display().to_string(),
        )?;
        let document: toml::Table = text
            .parse()
            .map_err(|error| format!("{} を解析できません: {error}", manifest_path.display()))?;
        Ok(Self {
            publish: read_publish_setting(&document, manifest_path)?,
            proc_macro: read_proc_macro_flag(&document, manifest_path)?,
        })
    }

    // この関数は、publish の設定が内部領域を表すか `[lib] proc-macro = true`
    // を持つ場合に真を返す。
    pub(super) fn is_internal(&self) -> bool {
        self.publish.is_internal() || self.proc_macro
    }
}

// この関数は、`document["package"]["publish"]` を判別共用体として読み取る。
// この関数は、真偽値・配列のどちらでもない値 (ワークスペース継承
// `{ workspace = true }` を含む) をエラーとして返す。
fn read_publish_setting(
    document: &toml::Table,
    manifest_path: &Path,
) -> Result<PublishSetting, Box<dyn Error>> {
    let Some(package) = document.get("package") else {
        return Ok(PublishSetting::Absent);
    };
    match package.get("publish") {
        None => Ok(PublishSetting::Absent),
        Some(toml::Value::Boolean(true)) => Ok(PublishSetting::True),
        Some(toml::Value::Boolean(false)) => Ok(PublishSetting::False),
        Some(toml::Value::Array(items)) if items.is_empty() => Ok(PublishSetting::EmptyList),
        Some(toml::Value::Array(_)) => Ok(PublishSetting::Registries),
        Some(other) => Err(format!(
            "{} の package.publish の値を判定できません(ワークスペース継承 `{{ workspace = true }}` は未対応です): {other:?}",
            manifest_path.display()
        )
        .into()),
    }
}

// この関数は、`document["lib"]["proc-macro"]` を読み取る。Cargo の正式な綴りは
// ハイフンだが、この関数はアンダースコア表記 `proc_macro` も受理する。
fn read_proc_macro_flag(
    document: &toml::Table,
    manifest_path: &Path,
) -> Result<bool, Box<dyn Error>> {
    let Some(lib) = document.get("lib") else {
        return Ok(false);
    };
    match lib.get("proc-macro").or_else(|| lib.get("proc_macro")) {
        None => Ok(false),
        Some(toml::Value::Boolean(flag)) => Ok(*flag),
        Some(other) => Err(format!(
            "{} の lib.proc-macro の値を判定できません: {other:?}",
            manifest_path.display()
        )
        .into()),
    }
}
