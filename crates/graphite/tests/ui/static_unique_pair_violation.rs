// 静的グラフのinstanceが対一意制約 (`unique pair`) を満たさないときの診断を
// 固定する (issue #41 段階4)。無向辺は端点の順序に依らず正規化して比較する
// ため、`太郎-次郎` と `次郎-太郎` は同じ対として重複扱いになる
// (examples/static-org/src/main.rs が手動で確認していた実測、issue #24
// 段階2、と同じ文言)。

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

fn 社員を作る() -> 社員 {
    社員
}

組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 太郎: 社員 = 社員を作る();
    node 次郎: 社員 = 社員を作る();
    node 開発部: 部署;
    edge 太郎の所属 = 所属(太郎 -> 開発部);
    edge 次郎の所属 = 所属(次郎 -> 開発部);
    edge 太郎と次郎 = 友人(太郎 -- 次郎);
    edge 次郎と太郎 = 友人(次郎 -- 太郎);
}

fn main() {}
