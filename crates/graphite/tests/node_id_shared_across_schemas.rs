//! v4.2 (`docs/node_id_v4_2.md`) の意味論そのものを実証する統合テスト。
//!
//! 「キーは個体の名前であり、`PersonId` は特定のグラフにではなく `Person`
//! という型に1個だけ属する」という決定により、組織図 (`OrgChart`) と
//! 承認フロー (`ApprovalFlow`) という**別々の schema** が同じ `Person`/
//! `PersonId` を共有できる。このテストは、一方のグラフで得たキーをもう
//! 一方のグラフのクエリにそのまま渡せることを確認する
//! (`docs/node_id_v4_2.md` 「複数 schema での `PersonId` 共有は... 当然の
//! 帰結になる」)。
//!
//! 2つの schema をそれぞれ専用モジュールに隔離しているのは、
//! `{Schema}Node` トレイトの衝突回避のため (README「同一モジュール内で
//! 複数 schema がノード型を共有する場合の制約」節と同じ理由: 両方の
//! `{Schema}Node` トレイトが同一スコープにあると `Person::get(..)` が
//! `OrgChartNode::get` と `ApprovalFlowNode::get` のどちらかに解決できず
//! 曖昧になる、通常の Rust のトレイトメソッド解決規則)。`PersonId` という
//! **値**そのものはモジュールを跨いで自由に受け渡せる — 曖昧になるのは
//! 「どちらの schema のグラフに対して引くか」を表すメソッド解決だけであり、
//! それはどのグラフを引数に渡すかで人間にもコンパイラにも自明なので、
//! 完全修飾記法 (`<Person as ApprovalFlowNode>::get`) で簡単に解消できる。

/// `Person` を宣言した者として、`PersonId` もここで1個だけ宣言する
/// (「型を宣言した者が Id も宣言する」規則)。両方の schema から共有される。
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
    graphite::graph_schema! {
        schema OrgChart {
            node Person;
            node Department;

            edge BelongsTo = Person -> Department where each Person: 0..1;
        }
    }

    pub fn build() -> OrgChart {
        OrgChart::create(|b| {
            b.person(PersonId("tanaka".to_string()), Person { name: "田中".to_string() });
            b.person(PersonId("sato".to_string()), Person { name: "佐藤".to_string() });
            b.department(DepartmentId("sales".to_string()), Department { name: "営業".to_string() });
            b.belongs_to(
                BelongsToId("bt1".to_string()),
                BelongsTo(PersonId("tanaka".to_string()), DepartmentId("sales".to_string())),
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
    graphite::graph_schema! {
        schema ApprovalFlow {
            node Person;

            edge Approves = Person -> Person;
        }
    }

    pub fn build() -> ApprovalFlow {
        ApprovalFlow::create(|b| {
            b.person(PersonId("tanaka".to_string()), Person { name: "田中".to_string() });
            b.person(PersonId("sato".to_string()), Person { name: "佐藤".to_string() });
            b.approves(
                ApprovesId("ap1".to_string()),
                Approves(PersonId("sato".to_string()), PersonId("tanaka".to_string())),
            );
        })
        .expect("承認フローの構築に成功するはず")
    }
}

use org_chart::OrgChartNode;
use approval_flow::ApprovalFlowNode;

#[test]
fn 組織図で得たキーを承認フローのクエリにそのまま渡せる() {
    let org = org_chart::build();
    let flow = approval_flow::build();

    // 組織図側で「田中さんのキー」を取得する。
    let tanaka_id_in_org: &PersonId = <Person as OrgChartNode>::ids(&org)
        .find(|id| <Person as OrgChartNode>::get(&org, id).unwrap().name == "田中")
        .expect("組織図に田中さんがいるはず");

    // そのキーを、型変換もラップも一切せずに承認フロー側のクエリへ渡せる
    // (`PersonId` は `OrgChart`/`ApprovalFlow` のどちらにも属さず、`Person`
    // 型そのものに1個だけ属するため)。
    let tanaka_in_flow = <Person as ApprovalFlowNode>::get(&flow, tanaka_id_in_org)
        .expect("組織図で得たキーがそのまま承認フローでも引けるはず");
    assert_eq!(tanaka_in_flow.name, "田中");

    // 逆方向 (承認フロー → 組織図) も同様に成立する。
    let sato_id_in_flow: &PersonId = <Person as ApprovalFlowNode>::ids(&flow)
        .find(|id| <Person as ApprovalFlowNode>::get(&flow, id).unwrap().name == "佐藤")
        .expect("承認フローに佐藤さんがいるはず");
    let sato_in_org = <Person as OrgChartNode>::get(&org, sato_id_in_flow)
        .expect("承認フローで得たキーがそのまま組織図でも引けるはず");
    assert_eq!(sato_in_org.name, "佐藤");

    // 承認フロー自体の意味論も一応確認しておく: 佐藤 -> 田中 の承認関係。
    let approves_target = approval_flow::Approves::of(&flow, sato_id_in_flow);
    assert_eq!(approves_target.first().unwrap().name, "田中");
}
