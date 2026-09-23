//! `inline/` の回帰試験。DSLトークンの型参照・値の供給関数が、instanceの
//! トークンの実際のspan (行・桁) を保つことを固定する (issue #41)。
//! `quote!` はリテラルトークンにcall-site既定span
//! (proc-macro2のフォールバック実装では行1桁0に固定される) しか付けない
//! ため、`crate::static_graph::naming::tests` 等の既存フィクスチャ
//! (`quote!` 組み立て) では行・桁を検証できない。ここでは `syn::parse_str`
//! で実際に複数行のソース文字列を構文解析し、本物の行・桁を持つトークンで
//! 検証する。

mod span_regression;
