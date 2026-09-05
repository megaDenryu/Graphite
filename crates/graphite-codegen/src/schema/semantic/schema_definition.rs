//! スキーマ1つ分の意味モデル全体を所有し、添字ハンドルからの取り出しを提供する。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。
//! スキーマ1つ分の意味モデル全体を所有し、添字ハンドルからの取り出しを提供する。
//! 本体は62行で、残りは同居する単体テストである。超過を許す根拠の台帳は
//! `docs/development/line_count_ledger.md` にある。

use proc_macro2::Ident;

use super::edge_definition::辺定義;
use super::node_definition::{ノード定義, ノード定義番号};
use super::traversal_plan::ノードの探索計画;
use super::violation_catalog::{違反定義, 違反定義の目録};

// `schema Name { .. }` 1つ分の確定した意味。コード生成層はこの値だけを読む。
pub struct スキーマ定義 {
    スキーマ名: Ident,
    ノード定義の列: Vec<ノード定義>,
    辺定義の列: Vec<辺定義>,
    違反定義の目録: 違反定義の目録,
    ノードごとの探索計画: Vec<ノードの探索計画>,
}

impl スキーマ定義 {
    pub(super) fn 定義の列から作る(
        スキーマ名: Ident,
        ノード定義の列: Vec<ノード定義>,
        辺定義の列: Vec<辺定義>,
    ) -> Self {
        let 違反定義の目録 = 違反定義の目録::定義の列から組み立てる(
            ノード定義の列.len(),
            &辺定義の列,
        );
        let ノードごとの探索計画 = (0..ノード定義の列.len())
            .map(|添字| {
                ノードの探索計画::ノードと辺定義から組み立てる(
                    ノード定義番号::添字から作る(添字),
                    &辺定義の列,
                )
            })
            .collect();
        Self {
            スキーマ名,
            ノード定義の列,
            辺定義の列,
            違反定義の目録,
            ノードごとの探索計画,
        }
    }

    pub fn スキーマ名(&self) -> &Ident {
        &self.スキーマ名
    }

    // ノード定義を宣言順で返す。添字は `ノード定義番号` と一致する。
    pub fn ノード定義の列(&self) -> &[ノード定義] {
        &self.ノード定義の列
    }

    // 辺定義を宣言順で返す。添字は辺定義番号と一致する。
    pub fn 辺定義の列(&self) -> &[辺定義] {
        &self.辺定義の列
    }

    // 違反列挙型の variant になる違反定義を、生成する順で返す。
    pub fn 違反定義の列(&self) -> &[違反定義] {
        self.違反定義の目録.違反定義の列()
    }

    // ノードごとの探索計画を、ノード定義と同じ並びで返す。
    pub fn ノードごとの探索計画(&self) -> &[ノードの探索計画] {
        &self.ノードごとの探索計画
    }

    // この schema 宣言の形 (`schema OrgChart`)。生成物の doc から宣言元を
    // 指すときに、schema 全体に属する生成物 (`Graph`・`Builder`・`Violation`)
    // が参照する。
    pub fn 宣言の形(&self) -> String {
        format!("schema {}", self.スキーマ名)
    }

    // 辺定義の宣言の形を、端点のノード型名まで解決して組み立てる。
    //
    // 辺定義は端点を添字ハンドルで持つため、ノード定義の列を所有するこの型
    // だけが型名を解決できる。
    pub fn 辺の宣言の形(&self, 辺: &辺定義) -> String {
        let 始点 = self.ノード定義の列[辺.始点のノード定義番号().添字()].ノード値型名();
        let 終点 = self.ノード定義の列[辺.終点のノード定義番号().添字()].ノード値型名();
        辺.宣言の形(始点, 終点)
    }
}

#[cfg(test)]
mod tests {
    use super::super::analyze::検査用にdslからスキーマ定義を組み立てる;

    #[test]
    fn 宣言の形をノード種別と辺種別ごとに組み立てる() {
        let 定義 = 検査用にdslからスキーマ定義を組み立てる(
            "schema Commerce {
                node Person;
                node Product(id: self::ExternalProductId);
                edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair;
                edge Subscription = (member: Person) -> (product: Product) where each member: 1..*;
                edge Friend(id: ExternalEdgeId) = Person -- Person;
                edge Cable = Person -[cable: Cable]- Person;
            }",
        );
        assert_eq!(定義.宣言の形(), "schema Commerce");
        assert_eq!(定義.ノード定義の列()[0].宣言の形(), "node Person");
        assert_eq!(
            定義.ノード定義の列()[1].宣言の形(),
            "node Product(id: super::ExternalProductId)",
            "明示ID型は生成 module 内から解決できる形へ正規化した綴りで書く"
        );
        let 辺の列 = 定義.辺定義の列();
        assert_eq!(
            定義.辺の宣言の形(&辺の列[0]),
            "edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair"
        );
        assert_eq!(
            定義.辺の宣言の形(&辺の列[1]),
            "edge Subscription = (member: Person) -> (product: Product) where each member: 1..*"
        );
        assert_eq!(
            定義.辺の宣言の形(&辺の列[2]),
            "edge Friend(id: ExternalEdgeId) = Person -- Person"
        );
        assert_eq!(
            定義.辺の宣言の形(&辺の列[3]),
            "edge Cable = Person -[cable: Cable]- Person"
        );
    }

    #[test]
    fn ちょうど1本の多重度は範囲を畳んだ綴りになる() {
        let 定義 = 検査用にdslからスキーマ定義を組み立てる(
            "schema Org {
                node Person;
                node Team;
                edge Belongs = (member: Person) -> (team: Team) where each member: 1;
            }",
        );
        assert_eq!(
            定義.辺の宣言の形(&定義.辺定義の列()[0]),
            "edge Belongs = (member: Person) -> (team: Team) where each member: 1"
        );
    }
}
