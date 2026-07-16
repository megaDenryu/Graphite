//! `main.rs` と `tests/integration.rs` の両方から使う固定のサンプルグラフ。
//! 同じデータを2箇所に手書きして食い違わせないよう、ここに1箇所だけ置く。

use crate::schema::{Orchestration, Service};

/// 本編の10サービス依存グラフ:
/// `config -> (logger, db, cache, queue) -> (migration, metrics) ->
/// (api, worker) -> healthcheck`。
///
/// 起動時間 (ms) は「敵1のベースライン (直列実行)」との対比が分かりやすい
/// ように、依存の合流点 (`migration`・`api`/`worker`) を重めに設定してある。
#[rustfmt::skip]
pub fn sample_orchestration() -> Orchestration {
    graphite::graph!(Orchestration {
        config      = Service { name: "config".into(),      startup_ms: 15 },
        logger      = Service { name: "logger".into(),      startup_ms: 8  },
        db          = Service { name: "db".into(),          startup_ms: 35 },
        cache       = Service { name: "cache".into(),       startup_ms: 25 },
        queue       = Service { name: "queue".into(),       startup_ms: 20 },
        migration   = Service { name: "migration".into(),   startup_ms: 55 },
        metrics     = Service { name: "metrics".into(),     startup_ms: 12 },
        api         = Service { name: "api".into(),         startup_ms: 45 },
        worker      = Service { name: "worker".into(),      startup_ms: 40 },
        healthcheck = Service { name: "healthcheck".into(), startup_ms: 28 },

        logger    -[depends_on]-> config,
        db        -[depends_on]-> config,
        cache     -[depends_on]-> config,
        queue     -[depends_on]-> config,
        migration -[depends_on]-> db,
        migration -[depends_on]-> cache,
        metrics   -[depends_on]-> logger,
        api       -[depends_on]-> migration,
        api       -[depends_on]-> logger,
        worker    -[depends_on]-> migration,
        worker    -[depends_on]-> queue,
        healthcheck -[depends_on]-> api,
        healthcheck -[depends_on]-> worker,
    })
    .expect("本編のサービスグラフは正常に構築できるはず")
}

/// 循環依存デモ用の小さな3サービス例 (a -> b -> c -> a)。
///
/// `depends_on: Service -> Service (0..*)` という図式適合の検査は
/// 循環を禁止しない (多重度は「1本のエッジの本数」の制約であり、
/// 「グラフ全体の形」の制約ではないため) ので、この呼び出し自体は
/// 成功する。循環の検出は `depgraph::compute_waves`
/// (=汎用`graphite::Graph`への射影+`topological_levels`) が担う。
#[rustfmt::skip]
pub fn cyclic_demo() -> Orchestration {
    graphite::graph!(Orchestration {
        a = Service { name: "service-a".into(), startup_ms: 10 },
        b = Service { name: "service-b".into(), startup_ms: 10 },
        c = Service { name: "service-c".into(), startup_ms: 10 },

        a -[depends_on]-> b,
        b -[depends_on]-> c,
        c -[depends_on]-> a,
    })
    .expect("スキーマ上は正常 (循環検査は図式適合の範囲外)")
}
