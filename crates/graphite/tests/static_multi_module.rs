//! schema宣言とinstance宣言を別モジュールに分けたときに必要な配線を固定する
//! (issue #41)。`static_graph_schema!` が生成する `macro_rules!` は
//! 通常のRustのマクロと同じテキスト順の可視性しか持たない (`#[macro_export]`
//! も `pub(crate) use` も無い)。`schema` モジュールの宣言に `#[macro_use]`
//! を付け、`instance` モジュールの宣言より前に置かないと、`instance`
//! モジュール側の `組織! { .. }` 呼び出しは `cannot find macro 組織`
//! (E0433 相当) で失敗する。この配線は
//! `crates/graphite/tests/graph_cross_module.rs` と同じくモジュール境界を
//! 再現するが、動的グラフには無い `#[macro_use]` の要求がここでの検証対象
//! である (`docs/static_graph.md` 「2層マクロの使い方」)。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

/// schemaモジュール。`#[macro_use]` を付けて宣言することで、内部で生成される
/// `macro_rules! 組織` が、宣言位置より後ろの兄弟モジュールへテキスト順で
/// 伝播する。
#[macro_use]
mod organization {
    use super::{社員, 部署};

    #[rustfmt::skip]
    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod 組織 {
        include!("generated/static_multi_module_組織.rs");
    }

    #[rustfmt::skip]
    graphite::static_graph_schema! {
        generated = "generated/static_multi_module_組織.rs";
        schema 組織 {
            node 社員;
            node 部署;
            edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
        }
    }
}

/// instanceモジュール。schema moduleを`use`し、`組織!` を呼ぶ
/// (`organization` モジュールの `#[macro_use]` がこの呼び出しを可能にする)。
mod dev_team {
    use super::organization::組織;
    use super::{社員, 部署};

    #[rustfmt::skip]
    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod 開発チーム {
        include!("generated/static_multi_module_開発チーム.rs");
    }

    #[rustfmt::skip]
    組織! {
        generated = "generated/static_multi_module_開発チーム.rs";
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }

    pub fn 太郎の所属先を求める() -> String {
        let nodes = 開発チーム::construct::nodes!();
        let edges = 開発チーム::construct::edges!(&nodes);
        let g = 開発チーム::Graph::new(&edges);
        g.node_refs.太郎.太郎の所属().team().entity().名前.clone()
    }
}

#[test]
fn schemaとinstanceを別モジュールに分けても解決できる() {
    assert_eq!(dev_team::太郎の所属先を求める(), "開発部");
}
