// 別moduleにある、同じグラフ名を持つ別instanceの`construct!`を誤って
// 呼んでも、その別instanceの値・`Graph`を無言ですり替えて使わないことを
// 固定する回帰試験。schema・instanceの宣言は
// `static_same_graph_name_different_modules.rs`と同じ内容にし、
// fingerprintが一致する既存の生成ファイルをそのまま`include!`する
// (`static_construct_from_different_module.rs`と同じ手法)。
//
// `m1`・`m2`はどちらも同じschemaから同じグラフ名`衝突検査グラフ`の
// instanceを作るが、`generated = "..."`は別ファイルを指すため、値マクロ
// (`__graphite_values_衝突検査グラフ_{instance印}!`) の名前がinstanceごとに
// 違う (`naming::internal_names::個体値マクロ名`参照)。`m2`の関数から
// `crate::m1::衝突検査グラフ::construct!()`を呼ぶと、`construct!`の内部が
// 無修飾で参照する値マクロは呼び出し位置 (`m2`の関数の中) を起点に解決
// されるため、`m1`の値マクロは`m2`から見えず`cannot find macro`になる。
// `instance印`が無かった設計では、`m2`が自分自身の同名グラフ
// `衝突検査グラフ`の値マクロを偶然同じ名前で持っていたため、この呼び出しは
// 無言で`m2`自身の`Graph`を構築していた (`docs/static_graph.md`「制約」節)。

pub struct 社員 {
    pub 名前: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
mod 衝突検査組織 {
    include!("../generated/static_same_graph_name_different_modules_衝突検査組織.rs");
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
        include!("../generated/static_same_graph_name_different_modules_m1.rs");
    }

    #[rustfmt::skip]
    衝突検査組織! {
        generated = "generated/static_same_graph_name_different_modules_m1.rs";
        graph 衝突検査グラフ;
        node 甲 = 社員 { 名前: "m1".into() };
    }
}

pub mod m2 {
    use crate::社員;

    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
    pub mod 衝突検査グラフ {
        include!("../generated/static_same_graph_name_different_modules_m2.rs");
    }

    #[rustfmt::skip]
    衝突検査組織! {
        generated = "generated/static_same_graph_name_different_modules_m2.rs";
        graph 衝突検査グラフ;
        node 甲 = 社員 { 名前: "m2".into() };
    }

    pub fn 別moduleのグラフを構築してしまう() -> String {
        let g = crate::m1::衝突検査グラフ::construct!();
        g.node_refs().甲().entity().名前.clone()
    }
}

fn main() {
    let _ = m2::別moduleのグラフを構築してしまう();
}
