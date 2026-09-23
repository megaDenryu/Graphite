// 個体を持つinstanceで生成ファイルが古いときの診断を固定する (issue #41
// 検収指摘2)。`static_stale_generated_instance.rs` は個体0件のため
// DSLトークンの型参照・値供給関数の参照が1つも生成されず、指紋照合の
// E0080だけが出る。だが実際に最も多い古さ (個体・辺を足して再生成し忘れる)
// では、DSLトークンの型参照が先に「型が見つからない」エラーを複数件出し、
// 再生成を促すE0080は最後に出る。この断片は個体を持つ状態で古い生成
// ファイルを再現し、実際に出るエラーの並びをそのまま固定する。

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
// 手で偽装する)。instance側のDSLは `一郎` に加えて新しい個体 `三郎` を
// 足した状態にし、生成ファイルの再生成を忘れた状況を再現する。
#[allow(non_snake_case)]
mod 開発チーム {
    pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [0, 0, 0, 0];

    pub struct 一郎Ref<'a> {
        pub(super) entity: &'a super::社員,
    }
}

組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 一郎: 社員 = 社員;
    node 三郎: 社員 = 社員;
}

fn main() {}
