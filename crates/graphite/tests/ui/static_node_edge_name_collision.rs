// instanceのnode名とedge名が同名だと、生成される具体個体参照・具体辺参照が
// どちらも `{名前}Ref` という同じRust識別子になり衝突する (issue #41
// 生成される名前の公開契約)。

struct 社員;
struct 部署;

#[allow(non_snake_case)]
mod 組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
        18051491707423330504,
        10655855739410930067,
        13880085540787354614,
        3139610238445213546,
    ];
}

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署);
    }
}

fn 社員を作る() -> 社員 {
    社員
}

組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 太郎: 社員 = 社員を作る();
    node 開発部: 部署;
    edge 太郎 = 所属(太郎 -> 開発部);
}

fn main() {}
