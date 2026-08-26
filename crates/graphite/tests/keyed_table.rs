//! キー付き要素表の挿入・検索・走査・内部位置の契約を検査する。

use graphite::{KeyedTable, TablePosition};

#[test]
fn insert_と_get() {
    let mut t: KeyedTable<String, i32> = KeyedTable::new();
    assert!(t.insert("a".to_string(), 1));
    assert_eq!(t.get(&"a".to_string()), Some(&1));
    assert_eq!(t.get(&"b".to_string()), None);
}

#[test]
fn insert_は重複キーでfalseを返す() {
    let mut t: KeyedTable<String, i32> = KeyedTable::new();
    assert!(t.insert("a".to_string(), 1));
    assert!(!t.insert("a".to_string(), 2));
    // 元の値は上書きされない。
    assert_eq!(t.get(&"a".to_string()), Some(&1));
}

#[test]
fn ids_iter_len_is_empty() {
    let mut t: KeyedTable<String, i32> = KeyedTable::new();
    assert!(t.is_empty());
    t.insert("a".to_string(), 1);
    t.insert("b".to_string(), 2);
    assert_eq!(t.len(), 2);
    assert!(!t.is_empty());

    let mut ids: Vec<&String> = t.ids().collect();
    ids.sort();
    assert_eq!(ids, vec![&"a".to_string(), &"b".to_string()]);

    let mut pairs: Vec<(&String, &i32)> = t.iter().collect();
    pairs.sort();
    assert_eq!(pairs, vec![(&"a".to_string(), &1), (&"b".to_string(), &2)]);
}

#[test]
fn contains_key() {
    let mut t: KeyedTable<String, i32> = KeyedTable::new();
    t.insert("a".to_string(), 1);
    assert!(t.contains_key(&"a".to_string()));
    assert!(!t.contains_key(&"b".to_string()));
}

#[test]
fn 内部位置から同じ要素を参照できる() {
    let mut table: KeyedTable<String, i32> = KeyedTable::new();
    table.insert("a".to_string(), 1);
    table.insert("b".to_string(), 2);

    let position = table.position(&"b".to_string()).unwrap();
    assert_eq!(table.get_at(position), Some((&"b".to_string(), &2)));
    assert_eq!(
        table.positions().collect::<Vec<_>>(),
        vec![TablePosition(0), TablePosition(1)]
    );
}

#[test]
fn get_mutは値だけを更新する() {
    let mut table: KeyedTable<String, i32> = KeyedTable::new();
    table.insert("a".to_string(), 1);

    *table.get_mut(&"a".to_string()).unwrap() = 2;

    assert_eq!(table.get(&"a".to_string()), Some(&2));
    assert_eq!(table.position(&"a".to_string()), Some(TablePosition(0)));
}
