//! `src/lib.rs`と`src/main.rs`が同じ名前のstatic schemaを持っていても、
//! 別のCargo targetとして区別され衝突しないことを固定する回帰試験
//! (Cargoの自動target発見規則どおりに分ける裁定、`cargo_target`のdoc参照)。
//! `lib.rs`側を1つ、`main.rs`側を1つの一時パッケージへ書き、
//! `graphite_cli::generate`が「同名schemaが重複しています」エラーに
//! ならないことを確かめる。

use std::sync::atomic::{AtomicUsize, Ordering};

use graphite_cli::PackageRoot;

static 連番: AtomicUsize = AtomicUsize::new(0);

#[test]
fn libとmainが同名schemaを持っても衝突しない() {
    let dir = std::env::temp_dir().join(format!(
        "graphite_cli_lib_and_bin_share_schema_name_{}_{}",
        std::process::id(),
        連番.fetch_add(1, Ordering::SeqCst)
    ));
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"一時パッケージ\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();

    std::fs::write(
        src.join("lib.rs"),
        r#"
        graphite::static_graph_schema! {
            generated = "generated/組織_lib.rs";
            schema 組織 { node 社員; }
        }
        "#,
    )
    .unwrap();
    std::fs::write(
        src.join("main.rs"),
        r#"
        graphite::static_graph_schema! {
            generated = "generated/組織_main.rs";
            schema 組織 { node 部署; }
        }
        fn main() {}
        "#,
    )
    .unwrap();

    let package = PackageRoot::at(dir.clone()).unwrap();
    let result = graphite_cli::generate(package.generation_tree());

    std::fs::remove_dir_all(&dir).unwrap();

    let error = result.err().map(|error| error.to_string());
    assert_eq!(error, None, "lib.rsとmain.rsの同名schemaは別targetなので衝突しないはず: {error:?}");
}
