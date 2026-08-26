# テストの構成と実行

> **Development document** — 索引: `docs/README.md`

この文書は、Graphite 自身のテストが何を担保し、どう実行するのかを定める文書で
あり、実装が変わるたびに追随して更新する。

## 実行

以下はリポジトリルート (ワークスペースの `Cargo.toml` があるディレクトリ) で実行する。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo xtask generate --check
cargo xtask check-docs
cargo xtask check-external
```

`examples/` 配下の7本はルートの Cargo workspace から独立したスタンドアロン
クレートであり、ルートの `cargo test` の対象には含まれない。個別に `cd` して
ビルド・実行する。

`verification/external-crate` もワークスペースの外にある。こちらは外部 crate から
の生成経路 (`cargo graphite generate` → `cargo build`) が壊れていないことを確かめる
ためのパッケージであり、`cargo xtask check-external` が生成の差分検査とビルドと
テストをまとめて実行する。

## テストファイルの役割

- `crates/graphite/tests/orgchart_handwritten.rs` — フェーズ2で手書きした
  `OrgChart` である。schema生成コードの目標形をテンプレートとして残置している。
- `crates/graphite/tests/orgchart_macro.rs` — `graph_schema!` で `OrgChart` を
  宣言し、通常のRust生成ファイルを読み込む同等テストと、`graph!` リテラルの
  テストである。
- `crates/graphite/tests/compile_fail.rs` + `tests/ui/*.rs` —
  [`trybuild`](https://docs.rs/trybuild) によるコンパイルエラーの検査テストである。
  未宣言ノード型を端点に指定した場合、不正な `where each` 指定、`graph!` で
  存在しないエッジ種別を書いた場合、ノードキーの重複、宣言単位のエラー回復を
  検査する。stderr の再生成は
  `TRYBUILD=overwrite cargo test --test compile_fail` で行う。
- `xtask/tests/generate_check.rs` — 生成ファイルが全て最新であることを
  `cargo test` から検査する。
- `xtask/tests/docs_check.rs` — 文書参照の綴りの実在と docs/README.md 索引の
  網羅を `cargo test` から検査する。
- `verification/external-crate/src/lib.rs` — 外部 crate から生成した公開APIを
  組み立てて読み出す検証である。`cargo test --workspace` には含まれず、
  `cargo xtask check-external` が実行する。

## IDE サポート (rust-analyzer)

`examples/` 配下はルートの Cargo workspace から独立したスタンドアロンクレート
であるが、`.vscode/settings.json` の `rust-analyzer.linkedProjects` で明示的に
リンクしているため、VSCode で開けば通常のクレートと同様に rust-analyzer の
解析対象になる。example を追加したときは `linkedProjects` へ1行足すことを
運用ルールとする。詳細は `docs/development/ide_support_spec.md` を参照する。
