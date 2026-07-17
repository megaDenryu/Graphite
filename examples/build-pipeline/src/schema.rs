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

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub cmd: String,
    pub secs: u32,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Artifact {
    pub path: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema BuildPipeline {
        node Task;
        node Artifact;

        edge Produces = Task -> Artifact where unique pair;
        edge Consumes = Task -> Artifact where unique pair;
    }
}

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
            fetch = Task { name: "fetch".into(), cmd: "cargo fetch".into(), secs: 10 },
            build = Task { name: "build".into(), cmd: "cargo build".into(), secs: 60 },
            test  = Task { name: "test".into(), cmd: "cargo test".into(), secs: 30 },

            index = Artifact { path: "vendor/registry-index".into() },
            rlib  = Artifact { path: "target/core.rlib".into() },

            fetch_index = Produces(fetch -> index),
            build_index = Consumes(build -> index),
            build_rlib  = Produces(build -> rlib),
            test_rlib   = Consumes(test -> rlib),
        })
        .expect("正常な固定パイプラインは構築に成功するはず");

        assert_eq!(Task::ids(&g).count(), 3);
        assert_eq!(Artifact::ids(&g).count(), 2);

        let produced: Vec<&Artifact> = Produces::of(&g, &TaskId("build".to_string()));
        assert_eq!(produced.len(), 1);
        assert_eq!(produced[0].path, "target/core.rlib");

        let consumed: Vec<&Artifact> = Consumes::of(&g, &TaskId("test".to_string()));
        assert_eq!(consumed.len(), 1);
        assert_eq!(consumed[0].path, "target/core.rlib");
    }
}
