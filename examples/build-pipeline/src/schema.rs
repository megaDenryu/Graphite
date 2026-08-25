//! ビルドパイプラインのグラフスキーマ定義。
//!
//! `Task` (実行単位) と `Artifact` (成果物ファイル) という異種ノードを、
//! `Produces` (タスク→成果物、生成する) と `Consumes` (タスク→成果物、
//! 読み込む) という 2 種のエッジ種別で結ぶ (`docs/schema_v4.md`)。
//!
//! どちらも `where unique pair;` を付けている: 「あるタスクがある成果物を
//! 生成/消費する」という事実は有るか無いかの二値であり、同じ
//! (task, artifact) の対に2本目のエッジを張ることに意味が無いため
//! (多重グラフの平行辺を許す積極的な理由が無いケース)。多重度 `(0..*)`
//! 自体は各制約なので where 節には出てこない (unique pair 以外の制約が
//! 無いという意味)。
//!
//! v3 (`docs/graph_literal_v3.md` §4) でハンドシェイクマクロを全廃したため
//! `graph_schema!` と `graph!` を同一ファイルに置く必要は無くなったが、
//! テスト用の固定サンプルを組み立てる `graphリテラルで小さな固定パイプライン
//! を組み立てられる` は型定義に近い方が読みやすいためこのファイルに
//! 同居させている。

/// ノードキー。`graph_schema!` はこれも生成せず参照するだけ
/// (`docs/node_id_v4_2.md`)。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskId(pub String);

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub cmd: String,
    pub secs: u32,
}

/// ノードキー。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactId(pub String);

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Artifact {
    pub path: String,
}

/// 外部のID領域をschemaへ明示する例。DebugやDisplayは実装しない。
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ProducesId(pub String);

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod BuildPipeline {
    include!("generated/schema_build_pipeline.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/schema_build_pipeline.rs";
    schema BuildPipeline {
        node Task(id: TaskId);
        node Artifact(id: ArtifactId);

        edge Produces(id: ProducesId) = (task: Task) -> (artifact: Artifact) where unique pair;
        edge Consumes = (task: Task) -> (artifact: Artifact) where unique pair;
    }
}

// 綴り短縮のための再輸出。同名edgeを持つschemaを足したらこの行を消す。
pub use BuildPipeline::{Consumes, ConsumesId, Produces};

#[cfg(test)]
mod fixed_pipeline_showcase {
    //! `graph!` リテラルのショーケース。動的パース経由の本編とは別に、
    //! ごく小さい固定パイプライン (fetch -> build -> test) を宣言的に
    //! 組み立てられることを示す。
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn graphリテラルで小さな固定パイプラインを組み立てられる() {
        let g = graphite::graph!(BuildPipeline {
            fetch @ TaskId("fetch".into()) = Task { name: "fetch".into(), cmd: "cargo fetch".into(), secs: 10 },
            build @ TaskId("build".into()) = Task { name: "build".into(), cmd: "cargo build".into(), secs: 60 },
            test  @ TaskId("test".into()) = Task { name: "test".into(), cmd: "cargo test".into(), secs: 30 },

            index @ ArtifactId("index".into()) = Artifact { path: "vendor/registry-index".into() },
            rlib  @ ArtifactId("rlib".into()) = Artifact { path: "target/core.rlib".into() },

            fetch_index @ ProducesId("fetch_index".into()) = Produces(fetch -> index),
            build_index = Consumes(build -> index),
            build_rlib @ ProducesId("build_rlib".into()) = Produces(build -> rlib),
            test_rlib   = Consumes(test -> rlib),
        })
        .expect("正常な固定パイプラインは構築に成功するはず");

        assert_eq!(g.task_ids().count(), 3);
        assert_eq!(g.artifact_ids().count(), 2);

        let build = g.task_by_id(&TaskId("build".to_string())).unwrap();
        let produced: Vec<_> = build.produces_as_task().collect();
        assert_eq!(produced.len(), 1);
        assert_eq!(produced[0].artifact().path, "target/core.rlib");

        let test = g.task_by_id(&TaskId("test".to_string())).unwrap();
        let consumed: Vec<_> = test.consumes_as_task().collect();
        assert_eq!(consumed.len(), 1);
        assert_eq!(consumed[0].artifact().path, "target/core.rlib");
    }
}
