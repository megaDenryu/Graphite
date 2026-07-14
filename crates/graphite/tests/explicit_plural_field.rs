//! ノード宣言に内部ストレージの複数形フィールド名を明示指定できることを
//! 確認する (`node Category(categories);`)。
//!
//! `plural_field_name` の素朴な `+ "s"` 複数形化では `Category` は
//! `Categorys` になってしまう (README「手書きテンプレートとの差異」節・
//! 「未決事項」節)。省略可能な `(識別子)` 構文でこれを上書きできるように
//! した。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    pub name: String,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema Catalog {
        node Category(categories);
        node Item;

        edge Item -[belongs_to]-> Category (1);
    }
}

// 私有フィールド `categories` (明示指定した複数形) へ同一モジュール内から
// アクセスできることを確認する。もし `plural_field_name` の素朴な複数形化
// (`categorys`) しかサポートしていなければ、このフィールド名では
// コンパイルが通らない。
impl Catalog {
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }
}

#[test]
fn 明示指定した複数形の内部フィールド名でスキーマが構築できる() {
    let g = Catalog::create(|b| {
        b.category(
            CategoryId("c1".to_string()),
            Category {
                name: "小説".to_string(),
            },
        );
        b.item(
            ItemId("i1".to_string()),
            Item {
                name: "銀河鉄道の夜".to_string(),
            },
        );
        b.belongs_to(ItemId("i1".to_string()), CategoryId("c1".to_string()));
    })
    .expect("明示指定した複数形でも正常に構築できるはず");

    assert_eq!(
        g.category(&CategoryId("c1".to_string())).unwrap().name,
        "小説"
    );
    assert_eq!(g.category_count(), 1);
}
