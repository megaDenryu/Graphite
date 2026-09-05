//! 利用者が書いた型パスを、宣言の形へ書くための綴りへ写す。

use syn::Path;

// 型パスを `::` 区切りの綴りへ写す。
//
// 注意: 型引数 (`Foo<Bar>`) は綴りに含めない。宣言の形は生成物の doc から
// 宣言元を指すための表示であり、再び構文解析される用途を持たないため、
// パスの要素の並びだけで足りる。
pub(super) fn 型パスの綴りを組み立てる(パス: &Path) -> String {
    let 先頭のコロン = if パス.leading_colon.is_some() {
        "::"
    } else {
        ""
    };
    let 要素の並び = パス
        .segments
        .iter()
        .map(|要素| 要素.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");
    format!("{先頭のコロン}{要素の並び}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn パス(綴り: &str) -> Path {
        syn::parse_str(綴り).expect("テスト用のパスは構文解析を通る")
    }

    #[test]
    fn 修飾のないパスはそのままの綴りになる() {
        assert_eq!(型パスの綴りを組み立てる(&パス("ExternalNodeId")), "ExternalNodeId");
    }

    #[test]
    fn 多段のパスはコロン2つで繋いだ綴りになる() {
        assert_eq!(型パスの綴りを組み立てる(&パス("super::既存のID")), "super::既存のID");
    }

    #[test]
    fn 先頭のコロン2つを保った綴りになる() {
        assert_eq!(型パスの綴りを組み立てる(&パス("::std::string::String")), "::std::string::String");
    }
}
