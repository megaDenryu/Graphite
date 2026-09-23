// `Nodes` のフィールドが非公開であることを固定する (issue #41, PR #45
// レビューA)。値ありの個体はinstance宣言の式からのみ供給され、利用者が
// フィールドへ直接アクセスして迂回できない。生成ファイルを`include!`せず、
// 実物と同じ形 (フィールドが非公開) を手で模したmoduleを直接検査する
// (`static_stale_generated_instance_with_individuals.rs`と同じ手法)。

mod 開発チーム {
    pub struct 社員 {
        pub 名前: String,
    }

    pub struct Nodes {
        太郎: 社員,
    }

    impl Nodes {
        #[doc(hidden)]
        pub(crate) fn __graphite_internal_new(太郎: 社員) -> Self {
            Self { 太郎 }
        }
    }
}

fn main() {
    let nodes = 開発チーム::Nodes::__graphite_internal_new(開発チーム::社員 { 名前: "太郎".into() });
    // フィールドへの直接アクセスはできない (private field)。
    let _ = nodes.太郎;
}
