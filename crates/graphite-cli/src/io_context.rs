use std::error::Error;
use std::io;

/// I/O結果へパスの文脈を付けて `Box<dyn Error>` へ変換する。
///
/// `std::io::Error` はどのパスに対する操作で失敗したかを持たない。素通しすると
/// 「ファイルが見つかりません」とだけ表示され、リポジトリのどのファイルかが
/// 分からなくなる。
pub fn with_path_context<T>(result: io::Result<T>, description: &str) -> Result<T, Box<dyn Error>> {
    result.map_err(|error| format!("{description}: {error}").into())
}
