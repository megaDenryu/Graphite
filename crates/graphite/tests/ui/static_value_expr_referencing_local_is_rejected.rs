// 値の式が関数のローカル変数を参照するとコンパイルエラーになることを固定する
// compile-fail回帰試験 (issue #46の再設計)。instance宣言は、モジュール直下に
// あっても関数の中にあっても、常に宣言位置に置いた捕捉しない`fn`の本体として
// 値の式を固定する (`docs/static_graph.md`「値の式の名前解決」節)。入れ子の
// `fn`は外側の関数のローカル変数・引数を捕捉できないため、値の式がそれらを
// 参照すると通常のRustのE0434になり、外側の型引数を参照すると通常の
// RustのE0401になる。対処は、その個体を値なし宣言 (`node 名前: 型;`) に
// して、実体を`{instance名}::construct!(..)`の引数として実行時に渡す
// ことである (積み荷の式にはこの対処が無く、常にinstance宣言の位置で
// 決まる式でしか与えられない)。
//
// schema・instanceの指紋定数は、`static_multiplicity_violation.rs`と同じ
// 手法で手書きする (`graphite_codegen::parse_tracked_static_schema`/
// `parse_tracked_static_instance`をこのファイルと同じschema・instance
// トークン列で呼び、`TrackedStaticSchema::fingerprint`/
// `TrackedStaticInstance::fingerprint`の返り値を1回だけ書き写した値)。
// 値の式がローカルを参照する時点でこのinstanceはどこに置いても構築まで
// 到達できないため、`generated/`配下に実在する生成ファイルを他の試験と
// 共有する通常の経路 (`include!`) が使えない。`mod ローカル参照チーム`が
// 実在の生成ファイルを持たないため、instance展開が宣言位置へ直接置く
// DSLトークンの型参照 (`{個体名}Ref`、F12を助けるためだけの読むだけの参照)
// も解決できず、目的のE0434 (このファイルの主張) に加えて`太郎Ref`が
// 見つからないというE0425が副次的に1件出る。node/edgeを最小 (node 1件、
// edge 0件) にして、この副次的なエラーの件数をこれ以上増やさないように
// してある。

pub struct 社員 {
    pub 名前: String,
}

#[allow(non_snake_case)]
mod ローカル参照組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] =
        [4152441171879719052, 5655425656053411479, 9497913891967806338, 7249013324849182134];
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_referencing_local_is_rejected_ローカル参照組織.rs";
    schema ローカル参照組織 {
        node 社員;
    }
}

#[allow(non_snake_case)]
mod ローカル参照チーム {
    pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] =
        [5292724756990767157, 12734513371102297536, 278596576832797751, 12510641482390856523];
}

fn 関数のローカルを値の式が参照する(社員名: String) {
    #[rustfmt::skip]
    ローカル参照組織! {
        generated = "generated/static_value_expr_referencing_local_is_rejected_ローカル参照チーム.rs";
        graph ローカル参照チーム;
        node 太郎 = 社員 { 名前: 社員名 };
    }
}

fn main() {}
