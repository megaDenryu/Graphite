//! ビルドパイプラインのグラフスキーマ定義。
//!
//! `Task` (実行単位) と `Artifact` (成果物ファイル) という異種ノードを、
//! `produces` (タスク→成果物、生成する) と `consumes` (タスク→成果物、
//! 読み込む) という 2 種類の多重度 `(0..*)` エッジで結ぶ。
//!
//! `graph_schema!` と `graph!` は同一ファイル内でのみ親切なエラーメッセージ
//! (未知エッジラベル検査マクロ) が効くというマクロハンドシェイク制約が
//! あるため (README「未決事項」節参照)、テスト用の固定サンプルを組み立てる
//! `sample_fixed_pipeline` もこのファイルに同居させている。

#[rustfmt::skip]
graphite::graph_schema! {
    schema BuildPipeline {
        node Task { name: String, cmd: String, secs: u32 }
        node Artifact { path: String }

        edge produces: Task -> Artifact (0..*);
        edge consumes: Task -> Artifact (0..*);
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
            fetch: Task { name: "fetch".into(), cmd: "cargo fetch".into(), secs: 10 },
            build: Task { name: "build".into(), cmd: "cargo build".into(), secs: 60 },
            test:  Task { name: "test".into(), cmd: "cargo test".into(), secs: 30 },

            index: Artifact { path: "vendor/registry-index".into() },
            rlib:  Artifact { path: "target/core.rlib".into() },

            fetch -[produces]-> index,
            build -[consumes]-> index,
            build -[produces]-> rlib,
            test  -[consumes]-> rlib,
        })
        .expect("正常な固定パイプラインは構築に成功するはず");

        assert_eq!(g.task_ids().count(), 3);
        assert_eq!(g.artifact_ids().count(), 2);

        let produced: Vec<&Artifact> = g.produces(&TaskId("build".to_string()));
        assert_eq!(produced.len(), 1);
        assert_eq!(produced[0].path, "target/core.rlib");

        let consumed: Vec<&Artifact> = g.consumes(&TaskId("test".to_string()));
        assert_eq!(consumed.len(), 1);
        assert_eq!(consumed[0].path, "target/core.rlib");
    }
}
