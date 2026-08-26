//! 汎用Graphのテストが共有する人物ノードの標本データを提供する。

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn sample_people() -> Vec<(String, Person)> {
    vec![
        (
            "田中".to_string(),
            Person {
                name: "田中".to_string(),
                age: 30,
            },
        ),
        (
            "佐藤".to_string(),
            Person {
                name: "佐藤".to_string(),
                age: 25,
            },
        ),
        (
            "鈴木".to_string(),
            Person {
                name: "鈴木".to_string(),
                age: 40,
            },
        ),
    ]
}
