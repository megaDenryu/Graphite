use std::error::Error;
use std::process::Command;

use graphite_cli::PackageRoot;

// ワークスペースの外に置いた検証用パッケージ。
//
// 外部 crate からの生成経路 (`cargo graphite generate` → `cargo build`) は、
// ワークスペースの `cargo build` にも `cargo test --workspace` にも入らない。
// 機械が触らない経路は壊れても誰も気付かないため、この型が生成物の差分と
// ビルドの成否を検査する。
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

    // 生成物が最新であること、そのままビルドとテストが通ることを確かめる。
    pub fn check(&self) -> Result<(), Box<dyn Error>> {
        println!("検証用パッケージ: {}", self.package.display());
        graphite_cli::verify(self.package.generation_tree())?;
        self.run_cargo("build")?;
        self.run_cargo("test")?;
        Ok(())
    }

    // 検証用パッケージのディレクトリで cargo の1コマンドを実行する。
    //
    // 前提: `CARGO` は cargo が子プロセス向けに設定する。`cargo xtask` 以外の
    // 起動 (実行ファイルの直接起動) では設定されないため、その場合は PATH 上の
    // `cargo` を使う。
    fn run_cargo(&self, subcommand: &str) -> Result<(), Box<dyn Error>> {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let directory = self.package.directory();
        println!("実行: cargo {subcommand} ({})", directory.display());
        let status = Command::new(cargo)
            .arg(subcommand)
            .current_dir(directory)
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "検証用パッケージの `cargo {subcommand}` が失敗しました: {}",
                directory.display()
            )
            .into())
        }
    }
}
