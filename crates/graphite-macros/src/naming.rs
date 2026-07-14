//! 識別子の命名規則変換ヘルパー。
//!
//! `graph_schema!` と `graph!` の両方が同じ変換規則に従う必要がある
//! (`graph!` はスキーマの中身を知らずにビルダーメソッド名・属性型名を
//! 機械的に導出するため)。この対応がずれると `graph!` が生成する呼び出しが
//! `graph_schema!` の生成物と噛み合わずコンパイルエラーになる。

/// `PascalCase` / `camelCase` の識別子を `snake_case` に変換する。
///
/// 例: `Employee` -> `employee`, `OrgChart` -> `org_chart`。
/// ノード型名からビルダーメソッド名・アクセサ名を導出するのに使う。
pub fn to_snake_case(ident: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower_or_digit = false;
    for (i, c) in ident.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 && prev_is_lower_or_digit {
                result.push('_');
            }
            result.extend(c.to_lowercase());
            prev_is_lower_or_digit = false;
        } else {
            result.push(c);
            prev_is_lower_or_digit = c.is_lowercase() || c.is_numeric();
        }
    }
    result
}

/// `snake_case` の識別子を `PascalCase` に変換する。
///
/// 例: `boss` -> `Boss`, `belongs_to` -> `BelongsTo`。
/// エッジ種別名から違反 enum のバリアント名
/// (`{PascalCase}Multiplicity`/`{PascalCase}UnknownSource` 等) を導出するのに
/// 使う。
pub fn to_pascal_case(ident: &str) -> String {
    ident
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// ノード型の内部ストレージ用フィールド名 (複数形) を導出する。
///
/// 英語の不規則複数形には対応しない素朴な "s" 付与だが、この名前は
/// 生成コード内部にのみ現れる非公開フィールド名であり利用者からは
/// 見えないため、機能上の問題にはならない (詳細は README の未決事項欄)。
pub fn plural_field_name(type_name: &str) -> String {
    format!("{}s", to_snake_case(type_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_caseへ変換できる() {
        assert_eq!(to_snake_case("Employee"), "employee");
        assert_eq!(to_snake_case("OrgChart"), "org_chart");
        assert_eq!(to_snake_case("belongs_to"), "belongs_to");
    }

    #[test]
    fn pascal_caseへ変換できる() {
        assert_eq!(to_pascal_case("boss"), "Boss");
        assert_eq!(to_pascal_case("belongs_to"), "BelongsTo");
        assert_eq!(to_pascal_case("reports"), "Reports");
    }

    #[test]
    fn 複数形フィールド名を導出できる() {
        assert_eq!(plural_field_name("Employee"), "employees");
        assert_eq!(plural_field_name("Department"), "departments");
    }
}
