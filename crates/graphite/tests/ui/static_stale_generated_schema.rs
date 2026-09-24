// 静的グラフのschemaの生成ファイルの指紋が古いときの診断を固定する
// (issue #41 段階4)。`stale_generated_schema.rs` (動的グラフ) と同じ手法で、
// 生成ファイルへ手で偽の指紋定数を埋め込む。

struct 社員;

#[allow(non_snake_case)]
mod 組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [0, 0, 0, 0];
}

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
    }
}

fn main() {}
