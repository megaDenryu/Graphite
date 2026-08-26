//! 完成済みシナリオグラフへの導出クエリ。
//!
//! README.md 「使用例3」節のパターン (保存エッジ=フィールド、導出エッジ=
//! 普通のメソッド) を、`schema.rs` が宣言した `DialogueGraph::Graph` への
//! 追加の `impl` として書く。

use graphite::Graph;

use crate::schema::{DialogueGraph, SceneId};

impl DialogueGraph::Graph {
    /// あるシーンから出ている選択肢一覧を `(行き先キー, 選択肢ラベル)` で返す。
    /// `choice_as_scene()` は `SceneRef` を要求する (`SceneId` から呼ぶには
    /// `scene_by_id` の追加の検索が要る) ため、ここでは生の辺 (キー付き) を
    /// 走査する `choice_iter` をフィルタして使う。
    pub fn scene_choices(&self, id: &SceneId) -> Vec<(SceneId, String)> {
        self.choice_iter()
            .filter(|edge| edge.scene().id() == id)
            .map(|edge| (edge.next().id().clone(), edge.payload().label.clone()))
            .collect()
    }

    /// choice 辺だけを汎用グラフ `Graph<SceneId, String, SceneId>` へ射影する。
    /// `reachable_from`/`has_cycle`/`path`/`filter_nodes` のような、図式グラフ
    /// (`graph_schema!`) には無いグラフアルゴリズムを使うための橋渡し。
    /// ノードの値には (使わないが) キー自身を積んでおく。辺の値には選択肢
    /// ラベルを積み、`route` コマンドでの表示に使う。
    ///
    /// 構築は `Scene` の集合と `Choice` 辺だけから機械的に決まるため、
    /// このシナリオが `DialogueGraph::create` を通過している時点で
    /// 重複キー・未知キーは有り得ず、`expect` で握り潰してよい。
    pub fn scene_graph(&self) -> Graph<SceneId, String, SceneId> {
        Graph::create(|b| {
            for id in self.scene_ids() {
                b.node(id.clone(), id.clone());
            }
            for edge in self.choice_iter() {
                b.edge(
                    edge.scene().id().clone(),
                    edge.next().id().clone(),
                    edge.payload().label.clone(),
                );
            }
        })
        .expect("scene_graph の射影は DialogueGraph が既に検証済みなので必ず成功する")
    }

    /// このシーンに finale (エンディングへの到達) があるか。
    pub fn is_finale_scene(&self, id: &SceneId) -> bool {
        self.scene_by_id(id)
            .and_then(|scene| scene.finale_as_scene())
            .is_some()
    }

    /// このシーンに選択肢が 0 本、かつ finale も無いか (= デッドエンド)。
    pub fn is_dead_end(&self, id: &SceneId) -> bool {
        self.scene_by_id(id).is_none_or(|scene| {
            scene.choice_as_scene().next().is_none() && scene.finale_as_scene().is_none()
        })
    }
}
