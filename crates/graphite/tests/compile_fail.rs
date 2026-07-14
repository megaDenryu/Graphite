//! `graph_schema!`/`graph!` の入力エラーがきちんとコンパイルエラーとして
//! 報告されることを確認する trybuild テスト。
//!
//! 実行: `cargo test --test compile_fail`
//! 期待する stderr の再生成: `TRYBUILD=overwrite cargo test --test compile_fail`
//! (`.claude/skills/proc-macro-dev/SKILL.md` 参照)

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
