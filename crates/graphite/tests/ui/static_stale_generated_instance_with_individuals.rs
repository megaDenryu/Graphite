// 個体を持つinstanceで生成ファイルが古いときの診断を固定する (issue #41)。
// `static_stale_generated_instance.rs` は個体0件のため、DSLトークンの
// 型参照・値供給関数の参照が1つも生成されず、指紋照合のE0080だけが出る。
// だが実際に最も多い古さ (個体・辺を足して再生成し忘れる) では、DSLに
// 出現するが古い生成ファイルには無い名前 (この断片では `三郎`) ごとに
// 「型が見つからない」E0425が1件出て、再生成を促すE0080が最後に出る。
// この断片は個体を持つ状態で古い生成ファイルを再現し、実際に出る
// エラーの並びをそのまま固定する。
//
// instance宣言だけでは構築の入口 (`construct!`) の内部構築子を呼ばない
// ため、このテストのようにinstance宣言だけを書いて構築を呼ばない場面では
// E0061 (引数の個数不一致) は出ない。再生成を促す結論 (E0080) が最後に出る。

struct 社員;

#[allow(non_snake_case)]
mod 組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
        10319720238056268413,
        1420965213717410858,
        810849268007507131,
        13845130453493871279,
    ];
}

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
    }
}

// `開発チーム` moduleは、個体 `一郎` だけを持つ古い生成ファイルを模する
// (実際には `cargo graphite generate` が書く構造だが、このテストでは
// 手で偽装する)。`一郎Ref`・`NodeRefs`・`EdgeRefs`・`Graph`・内部構築子
// `__graphite_internal_new` を実物の生成ファイルと同じ形 (個体・積み荷を
// `Graph`自身のフィールドへ直接持つ、`graph_struct.rs`参照) で揃え、
// `Graph`が丸ごと見つからないという実物には起こらない誤りを固定しない
// ようにする。`__graphite_internal_new`も実物と同じ形 (全個体・全積み荷を
// 引数に取る) で揃え、構築呼び出しがこの誤りとは無関係な
// `no function __graphite_internal_new` を混ぜないようにする。
// instance側のDSLは `一郎` に加えて新しい個体 `三郎` を足した状態にし、
// 生成ファイルの再生成を忘れた状況を再現する。
#[allow(non_snake_case, dead_code)]
mod 開発チーム {
    pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [0, 0, 0, 0];

    pub struct 一郎Ref<'a> {
        graph: &'a Graph,
    }
    impl<'a> 一郎Ref<'a> {
        pub fn entity(&self) -> &'a super::社員 {
            &self.graph.一郎
        }
    }

    pub struct NodeRefs<'a> {
        graph: &'a Graph,
    }
    impl<'a> NodeRefs<'a> {
        pub fn 一郎(&self) -> 一郎Ref<'a> {
            一郎Ref { graph: self.graph }
        }
    }

    pub struct EdgeRefs<'a> {
        graph: &'a Graph,
    }

    pub struct Graph {
        一郎: super::社員,
    }
    impl Graph {
        pub(crate) fn __graphite_internal_new(一郎: super::社員) -> Self {
            Graph { 一郎 }
        }
        pub fn node_refs(&self) -> NodeRefs<'_> {
            NodeRefs { graph: self }
        }
        pub fn edge_refs(&self) -> EdgeRefs<'_> {
            EdgeRefs { graph: self }
        }
    }
}

組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 一郎: 社員 = 社員;
    node 三郎: 社員 = 社員;
}

fn main() {}
