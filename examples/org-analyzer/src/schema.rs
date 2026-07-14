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

#[rustfmt::skip]
graphite::graph_schema! {
    schema OrgChart {
        node Employee { name: String, title: String, grade: u8 }
        node Department { name: String }
        node Project { name: String, priority: u8 }

        edge belongs_to: Employee -> Department (1);
        edge boss:       Employee -> Employee   (0..1) { since: i32 };
        edge assigned:   Employee -> Project    (0..*) { role: String };
        edge sponsors:   Department -> Project  (0..1);
    }
}
