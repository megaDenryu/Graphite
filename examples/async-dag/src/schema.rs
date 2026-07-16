//! サービス起動グラフのスキーマ定義。
//!
//! `Service` (起動するマイクロサービス1個) というノード種別を、
//! `depends_on` (「このサービスは、あのサービスに依存する」) という
//! 多重度 `(0..*)` の1種のエッジで結ぶ。`A -[depends_on]-> B` は
//! 「A は B に依存する (B が起動完了していないと A は起動できない)」
//! と読む — Rust の `impl Trait for X` の依存方向と同じで、矢印の始点が
//! 「これから作るもの」、終点が「先に必要なもの」になる。
//!
//! この向きは実行順序 (トポロジカル順序) とは逆になる点に注意。
//! `depends_on: A -> B` は「A は B の後」を意味するので、Kahn 法や
//! `topological_sort`/`topological_levels` が仮定する「辺の始点が先」
//! という向きに合わせるには、汎用 `graphite::Graph` へ射影する際に
//! 辺の向きを反転する必要がある。この反転は `depgraph.rs` の
//! `build_dependency_graph` が一箇所で担う (README「グラフによる
//! 再定式化」節参照)。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Service {
    /// サービス名 (表示用。キーである `ServiceId` とは独立)。
    pub name: String,
    /// 起動所要時間 (ミリ秒)。`engine::simulate_startup` がこの時間だけ
    /// `std::thread::sleep` することで、実際のサービス起動を模擬する。
    pub startup_ms: u64,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema Orchestration {
        node Service;

        edge depends_on: Service -> Service (0..*);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn graphリテラルで小さな依存グラフを組み立てられる() {
        let g = graphite::graph!(Orchestration {
            config = Service { name: "config".into(), startup_ms: 10 },
            db     = Service { name: "db".into(), startup_ms: 20 },
            api    = Service { name: "api".into(), startup_ms: 15 },

            db  -[depends_on]-> config,
            api -[depends_on]-> db,
        })
        .expect("正常な依存グラフは構築に成功するはず");

        assert_eq!(g.service_ids().count(), 3);
        let deps: Vec<&Service> = g.depends_on().of(&ServiceId("api".to_string()));
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "db");
    }
}
