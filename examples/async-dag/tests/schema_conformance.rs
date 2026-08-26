//! 統合テスト: `graph_schema!` の図式適合検査 (未知の依存先) が独立に
//! 機能すること。

use async_dag::schema::{DependsOn, Orchestration, Service, ServiceId};

fn id(name: &str) -> ServiceId {
    ServiceId(name.to_string())
}

#[test]
fn 未知の依存先を参照するとunknowntarget違反になる() {
    let result: Result<Orchestration::Graph, Orchestration::Violation> =
        Orchestration::Graph::create(|b| {
            b.service(
                id("api"),
                Service {
                    name: "api".into(),
                    startup_ms: 10,
                },
            );
            b.depends_on(
                async_dag::schema::DependsOnId("api_missing".to_string()),
                DependsOn::new(id("api"), id("存在しないサービス")),
            );
        });
    assert!(matches!(
        result,
        Err(Orchestration::Violation::DependsOnUnknownTarget { .. })
    ));
}
