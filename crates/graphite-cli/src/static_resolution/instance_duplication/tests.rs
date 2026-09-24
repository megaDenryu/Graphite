use super::*;

#[test]
fn 同じtarget_同じグラフ名_同じgenerated文字列は重複エラーになる() {
    let target = CargoTarget::new("src");
    let mut 検査器 = 静的instance重複検査器::default();
    let 一つ目 = DeclarationSite::new("src/a.rs".to_string(), 3);
    検査器.検査する(&target, "T", "generated/T.rs", &一つ目).unwrap();

    let 二つ目 = DeclarationSite::new("src/b.rs".to_string(), 7);
    let error = 検査器.検査する(&target, "T", "generated/T.rs", &二つ目).err().unwrap();
    assert!(error.to_string().contains("重複"));
    assert!(error.to_string().contains("src/a.rs:3"));
    assert!(error.to_string().contains("src/b.rs:7"));
    assert!(error.to_string().contains("generated/T.rs"));
}

#[test]
fn グラフ名が違えば同じgenerated文字列でも許す() {
    let target = CargoTarget::new("src");
    let mut 検査器 = 静的instance重複検査器::default();
    検査器.検査する(&target, "A", "generated/T.rs", &DeclarationSite::new("src/a.rs".to_string(), 3)).unwrap();
    let 結果 =
        検査器.検査する(&target, "B", "generated/T.rs", &DeclarationSite::new("src/b.rs".to_string(), 7));
    assert!(結果.is_ok());
}

#[test]
fn targetが違えば同じグラフ名_同じgenerated文字列でも許す() {
    let a = CargoTarget::new("src");
    let b = CargoTarget::new("tests/foo");
    let mut 検査器 = 静的instance重複検査器::default();
    検査器.検査する(&a, "T", "generated/T.rs", &DeclarationSite::new("src/a.rs".to_string(), 3)).unwrap();
    let 結果 =
        検査器.検査する(&b, "T", "generated/T.rs", &DeclarationSite::new("tests/foo.rs".to_string(), 7));
    assert!(結果.is_ok());
}
