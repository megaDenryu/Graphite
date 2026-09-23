// 静的グラフのinstanceの生成ファイルの指紋が古いときの診断を固定する
// (issue #41 段階4)。schema側の指紋は正しい値を埋め込み (schemaの検査自体は
// 通す)、instance側だけを手で偽の指紋定数にする。schemaの正しい指紋は
// `graphite_codegen::parse_tracked_static_schema` を直接呼んで求めた値
// (このテストのschema本体だけから決まり、宣言元への参照を含まない)。

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

// 個体を1つも持たないinstanceにする。DSLトークンの錨・値供給関数は個体・
// 具体辺ごとに `開発チーム::{名前}Ref` 等を参照するため、個体を持つと
// このテストの意図 (指紋照合の診断だけを見る) とは無関係な「型が見つから
// ない」エラーが混ざってしまう (`開発チーム` は指紋定数しか持たない偽の
// モジュールのため)。個体0件ならそれらの参照が1つも生成されない。
#[allow(non_snake_case)]
mod 開発チーム {
    pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [0, 0, 0, 0];
}

組織! {
    generated = "generated/開発チーム.rs";
    graph 開発チーム;
}

fn main() {}
