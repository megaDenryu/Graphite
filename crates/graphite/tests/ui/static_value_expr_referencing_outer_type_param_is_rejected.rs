// この試験は、値の式が外側の関数のジェネリックの型引数を参照すると
// 通常のRustのE0401になることを固定するcompile-fail回帰試験である。
// この試験は、`static_multiplicity_violation.rs`と同じ手法で
// 手書きした指紋定数を使う。指紋定数がずれてE0080で失敗した場合、
// 保守者は`graphite_codegen::parse_tracked_static_schema`/
// `parse_tracked_static_instance`で測り直して書き写す
// (E0080が案内する`cargo xtask generate`ではこの試験は直らない)。
// 値の式が外側の型引数を参照する時点でinstanceは構築まで到達できず、
// DSLトークンの型参照 (`太郎Ref`) も解決できないため、目的のE0401に
// 加えてE0425が1件副次的に出る。
//
// 参照: `docs/static_graph.md`「値の式の名前解決」節

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
