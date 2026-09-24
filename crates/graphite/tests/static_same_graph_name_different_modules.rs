//! 別moduleが同じグラフ名を選んでも、各moduleの中で完結した呼び出しなら
//! 正しく動くことを固定する回帰試験。`m1`・`m2`はどちらも同じschemaから
//! グラフ名`衝突検査グラフ`のinstanceを作るが、`generated = "..."`は別
//! ファイルを指すため、値マクロの名前 (`naming::internal_names::個体値
//! マクロ名`が`generated`文字列から計算する`instance印`を含む) が
//! instanceごとに違い、混ざらない。
//!
//! この2つの生成ファイルは、別moduleの`construct!`を誤って呼んでも
//! すり替わらずコンパイルエラーになることを固定する
//! `crates/graphite/tests/ui/static_construct_same_graph_name_different_module_rejected.rs`
//! が、同じ宣言内容 (byte単位で一致する内容) を再利用する。

pub struct 社員 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 衝突検査組織 {
    include!("generated/static_same_graph_name_different_modules_衝突検査組織.rs");
}

graphite::static_graph_schema! {
    generated = "generated/static_same_graph_name_different_modules_衝突検査組織.rs";
    schema 衝突検査組織 {
        node 社員;
    }
}

pub mod m1 {
    use crate::社員;

    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod 衝突検査グラフ {
        include!("generated/static_same_graph_name_different_modules_m1.rs");
    }

    #[rustfmt::skip]
    衝突検査組織! {
        generated = "generated/static_same_graph_name_different_modules_m1.rs";
        graph 衝突検査グラフ;
        node 甲 = 社員 { 名前: "m1".into() };
    }

    pub fn 甲の名前を求める() -> String {
        衝突検査グラフ::construct!().node_refs().甲().entity().名前.clone()
    }
}

pub mod m2 {
    use crate::社員;

    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod 衝突検査グラフ {
        include!("generated/static_same_graph_name_different_modules_m2.rs");
    }

    #[rustfmt::skip]
    衝突検査組織! {
        generated = "generated/static_same_graph_name_different_modules_m2.rs";
        graph 衝突検査グラフ;
        node 甲 = 社員 { 名前: "m2".into() };
    }

    pub fn 甲の名前を求める() -> String {
        衝突検査グラフ::construct!().node_refs().甲().entity().名前.clone()
    }
}

#[test]
fn 別moduleが同じグラフ名を選んでもそれぞれの中で完結した呼び出しは混ざらない() {
    assert_eq!(m1::甲の名前を求める(), "m1");
    assert_eq!(m2::甲の名前を求める(), "m2");
}
