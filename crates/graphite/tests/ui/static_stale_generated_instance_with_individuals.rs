// 個体を持つinstanceで生成ファイルが古いときの診断を固定する (issue #41)。
// `static_stale_generated_instance.rs` は個体0件のため、DSLトークンの
// 型参照・値供給関数の参照が1つも生成されず、指紋照合のE0080だけが出る。
// だが実際に最も多い古さ (個体・辺を足して再生成し忘れる) では、DSLに
// 出現するが古い生成ファイルには無い名前 (この断片では `三郎`) ごとに
// 「型が見つからない」E0425が1件出て、再生成を促すE0080が最後に出る。
// この断片は個体を持つ状態で古い生成ファイルを再現し、実際に出る
// エラーの並びをそのまま固定する。
//
// E0080の後ろにE0061 (`Nodes::new` の引数の個数不一致) がもう1件続く。
// 組み立て関数は、instance側のDSLが持つ個体 (`一郎`・`三郎`) の全件を宣言順に
// `Nodes::new` へ渡そうとするが、この断片の古い `Nodes::new` は `一郎` の
// 1件しか受け取らない (`三郎`を追加する前の古い生成ファイルの形をそのまま
// 模しているため)。再生成を促す結論 (E0080) が最後に出ることに変わりはない。

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
// 手で偽装する)。`Nodes`・`NodeRefs`・`Edges`・`EdgeRefs`・`Graph`・
// `{個体名}Ref` を実物の生成ファイルと同じ形で揃え、`Nodes`が丸ごと
// 見つからないという実物には起こらない誤りを固定しないようにする。
// `new` も実物と同じ形 (全個体・全積み荷を引数に取る) で揃え、組み立て
// 関数の呼び出しがこの誤りとは無関係な `no function new` を混ぜないように
// する。
// instance側のDSLは `一郎` に加えて新しい個体 `三郎` を足した状態にし、
// 生成ファイルの再生成を忘れた状況を再現する。
#[allow(non_snake_case, dead_code)]
mod 開発チーム {
    pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [0, 0, 0, 0];

    pub struct Nodes {
        pub 一郎: super::社員,
    }
    impl Nodes {
        pub fn new(一郎: super::社員) -> Self {
            Nodes { 一郎 }
        }
    }

    pub struct 一郎Ref<'a> {
        pub(super) entity: &'a super::社員,
    }

    pub struct NodeRefs<'a> {
        pub 一郎: 一郎Ref<'a>,
    }

    pub struct Edges<'a> {
        pub(super) _marker: std::marker::PhantomData<&'a ()>,
    }
    impl<'a> Edges<'a> {
        pub fn new(_nodes: &'a Nodes) -> Self {
            Edges { _marker: std::marker::PhantomData }
        }
    }

    pub struct EdgeRefs<'a> {
        pub(super) _marker: std::marker::PhantomData<&'a ()>,
    }

    pub struct Graph<'a> {
        pub node_refs: NodeRefs<'a>,
        pub edge_refs: EdgeRefs<'a>,
    }
}

組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 一郎: 社員 = 社員;
    node 三郎: 社員 = 社員;
}

fn main() {}
