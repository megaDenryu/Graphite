// 明示したID型をわざと宣言しない。既定ID生成とは混同せず、指定した型が
// 見つからない診断になることを固定する。

pub struct Person {
    pub name: String,
}

graphite::__graph_schema_inline_for_test! {
    schema Missing {
        node Person(id: MissingPersonId);
    }
}

fn main() {}
