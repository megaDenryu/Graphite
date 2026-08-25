//! 明示ID型を複数schemaで共有できることを実証する統合テスト。
//!
//! 組織図 (`OrgChart`) と承認フロー (`ApprovalFlow`) がどちらも
//! `node Person(id: PersonId);` を宣言し、一方のグラフで得たキーをもう
//! 一方のグラフの問い合わせへ渡せることを確認する。
//!
//! 各 schema は生成物を `OrgChart`/`ApprovalFlow` module に分離するため、
//! 同じ `Person` 値型を共有しても問い合わせ名は衝突しない。`PersonId` という
//! 値は module を跨いで自由に受け渡せ、問い合わせ先は
//! `OrgChart::Person::get` と `ApprovalFlow::Person::get` で明示できる。

/// 2つのschemaから明示的に参照する共有ID型。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

/// 組織図: 誰がどの部署に属しているか。
mod org_chart {
    use super::{Person, PersonId};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct DepartmentId(pub String);

    #[derive(Debug, Clone, PartialEq)]
    pub struct Department {
        pub name: String,
    }

    #[rustfmt::skip]
    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod OrgChart {
        include!("generated/node_id_shared_across_schemas_org_chart.rs");
    }

    #[rustfmt::skip]
    graphite::graph_schema! {
        generated = "generated/node_id_shared_across_schemas_org_chart.rs";
        schema OrgChart {
            node Person(id: PersonId);
            node Department(id: DepartmentId);

            edge BelongsTo = (person: Person) -> (department: Department) where each person: 0..1;
        }
    }

    use OrgChart::{BelongsTo, BelongsToId};

    pub fn build() -> OrgChart::Graph {
        OrgChart::Graph::create(|b| {
            b.person(
                PersonId("tanaka".to_string()),
                Person {
                    name: "田中".to_string(),
                },
            );
            b.person(
                PersonId("sato".to_string()),
                Person {
                    name: "佐藤".to_string(),
                },
            );
            b.department(
                DepartmentId("sales".to_string()),
                Department {
                    name: "営業".to_string(),
                },
            );
            b.belongs_to(
                BelongsToId("bt1".to_string()),
                BelongsTo::new(
                    PersonId("tanaka".to_string()),
                    DepartmentId("sales".to_string()),
                ),
            );
        })
        .expect("組織図の構築に成功するはず")
    }
}

/// 承認フロー: 誰が誰の承認者か。`org_chart` とは別の schema だが、
/// 同じ `Person`/`PersonId` を参照する。
mod approval_flow {
    use super::{Person, PersonId};

    #[rustfmt::skip]
    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod ApprovalFlow {
        include!("generated/node_id_shared_across_schemas_approval_flow.rs");
    }

    #[rustfmt::skip]
    graphite::graph_schema! {
        generated = "generated/node_id_shared_across_schemas_approval_flow.rs";
        schema ApprovalFlow {
            node Person(id: PersonId);

            edge Approves = (approver: Person) -> (approved: Person);
        }
    }

    use ApprovalFlow::{Approves, ApprovesId};

    pub fn build() -> ApprovalFlow::Graph {
        ApprovalFlow::Graph::create(|b| {
            b.person(
                PersonId("tanaka".to_string()),
                Person {
                    name: "田中".to_string(),
                },
            );
            b.person(
                PersonId("sato".to_string()),
                Person {
                    name: "佐藤".to_string(),
                },
            );
            b.approves(
                ApprovesId("ap1".to_string()),
                Approves::new(PersonId("sato".to_string()), PersonId("tanaka".to_string())),
            );
        })
        .expect("承認フローの構築に成功するはず")
    }
}

#[test]
fn 組織図で得たキーを承認フローのクエリにそのまま渡せる() {
    let org = org_chart::build();
    let flow = approval_flow::build();

    // 組織図側で「田中さんのキー」を取得する。
    let tanaka_id_in_org: &PersonId = org_chart::OrgChart::Person::ids(&org)
        .find(|id| org_chart::OrgChart::Person::get(&org, id).unwrap().name == "田中")
        .expect("組織図に田中さんがいるはず");

    // 両schemaが同じ既存型を明示指定しているため、そのキーを型変換も
    // ラップもせずに承認フロー側のクエリへ渡せる。
    let tanaka_in_flow = approval_flow::ApprovalFlow::Person::get(&flow, tanaka_id_in_org)
        .expect("組織図で得たキーがそのまま承認フローでも引けるはず");
    assert_eq!(tanaka_in_flow.name, "田中");

    // 逆方向 (承認フロー → 組織図) も同様に成立する。
    let sato_id_in_flow: &PersonId = approval_flow::ApprovalFlow::Person::ids(&flow)
        .find(|id| {
            approval_flow::ApprovalFlow::Person::get(&flow, id)
                .unwrap()
                .name
                == "佐藤"
        })
        .expect("承認フローに佐藤さんがいるはず");
    let sato_in_org = org_chart::OrgChart::Person::get(&org, sato_id_in_flow)
        .expect("承認フローで得たキーがそのまま組織図でも引けるはず");
    assert_eq!(sato_in_org.name, "佐藤");

    // 承認フロー自体の意味論も一応確認しておく: 佐藤 -> 田中 の承認関係。
    let sato_ref = approval_flow::ApprovalFlow::Person::get(&flow, sato_id_in_flow).unwrap();
    let approves_target = approval_flow::ApprovalFlow::Approves::of_approver(sato_ref)
        .next()
        .unwrap();
    assert_eq!(approves_target.approved().name, "田中");
}
