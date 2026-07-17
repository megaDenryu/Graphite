// v4.2 (`docs/node_id_v4_2.md`): ノードキー型 (`PersonId`) はもう
// `graph_schema!` が生成しない。ユーザーが `{ノード型名}Id` という命名規約で
// 型の隣に宣言する必要があり、宣言し忘れると素の rustc の
// 「cannot find type `PersonId`」が出るはず。
//
// このエラーのスパンは (`schema_codegen::NodeInfo::new` の実装により)
// `node Person;` のノード型トークン自身なので、このスパンが
// 妥当な位置 (schema 宣言の `Person`) を指していることも確認する。

pub struct Person {
    pub name: String,
}

graphite::graph_schema! {
    schema Missing {
        node Person;
    }
}

fn main() {}
