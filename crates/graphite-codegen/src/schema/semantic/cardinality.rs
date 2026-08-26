//! `where each` 制約が指す端点の側と、生成分岐に使う多重度を確定して持つ。

use proc_macro2::Ident;

use crate::schema::syntax::{EachConstraint, EachSpec, EdgeDecl, EdgeShape};

/// `where each <参照名>` が意味する側 (出次数/入次数)。
///
/// - `Source`: 始点の役割名に対する出次数制約
/// - `Target`: 終点の役割名に対する入次数制約
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EachSide {
    Source,
    Target,
}

/// 役割の `each` 制約を、生成コードの分岐に使う多重度3値へ分類したもの。
/// 役割クエリの戻り型・索引の実装・doc コメントはすべてこの3値だけで分岐する。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoleCardinality {
    /// `each X: 1` — ちょうど1本。
    Exact,
    /// `each X: 0..1` — 高々1本。
    Optional,
    /// それ以外の範囲、または制約なし。
    Multiple,
}

impl RoleCardinality {
    /// 役割の `each` 制約 (無ければ `None`) を多重度3値へ分類する。
    pub fn classify(spec: Option<EachSpec>) -> Self {
        match spec {
            Some(spec) if spec.is_exactly_one() => Self::Exact,
            Some(spec) if spec.is_zero_or_one() => Self::Optional,
            _ => Self::Multiple,
        }
    }
}

/// 1つの役割に課された多重度制約。どちら側の端点を指すかと、生成分岐に使う
/// 多重度3値を確定済みで持つ。
#[derive(Clone)]
pub struct 役割の多重度制約 {
    役割名: Ident,
    側: EachSide,
    指定された範囲: EachSpec,
    多重度: RoleCardinality,
}

impl 役割の多重度制約 {
    pub(super) fn 宣言と側から作る(宣言: &EachConstraint, 側: EachSide) -> Self {
        Self {
            役割名: 宣言.role.clone(),
            側,
            指定された範囲: 宣言.spec,
            多重度: RoleCardinality::classify(Some(宣言.spec)),
        }
    }

    pub fn 役割名(&self) -> &Ident {
        &self.役割名
    }

    pub fn 側(&self) -> EachSide {
        self.側
    }

    /// 診断文言と凍結時の判定式が使う、DSL に書かれたままの範囲。
    pub fn 指定された範囲(&self) -> EachSpec {
        self.指定された範囲
    }

    pub fn 多重度(&self) -> RoleCardinality {
        self.多重度
    }
}

/// `where each <参照名>: ..` の `<参照名>` がどちら側の端点を指すかを判定する。
/// 判定できない場合は診断つきの `syn::Error` を返す。
///
/// - 有向辺: `<参照名>` は始点/終点いずれかの役割名と一致する必要がある。
/// - 無向辺: 端点の役割名が無いため `each` 自体を拒否する。
pub fn each制約が指す端点の側を判定する(
    edge: &EdgeDecl,
    each_ident: &Ident,
) -> syn::Result<EachSide> {
    let (from_role, to_role) = match &edge.shape {
        EdgeShape::Directed { from, to, .. } => (&from.role, &to.role),
        EdgeShape::Undirected { .. } => return Err(syn::Error::new_spanned(
            each_ident,
            format!("無向辺 `{}` には端点の役割名が無いため `each` は使えません。使える制約は `unique pair` のみです", edge.kind),
        )),
    };
    if each_ident == from_role {
        Ok(EachSide::Source)
    } else if each_ident == to_role {
        Ok(EachSide::Target)
    } else {
        Err(syn::Error::new_spanned(
            each_ident,
            format!(
                "辺 `{}` の `each` は端点の役割名 (`{}`/`{}`) を参照してください。役割名 `{}` は存在しません",
                edge.kind, from_role, to_role, each_ident
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::semantic::analyze::検査用にdslからスキーマ定義を組み立てる;

    #[test]
    fn each制約の範囲を多重度3値へ分類できる() {
        let 定義 = 検査用にdslからスキーマ定義を組み立てる(
            "schema Org {
                node Person;
                node Team;
                edge ExactOne = (member: Person) -> (team: Team) where each member: 1;
                edge ZeroOrOne = (member: Person) -> (team: Team) where each member: 0..1;
                edge RangeMulti = (member: Person) -> (team: Team) where each member: 1..3;
                edge LowerOnly = (member: Person) -> (team: Team) where each member: 2..*;
                edge NoConstraint = (member: Person) -> (team: Team);
            }",
        );
        let 辺定義の列 = 定義.辺定義の列();
        assert_eq!(
            辺定義の列[0].側の多重度(EachSide::Source),
            RoleCardinality::Exact,
            "1 はちょうど1本"
        );
        assert_eq!(
            辺定義の列[1].側の多重度(EachSide::Source),
            RoleCardinality::Optional,
            "0..1 は高々1本"
        );
        assert_eq!(
            辺定義の列[2].側の多重度(EachSide::Source),
            RoleCardinality::Multiple,
            "1..3 は範囲指定なので Multiple"
        );
        assert_eq!(
            辺定義の列[3].側の多重度(EachSide::Source),
            RoleCardinality::Multiple,
            "2..* は下限だけなので Multiple"
        );
        assert_eq!(
            辺定義の列[4].側の多重度(EachSide::Source),
            RoleCardinality::Multiple,
            "制約が無ければ Multiple"
        );
    }
}
