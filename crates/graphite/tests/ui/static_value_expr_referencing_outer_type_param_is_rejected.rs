// 値の式が外側の関数のジェネリックの型引数を参照するとコンパイルエラーになる
// ことを固定するcompile-fail回帰試験。instance宣言は、モジュール直下にあっても
// 関数の中にあっても、常に宣言位置に置いた捕捉しない`fn`の本体として値の式を
// 固定する (`docs/static_graph.md`「値の式の名前解決」節)。入れ子の`fn`は
// 外側の関数のジェネリックの型引数を使えないため、値の式がそれを参照すると
// 通常のRustのE0401になる。対処は、その個体を値なし宣言 (`node 名前: 型;`) に
// して、実体を`{instance名}::construct!(..)`の引数として実行時に渡すことである
// (積み荷の式にはこの対処が無く、常にinstance宣言の位置で決まる式でしか
// 与えられない)。ローカル変数・引数を参照した場合のE0434は
// `static_value_expr_referencing_local_is_rejected.rs`が別に固定する。
//
// schema・instanceの指紋定数は、`static_multiplicity_violation.rs`と同じ
// 手法で手書きする (`graphite_codegen::parse_tracked_static_schema`/
// `parse_tracked_static_instance`をこのファイルと同じschema・instance
// トークン列で呼び、`TrackedStaticSchema::fingerprint`/
// `TrackedStaticInstance::fingerprint`の返り値を1回だけ書き写した値)。
// 値の式が外側の型引数を参照する時点でこのinstanceはどこに置いても構築まで
// 到達できないため、`generated/`配下に実在する生成ファイルを他の試験と
// 共有する通常の経路 (`include!`) が使えない。`mod 型引数参照チーム`が
// 実在の生成ファイルを持たないため、instance展開が宣言位置へ直接置く
// DSLトークンの型参照 (`{個体名}Ref`、F12を助けるためだけの読むだけの参照)
// も解決できず、目的のE0401 (このファイルの主張) に加えて`太郎Ref`が
// 見つからないというE0425が副次的に1件出る。node/edgeを最小 (node 1件、
// edge 0件) にして、この副次的なエラーの件数をこれ以上増やさないように
// してある。

pub struct 社員 {
    pub 名前: String,
}

#[allow(non_snake_case)]
mod 型引数参照組織 {
    pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] =
        [18327709500980267106, 17330416255313685677, 14397175102129428872, 6645826734161640004];
}

graphite::static_graph_schema! {
    generated = "generated/static_value_expr_referencing_outer_type_param_is_rejected_型引数参照組織.rs";
    schema 型引数参照組織 {
        node 社員;
    }
}

#[allow(non_snake_case)]
mod 型引数参照チーム {
    pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] =
        [5411438033100756185, 9223932100807732164, 950476644018831863, 15411006803110201811];
}

fn 関数の型引数を値の式が参照する<T: Default + ToString>() {
    #[rustfmt::skip]
    型引数参照組織! {
        generated = "generated/static_value_expr_referencing_outer_type_param_is_rejected_型引数参照チーム.rs";
        graph 型引数参照チーム;
        node 太郎 = 社員 { 名前: T::default().to_string() };
    }
}

fn main() {}
