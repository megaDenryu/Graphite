use std::error::Error;
use std::process::Command;

use graphite_cli::PackageRoot;

// ワークスペースの外に置いた検証用パッケージ。
//
// 外部 crate からの生成経路 (`cargo graphite generate` → `cargo build`) は、
// ワークスペースの `cargo build` にも `cargo test --workspace` にも入らない。
// 機械が触らない経路は壊れても誰も気付かないため、この型が生成物の差分と
// ビルド・clippy・テストの成否を検査する。clippy は利用側と同じ `-D warnings` で
// 走らせる。検証用パッケージは `Cargo.toml` で `clippy::expect_used`・
// `clippy::unwrap_used` を deny にしており、生成物がこの2つを使わないことも
// ここで固定される (issue #48)。
//
// 検査は `cargo graphite generate --check` と同じ経路を通す。つまり走査開始点を
// `PackageRoot` が決め、生成計画と差分の判定は `graphite-cli` が行う。
pub struct ExternalVerificationPackage {
    package: PackageRoot,
}

impl ExternalVerificationPackage {
    pub fn new(package: PackageRoot) -> Self {
        Self { package }
    }

    // 生成物が最新であること、そのままビルドと clippy とテストが通ることを確かめる。
    pub fn check(&self) -> Result<(), Box<dyn Error>> {
        println!("検証用パッケージ: {}", self.package.display());
        graphite_cli::verify(self.package.generation_tree())?;
        self.run_cargo(&["build"])?;
        self.run_cargo(&["clippy", "--all-targets", "--", "-D", "warnings"])?;
        self.run_cargo(&["test"])?;
        Ok(())
    }

    // 検証用パッケージのディレクトリで cargo の1コマンドを実行する。
    //
    // 前提: `CARGO` は cargo が子プロセス向けに設定する。`cargo xtask` 以外の
    // 起動 (実行ファイルの直接起動) では設定されないため、その場合は PATH 上の
    // `cargo` を使う。
    fn run_cargo(&self, arguments: &[&str]) -> Result<(), Box<dyn Error>> {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let directory = self.package.directory();
        let command_line = arguments.join(" ");
        println!("実行: cargo {command_line} ({})", directory.display());
        let status = Command::new(cargo)
            .args(arguments)
            .current_dir(directory)
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "検証用パッケージの `cargo {command_line}` が失敗しました: {}",
                directory.display()
            )
            .into())
        }
    }
}
