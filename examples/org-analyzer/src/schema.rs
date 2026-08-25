//! 組織グラフのスキーマ定義 (`docs/schema_v4.md` 準拠)。
//!
//! 3 ノード種別 (`Employee` / `Department` / `Project`) と 4 種の型付き
//! エッジ (`Kind`) からなる。`where` 制約の意味付け:
//!
//! - `BelongsTo where each employee: 1`     : 全社員は必ずちょうど1つの部署に
//!   所属する。`OrgChart::Graph::create` はこれを一括検査するので、所属部署のない
//!   社員や複数部署に所属する社員のデータは構築時点で `Err` になる。
//! - `Boss where each subordinate: 0..1`       : 上司は高々1人 (トップ層は0人)。
//! - `Assigned` (制約なし)                   : プロジェクトへの割当は0件以上
//!   (兼務・未アサイン可)。1人の社員が同じプロジェクトに異なる役割 (role) で
//!   複数アサインされる (兼務・役割変更の履歴等) ケースを排除しない設計判断
//!   のため、あえて `unique pair` を付けない。
//! - `Sponsors where each department: 0..1` : 部署がスポンサーするプロジェクト
//!   は高々1件 (多くの部署はスポンサー活動をしないので0件が普通)。
//!
//! `BelongsTo`/`Boss`/`Sponsors` は既に `each` 制約が同一始点の重複を防いで
//! いるので、`unique pair` の重ねづけは冗長 (`docs/schema_v4.md` §1 の
//! 「実装を単純にするため特別扱いしない」方針に合わせ、冗長な併記自体を
//! 避けている)。
//!
//! `graph_schema!` は同一ファイル内に `graph!` を書く場合のみ親切な
//! コンパイルエラーのハンドシェイクが効く制約があるが、本アプリはデータを
//! すべて `dataset.rs` の合成生成器 (`OrgChart::Graph::create` の builder 呼び出し)
//! から組み立てるため `graph!` リテラルは使わない。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub name: String,
    pub title: String,
    pub grade: u8,
}

/// ノード型。`reorg.rs` が部署を再構築する際に値を `.clone()` するため
/// `Clone` を derive している。
#[derive(Debug, Clone, PartialEq)]
pub struct Department {
    pub name: String,
}

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
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod OrgChart {
    include!("generated/schema_org_chart.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/schema_org_chart.rs";
    schema OrgChart {
        node Employee;
        node Department;
        node Project;

        edge BelongsTo = (employee: Employee) -> (department: Department) where each employee: 1;
        edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;
        edge Assigned = (employee: Employee) -[assignment: AssignedEdge]-> (project: Project);
        edge Sponsors = (department: Department) -> (project: Project) where each department: 0..1;
    }
}

// 綴り短縮のための再輸出。同名edgeを持つschemaを足したらこの行を消す。
pub use OrgChart::{
    Assigned, AssignedId, BelongsTo, BelongsToId, Boss, BossId, DepartmentId, EmployeeId,
    ProjectId, Sponsors, SponsorsId,
};

macro_rules! impl_id_order {
    ($($ty:ty),+ $(,)?) => {$ (
        impl PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
        }
        impl Ord for $ty {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.cmp(&other.0) }
        }
    )+ };
}
impl_id_order!(EmployeeId, DepartmentId, ProjectId);
