// instanceの具体辺名が固定語彙 `entity` だと、端点個体の具体参照が実体を
// 取り出す固定メソッド `entity()` と衝突する (issue #41 生成される名前の
// 公開契約、PR #45レビューE)。

struct 社員;
struct 部署;

#[allow(non_snake_case)]
mod 組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
        4854036605852261083,
        12849576861286030130,
        1359702280471291685,
        4490724032744592217,
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
    edge entity = 所属(太郎 -> 開発部);
}

fn main() {}
