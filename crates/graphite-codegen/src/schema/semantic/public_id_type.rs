//! ノード・辺の公開ID型が、スキーマの生成物と利用者の既存型のどちらかを確定して持つ。

use proc_macro2::Ident;
use syn::{Path, PathSegment};

use super::type_path_spelling::型パスの綴りを組み立てる;

/// 宣言を省略した既定生成ID、または `(id: 型パス)` で明示された既存ID型。
///
/// 利用者が書いたパスは schema module の**内側**から見えるパスへ構築時に1回だけ
/// 読み替える。生成のたびに読み替えると、同じ意味の判断がトークン出力の回数だけ
/// 繰り返されるためである。
pub enum 公開ID型 {
    /// schema module 内に `pub struct {型名}(pub String);` を生成する。
    スキーマが生成するID型 { 型名: Ident },
    /// 利用者が別に宣言した型を参照する。パスは生成 module 内から解決できる形に
    /// 読み替え済みである。
    利用者が宣言した既存のID型 {
        生成module内から見たパス: Path
    },
}

impl 公開ID型 {
    /// 既定生成ID型名と、宣言に書かれた明示パス (無ければ `None`) から作る。
    pub(super) fn 宣言から作る(
        既定の型名: Ident, 明示されたパス: Option<Path>
    ) -> Self {
        match 明示されたパス {
            None => Self::スキーマが生成するID型 {
                型名: 既定の型名
            },
            Some(パス) => Self::利用者が宣言した既存のID型 {
                生成module内から見たパス:
                    生成module内から見たパスへ読み替える(パス),
            },
        }
    }

    /// スキーマがこのID型を生成するか (明示指定なら生成しない)。
    pub fn スキーマが生成するid型か(&self) -> bool {
        matches!(self, Self::スキーマが生成するID型 { .. })
    }

    /// このID型の値を `Debug` 表示に載せてよいか。
    ///
    /// スキーマが生成するID型だけが `#[derive(Debug)]` を持つ。利用者が宣言した
    /// 型へ `Debug` を無条件に要求しない契約 (`docs/schema_v4.md`) を守るため、
    /// 表示できるのは生成ID型に限る。
    pub fn デバッグ表示に使えるか(&self) -> bool {
        self.スキーマが生成するid型か()
    }

    /// 宣言の形へ書く明示ID型の綴り。schema が生成するID型は宣言に書かれて
    /// いないため `None` を返す。
    ///
    /// 綴りは生成 module 内から解決できる形へ正規化済みである (`self::X` は
    /// `super::X` になる)。宣言の形は DSL 原文の逐語ではなく、宣言の意味を
    /// 1行で表す正規形として扱う。
    pub(super) fn 明示された型パスの綴り(&self) -> Option<String> {
        match self {
            Self::スキーマが生成するID型 { .. } => None,
            Self::利用者が宣言した既存のID型 {
                生成module内から見たパス,
            } => Some(型パスの綴りを組み立てる(生成module内から見たパス)),
        }
    }

    /// スキーマが生成する場合のその型名。明示指定なら `None`。
    pub fn スキーマが生成する型名(&self) -> Option<&Ident> {
        match self {
            Self::スキーマが生成するID型 { 型名 } => Some(型名),
            Self::利用者が宣言した既存のID型 { .. } => None,
        }
    }
}

/// 利用者が呼び出し箇所の名前空間で書いたパスを、生成 module の内側から解決できる
/// パスへ読み替える。
///
/// 生成 module は `use super::*;` で呼び出し側の名前を取り込むため、外部crate名・
/// プリミティブ型・取り込まれた名前はそのまま解決できる。読み替えが要るのは、
/// 呼び出し側 module を指す `self::` だけである。
fn 生成module内から見たパスへ読み替える(パス: Path) -> Path {
    let 先頭 = パス
        .segments
        .first()
        .map(|segment| segment.ident.to_string());
    if パス.leading_colon.is_some() || 先頭.as_deref() != Some("self") {
        return パス;
    }
    let superの位置 = パス.segments[0].ident.span();
    let mut 読み替え後 = パス.clone();
    読み替え後.segments = std::iter::once(PathSegment::from(Ident::new("super", superの位置)))
        .chain(パス.segments.iter().skip(1).cloned())
        .collect();
    読み替え後
}

#[cfg(test)]
mod tests {
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
}
