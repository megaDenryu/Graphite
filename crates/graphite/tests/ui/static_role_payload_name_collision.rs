// schemaの端点役割名と積み荷役割名が同名だと、生成される辺値structの
// フィールド・{辺名}Refのアクセサが衝突する (issue #41 生成される名前の
// 公開契約、PR #45レビューE)。schema単体の検証であり、instanceの宣言は不要。

struct 社員;
struct 任命記録;

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
        edge 上司 = (subordinate: 社員) -[subordinate: 任命記録]-> (superior: 社員);
    }
}

fn main() {}
