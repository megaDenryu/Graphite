use super::*;
use proc_macro2::Span;

fn 型名(名前: &str) -> Ident {
    Ident::new(名前, Span::call_site())
}

fn パスの綴り(パス: &Path) -> String {
    パス
        .segments
        .iter()
        .map(|区切り| 区切り.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn 明示パス(綴り: &str) -> Option<Path> {
    Some(syn::parse_str(綴り).expect("テスト用のパスは構文解析を通る"))
}

#[test]
fn 明示指定がなければスキーマがid型を生成する() {
    let 型 = 公開ID型::宣言から作る(型名("PersonId"), None);
    assert!(型.スキーマが生成するid型か());
    assert!(型.デバッグ表示に使えるか());
    assert_eq!(型.スキーマが生成する型名().unwrap().to_string(), "PersonId");
}

#[test]
fn selfから始まる明示idパスは構築時にsuperへ読み替える() {
    let 型 = 公開ID型::宣言から作る(型名("PersonId"), 明示パス("self::既存のID"));
    assert!(!型.スキーマが生成するid型か());
    assert!(!型.デバッグ表示に使えるか());
    let 公開ID型::利用者が宣言した既存のID型 {
        生成module内から見たパス,
    } = &型
    else {
        panic!("明示指定は既存のID型になる");
    };
    assert_eq!(パスの綴り(生成module内から見たパス), "super::既存のID");
}

#[test]
fn crateから始まる明示idパスはそのまま使う() {
    let 型 =
        公開ID型::宣言から作る(型名("PersonId"), 明示パス("crate::ids::PersonId"));
    let 公開ID型::利用者が宣言した既存のID型 {
        生成module内から見たパス,
    } = &型
    else {
        panic!("明示指定は既存のID型になる");
    };
    assert_eq!(パスの綴り(生成module内から見たパス), "crate::ids::PersonId");
}

#[test]
fn 修飾のない明示idパスはそのまま使う() {
    let 型 = 公開ID型::宣言から作る(型名("PersonId"), 明示パス("既存のID"));
    let 公開ID型::利用者が宣言した既存のID型 {
        生成module内から見たパス,
    } = &型
    else {
        panic!("明示指定は既存のID型になる");
    };
    assert_eq!(パスの綴り(生成module内から見たパス), "既存のID");
}
