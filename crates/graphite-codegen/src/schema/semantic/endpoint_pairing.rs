//! 端点対索引のキーの形と、同じ対に持てる辺の本数を確定して持つ。

// 同じ端点対に何本の辺を張れるか。`where unique pair` の有無で決まる。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum 端点対の重複可否 {
    対ごとに1本だけ許す,
    対ごとに何本でも許す,
}

impl 端点対の重複可否 {
    pub(super) fn unique_pair指定から作る(unique_pair指定がある: bool) -> Self {
        if unique_pair指定がある {
            Self::対ごとに1本だけ許す
        } else {
            Self::対ごとに何本でも許す
        }
    }

    pub fn 対ごとに1本だけか(self) -> bool {
        self == Self::対ごとに1本だけ許す
    }
}

// 端点対索引のキーの形。有向辺は始点と終点の順序に意味があり、無向辺は無い。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum 端点対のキーの形 {
    順序付きの対,
    順序なしの対,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::semantic::analyze::検査用にdslからスキーマ定義を組み立てる;

    #[test]
    fn unique_pairの有無が端点対の重複可否になる() {
        let 定義 = 検査用にdslからスキーマ定義を組み立てる(
            "schema Org {
                node Person;
                edge Reports = (reporter: Person) -> (recipient: Person) where unique pair;
                edge Knows = (source: Person) -> (target: Person);
            }",
        );
        assert_eq!(
            定義.辺定義の列()[0].端点対の重複可否(),
            端点対の重複可否::対ごとに1本だけ許す
        );
        assert_eq!(
            定義.辺定義の列()[1].端点対の重複可否(),
            端点対の重複可否::対ごとに何本でも許す
        );
    }

    #[test]
    fn 端点対のキーの形は辺の向きで決まる() {
        let 定義 = 検査用にdslからスキーマ定義を組み立てる(
            "schema Social {
                node Person;
                edge Knows = (source: Person) -> (target: Person);
                edge Friends = Person -- Person;
            }",
        );
        assert_eq!(
            定義.辺定義の列()[0].端点対のキーの形(),
            端点対のキーの形::順序付きの対
        );
        assert_eq!(
            定義.辺定義の列()[1].端点対のキーの形(),
            端点対のキーの形::順序なしの対
        );
    }
}
