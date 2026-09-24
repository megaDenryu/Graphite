// この試験は、値の式が関数のローカル変数・引数を参照すると通常の
// RustのE0434になることを固定するcompile-fail回帰試験である。
// この試験は、`static_multiplicity_violation.rs`と同じ手法で
// 手書きした指紋定数を使う。指紋定数がずれてE0080で失敗した場合、
// 保守者は`graphite_codegen::parse_tracked_static_schema`/
// `parse_tracked_static_instance`で測り直して書き写す
// (E0080が案内する`cargo xtask generate`ではこの試験は直らない)。
// 値の式がローカルを参照する時点でinstanceは構築まで到達できず、
// DSLトークンの型参照 (`太郎Ref`) も解決できないため、目的のE0434に
// 加えてE0425が1件副次的に出る。
//
// 参照: `docs/static_graph.md`「値の式の名前解決」節

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
