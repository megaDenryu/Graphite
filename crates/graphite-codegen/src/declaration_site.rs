//! 生成ファイルのヘッダへ埋め込む「元DSLの宣言位置」。

/// `graph_schema!` 宣言の位置 (パッケージ相対パスと行番号)。基準は生成の入口が
/// 渡す基準ディレクトリであり、どちらの入口もパッケージルートを渡す。
///
/// パスと行番号を別々の引数として運ぶと、呼び出し側が2つの生値を対応付けて
/// 渡す責務を負う。宣言位置という1つの概念を1つの型にまとめることで、
/// 生成ファイルのヘッダに書く表示形式もこの型のメソッドへ閉じる。
pub struct DeclarationSite {
    source_path: String,
    line: usize,
}

impl DeclarationSite {
    /// 宣言元ファイルのパッケージ相対パスと、`graph_schema!` 呼び出しの行番号から作る。
    pub fn new(source_path: String, line: usize) -> Self {
        Self { source_path, line }
    }

    /// 生成ファイルのヘッダへ書く「パス:行番号」形式の表示。
    pub fn display(&self) -> String {
        format!("{}:{}", self.source_path, self.line)
    }
}
