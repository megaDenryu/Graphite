//! コマンドライン引数の解釈と、使い方の表示。

const DEFAULT_SEED: u64 = 42;

pub struct Options {
    pub seed: u64,
    pub inject_anomalies: bool,
    /// フラグ以外の残り引数 (サブコマンドの位置引数)。
    pub positional: Vec<String>,
}

pub fn parse_options(args: &[String]) -> Result<Options, String> {
    let mut seed = DEFAULT_SEED;
    let mut inject_anomalies = false;
    let mut positional = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| "--seed の後に数値を指定してください".to_string())?;
                seed = value
                    .parse::<u64>()
                    .map_err(|_| format!("--seed の値が数値ではありません: {value}"))?;
                i += 2;
            }
            "--inject-anomalies" => {
                inject_anomalies = true;
                i += 1;
            }
            other => {
                positional.push(other.to_string());
                i += 1;
            }
        }
    }

    Ok(Options {
        seed,
        inject_anomalies,
        positional,
    })
}

pub fn print_usage() {
    eprintln!(
        "使い方:\n\
         \x20 org-analyzer summary   [--seed N] [--inject-anomalies]\n\
         \x20 org-analyzer chain <社員キー>      [--seed N] [--inject-anomalies]\n\
         \x20 org-analyzer anomalies [--seed N] [--inject-anomalies]\n\
         \x20 org-analyzer reorg <部署キー>      [--seed N] [--inject-anomalies]\n\
         \n\
         社員キーの例: E001..E120 / 部署キーの例: D01..D08\n\
         (実際に生成されたキーは `summary` の出力で確認できます)"
    );
}
