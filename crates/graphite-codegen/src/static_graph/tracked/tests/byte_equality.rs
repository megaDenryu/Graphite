// 同じ入力から本文がバイト単位で一致することを確かめる (錨になる固定
// ハッシュを置く、issue #41 段階2の完了条件)。

use quote::quote;

use crate::declaration_site::DeclarationSite;
use crate::fingerprint::fnv1a;

use super::super::{parse_tracked_static_instance, parse_tracked_static_schema};
use super::{instance入力, schema入力};

#[test]
fn schema本文は同じ入力からバイト単位で一致する() {
    let 一回目 = parse_tracked_static_schema(schema入力()).unwrap();
    let 二回目 = parse_tracked_static_schema(schema入力()).unwrap();
    let site = DeclarationSite::new("src/main.rs".to_string(), 3);
    let 本文 = 一回目.render_module_source(&site).unwrap();
    assert_eq!(本文, 二回目.render_module_source(&site).unwrap());
    // 固定値は生成物の意図しない変化を検出するための錨である
    // (`crate::tests::同じ入力の生成結果はバイト単位で一致する` と同じ方針)。
    assert_eq!(fnv1a(本文.as_bytes(), 0xcbf29ce484222325), 11426292115700224459);
}

#[test]
fn instance本文は同じschema参照からバイト単位で一致する() {
    let schema = parse_tracked_static_schema(schema入力()).unwrap();
    let 一回目 = parse_tracked_static_instance(&schema, instance入力()).unwrap();
    let 二回目 = parse_tracked_static_instance(&schema, instance入力()).unwrap();
    let site = DeclarationSite::new("src/main.rs".to_string(), 12);
    let 本文 = 一回目.render_module_source(&site, &site).unwrap();
    assert_eq!(本文, 二回目.render_module_source(&site, &site).unwrap());
    assert_eq!(fnv1a(本文.as_bytes(), 0xcbf29ce484222325), 14382902918527045870);
}

#[test]
fn schemaを変えるとinstanceの指紋も変わる() {
    let schema甲 = parse_tracked_static_schema(schema入力()).unwrap();
    let 甲 = parse_tracked_static_instance(&schema甲, instance入力()).unwrap();

    let schema乙 = parse_tracked_static_schema(quote! {
        generated = "generated/組織.rs";
        schema 組織 {
            node 社員;
            node 部署;
            edge 所属 = (member: 社員) -> (team: 部署) where each member: 1..*;
            edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1;
        }
    })
    .unwrap();
    let 乙 = parse_tracked_static_instance(&schema乙, instance入力()).unwrap();

    assert_ne!(甲.fingerprint(), 乙.fingerprint());
}

// `node` 宣言の値の式の種類 (struct式か関数呼び出しか) を変えても、構造
// (名前・型・値の有無) が同じなら本文はバイト単位で一致する。
// 一致しないと、値だけを書き換えた編集で `cargo graphite generate --check`
// が再生成を要求してしまう。
#[test]
fn 値の式の種類を変えても本文はバイト単位で一致する() {
    let schema = parse_tracked_static_schema(schema入力()).unwrap();
    let struct式版 = parse_tracked_static_instance(&schema, instance入力()).unwrap();

    let 関数呼び出し版 = parse_tracked_static_instance(
        &schema,
        quote! {
            generated = "generated/開発チーム.rs";
            graph 開発チーム;
            node 太郎: 社員 = 社員を作る("太郎");
            node 次郎: 社員 = 社員を作る("次郎");
            node 開発部: 部署;
            edge 太郎の所属 = 所属(太郎 -> 開発部);
            edge 次郎の所属 = 所属(次郎 -> 開発部);
            edge 太郎の上司 = 上司(太郎 -[任命記録を作る(2020)]-> 次郎);
        },
    )
    .unwrap();

    let site = DeclarationSite::new("src/main.rs".to_string(), 12);
    assert_eq!(
        struct式版.render_module_source(&site, &site).unwrap(),
        関数呼び出し版.render_module_source(&site, &site).unwrap()
    );
    assert_eq!(struct式版.fingerprint(), 関数呼び出し版.fingerprint());
}
