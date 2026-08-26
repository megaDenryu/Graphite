//! `graph_schema!` の入力 DSL のパース (構文木を組み立てるだけで、
//! ノード型の重複や未宣言参照といった意味検査は `schema_validate.rs` で行う)。
//!
//! 対応する文法 (v4、`docs/schema_v4.md` §1 参照):
//!
//! ```text
//! pub struct Person { pub name: String }
//! pub struct Team { pub name: String }
//! pub struct BossEdge { pub since: i32 }
//!
//! graphite::graph_schema! {
//!     schema Org {
//!         node Person;
//!         node Team(id: ExistingTeamId);
//!
//!         edge BelongsTo = (member: Person) -> (team: Team) where each member: 1;
//!         edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;
//!         edge DependsOn = (dependent: Service) -> (dependency: Service) where unique pair;
//!         edge Assigned = (person: Person) -[role: Role]-> (project: Project);
//!     }
//! }
//! ```
//!
//! ノード型・エッジ属性型はいずれも `graph_schema!` の外でユーザーが普通の
//! struct として宣言したものを参照するだけで、このマクロは生成しない。
//! `node Type;` と `edge Kind = ...;` は schema module 内にそれぞれ
//! `TypeId(String)` と `KindId(String)` を生成する。既存ID型を使う場合は
//! `node Type(id: 型パス);` / `edge Kind(id: 型パス) = ...;` と明示する。
//! ノード型名は端点照合に使うため単純 `Ident` のみ (モジュール修飾したい
//! 場合は `use` で名前をこのスコープに持ち込む)。エッジ属性型は照合には
//! 使わず参照するだけなので `syn::Path` (モジュール修飾可) を許す。
//!
//! 有向辺宣言は `edge Kind = (役割名: From) -> (役割名: To) (where ...)?;`
//! (積み荷なし) または `edge Kind = (役割名: From) -[役割名: Attrs]-> (役割名: To)`
//! (積み荷あり) の形。
//! **`Kind` は新しい nominal 型として生成される** (透過的別名ではない)。
//! 旧多重度注釈 `(1)`/`(0..1)`/`(0..*)` は廃止 (字面ごと消滅、検出もしない)。
//!
//! `where` 節はカンマ区切りで複数の制約を書ける:
//! - `each <役割名>: N | N..M | N..*` — 端点の役割名ごとの多重度制約
//! - `unique pair` — 同じ (始点, 終点) の対に2本目を張ることを禁止
//!
//! `each` の役割名が宣言した始点か終点の役割名と一致するかは意味検査
//! (`schema_validate.rs`) で行う。`each` と `unique pair` は独立した制約
//! として扱い、両方を同時に書くことも許す (`each 0..1` の下では同対2本は
//! 既に不可能なので `unique pair` の併記は冗長だが、実装を単純にするため
//! 特別扱い・警告はしない — `docs/schema_v4.md` §1 が明記する「実装時に
//! 単純な方を選ぶ」を適用した箇所)。
//!
//! ## v4.1 での拡張 (`docs/edge_endpoints_v4_1.md`)
//!
//! - 有向端点は `(役割名: 型名)` が必須。積み荷も `[役割名: 型パス]` が必須。
//! - 柄は4形: `->` / `-[役割名: Attrs]->` (有向) / `--` / `-[役割名: Attrs]-` (無向)。
//!   無向辺には役割名を書けない (構文エラー)。
//! - `each <参照名>` の `<参照名>` は有向辺の役割名を指す (型名参照は
//!   意味検査でエラー)。無向辺は役割名を持たないため `each` を指定できない。
//!   役割名により
//!   終点側の入次数制約 (`each <終点役割名>: ..`) も書けるようになる
//!   (意味解決は `schema_validate.rs::resolve_each_side`)。

pub mod each_specification;
pub mod edge_arrow;
pub mod edge_declaration;
pub mod edge_endpoint;
pub mod edge_payload;
pub mod identifier_type;
pub mod keywords;
pub mod node_declaration;
pub mod schema_declaration;
pub mod token_drain;
pub mod where_clause;

pub use each_specification::EachSpec;
pub use edge_declaration::{EdgeDecl, EdgeShape};
pub use edge_payload::EdgePayload;
pub use node_declaration::NodeDecl;
pub use schema_declaration::{SchemaInput, SchemaParse};
pub use where_clause::EachConstraint;
