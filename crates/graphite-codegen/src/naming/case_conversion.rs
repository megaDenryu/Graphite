//! 識別子のケース変換規則だけを持つ。

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

/// 役割名 (`line_item`) を variant 名に埋め込める `PascalCase` (`LineItem`) へ
/// 変換する。区切りは `_` のみで、日本語の役割名はそのまま通る。
pub fn to_pascal_case(ident: &str) -> String {
    ident
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<String>()
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
        assert_eq!(to_pascal_case("line_item"), "LineItem");
        assert_eq!(to_pascal_case("始点"), "始点");
    }
}
