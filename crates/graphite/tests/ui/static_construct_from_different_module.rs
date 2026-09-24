// 別moduleから構築が呼べないことを固定する回帰試験。2つの経路を検査する。
// (1) `construct!`自体は`pub(crate) use`されたマクロなので別moduleからも
// 呼び出せるが、その展開の内部で無修飾のまま参照する値マクロ
// (`__graphite_values_{グラフ名}_{instance印}!`) の名前は呼び出し位置
// (`mod 別`) を起点に解決されるため、instance宣言を持つ`mod 元`の外からは
// 見えず解決に失敗する。(2) 値マクロ自身を
// `元::__graphite_values_開発チーム_{instance印}!`のように修飾パスで直接
// 呼ぶ迂回も検査する (印は`generated = "generated/static_multi_module_開発チーム.rs"`
// から計算する値であり、生成ファイル自身の呼び出しと一致させる)。値マクロは
// 意図的に`pub(crate) use`を持たない
// (`crates/graphite-codegen/src/static_graph/inline/value_supply.rs`) ため、
// この迂回もコンパイルエラーになる。`pub(crate) use`を一時的に戻して実測
// すると、この迂回だけが成立するようになり (値の式の中の`社員`/`部署`が
// `mod 別`のスコープを起点に解決されて見つからないという別のエラーへ変わる
// — instance宣言と無関係な別moduleに同名の型・関数があれば、値がすり替わる
// 危険の実体)、値マクロを公開しないことが対策そのものであると確認できる。
// schema・instanceの宣言は`static_multi_module.rs`と同じ内容にし、
// fingerprintが一致する既存の生成ファイルをそのまま`include!`する。

pub struct 社員 {
    pub 名前: String,
}

pub struct 部署 {
    pub 名前: String,
}

mod 元 {
    use super::{社員, 部署};

    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod 組織 {
        include!("../generated/static_multi_module_組織.rs");
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

    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod 開発チーム {
        include!("../generated/static_multi_module_開発チーム.rs");
    }

    #[rustfmt::skip]
    組織! {
        generated = "generated/static_multi_module_開発チーム.rs";
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 開発部 = 部署 { 名前: "開発部".into() };
        edge 太郎の所属 = 所属(太郎 -> 開発部);
    }
}

mod 別 {
    pub fn 試す() {
        let _g = super::元::開発チーム::construct!();
        let _v = super::元::__graphite_values_開発チーム_74affc48978331c5!();
    }
}

fn main() {}
