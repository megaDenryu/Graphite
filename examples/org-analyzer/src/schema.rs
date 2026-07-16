//! 組織グラフのスキーマ定義。
//!
//! 3 ノード種別 (`Employee` / `Department` / `Project`) と 4 種の型付き
//! エッジからなる。多重度の意味付け:
//!
//! - `belongs_to (1)`    : 全社員は必ずちょうど1つの部署に所属する。
//!   `OrgChart::create` はこれを一括検査するので、所属部署のない社員や
//!   複数部署に所属する社員のデータは構築時点で `Err` になる。
//! - `boss (0..1)`       : 上司は高々1人 (トップ層は0人)。
//! - `assigned (0..*)`   : プロジェクトへの割当は0件以上 (兼務・未アサイン可)。
//! - `sponsors (0..1)`   : 部署がスポンサーするプロジェクトは高々1件
//!   (多くの部署はスポンサー活動をしないので0件が普通)。
//!
//! `graph_schema!` は同一ファイル内に `graph!` を書く場合のみ親切な
//! コンパイルエラーのハンドシェイクが効く制約があるが、本アプリはデータを
//! すべて `dataset.rs` の合成生成器 (`OrgChart::create` の builder 呼び出し)
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

/// `boss` エッジの属性。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

/// `assigned` エッジの属性。
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

        edge belongs_to: Employee -> Department (1);
        edge boss:       Employee -[BossEdge]-> Employee (0..1);
        edge assigned:   Employee -[AssignedEdge]-> Project (0..*);
        edge sponsors:   Department -> Project (0..1);
    }
}
