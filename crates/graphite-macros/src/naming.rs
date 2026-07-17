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
    fn 複数形フィールド名を導出できる() {
        assert_eq!(plural_field_name("Employee"), "employees");
        assert_eq!(plural_field_name("Department"), "departments");
    }
}
