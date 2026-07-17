//! 組織グラフのスキーマ定義 (`docs/schema_v4.md` 準拠)。
//!
//! 3 ノード種別 (`Employee` / `Department` / `Project`) と 4 種の型付き
//! エッジ (`Kind`) からなる。`where` 制約の意味付け:
//!
//! - `BelongsTo where each Employee: 1`     : 全社員は必ずちょうど1つの部署に
//!   所属する。`OrgChart::create` はこれを一括検査するので、所属部署のない
//!   社員や複数部署に所属する社員のデータは構築時点で `Err` になる。
//! - `Boss where each Employee: 0..1`       : 上司は高々1人 (トップ層は0人)。
//! - `Assigned` (制約なし)                   : プロジェクトへの割当は0件以上
//!   (兼務・未アサイン可)。1人の社員が同じプロジェクトに異なる役割 (role) で
//!   複数アサインされる (兼務・役割変更の履歴等) ケースを排除しない設計判断
//!   のため、あえて `unique pair` を付けない。
//! - `Sponsors where each Department: 0..1` : 部署がスポンサーするプロジェクト
//!   は高々1件 (多くの部署はスポンサー活動をしないので0件が普通)。
//!
//! `BelongsTo`/`Boss`/`Sponsors` は既に `each` 制約が同一始点の重複を防いで
//! いるので、`unique pair` の重ねづけは冗長 (`docs/schema_v4.md` §1 の
//! 「実装を単純にするため特別扱いしない」方針に合わせ、冗長な併記自体を
//! 避けている)。
//!
//! `graph_schema!` は同一ファイル内に `graph!` を書く場合のみ親切な
//! コンパイルエラーのハンドシェイクが効く制約があるが、本アプリはデータを
//! すべて `dataset.rs` の合成生成器 (`OrgChart::create` の builder 呼び出し)
//! から組み立てるため `graph!` リテラルは使わない。

/// ノードキー。`graph_schema!` はこれも生成せず参照するだけ
/// (`docs/node_id_v4_2.md`)。
/// `PartialOrd`/`Ord` は `graph_schema!` の要求ではなく (必須なのは
/// `Debug, Clone, PartialEq, Eq, Hash` だけ、`docs/node_id_v4_2.md`)、
/// `analysis.rs`/`reorg.rs` が決定的な表示順のためにキーをソートする箇所
/// (`result.sort()` 等) がこのアプリ側の都合で要求している。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmployeeId(pub String);

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub name: String,
    pub title: String,
    pub grade: u8,
}

/// ノードキー。`PartialOrd`/`Ord` はこのアプリ (`reorg.rs`) がソート表示
/// のために要求している (`graph_schema!` 自体の要求ではない)。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DepartmentId(pub String);

/// ノード型。`reorg.rs` が部署を再構築する際に値を `.clone()` するため
/// `Clone` を derive している。
#[derive(Debug, Clone, PartialEq)]
pub struct Department {
    pub name: String,
}

/// ノードキー。`PartialOrd`/`Ord` はこのアプリ (`analysis.rs`) がソート
/// 表示のために要求している (`graph_schema!` 自体の要求ではない)。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProjectId(pub String);

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub priority: u8,
}

/// `Boss` エッジの積み荷。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

/// `Assigned` エッジの積み荷。
#[derive(Debug, Clone, PartialEq)]
pub struct AssignedEdge {
    pub role: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;
        node Project;

        edge BelongsTo = Employee -> Department              where each Employee: 1;
        edge Boss      = Employee -[BossEdge]-> Employee     where each Employee: 0..1;
        edge Assigned  = Employee -[AssignedEdge]-> Project;
        edge Sponsors  = Department -> Project                where each Department: 0..1;
    }
}
