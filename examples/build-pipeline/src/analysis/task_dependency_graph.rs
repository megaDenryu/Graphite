//! タスク依存グラフの射影 (`consumes ∘ produces⁻¹`)。

use std::collections::HashMap;

use graphite::Graph;

use crate::schema::{ArtifactId, BuildPipeline, TaskId};

/// タスク依存グラフ (`consumes ∘ produces⁻¹` の射影)。
///
/// 辺 `producer -> consumer` は「`producer` が生成した成果物を `consumer`
/// が読み込む (=`producer` は `consumer` より先に実行されなければならない)」
/// ことを表す。ノード値・辺値は不要 (依存関係の形だけが要る) なので両方 `()`
/// にし、キー型だけ `TaskId` にしている。
pub type TaskDependencyGraph = Graph<(), (), TaskId>;

/// [`BuildPipeline`] からタスク依存グラフを射影する。
///
/// `g.produces_iter()`/`g.consumes_iter()` はどちらも `BuildPipeline` の生成物
/// (図式グラフのクエリ API) であり、ここで初めて「タスク間の順序」という
/// 導出情報を組み立てる。エッジの終点キーは常に `g.task_ids()` 由来なので
/// `Graph::build` が `UnknownEndpoint` を返すことはない (`expect` で妥当)。
pub fn task_dependency_graph(g: &BuildPipeline::Graph) -> TaskDependencyGraph {
    let mut producers_of: HashMap<&ArtifactId, Vec<&TaskId>> = HashMap::new();
    for edge in g.produces_iter() {
        producers_of
            .entry(edge.artifact().id())
            .or_default()
            .push(edge.task().id());
    }

    let nodes: Vec<(TaskId, ())> = g.task_ids().map(|id| (id.clone(), ())).collect();

    // `flat_map` にすると内側のイテレータが `producers_of` への借用を
    // `FnMut` クロージャの呼び出しをまたいで持ち越そうとしてしまい
    // 借用検査器に拒否される (「呼び出し毎に排他アクセスを得る」という
    // `FnMut` の性質上、その借用は呼び出しの外へ逃がせない)。ループで
    // 即座に `Vec` へ確定させることで回避する。
    let mut edges: Vec<(TaskId, TaskId, ())> = Vec::new();
    for edge in g.consumes_iter() {
        let consumer = edge.task().id();
        let artifact = edge.artifact().id();
        if let Some(producers) = producers_of.get(artifact) {
            for producer in producers {
                edges.push(((*producer).clone(), consumer.clone(), ()));
            }
        }
    }

    Graph::build(nodes, edges)
        .expect("g.タスク依存グラフの辺の端点は必ずtask_ids()由来なので未知キーにはならない")
}
