// instanceの `generated = "...";` は書いたが、instance module
// (`mod 開発チーム { include!(..); }`) の宣言そのものを忘れた場合の診断を
// 固定する (issue #41 段階4)。指紋照合コードが参照する `開発チーム::..` が
// 解決できず、コンパイルエラーになる。このエラーのspanが `generated = "..."`
// のリテラルの行を指すこと (`fingerprint_check::指紋照合コードを生成する`
// のspan方針) をこの断片で固定する。

struct 社員;

#[allow(non_snake_case)]
mod 組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
        10319720238056268413,
        1420965213717410858,
        810849268007507131,
        13845130453493871279,
    ];
}

graphite::static_graph_schema! {
    generated = "generated/組織.rs";
    schema 組織 {
        node 社員;
    }
}

fn 社員を作る() -> 社員 {
    社員
}

// `mod 開発チーム { .. }` の宣言を書き忘れた状態を再現する。
組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
    node 太郎: 社員 = 社員を作る();
}

fn main() {}
