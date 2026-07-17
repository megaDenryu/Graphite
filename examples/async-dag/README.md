# async-dag

Graphite が倒すべき敵その2、「**非同期オーケストレーション地獄**」の実証example。
マイクロサービス群の起動オーケストレータを題材に、「実際にある問題」と
「グラフ構文による解決のベストプラクティス」を、動くプログラムとして提示する。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run
```

## 1. 敵の紹介 — 素朴な非同期コードの実在アンチパターン

マイクロサービス (`config`・`db`・`cache`・`migration`・`api`・`worker`・
`healthcheck`...) を起動するオーケストレータを素朴に書くと、だいたい次の
どちらかに行き着く。

### アンチパターンA: 手書きの直列 `await`

```rust
// 「動けばいい」で書いた起動シーケンス。正しいが遅い。
start_config().await;
start_db().await;
start_cache().await;
start_migration().await;
start_api().await;
start_worker().await;
start_healthcheck().await;
```

`db` と `cache` は互いに依存していないのに、`db` の完了を待ってから
`cache` を始めている。並行に起動できるものを勝手に直列化してしまう。

### アンチパターンB: 手書きの `spawn` + チャネル/フラグ待ち合わせ

```rust
// (**擬似コード** — 実際に動かすものではない。地獄の見本として掲載)
let (db_tx, db_rx) = channel();
let (cache_tx, cache_rx) = channel();
spawn(async move { start_config().await; db_tx.send(()).ok(); cache_tx.send(()).ok(); });
spawn(async move { db_rx.recv().await; start_db().await; migration_db_tx.send(()).ok(); });
spawn(async move { cache_rx.recv().await; start_cache().await; migration_cache_tx.send(()).ok(); });
spawn(async move { migration_db_rx.recv().await; migration_cache_rx.recv().await; start_migration().await; ... });
// ... api/worker/healthcheck 分もこの調子で増殖する
```

速くはなるが、「どれとどれが並行可能か」という設計判断がチャネル配線
そのものに溶けてしまう。`healthcheck` の前に `logging` サービスへの
依存を1本追加したくなったら、`spawn` ブロックを何箇所も手で書き直す
必要がある。さらに、誰かが依存を1本間違えて循環 (`api` が `worker` を
待ち、`worker` が `api` を待つ、のような) を作ってしまうと、両方の
タスクが相手の `recv()` で永久に止まる — **ハングするだけで、どこにも
エラーメッセージが出ない**。

## 2. なぜ死ぬか

- **依存関係がコードの制御フロー (await の並び・spawn の配線) に溶けて
  不可視になる。** 「このサービスは何に依存しているか」を知るには、
  `spawn`/チャネルのコードを丹念に読み解くしかない。
- **「どれとどれが並行可能か」を人間が手計算する羽目になる。** サービスが
  増えるたびに、依存グラフを頭の中で描き直して並行化の余地を探す必要が
  ある。当然、間違える。
- **循環依存はデッドロックとして実行時に発覚する。しかもハングなので
  原因が見えない。** コンパイルは通り、テストも「たまたま」通り、本番の
  ある日に起動シーケンスがそのまま固まる。ログにはエラーの1行も出ない。

## 3. グラフによる再定式化

タスク (サービス) = ノード、依存関係 = エッジ、と読み替える。

- **循環 = 構築時に `CycleError` で拒否される。** 「ハングする前に、
  データとして死ぬ」。実行を試みる前に、依存グラフを作った瞬間に
  分かる (§5「循環依存デモ」参照)。
- **`topological_levels` が「並行実行できる波」をデータから計算する。**
  実行計画は人間が書くものではなく、依存関係というデータから**導出**
  されるものになる。依存を1本追加/削除しても、波の再計算は
  `topological_levels()` の呼び直し1回で済む — spawn 配線の手直しは
  発生しない。

### スキーマ

```rust
pub struct Service {
    pub name: String,
    pub startup_ms: u64,
}

graphite::graph_schema! {
    schema Orchestration {
        node Service;

        edge DependsOn = Service -> Service where unique pair;
    }
}
```

`DependsOn(a -> b)` は「a は b に依存する (b が起動完了していないと
a は起動できない)」と読む。これは実行順序 (トポロジカル順序) とは
**逆**の向きになる点に注意 — `DependsOn` は「これから作るもの→先に
必要なもの」の向き、実行順序は「先に必要なもの→これから作るもの」の
向きだから。この反転は `src/depgraph.rs::build_dependency_graph` が
1箇所で引き受け、以後 `topological_sort`/`topological_levels` が仮定する
「辺の始点が先」という向きに揃える (`examples/build-pipeline` の
`Consumes ∘ Produces⁻¹` 射影と同じ発想)。`where unique pair` は「同じ
(a, b) に2本目の依存を張ることに意味は無い」という判断 (依存は有るか
無いかの二値であり、平行辺を許す積極的な理由が無い)。

## 4. 対応表 — 非同期オーケストレーションの概念 ↔ Graphite の概念

| 非同期オーケストレーションの概念 | 素朴な実装 | Graphite での対応 |
|---|---|---|
| 「AはBの後に起動する」という制約 | `spawn` の中の `recv().await` / 手書きの `await` の並び | `edge DependsOn = Service -> Service where unique pair;` の1本のエッジ |
| 「今並行に起動できるものは何か」 | 人間が依存を目で追って手計算 | `topological_levels()` が波として自動導出 |
| 循環依存の検出 | 実行時にハングして気づく (エラーなし) | 構築直後に `CycleError { cycle }` として拒否 (循環パス付き) |
| 依存を1本追加する | チャネル配線・`spawn` ブロックを複数箇所手直し | `graph!` に1行 `key = DependsOn(a -> b)` を追加するだけ。波は再計算されるだけ |
| 「実際に並行に実行する」実行主体 | 自作 `spawn` + 同期プリミティブ | 波ごとに `std::thread::scope` (§5) |
| 依存順序が守られているかの検証 | 手作業でログを目で確認 (or 検証コード無し) | 実行ログの `start`/`end` を `DependsOn::iter(&g)` で全数検査可能 (`tests/integration.rs`) |

## 5. 実装の要点

- `src/schema.rs` — `Service` ノード + `DependsOn` エッジのスキーマ宣言。
- `src/fixtures.rs` — `main.rs`/`tests/` が共有する固定サンプル
  (本編10サービスグラフ・循環依存デモ用の3サービスグラフ)。
- `src/depgraph.rs` — `Orchestration` から汎用 `graphite::Graph<(), (),
  ServiceId>` への射影 (`build_dependency_graph`、辺の向きを反転)、
  `compute_waves` (循環検出 + 波分割)、波・直列実行時間の集計関数。
- `src/engine.rs` — `run_waves`: 波ごとに `std::thread::scope` で
  スレッドを起こし、`std::thread::sleep` で起動をシミュレートしながら
  実際に並列実行する。`run_serial`: 「敵1」のベースライン (直列実行)。
  外部の非同期ランタイム (tokio 等) には依存しない — 本物のI/Oが無い
  シミュレーションなので `std::thread` で十分。
- `src/main.rs` — 上記を順に実行するデモ CLI。

### `cargo run` の実行例

```
=== 1. 循環依存デモ (ハングする前にデータ検証で死ぬ) ===
has_cycle() = true
波の計算は CycleError で拒否された (実行を試みる前に判明): グラフに循環があります: ServiceId("c") -> ServiceId("b") -> ServiceId("a") -> ServiceId("c")

=== 2. 本編サービスグラフを構築 (サービス数=10, DependsOn本数=13) ===

=== 3. topological_levels() で波を計算 ===
wave 1: [config] (この波の所要時間 = 15ms)
wave 2: [queue, logger, cache, db] (この波の所要時間 = 35ms)
wave 3: [migration, metrics] (この波の所要時間 = 55ms)
wave 4: [api, worker] (この波の所要時間 = 45ms)
wave 5: [healthcheck] (この波の所要時間 = 28ms)

=== 4. std::thread::scope で波を実際に並列実行 ===
  wave 1: config 開始=0ms 完了=15ms
  wave 2: queue 開始=15ms 完了=35ms
  wave 2: logger 開始=15ms 完了=23ms
  wave 2: cache 開始=15ms 完了=40ms
  wave 2: db 開始=15ms 完了=50ms
  wave 3: migration 開始=51ms 完了=106ms
  wave 3: metrics 開始=51ms 完了=63ms
  wave 4: worker 開始=106ms 完了=147ms
  wave 4: api 開始=106ms 完了=152ms
  wave 5: healthcheck 開始=152ms 完了=180ms
実測合計時間 = 180ms

=== 5. 直列実行 (敵1のベースライン) との比較 ===
直列実行 (実測) = 286ms (起動時間の総和 = 283ms)
並列実行 (実測) = 180ms
実測の高速化率 = 1.59倍
```

同じ波の中のサービス列挙順 (例: `[queue, logger, cache, db]`) は実行毎に
変わりうる (`Orchestration` のノード格納が `HashMap` のため)。これは
バグではなく、**波の中は「順序が無い並行実行可能な集合」であることが
そのまま表れている**、と読むのが正しい — 順序を固定する必要が無いことを
検証しているのが `tests/integration.rs` である (波の**内容**は
`HashSet` で比較し、波の**順序**そのものは固定してテストする)。

直列実行の実測値 (286ms) が起動時間の総和 (283ms) よりわずかに大きいのは
スレッド生成・スケジューリングのオーバーヘッド。並列実行 (180ms) は
波の理想値の合計 (15+35+55+45+28=178ms) に近い実測値になっている。

## 6. テスト

```powershell
cargo test
```

15件 (単体テスト5件 + `tests/integration.rs` 10件):

- 本編グラフの波数・各波の内容 (手計算した期待値との一致)
- 全サービスがちょうど1つの波に現れること (循環無し・欠落無しの確認)
- 循環依存サンプルが `has_cycle()`/`compute_waves()` の両方で検出され、
  具体的な循環パス (`CycleError.cycle`) が得られること
- `run_waves` の実行ログから、依存先 (`prerequisite`) が依存元
  (`dependent`) より先に完了していることを `DependsOn::iter(&g)` の
  全ペアについて検証すること
- 並列実行が直列実行より実測で速いこと
- 未知の依存先を参照すると `DependsOnUnknownTarget` 違反になること
  (図式適合検査)
