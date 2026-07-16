//! エッジアクセス API のビュー型。
//!
//! 設計の一次資料: `docs/edge_view_api.md`。
//!
//! `graph_schema!` はラベルごとに、以下 6 種のジェネリックビュー型のいずれか
//! 1 つを返す薄いメソッド `pub fn {label}(&self) -> EdgeXxx<'_, ...>` だけを
//! 生成する (`crates/graphite-macros/src/schema_codegen.rs` の
//! `gen_edge_accessor` 参照)。旧版にあった `try_{label}`/`{label}_id`/
//! `try_{label}_id`/`{label}_ids`/`{label}_pairs` という「ラベル名の文字列
//! 連結によるメソッド群の合成」は全廃し、操作の語彙はこのモジュールの
//! ビュー型 (全 schema・全ラベル共通) に集約した。rustdoc もここに 1 回だけ
//! 書く (`# Panics` を含む)。
//!
//! | 型 | 多重度 | 属性 | 内部表 |
//! |---|---|---|---|
//! | [`EdgeOne`] | (1) | なし | `HashMap<F, T>` |
//! | [`EdgeOneWith`] | (1) | あり | `HashMap<F, (T, A)>` |
//! | [`EdgeOption`] | (0..1) | なし | `HashMap<F, T>` |
//! | [`EdgeOptionWith`] | (0..1) | あり | `HashMap<F, (T, A)>` |
//! | [`EdgeMany`] | (0..*) | なし | `HashMap<F, Vec<T>>` |
//! | [`EdgeManyWith`] | (0..*) | あり | `HashMap<F, Vec<(T, A)>>` |
//!
//! (`F` = 始点キー型、`T` = 終点キー型、`To` = 相手ノード値の型、
//! `A` = エッジ属性型)
//!
//! # 語彙 (覚えるのはこれだけ)
//!
//! - **`of(&from_id)`** — そのラベルの自然な戻り値。**多重度が型を決める**:
//!   `(1)` → 参照そのもの (未知キーはパニック、`# Panics` 明記)、
//!   `(0..1)` → `Option`、`(0..*)` → `Vec`。属性ありエッジは相手が
//!   `(&To, &Attrs)` のタプルになる。
//! - **`get(&from_id)`** — `of` の `Option` 版。**`(1)` のビューにのみ
//!   存在する** (`(0..1)`/`(0..*)` は `of` が既に全域関数なので生成しない)。
//! - **`id_of`/`get_id`/`ids_of`** — 相手のノード値ではなくキーが欲しいとき。
//!   `of`/`get` と同じ多重度規則 (`(0..*)` は複数形 `ids_of`)。
//! - **`iter()`** — 表全体を辺単位で走査する。属性なしは
//!   `(&F, &T)` の2つ組、属性ありは `(&F, &T, &A)` の3つ組。`(0..*)` の
//!   記述順保証 (構築時の追加順を保持) は維持する。
//! - **`len()`/`is_empty()`** — 表の辺の本数 (`(0..*)` は始点キーごとの
//!   終点数の総和)。
//!
//! 各ビューは「エッジ表への参照」と「相手ノードのストレージへの参照」の
//! 2 つの `&'g` を持つだけの借用ラッパーであり、メソッドは全て inline 可能
//! (`docs/design_principles.md` 原則5: ゼロコスト志向)。返す参照・イテレータ
//! の要素は全てビュー自身の `'g` に紐付き、`&self` の借用には縛られない。

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// 多重度 (1)・属性なしのエッジビュー。
///
/// 内部表は `HashMap<F, T>` (始点キー → 終点キー)。
pub struct EdgeOne<'g, F, T, To> {
    table: &'g HashMap<F, T>,
    nodes: &'g HashMap<T, To>,
    label: &'static str,
    schema: &'static str,
}

impl<'g, F, T, To> EdgeOne<'g, F, T, To>
where
    F: Eq + Hash + Debug,
    T: Eq + Hash,
{
    /// `graph_schema!` の生成コードが使う構築関数。利用者がこれを直接
    /// 呼ぶことは想定しない (schema struct のアクセサメソッド経由で使う)。
    pub fn new(
        table: &'g HashMap<F, T>,
        nodes: &'g HashMap<T, To>,
        label: &'static str,
        schema: &'static str,
    ) -> Self {
        Self {
            table,
            nodes,
            label,
            schema,
        }
    }

    fn panic_unknown(&self, method: &str, from: &F) -> ! {
        panic!(
            "{}().{}: 未知のキーです (この{}が発行したキーではありません): {:?}",
            self.label, method, self.schema, from
        )
    }

    /// 多重度 (1) → 相手ノードの値そのものを返す。
    ///
    /// # Panics
    /// `from` がこのグラフに存在しない (このグラフが発行したものではない)
    /// キーの場合パニックする。これは入力検証の欠如ではなく呼び出し規約の
    /// 違反であり (`docs/design_principles.md` 原則2)、非パニック版
    /// [`Self::get`] も併せて提供する。
    pub fn of(&self, from: &F) -> &'g To {
        let to_id = self
            .table
            .get(from)
            .unwrap_or_else(|| self.panic_unknown("of", from));
        &self.nodes[to_id]
    }

    /// [`Self::of`] の非パニック版。未知キーは (パニックせず) `None` を返す。
    pub fn get(&self, from: &F) -> Option<&'g To> {
        self.table.get(from).map(|to_id| &self.nodes[to_id])
    }

    /// 相手ノードの値ではなくキーを返す。
    ///
    /// # Panics
    /// [`Self::of`] と同じ契約 (未知キーでパニック)。
    pub fn id_of(&self, from: &F) -> &'g T {
        self.table
            .get(from)
            .unwrap_or_else(|| self.panic_unknown("id_of", from))
    }

    /// [`Self::id_of`] の非パニック版。
    pub fn get_id(&self, from: &F) -> Option<&'g T> {
        self.table.get(from)
    }

    /// 表全体を (始点キー, 終点キー) の2つ組で走査する。
    pub fn iter(&self) -> impl Iterator<Item = (&'g F, &'g T)> {
        self.table.iter()
    }

    /// この表に含まれる辺の本数。
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// 辺が1本も無いか。
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

/// 多重度 (1)・属性ありのエッジビュー。
///
/// 内部表は `HashMap<F, (T, A)>` (始点キー → (終点キー, 属性))。
pub struct EdgeOneWith<'g, F, T, To, A> {
    table: &'g HashMap<F, (T, A)>,
    nodes: &'g HashMap<T, To>,
    label: &'static str,
    schema: &'static str,
}

impl<'g, F, T, To, A> EdgeOneWith<'g, F, T, To, A>
where
    F: Eq + Hash + Debug,
    T: Eq + Hash,
{
    /// `graph_schema!` の生成コードが使う構築関数。利用者がこれを直接
    /// 呼ぶことは想定しない。
    pub fn new(
        table: &'g HashMap<F, (T, A)>,
        nodes: &'g HashMap<T, To>,
        label: &'static str,
        schema: &'static str,
    ) -> Self {
        Self {
            table,
            nodes,
            label,
            schema,
        }
    }

    fn panic_unknown(&self, method: &str, from: &F) -> ! {
        panic!(
            "{}().{}: 未知のキーです (この{}が発行したキーではありません): {:?}",
            self.label, method, self.schema, from
        )
    }

    /// 多重度 (1) + 属性あり → `(&相手ノード値, &属性)` を返す。
    ///
    /// # Panics
    /// `from` がこのグラフに存在しないキーの場合パニックする
    /// (`docs/design_principles.md` 原則2)。非パニック版は [`Self::get`]。
    pub fn of(&self, from: &F) -> (&'g To, &'g A) {
        let (to_id, attrs) = self
            .table
            .get(from)
            .unwrap_or_else(|| self.panic_unknown("of", from));
        (&self.nodes[to_id], attrs)
    }

    /// [`Self::of`] の非パニック版。未知キーは `None` を返す。
    pub fn get(&self, from: &F) -> Option<(&'g To, &'g A)> {
        let (to_id, attrs) = self.table.get(from)?;
        Some((&self.nodes[to_id], attrs))
    }

    /// 相手ノードの値ではなくキーを返す (属性は含まない。属性が欲しい場合は
    /// [`Self::of`]/[`Self::get`] を使う)。
    ///
    /// # Panics
    /// [`Self::of`] と同じ契約 (未知キーでパニック)。
    pub fn id_of(&self, from: &F) -> &'g T {
        let (to_id, _attrs) = self
            .table
            .get(from)
            .unwrap_or_else(|| self.panic_unknown("id_of", from));
        to_id
    }

    /// [`Self::id_of`] の非パニック版。
    pub fn get_id(&self, from: &F) -> Option<&'g T> {
        self.table.get(from).map(|(to_id, _attrs)| to_id)
    }

    /// 表全体を (始点キー, 終点キー, 属性) の3つ組で走査する。
    pub fn iter(&self) -> impl Iterator<Item = (&'g F, &'g T, &'g A)> {
        self.table.iter().map(|(from, (to, attrs))| (from, to, attrs))
    }

    /// この表に含まれる辺の本数。
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// 辺が1本も無いか。
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

/// 多重度 (0..1)・属性なしのエッジビュー。
///
/// 内部表は `HashMap<F, T>` (始点キー → 終点キー)。`(1)` と異なり無い/未知
/// キーはどちらも `None` に落ちるため、`of`/`id_of` は非パニックの全域関数
/// であり、対になる `get`/`get_id` は生成しない
/// (`docs/edge_view_api.md` §2 参照)。
pub struct EdgeOption<'g, F, T, To> {
    table: &'g HashMap<F, T>,
    nodes: &'g HashMap<T, To>,
}

impl<'g, F, T, To> EdgeOption<'g, F, T, To>
where
    F: Eq + Hash,
    T: Eq + Hash,
{
    /// `graph_schema!` の生成コードが使う構築関数。利用者がこれを直接
    /// 呼ぶことは想定しない。
    pub fn new(table: &'g HashMap<F, T>, nodes: &'g HashMap<T, To>) -> Self {
        Self { table, nodes }
    }

    /// 多重度 (0..1) → `Option<&相手ノード値>`。無い/未知キーはどちらも
    /// `None` に落ちる (「無い」ことが正常なドメイン状態なのでパニックしない)。
    pub fn of(&self, from: &F) -> Option<&'g To> {
        self.table.get(from).map(|to_id| &self.nodes[to_id])
    }

    /// 相手ノードの値ではなくキーを `Option` で返す。
    pub fn id_of(&self, from: &F) -> Option<&'g T> {
        self.table.get(from)
    }

    /// 表全体を (始点キー, 終点キー) の2つ組で走査する。
    pub fn iter(&self) -> impl Iterator<Item = (&'g F, &'g T)> {
        self.table.iter()
    }

    /// この表に含まれる辺の本数。
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// 辺が1本も無いか。
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

/// 多重度 (0..1)・属性ありのエッジビュー。
///
/// 内部表は `HashMap<F, (T, A)>`。
pub struct EdgeOptionWith<'g, F, T, To, A> {
    table: &'g HashMap<F, (T, A)>,
    nodes: &'g HashMap<T, To>,
}

impl<'g, F, T, To, A> EdgeOptionWith<'g, F, T, To, A>
where
    F: Eq + Hash,
    T: Eq + Hash,
{
    /// `graph_schema!` の生成コードが使う構築関数。利用者がこれを直接
    /// 呼ぶことは想定しない。
    pub fn new(table: &'g HashMap<F, (T, A)>, nodes: &'g HashMap<T, To>) -> Self {
        Self { table, nodes }
    }

    /// 多重度 (0..1) + 属性あり → `Option<(&相手ノード値, &属性)>`。
    pub fn of(&self, from: &F) -> Option<(&'g To, &'g A)> {
        let (to_id, attrs) = self.table.get(from)?;
        Some((&self.nodes[to_id], attrs))
    }

    /// 相手ノードの値ではなくキーを `Option` で返す (属性は含まない)。
    pub fn id_of(&self, from: &F) -> Option<&'g T> {
        self.table.get(from).map(|(to_id, _attrs)| to_id)
    }

    /// 表全体を (始点キー, 終点キー, 属性) の3つ組で走査する。
    pub fn iter(&self) -> impl Iterator<Item = (&'g F, &'g T, &'g A)> {
        self.table.iter().map(|(from, (to, attrs))| (from, to, attrs))
    }

    /// この表に含まれる辺の本数。
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// 辺が1本も無いか。
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

/// 多重度 (0..*)・属性なしのエッジビュー。
///
/// 内部表は `HashMap<F, Vec<T>>`。同一始点キーに対する複数終点の相対順序は
/// 構築時の追加順 (`graph!` の場合はソース中の記述順) を保持する
/// (`docs/edge_view_api.md` §2「`iter()`」節)。
pub struct EdgeMany<'g, F, T, To> {
    table: &'g HashMap<F, Vec<T>>,
    nodes: &'g HashMap<T, To>,
}

impl<'g, F, T, To> EdgeMany<'g, F, T, To>
where
    F: Eq + Hash,
    T: Eq + Hash,
{
    /// `graph_schema!` の生成コードが使う構築関数。利用者がこれを直接
    /// 呼ぶことは想定しない。
    pub fn new(table: &'g HashMap<F, Vec<T>>, nodes: &'g HashMap<T, To>) -> Self {
        Self { table, nodes }
    }

    /// 多重度 (0..*) → `Vec<&相手ノード値>`。無い/未知キーはどちらも空
    /// `Vec` に落ちる。格納順 (構築時の追加順) を保持する。
    pub fn of(&self, from: &F) -> Vec<&'g To> {
        match self.table.get(from) {
            Some(ids) => ids.iter().map(|to_id| &self.nodes[to_id]).collect(),
            None => Vec::new(),
        }
    }

    /// 相手ノードの値ではなくキーの列を `Vec` で返す。無い/未知キーはどちらも
    /// 空。格納順を保持する。
    pub fn ids_of(&self, from: &F) -> Vec<&'g T> {
        match self.table.get(from) {
            Some(ids) => ids.iter().collect(),
            None => Vec::new(),
        }
    }

    /// 表全体を (始点キー, 終点キー) の2つ組で走査する。多重度 `(0..*)` は
    /// 始点ごとの複数終点へ展開する。
    ///
    /// 順序保証: 同一始点キーに対する複数終点の相対順序は構築時の追加順を
    /// 保持する。ただし始点キーをまたぐ列挙順は保証しない (内部ストレージが
    /// `HashMap` のため)。
    pub fn iter(&self) -> impl Iterator<Item = (&'g F, &'g T)> {
        self.table
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from, to)))
    }

    /// この表に含まれる辺の本数 (始点キーごとの終点数の総和)。
    pub fn len(&self) -> usize {
        self.table.values().map(Vec::len).sum()
    }

    /// 辺が1本も無いか。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// 多重度 (0..*)・属性ありのエッジビュー。
///
/// 内部表は `HashMap<F, Vec<(T, A)>>`。
pub struct EdgeManyWith<'g, F, T, To, A> {
    table: &'g HashMap<F, Vec<(T, A)>>,
    nodes: &'g HashMap<T, To>,
}

impl<'g, F, T, To, A> EdgeManyWith<'g, F, T, To, A>
where
    F: Eq + Hash,
    T: Eq + Hash,
{
    /// `graph_schema!` の生成コードが使う構築関数。利用者がこれを直接
    /// 呼ぶことは想定しない。
    pub fn new(table: &'g HashMap<F, Vec<(T, A)>>, nodes: &'g HashMap<T, To>) -> Self {
        Self { table, nodes }
    }

    /// 多重度 (0..*) + 属性あり → `Vec<(&相手ノード値, &属性)>`。無い/未知
    /// キーはどちらも空 `Vec` に落ちる。格納順を保持する。
    pub fn of(&self, from: &F) -> Vec<(&'g To, &'g A)> {
        match self.table.get(from) {
            Some(items) => items
                .iter()
                .map(|(to_id, attrs)| (&self.nodes[to_id], attrs))
                .collect(),
            None => Vec::new(),
        }
    }

    /// 相手ノードの値ではなくキーの列を `Vec` で返す (属性は含まない)。
    /// 格納順を保持する。
    pub fn ids_of(&self, from: &F) -> Vec<&'g T> {
        match self.table.get(from) {
            Some(items) => items.iter().map(|(to_id, _attrs)| to_id).collect(),
            None => Vec::new(),
        }
    }

    /// 表全体を (始点キー, 終点キー, 属性) の3つ組で走査する。多重度
    /// `(0..*)` は始点ごとの複数終点へ展開する。順序保証は [`EdgeMany::iter`]
    /// と同じ。
    pub fn iter(&self) -> impl Iterator<Item = (&'g F, &'g T, &'g A)> {
        self.table
            .iter()
            .flat_map(|(from, items)| items.iter().map(move |(to, attrs)| (from, to, attrs)))
    }

    /// この表に含まれる辺の本数 (始点キーごとの終点数の総和)。
    pub fn len(&self) -> usize {
        self.table.values().map(Vec::len).sum()
    }

    /// 辺が1本も無いか。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("dept-1".to_string(), "営業部".to_string());
        m.insert("dept-2".to_string(), "開発部".to_string());
        m
    }

    // --- EdgeOne (1, 属性なし) ---

    #[test]
    fn edge_one_ofは参照そのものを返す() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert("emp-1".to_string(), "dept-1".to_string());
        let view = EdgeOne::new(&table, &nodes, "belongs_to", "Org");

        assert_eq!(view.of(&"emp-1".to_string()), "営業部");
        assert_eq!(view.id_of(&"emp-1".to_string()), "dept-1");
        assert_eq!(view.get(&"emp-1".to_string()), Some(&"営業部".to_string()));
        assert_eq!(view.get(&"emp-x".to_string()), None);
        assert_eq!(view.get_id(&"emp-x".to_string()), None);
        assert_eq!(view.len(), 1);
        assert!(!view.is_empty());
    }

    #[test]
    #[should_panic(expected = "belongs_to().of")]
    fn edge_one_ofは未知キーでパニックする() {
        let nodes = nodes();
        let table: HashMap<String, String> = HashMap::new();
        let view = EdgeOne::new(&table, &nodes, "belongs_to", "Org");
        let _ = view.of(&"emp-x".to_string());
    }

    #[test]
    #[should_panic(expected = "belongs_to().id_of")]
    fn edge_one_id_ofは未知キーでパニックする() {
        let nodes = nodes();
        let table: HashMap<String, String> = HashMap::new();
        let view = EdgeOne::new(&table, &nodes, "belongs_to", "Org");
        let _ = view.id_of(&"emp-x".to_string());
    }

    #[test]
    fn edge_one_iterは表全体を走査する() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert("emp-1".to_string(), "dept-1".to_string());
        table.insert("emp-2".to_string(), "dept-2".to_string());
        let view = EdgeOne::new(&table, &nodes, "belongs_to", "Org");

        let mut pairs: Vec<(&String, &String)> = view.iter().collect();
        pairs.sort();
        assert_eq!(
            pairs,
            vec![
                (&"emp-1".to_string(), &"dept-1".to_string()),
                (&"emp-2".to_string(), &"dept-2".to_string()),
            ]
        );
    }

    // --- EdgeOneWith (1, 属性あり) ---

    #[test]
    fn edge_one_withは属性付きタプルを返す() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert("emp-1".to_string(), ("dept-1".to_string(), 2020_i32));
        let view = EdgeOneWith::new(&table, &nodes, "belongs_to", "Org");

        let (dept, since) = view.of(&"emp-1".to_string());
        assert_eq!(dept, "営業部");
        assert_eq!(*since, 2020);
        assert_eq!(view.id_of(&"emp-1".to_string()), "dept-1");
        assert_eq!(view.get(&"emp-x".to_string()), None);
        assert_eq!(view.get_id(&"emp-1".to_string()), Some(&"dept-1".to_string()));
    }

    #[test]
    #[should_panic(expected = "boss().of")]
    fn edge_one_with_ofは未知キーでパニックする() {
        let nodes = nodes();
        let table: HashMap<String, (String, i32)> = HashMap::new();
        let view = EdgeOneWith::new(&table, &nodes, "boss", "Org");
        let _ = view.of(&"emp-x".to_string());
    }

    #[test]
    fn edge_one_with_iterは3つ組で走査する() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert("emp-1".to_string(), ("dept-1".to_string(), 2020_i32));
        let view = EdgeOneWith::new(&table, &nodes, "boss", "Org");

        let triples: Vec<(&String, &String, &i32)> = view.iter().collect();
        assert_eq!(
            triples,
            vec![(&"emp-1".to_string(), &"dept-1".to_string(), &2020)]
        );
        assert_eq!(view.len(), 1);
    }

    // --- EdgeOption (0..1, 属性なし) ---

    #[test]
    fn edge_optionは無いキーでnoneを返す() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert("emp-1".to_string(), "dept-1".to_string());
        let view = EdgeOption::new(&table, &nodes);

        assert_eq!(view.of(&"emp-1".to_string()), Some(&"営業部".to_string()));
        assert_eq!(view.of(&"emp-x".to_string()), None);
        assert_eq!(view.id_of(&"emp-1".to_string()), Some(&"dept-1".to_string()));
        assert_eq!(view.id_of(&"emp-x".to_string()), None);
        assert_eq!(view.len(), 1);
        assert!(!view.is_empty());
    }

    // --- EdgeOptionWith (0..1, 属性あり) ---

    #[test]
    fn edge_option_withは属性付きoptionを返す() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert("emp-1".to_string(), ("dept-1".to_string(), 2020_i32));
        let view = EdgeOptionWith::new(&table, &nodes);

        let (dept, since) = view.of(&"emp-1".to_string()).expect("存在するはず");
        assert_eq!(dept, "営業部");
        assert_eq!(*since, 2020);
        assert!(view.of(&"emp-x".to_string()).is_none());
        assert_eq!(view.id_of(&"emp-1".to_string()), Some(&"dept-1".to_string()));
    }

    // --- EdgeMany (0..*, 属性なし) ---

    #[test]
    fn edge_manyは追加順を保持したvecを返す() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert(
            "boss-1".to_string(),
            vec!["dept-2".to_string(), "dept-1".to_string()],
        );
        let view = EdgeMany::new(&table, &nodes);

        let vals: Vec<&String> = view.of(&"boss-1".to_string());
        assert_eq!(
            vals,
            vec![&"開発部".to_string(), &"営業部".to_string()]
        );
        let ids: Vec<&String> = view.ids_of(&"boss-1".to_string());
        assert_eq!(ids, vec![&"dept-2".to_string(), &"dept-1".to_string()]);

        assert!(view.of(&"boss-x".to_string()).is_empty());
        assert!(view.ids_of(&"boss-x".to_string()).is_empty());
    }

    #[test]
    fn edge_manyのlenは辺の総本数() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert(
            "boss-1".to_string(),
            vec!["dept-1".to_string(), "dept-2".to_string()],
        );
        table.insert("boss-2".to_string(), vec!["dept-1".to_string()]);
        let view = EdgeMany::new(&table, &nodes);

        assert_eq!(view.len(), 3);
        assert!(!view.is_empty());

        let empty_table: HashMap<String, Vec<String>> = HashMap::new();
        let empty_view = EdgeMany::new(&empty_table, &nodes);
        assert_eq!(empty_view.len(), 0);
        assert!(empty_view.is_empty());
    }

    #[test]
    fn edge_many_iterは始点ごとに展開する() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert(
            "boss-1".to_string(),
            vec!["dept-1".to_string(), "dept-2".to_string()],
        );
        let view = EdgeMany::new(&table, &nodes);

        let pairs: Vec<(&String, &String)> = view.iter().collect();
        assert_eq!(
            pairs,
            vec![
                (&"boss-1".to_string(), &"dept-1".to_string()),
                (&"boss-1".to_string(), &"dept-2".to_string()),
            ]
        );
    }

    // --- EdgeManyWith (0..*, 属性あり) ---

    #[test]
    fn edge_many_withは追加順を保持したタプルvecを返す() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert(
            "sensor-1".to_string(),
            vec![
                ("dept-1".to_string(), 0.5_f64),
                ("dept-2".to_string(), 0.9_f64),
            ],
        );
        let view = EdgeManyWith::new(&table, &nodes);

        let vals = view.of(&"sensor-1".to_string());
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0].0, "営業部");
        assert_eq!(*vals[0].1, 0.5);
        assert_eq!(vals[1].0, "開発部");
        assert_eq!(*vals[1].1, 0.9);

        let ids = view.ids_of(&"sensor-1".to_string());
        assert_eq!(ids, vec![&"dept-1".to_string(), &"dept-2".to_string()]);

        assert!(view.of(&"sensor-x".to_string()).is_empty());
    }

    #[test]
    fn edge_many_with_iterは3つ組で始点ごとに展開する() {
        let nodes = nodes();
        let mut table = HashMap::new();
        table.insert(
            "sensor-1".to_string(),
            vec![("dept-1".to_string(), 0.5_f64)],
        );
        let view = EdgeManyWith::new(&table, &nodes);

        let triples: Vec<(&String, &String, &f64)> = view.iter().collect();
        assert_eq!(
            triples,
            vec![(&"sensor-1".to_string(), &"dept-1".to_string(), &0.5)]
        );
        assert_eq!(view.len(), 1);
        assert!(!view.is_empty());
    }
}
