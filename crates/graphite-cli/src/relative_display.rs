use std::path::Path;

// 基準ディレクトリからの相対パスを、環境によらない綴りで表示する。
//
// 生成ファイルのヘッダと診断文に出る綴りはこの1箇所で決める。区切り文字を
// スラッシュへ揃えないと、同じ内容でも Windows と他環境で生成本文が食い違い、
// `generate --check` が環境ごとに落ちる。
//
// 基準の外を指すパスはそのまま表示する。基準を跨いだ絶対パスを黙って相対の
// ように見せると、どこのファイルなのか読み手が決められなくなる。
pub fn relative_display(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::relative_display;
    use std::path::Path;

    #[test]
    fn 基準の配下はスラッシュ区切りの相対綴りになる() {
        assert_eq!(
            relative_display(
                Path::new("/repo"),
                Path::new("/repo/crates/graphite/src/lib.rs")
            ),
            "crates/graphite/src/lib.rs"
        );
    }

    #[test]
    fn 基準の外はそのままの綴りで表示する() {
        assert_eq!(
            relative_display(Path::new("/repo"), Path::new("/other/lib.rs")),
            "/other/lib.rs"
        );
    }
}
