# 開発履歴: 2026-07-14 立ち上げセッション (Bullet側セッションからの引き継ぎ)

Graphite はもともと Bullet リポジトリ (Vertex言語) のセッション内で構想・立ち上げされた。
このファイルはそのセッションの開発履歴を、次のセッション (Graphite側で開く) に完全に
引き継ぐための記録。ユーザー指示は**原文のまま**引用する。

---

## 0. 系譜 (Graphite誕生の前段)

Bullet の `docs/graph_design_sketches.md` (グラフ型の設計決定1〜6) の末尾に
「既存言語(Rust)の拡張として作れないか」という問いがあり、セッションでユーザーから:

> グラフデザインスケッチ.mdみたいなのを読んでください。rustの拡張にする案について、深堀したいです。今の言語はいろいろrustから機能を引いたものもありますが、グラフ以外の機能はあまり関心がないです。なので何をするだけならrustを拡張する形で実現できるか知りたいです。今の言語は今の言語で置いておき、新たに別のプロジェクトを始める土台として考える感じです。

これに対する分析の核心: **Vertexのグラフ設計決定1〜6の大半はRustの型システムに「タダで」乗る**。

- 異種ノードenum → Rustのenum+matchそのまま (checker不要)
- builder→freeze不変性 → 借用検査が自然に保証 (アリーナ逃亡検査の新規実装が不要)
- 多重度→戻り型 (1)/(0..1)/(0..*) → 参照/Option/Vec を生成するだけ
- 可視性 → pub + private constructor (Rust標準作法)
- 異種混在エラー → rustcが最初から拒否

新規実装が要るのは3点のみ: `graph_schema!` マクロ / `graph!` リテラルマクロ / ランタイムクレート (petgraphラッパー)。
輸入できないもの: 辺ラベル付きパターンでの `match` (Rust安定版の制約。クエリAPIで妥協)。

> そうですね。ではその方向性で、新たな言語？かライブラリ？なのかわからないですがその枠組みのスケッチのmdを新しく作って欲しい。それから、ほかにVertexの機能でrust拡張に追加できそうなものがあればそれも知りたい。

→ Bullet の `docs/rust_graph_extension_sketch.md` を作成 (Graphite設計の一次資料)。
Vertex機能の棚卸し結果: 良い候補 = パイプ演算子 (pipe-trait相当で足りる) / 精密型 (newtype+create()のResult検証パターン) / 状態機械 (typestateの方が強い) / アリーナ (bumpalo等で解決済み)。悪い候補 = 値型継承 (Rust哲学への逆行) / gradual typing・phase並行・アリーナ意味論 (脱糖で済まず実行意味論の追加が必要) / DTO規律 (引き算は足し算で再現できない)。

---

## 1. プロジェクト開始指示 (運用体制の確立)

> ではrustの拡張としてGraphiteとして作ってみて欲しいです。あなたは方針策定などのオーケストレーションに徹して実装はSonet5 effort highのサブエージェントに任せて欲しいです。git操作や複数運用で問題が発生した時の競合解消・収斂作業もサブエージェントに任せてください。気を付けるべきことなどはスキルなどにもしてほしいです。

オーケストレータ (Fable) の決定事項:
- 配置: `C:\Users\t-yamanaka\ripository\megaDenryu\Graphite` — Bulletとは**独立した新規gitリポジトリ**
- 構成: `crates/graphite` (ランタイム) + `crates/graphite-macros` (proc-macro) の2クレートworkspace (serde/serde_derive型。proc-macroクレートはランタイム型を持てないため技術的必然)
- 運用ルールを `CLAUDE.md` / `.claude/agents/impl.md` (sonnet, effort high) / `.claude/skills/proc-macro-dev/SKILL.md` に固定化
- コミットメッセージは日本語

### フェーズ1: 足場 (コミット `75e347a`)
workspace + 2クレート + rust-toolchain.toml (1.94.0) + CLAUDE.md + agents/skills。

### フェーズ2: ランタイム (コミット `586425e`, `d503381`)
- 水準1: 不変 `Graph<N, E = (), K = String>` (petgraph DiGraph + HashMap索引。GraphMapはK: Copy制約で不可)。`build` / `create(|b|)` / アクセサ / has_cycle / topological_sort / reachable_from / path / map_nodes / filter_nodes
- 水準2手書きターゲット: `crates/graphite/tests/orgchart_handwritten.rs` — graph_schema!が生成すべきコードを手書きしたテンプレート (**意図的に残置。消さない**)

### フェーズ3: procマクロ (コミット `d5b9367`, `6316493`, `15b3b41`)
- `graph_schema!`: schema宣言→ノードstruct・`{Node}Id` newtypeキー・属性struct・Builder・`create()->Result<S, {S}Violation>`・多重度対応アクセサを生成
- `graph!`: `a -[label { attrs }]-> b` の矢印リテラルを create 呼び出しへ脱糖
- trybuildコンパイルエラーテスト・README
- オーケストレータ設計決定: **多重度(1)アクセサの未知キーはv0ではパニック** (Vec添字と同じ契約違反扱い)

---

## 2. 実践example指示

> 自明なだけのテストの例ではなく、実践的なexampleアプリを作ってみてください。いくつか作って欲しいので、exampleフォルダの中にさらにプロジェクトを作ってみてください。

> READMEの未決事項もやってください。

> exampleアプリはそれなりにボリュームのあるものにしてほしいです。最小なものだと結局それそもそもGraphite使わなくてよくね？みたいになるので。

### フェーズ4: README未決事項の解消 (コミット `b1b39d4`, `4eaf90e`, `ac526db`, `47b7aca`)
オーケストレータの設計決定と実装:
1. `try_{label}()` 非パニック版 (多重度(1)のみ。Vecの`[i]`/`get(i)`の対)
2. クエリAPI: コンビネータDSLは作らず (独自パーサ再演の警告に従う)、`{label}_pairs()` / `{node}_ids()` イテレータ生成
3. ノード/属性structから `Eq` derive撤去 (f64属性対応。PartialEqは残す。キー型はHash+Eq維持)
4. `node Category(categories)` 複数形明示構文
5. graph!未知エッジラベルへの**マクロハンドシェイク**: graph_schema!が `__graphite_check_edge_{Schema}!` を生成し、graph!が呼び出しを埋め込む → 利用可能エッジ一覧つきcompile_error。**制約: graph_schema!とgraph!は同一ファイル内** (macro_rulesのテキストスコープ)

### examples 3本 (並列実装→統合。コミット `33dd4ae`, `e997b95`, `98e4c4e`, `84cda67`)
各exampleは**スタンドアロンクレート** (Cargo.toml先頭の空`[workspace]`でルートworkspaceから除外、依存はgraphiteのみ・外部クレートなし)。各READMEに「Graphiteを使う意味」節 (生HashMap自作との対比) を必須で記載。

| example | 題材 | 規模 |
|---|---|---|
| `examples/build-pipeline` | Task/Artifact異種ノードのビルドオーケストレータ。validate/plan/critical-path/mermaid | 23タスク8段構成、テスト32件 |
| `examples/org-analyzer` | LCG合成データ (社員120/部署8/PJ15) の組織分析。summary/chain/anomalies/reorg | テスト11件 |
| `examples/dialogue-engine` | graph!リテラルで30シーン/4エンディング/56選択肢の分岐ノベル。play/validate/map/route/stats | テスト14件 |

副産物: 実装エージェントが報告したAPI不足10件 (a)〜(j) を `docs/phase5_candidates.md` に集約。

---

## 3. フェーズ5 (API改善) と設計原則の確立

> やりましょう。

に続き、作業中にユーザーから設計方針の指示:

> グラファイトの設計はあくまでもRust的な精神の遵守です。

> 型についてはstrictであることなど？

→ **6原則**として `docs/design_principles.md` に固定化 (CLAUDE.mdから参照):
1. 型のstrictness — stringly-typed API禁止。キーはnewtypeで運ぶ
2. パニックは契約違反のみ。対になる `try_` を必ず用意。docに `# Panics` 節
3. std命名規約準拠 (`try_`・`_with_key`・`impl Iterator`)
4. 借用検査と戦わないAPI (所有ベースヘルパー)
5. ゼロコスト志向 — 生成コードは手書きと同形。実行時リフレクション禁止
6. 消去可能な拡張のみ — 実行意味論を変えない (TS enumの教訓)

原則1の即時適用: フェーズ3の妥協 `MultiplicityViolation { source: String }` (Debug文字列) を廃し、**エッジ単位の型付きバリアント** (`BelongsToMultiplicity { source: EmployeeId, count }` 等) に置き換え。

### フェーズ5前半: ランタイム (コミット `9b4b1fa`, `bca4772`)
- (a) `in_neighbors` / (b) 射影ヘルパー `Graph::from_edges(nodes, edges)` (pairs→汎用Graph射影の借用の罠を所有ベースで吸収) / (c) `topological_levels()` (波分割) + `critical_path_by(重み関数)` / (e) `CycleError { cycle: Vec<K> }` へ拡張 (破壊的変更、tarjan_sccで復元) / (f) `filter_nodes_with_key` / `map_nodes_with_key`
- ドッグフーディング効果: org-analyzerの自前循環復元コードが `CycleError.cycle` で削除できた

### フェーズ5後半: マクロ (コミット `af8fcd3`, `9805a5f`, `8dc0cd1`, `ce574b2`)
- (k) 型付き違反enum (上記) / (d) ID返却アクセサ `{label}_id` / `try_{label}_id` / `{label}_ids` / (g) `create_collecting(|b|) -> Result<S, Vec<Violation>>` (全違反一括収集。freeze検証を収集版に一本化しcreateは先頭1件を返す) / (i) `(0..*)` エッジの記述順保持を**仕様に昇格** (docコメント+README+固定テスト) / (h) graph!重複識別子にspan付き診断 (名前空間再設計は見送り・文書化)

---

## 4. 現在の状態 (このファイル作成時点)

- テスト: **120件全通過** (コア63 + build-pipeline 32 + org-analyzer 11 + dialogue-engine 14)
- コミット履歴 (古→新): `75e347a` → `586425e` → `d503381` → `d5b9367` → `6316493` → `15b3b41` → `b1b39d4` → `4eaf90e` → `ac526db` → `47b7aca` → `33dd4ae` → `e997b95` → `98e4c4e` → `84cda67` → `9b4b1fa` → `bca4772` → `af8fcd3` → `9805a5f` → `8dc0cd1` → `ce574b2`
- リモート未設定 (ローカルのみ)

## 5. 未着手の種

1. `graph!` の平坦名前空間 (全ノード型でキー識別子が単一名前空間。原則1的には型別名前空間の検討余地。現状は診断+文書化で見送り)
2. `plural_field_name` の素朴な英語複数形化 (`node Type(plural)` の明示指定で回避可能)
3. `{label}_ids` (エッジ先ID) と `{node}_ids` (ノード全キー) の命名重なり (原則3観点のドキュメント整理課題)
4. `docs/phase5_candidates.md` に各項目の解決記録あり

## 6. 運用ルール (再確認)

- **オーケストレータ (Fable/Opus) は実装しない**。実装・テスト・git操作・競合解消は Sonnet 5 (effort: high) サブエージェントに委譲 (ユーザー明示指示)
- ビルドは `cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50` 形式
- コミットは日本語
- 設計判断は `docs/design_principles.md` の6原則に従う
- 詳細は `CLAUDE.md` / `.claude/agents/impl.md` / `.claude/skills/proc-macro-dev/SKILL.md`
