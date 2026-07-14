# フェーズ5候補: `examples/` 実装で見えた Graphite API の不足点

2026-07-14、`examples/build-pipeline` / `examples/org-analyzer` /
`examples/dialogue-engine` の3実践exampleを並行実装した際、各実装エージェント
から報告された Graphite ランタイム/マクロの不足点・改善候補をまとめたもの。
フェーズ5 (あるいはそれ以降) で `crates/graphite` / `crates/graphite-macros`
に手を入れる際の検討リストとして使う。ここに載っているのは「実装時に困った
実体験」であり、優先順位付けや採否の決定はまだ行っていない。

## Graph<N, E, K> ランタイムまわり

- **(a) `in_neighbors` が無い** — 後方 DP (例: クリティカルパス計算で
  「このタスクに依存しているタスク一覧」を辿る) には自前の逆引きマップを
  都度組む必要があった。`out_neighbors` 相当があるなら対になる
  `in_neighbors` も欲しい。
  (出典: `examples/build-pipeline`)
  → **解決 (フェーズ5)**: `Graph::in_neighbors(&K) -> Vec<&K>` を追加
  (`petgraph::Direction::Incoming` で実装、`out_neighbors` と対称)。
- **(c) 重み付き最長経路・レベル分割トポロジカルソートが頻出** —
  クリティカルパス (重み付き最長経路) と、依存のない先頭ノードから順に
  「波」に分けるレベル分割トポロジカルソートは、この種のパイプライン/
  DAG 系アプリで繰り返し必要になる。ライブラリ側にアルゴリズムとして
  持たせる候補。
  (出典: `examples/build-pipeline`)
  → **解決 (フェーズ5)**: `Graph::topological_levels() -> Result<Vec<Vec<&K>>,
  CycleError<K>>` (レベル内は挿入順で決定的) と `Graph::critical_path_by(node_weight)
  -> Result<(Vec<&K>, W), CycleError<K>>` (ノード重み付き最長経路 DP、空グラフは
  `(vec![], W::default())`) を追加。

## `graph_schema!` が生成するアクセサまわり

- **(b) `{label}_pairs()` から汎用 `Graph` への射影で借用エラーを踏みやすい** —
  スキーマ固有の型付きグラフから petgraph ベースの汎用 `Graph` へ変換
  (射影) する際、`flat_map` 内でのイテレータ借用が絡んでコンパイルエラーに
  なりやすかった。定型的な「ペアイテレータ→汎用 Graph」変換のための
  ヘルパー関数/メソッドがあると詰まりにくい。
  (出典: `examples/build-pipeline`)
  → **解決 (フェーズ5)**: `Graph<(), (), K>::from_edges(nodes, edges)` を
  追加。`{label}_pairs()` が返す `&K` は `.cloned()`/`.clone()` で渡す
  (doc例を用意)。`examples/org-analyzer` の `detect_boss_cycles` をこれで
  簡潔化した。
- **(d) 多重度 `(1)`/`(0..1)` アクセサが相手ノードの値だけを返し、ID を
  返さない** — 指揮系統チェーンのように「次のノードへ辿ってまたそこから
  辿る」処理をしたい場合、値ではなく ID (キー) が返る版のアクセサが必要
  になる。現状は値からキーを逆引きする追加コードが要る。
  (出典: `examples/org-analyzer`)
  → **解決 (フェーズ5)**: 多重度ごとに ID 版アクセサを追加生成した。
  `(1)`: `{label}_id(&SrcId) -> &DstId` (未知キーはパニック、`# Panics`
  明記) + `try_{label}_id(&SrcId) -> Option<&DstId>`。`(0..1)`:
  `{label}_id(&SrcId) -> Option<&DstId>`。`(0..*)`:
  `{label}_ids(&SrcId) -> Vec<&DstId>` (格納順を保持)。属性は既存の
  値アクセサで取れるため ID 版には含めない。
- **(f) `filter_nodes` の述語がノード値のみを受け取り、キーを参照できない** —
  「特定の ID 群に含まれるノードだけ抽出する」ような、キーに依存する
  フィルタ処理ができない。述語にキーも渡す形が欲しい。
  (出典: `examples/org-analyzer`)
  → **解決 (フェーズ5)**: `filter_nodes_with_key(|k, v| ...)` /
  `map_nodes_with_key(|k, v| ...)` を追加 (既存の `filter_nodes`/`map_nodes`
  は温存し、それぞれこちらへ委譲する形にリファクタ)。

## エラー/違反の表現まわり

- **(e) `CycleError` が循環メンバーを1つしか返さない** — 循環検出エラーの
  デバッグ・報告には循環を構成するノード全体 (経路) が要る。
  (出典: `examples/org-analyzer`)
  → **解決 (フェーズ5、破壊的変更)**: `CycleError<K> { node: K }` を
  `CycleError<K> { cycle: Vec<K> }` (循環を構成するノード列全体、
  `cycle[0]` から辿って `cycle[0]` に戻る閉路) に変更。`tarjan_scc` で
  強連結成分を求め、成分内の反復 DFS で単純閉路を復元する。
  `topological_sort`/`topological_levels`/`critical_path_by` 全てがこの形で
  返す。`examples/build-pipeline` (`DomainIssue::CyclicDependency`) と
  `examples/org-analyzer` (`detect_boss_cycles` の自前 `boss_of` 復元コード
  を削除) を追従させた。
- **(g) Violation が最初の1件で `Err` になり、複数違反の一括収集ができない** —
  検証系のユースケース (例: 組織図の全違反を一覧表示) では、最初の1件で
  即エラーになるのではなく、全違反を集めた `Vec<Violation>` を返すモードが
  欲しい。
  (出典: `examples/org-analyzer`)
  → **解決 (フェーズ5)**: `{Schema}::create_collecting(|b| ...) ->
  Result<Self, Vec<{Schema}Violation>>` を追加した。既存 `create` は温存
  (最初の1件で `Err`) しつつ、内部で `freeze_collecting` に委譲する形に
  リファクタし検証ロジックの二重実装を避けた。始点キーが正当だが終点キー
  が未知の行は、多重度カウント上「試行された1本」として数える (終点が
  壊れているだけの1つの根本原因から `UnknownTarget` と `Multiplicity` の
  2件が二重に生えるのを防ぐため)。

## `graph!` / `graph_schema!` リテラルの名前空間・順序保証

- **(h) `graph!` 内のノードキーが全ノード型で単一の平坦な名前空間** —
  例えば `Scene` と `Ending` のキーが同じ名前空間を共有するため、
  異なるノード型間でキーが衝突するリスクがある。命名規約 (プレフィックス
  等) で回避できたが、型ごとに名前空間を分ける設計の検討余地がある。
  (出典: `examples/dialogue-engine`)
- **(i) `(0..*)` エッジの列挙順がソース記述順で安定していることに依存した
  が、これは実装詳細であり文書化されていない** — 選択肢の表示順のように
  順序が意味を持つ場面で、記述順保持に依存するコードを書いてしまった。
  保証として明文化する (あるいは明示的に順序を指定できるAPIを用意する)
  価値がある。
  (出典: `examples/dialogue-engine`)

## ポジティブ知見 (課題ではないが記録)

- **(j) `graph!` リテラルは30シーン/56辺規模でもJSON/YAMLより読みやすい** —
  分岐ノベルのシナリオ定義のような、ノード数・エッジ数が多く相互参照が
  密なデータでも、`graph!` の宣言的記法は同等のJSON/YAML表現より可読性が
  高いという実感が得られた。DSLとしての設計方向性を支持する事例。
  (出典: `examples/dialogue-engine`)
