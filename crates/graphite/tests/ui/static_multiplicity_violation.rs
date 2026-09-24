// 静的グラフのinstanceが多重度制約 (`each member: 1`) を満たさないときの
// 診断を固定する (issue #41 段階4)。examples/static-org/src/main.rs が手動で
// 確認していた実測 (issue #24 段階2) と同じ文言を、trybuild で機械的に
// 固定する。

struct 社員;
struct 部署;

#[allow(non_snake_case)]
mod 組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
        8180126031996082553,
        3838496350105211698,
        17246313587057889135,
        14594317073826595331,
    ];
}

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
        edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair;
    }
}

// 太郎 は `所属` の member として一度も登場しないため、`each member: 1`
// (下限1) 制約に対して本数が0件になる。
組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 太郎: 社員;
    node 開発部: 部署;
}

fn main() {}
