# 開発履歴: 2026-07-14 セッション2 (IDEサポート: rust-analyzer 対応)

前セッション (`dev_history_2026-07-14_session1.md`) からの継続。ユーザー指示は原文のまま引用する。

## 0. セッション開始指示

> Graphite/docs/dev_history_2026-07-14_session1.md　を読んで。これの続きをしたい。やり方とかそのまま。この前のセッションのAIと同じように作業してほしい。Graphiteの課題は、人間がグラフ構造など独自構文を書くときにそれがちゃんとvscodeで反応して参照ジャンプとか、型追跡とか常識的に機能するようにしたいということです。ほかにも根本的な言語側の課題もあると思うんで、それに対して仕様を詰めて実装したいです。

運用体制は前セッション踏襲: オーケストレータ (Fable) は方針策定・タスク分解・レビュー・実測に徹し、実装・テスト・git 操作は Sonnet subagent (effort: high) に委譲。

## 1. 現状調査 (オーケストレータが vscode-lsp-mcp で実測)

rust-analyzer の definition/references プロバイダを直接叩き、`targetSelectionRange`
(F12 の着地点) まで確認するという計測手法を確立した。結果は
`docs/ide_support_spec.md` の「1. 現状マトリクス」に固定。要点:

- **スパン保存は既存実装がほぼ正解を出していた**。使用側→schema への定義ジャンプ
  (型名・アクセサ・`try_*` 派生名・属性フィールド) は全てトークン単位で精密。
  `format_ident!` が最初に補間した Ident のスパンを継承することが効いている (実測で発見)。
- 欠陥は3つ: (1) `graph!` のノードキーが文字列に脱糖され LSP に不可視、
  (2) examples/* (workspace 除外のスタンドアロンクレート) を rust-analyzer が未解析、
  (3) 編集途中にパースが壊れると生成型が全滅する (補完も死ぬ)。

## 2. 仕様策定と実装 (G1〜G6)

`docs/ide_support_spec.md` を新設し G1〜G6 として仕様化。本セッションで G1〜G4 を実装した。

### G1: graph! ノードキーの let 束縛化 (コミット `1268cba`)
展開を `let tanaka = EmployeeId("tanaka".to_string()); __graphite_b.employee(tanaka.clone(), ..)` 方式に変更。
- builder 変数を `b` → `__graphite_b` (ユーザーのキー `b` との衝突回避)
- 展開を「全ノード→全エッジ」の2段に並べ替え (let は使用より前に必要)
- 効果 (実測): エッジ内キー→宣言への精密ジャンプ ✅ / 参照検索 (宣言+全エッジ内出現) ✅ / hover (`EmployeeId` 型表示) ✅

### G2: examples の RA 解析 (コミット `c75d927`)
`.vscode/settings.json` の `rust-analyzer.linkedProjects` に examples 3本を列挙してコミット。
効果 (実測): `Scene`/`SceneId` 等がワークスペースシンボルで引けるようになった。

### G3: スパンポリシーの明文化 (同 `c75d927`)
proc-macro-dev スキルに「生成識別子はユーザートークン由来スパン」「format_ident! のスパン継承規則」「definition provider での確認手順」を追記。

### G4: 宣言単位のエラー回復展開 (コミット `6e4b120`) — 根本課題
パースを宣言 (schema側: node/edge) / 項目 (graph!側: カンマ区切り) 単位の回復型に変更。
壊れた宣言は syn::Error を蓄積して次の境界までスキップし、残りで通常の validate+codegen を行い compile_error! を併記する。
- 回復境界は proc_macro2 の Group の atomicity を利用 (深度カウンタ不要)
- 二次エラー抑制: パースエラーがあるとき「未知端点/未宣言キーのエッジ」は黙って除外
- 効果 (実測、プローブファイル): 1宣言壊した状態で診断はその1件のみ。他の型の利用コード・graph! リテラルに二次エラー0件
- trybuild UI テスト2件追加 (schema_partial_recovery / graph_partial_recovery)。既存4件の stderr 変化なし

### G5/G6: 制約の記録 (実装なし)
- G5: ハンドシェイクマクロの同一ファイル制約は現状維持を決定 (macro_rules 2.0 安定化まで)。副産物の発見: `__graphite_check_edge_{Schema}!` が補完候補に露出する (テキストスコープゆえ隠せない)
- G6: `graph!` 内フィールド名位置の補完は 0 件 (RA の関数様 proc-macro 内補完の制約。Graphite 側で直せない)。定義ジャンプ・参照検索・hover・診断は全て機能しているため主要導線は確保

## 3. セッション中に確立した知見

1. **rust-analyzer の反映には Restart Server が必要なことがある**: `.vscode/settings.json` の `linkedProjects` 変更や proc-macro dylib の変更は、reloadWorkspace / rebuildProcMacros では反映されず `rust-analyzer: Restart Server` で反映された (実測)。
2. **rename はこの環境では判定保留**: RA の rename provider がマクロと無関係な普通の関数でも「Unexpected type」例外を投げる (環境問題)。参照検索は全出現を正しく検出するので名前解決の土台は健全。
3. **syn::ParseBuffer の Drop 落とし穴** (G4 実装エージェントが発見): デリミタ内バッファに未消費トークンを残したまま Err を握りつぶして続行すると、Parser::parse2 の最終チェックで無関係な「unexpected token」幽霊エラーが再浮上する。デリミタ内で Err を返す前に残りトークンを読み捨てる (`drain_rest`) 必要がある。proc-macro-dev スキルに追記済み。

## 3.5 後半: rename 実験 → スキーマ宣言構文 v2 (大改訂)

ユーザーが F2 で `edge boss` を `boss_edge` に rename する実験を実施。派生名
(`boss_pairs` 等) は参照側までカスケードしたが `BossAttrs` だけ取り残された。

> F2でbossをboss_edgeに変えてみたらなんか一括でいろいろ変わりました。(中略) 今選択しているboss_edge_pair()をctrl+clickするとなぜか関数の定義ではなく、boss_edgeに飛びますが、これは適切？

→ ctrl+click の挙動は設計どおり (生成メソッドの定義元 = schema のラベルトークン)。
取り残しはスパン継承漏れと判断し `1c7d76d` で修正したが、**再実験でも取り残しは再発**。
実測により「RA の rename は派生名に元トークンが原文ママ含まれる場合のみ部分置換
できる。PascalCase 変換 (`boss`→`Boss`) を跨げない」という境界条件を特定
(`e824f12` で仕様書に記録)。

この議論からユーザーの設計判断が連鎖:

> 確かに{since:i32}て名前ないの？何こいつ？てなってました。typescriptかよ！(中略) 無名型みたいなやつは意味不明だし、コードも構造化を邪魔してただの横着なので犯罪だと思っています。

> 別にまだ配布してないから、そもそも旧構文は問答無用でエラーでいいけどね。旧構文のにおいを一切残さないで欲しい。言語をクリーンに保って

> そもそもschemaの中でnodeの型って宣言する必要あるのか？nodeはどうせstructなんだから、graph_schemaの外でstructとして定義してedgeで始点と終点に外の型を指定すればよくないか？

→ **スキーマ宣言構文 v2** (`docs/edge_syntax_v2.md`、決定3 の宣言側改訂) を策定・実装:
- ノード型・エッジ属性型は**ユーザーがマクロ外で普通の struct として宣言**し、
  schema は参照するだけ。マクロはグラフ機械 ({Node}Id・ストレージ・builder・
  アクセサ・違反enum) のみ生成 (ドメイン型を発明しない)
- エッジ宣言は矢印形 `edge Employee -[boss: BossEdge]-> Employee (0..1);`
  (ラベルは矢印内、多重度は矢印の後ろ)
- `node Employee;` は残す (孤立ノード種別の宣言・端点 typo 検出・図式の可読性のため)。
  ノード型名は単純 Ident のみ (端点照合のため)、属性型は syn::Path 可
- 旧構文は検出・移行診断なしの完全廃止 (素のパースエラーに任せる)
- graph! のラベル→属性型解決は統合ハンドシェイクマクロ `__graphite_edge_{Schema}!`
  (check/attrs 2アーム) で実現
- ノード型・属性型への trait 要求はゼロ (move と参照渡しのみ)

コミット: `9de33b7` (設計文書) → `75f597e` (マクロ実装) → `86b715a` (テスト・examples・README 移行)。
テスト: コア65 + examples (32/11/14) 全通過。

v2 後の実測 (仕様書 §1.7): schema 内の型参照・`attrs.since` はユーザー struct へ
精密ジャンプ (改善)。既知の制約: `graph!` リテラル内の属性フィールドは二段マクロ
展開 (proc-macro → macro_rules) を RA が追跡できず定義解決不能 (コンパイルは正常)。

## 3.6 リテラル構文 v3

v2 実装直後の実測 (§3.5 末尾) で「`graph!` リテラル内の属性フィールドは
二段マクロ展開 (proc-macro → macro_rules) を RA が追跡できず定義解決不能」
という制約が判明した。この状態でユーザーから構文自体への提案が来た。

> グラフインスタンスの中のノードインスタンスなのだから、`alice: Person { .. }`
> ではなく `alice = Person { .. }` のほうが自然。そうすればグラフの外で
> `let alice1 = Person { .. };` と作ってから `alice = alice1` のような変数渡し
> もできる。

→ **リテラル構文 v3** (`docs/graph_literal_v3.md`) を策定・実装:

- ノード項を `key: Type { .. }` から `key = 式` へ変更。属性ありエッジも
  `-[label { fields }]->` から `-[label = 式]->` (式ペイロード) へ変更。
  式は任意の Rust 式なので、グラフの外で構築済みの値をそのまま move できる
  (ユーザー提案の後半を満たす)。
- ハンドシェイクマクロ (`__graphite_edge_{Schema}!`、check/attrs 2アーム)
  を**完全に削除**。未知ラベルは `__graphite_b.#label(..)` の呼び出しが
  そのまま rustc の method-not-found (E0599) に落ちることで検出する
  (自前の「利用可能一覧」診断は失うが、健全性には関与しないためユーザー
  決定により許容)。
- 型解決は `graph_schema!` が生成する `{Schema}Node` trait と builder の
  総称 `insert` メソッド (`fn insert<N: OrgNode>(&mut self, key: impl
  Into<String>, value: N) -> N::Id`) に委譲する。マクロは値の型名を
  一切トークンとして読まない (旧実装は型名から `to_snake_case` で builder
  メソッド名を機械導出していたが、型名自体が構文から消えたため不可能に
  なった、というより本質的には「マクロが型推論をエミュレートするのを
  やめて rustc に投げる」という設計転換)。
- 効果 (実測済み、`docs/ide_support_spec.md` §1.7/§1.8):
  - 二段マクロ展開が構造的に消滅したことで、`since`/`BossEdge`/`boss` の
    定義ジャンプが復活 (v2 で失っていた導線の復旧)。
  - `graph_schema!`/`graph!` の同一ファイル制約 (旧 G5) も同時に消滅。
    ハンドシェイクマクロがテキストスコープの macro_rules だったための
    制約だったため、そのマクロ自体を削除したことで根から解消した。
    別モジュールの schema に対して `graph!` が動くことを
    `crates/graphite/tests/graph_cross_module.rs` で証明した。
  - ハンドシェイクマクロが補完候補に露出する副産物 (G5 副産物) も、
    マクロ自体の消滅により消えた。
  - 実測で新たに発見した軽微な副作用: エッジに使われない孤立ノードの
    let 束縛 (`let suzuki = __graphite_b.insert(..)`) が読まれないため
    rustc の `unused variable` 警告が出る。孤立ノードは正当なグラフなので
    これはノイズと判断し、本セッションの後続タスクで
    `#[allow(unused_variables)]` により抑制した
    (`crates/graphite-macros/src/instance_codegen.rs`、回帰テスト
    `crates/graphite/tests/orphan_node_no_warning.rs`)。
- この改訂で「**schema は `:` (型付け)、リテラルは `=` (代入)**」という
  言語規則が確立した。schema 側 (`docs/edge_syntax_v2.md`) は型を宣言する
  ので `:`、graph! リテラル側は値をキーに束縛するので `=` という対応が、
  文法上も一貫した形で言語全体に定着した。

コミット (このサブセッション分、古→新): `ed42b1e` (v3設計文書) →
`b434c48` (v3実装: instance_dsl/instance_codegen) → `3389ab9` (テストv3移行
+ 外部値渡し・別モジュール利用テスト追加) → `de07098` (examples v3移行、
hello-graph §4引用再採取) → `a1fb360` (README/仕様書のv3反映) →
`51cd679` (孤立ノードのunused variable警告をallow属性で抑制) →
(本ファイル更新の docs コミットが続く)。

## 3.7 エッジアクセスのビュー方式化

ユーザーからエッジアクセサ群 (`try_{label}`/`{label}_id(s)`/`{label}_pairs`
等の導出名) の複雑さについて指摘があった。

> (生成APIは) テンプレート文字列みたいなもんじゃないですか？…これを
> 読まないと無理ですし、覚えることが多すぎて複雑すぎる

→ 対処: ラベル1個につき生成するのはビューを返すメソッド `{label}()` の
1個だけにする。ビューが持つ語彙 (`of`/`get`/`id_of`/`ids_of`/`get_id`/
`iter`/`len`) はランタイム側に `EdgeOne`/`EdgeOneWith`/`EdgeOption`/
`EdgeOptionWith`/`EdgeMany`/`EdgeManyWith` (多重度×属性有無で6種) として
固定し、ラベルごとの導出名の合成メソッド群は全廃した。効果: 覚える語彙が
「ビューの呼び方1つ」+「共通6語彙」に縮退し、補完2回で全APIに到達できる。
ラベル rename もこの1メソッドへのカスケードだけで完結し、生成コード量も
削減された。

コミット: `5013315` (設計) → `779bdb9` (ランタイム実装) → `4325798`
(マクロ側をビュー返却へ全面移行) → `b95e244` (examples/docs移行)。

## 3.8 エッジ宣言 v3 (ラベルの型としての矢印式)

ユーザーから宣言構文の可読性について指摘があった。

> 『boss の型が BossEdge』ではないの部分を何とか解決したい。BossEdgeは
> bossの何なのか？が宣言的に書けるようになってないといけない

→ 対処: `edge boss: Person -[BossEdge]-> Person (0..1);` という形に改めた。
Rust の関数型 `f: impl Fn(A) -> B` の読み方を借用し、「`label:` の右側全体
がそのラベルの関係型」と読める構文にした。矢印の中に置くのは積み荷
(属性型) だけで、属性なしエッジは矢印内に何も書かない素の `->` になる。
対案の `boss<BossEdge>` (`Vec<T>` 類推) も検討したが、`snake_case<T>` が
Rust に実在しない形であるため棄却した。

コミット: `424a83a` (設計文書) → `ec76188` (マクロ実装、破壊的変更) →
`4959d8e` (examples/README/hello-graphの構文説明移行)。

## 3.9 三大敵の実証examples

ユーザーからexampleの方針指示があった。

> グラフ構文が倒すべき重大な既存の敵として、ステートマシンと非同期
> プログラミングとリアクティブスパゲッティを挙げる。実際にある問題と
> ベストプラクティスとして提示し、プログラムとして動作するところまで

→ 3本を並列実装した。共通の型は「暗黙の制御フローで表現されていた構造を、
宣言されたグラフデータに変え、性質の検証 (循環・到達性・順序) をグラフ
アルゴリズムに任せる」。

- **state-machine** (注文ライフサイクルFSM): イベント=エッジ種別、
  決定性=多重度 `(0..1)`。到達不能状態・行き止まり状態を
  `reachable_from`/`out_neighbors` で検出
- **async-dag** (マイクロサービス起動オーケストレータ): 循環依存=構築時
  `CycleError` (ハングではない)、`topological_levels` が導く「波」を
  `std::thread::scope` で実際に並列実行し、実測 1.59 倍の高速化を確認
- **reactive-cells** (ミニスプレッドシート/見積書): `topological_sort` =
  glitch-free な再計算順。observerパターンのアンチパターン実装
  (`antipattern.rs`) をグラフ版と並置してグリッチ・無限ループ・非決定性
  を実際に再現し、グラフ版が構造的に解決する様を対比する

3本は独立クレートとして並列実装され、統合作業 (root README・
`.vscode/settings.json` の `linkedProjects`・本ファイルの追記) は後続の
統合エージェントが担当した。並行してオーケストレータ側では Fudaba 札
#1〜#6 (ノード宣言の対称化・違反enum命名・複数schema名前空間・キー設計・
クエリ層・本examples方針) で未決の設計論点を記録しており、これまでの
設計変遷は読み物Artifact「ラベルは何者か — Graphite 設計の足跡」にも
まとめられている。

コミット: `6bc33cc` (state-machine) → `2887ca9` (async-dag) → `c444aef`
(reactive-cells) → (本統合コミット)。

## 3.10 スキーマ v4 — 辺の第一級化

`async-dag` の schema.rs にユーザーが設計議論用のコメントを書き残していた
ことをきっかけに、エッジ宣言構文そのものの再検討が始まった。

> boss: BossEdgeとかの属性の型はこの場合何を指すのでしょうか

この疑問の背景には、v3 構文 (`edge boss: Person -[BossEdge]-> Person
(0..1);`) が「`boss:` の右側全体が関係型」という読み方をユーザーに要求する
一方で、実装上は依然として「ラベル=ビューを返すメソッド名」というモデルの
ままであり、辺そのものがキーを持つ第一級の値になっていない、というねじれが
あった。ユーザーは async-dag の schema.rs に直接メモを書いて代案を提示した。

> edge DependsON = Service -> Service (0..*) の方が構文としていいかも？

ここから「ラベル: 型」という**型付け**の構文をやめ、「Kind = 定義」という
**名前の束縛**の構文に転換する案が生まれた。エッジ種別を関数型的な関係
としてではなく、ノードと対称な「名前を持つ nominal 型」として扱う発想
である。この転換は当初 `graph!` リテラル側の無名辺 (`a -[label]-> b`) にも
波及するかが焦点になった。

> edgeで変数名を付けないってのに俺はあんまり納得できない

ユーザーは「ノードには `alice = Person { .. }` という名前があるのに、辺には
無い」という非対称性そのものに不満を表明した。これが「全行が `名前 = 値`」
という v4 の規則1 (`docs/schema_v4.md` §0) に直結する。辺も第一級のキー付き
要素であるなら、ノードと同様にキーの束縛を持つべきという結論である。

多重度注釈 (`(1)`/`(0..1)`/`(0..*)`) の扱いも同時に見直された。

> 多重度てどっちかというと、ノードが何本の線とつながってるかを見るとかそういう感じじゃね？前の設計思想が間違ってたとしか思えない

この指摘により、多重度は「辺そのものの属性」ではなく「ノード側の出次数に
対する制約」として捉え直された。これが `where each <FromType>: <spec>`
という、制約を矢印式の外側に完全に切り離した構文につながっている。加えて
「関係 (対で一意)」という性質も基盤ではなく `where unique pair` という
個別の制約として明示的に宣言する対象になった (基盤は多重グラフ、関係は
その上の特殊ケース、という §0 の宣言)。

議論はしばらく行き来したが、最終的にユーザーが次のように総括して v4 の
方向性が確定した。

> まあじゃあ一回これで言ってみるかあ

到達した設計は3規則に集約される (`docs/schema_v4.md` §0):

1. **名前=キーの束縛** — schema もリテラルも全行が `名前 = 定義/値`
2. **矢印の中は積み荷だけ** — `-[X]->` の X は積み荷の型/値のみ
3. **where は制約** — `each`/`unique pair` は制約があるときだけ書く

Fudaba では継続して #7 (エッジ第一級化そのものの是非) と #8 (辺キー命名
規則・`unique pair` の適用判断基準) として記録され、実装フェーズ1 で
ランタイム (`KeyedTable` 共有機構への置換)・マクロ (schema_dsl/instance_dsl/
codegen 全面書き換え) が完了し (`3fff112`・`678727c`)、本セッションの
フェーズ2で examples 7本・hello-graph 教材・README・本ファイルの追記まで
移行が完了した。

フェーズ2の examples 移行 (7本を並列実装エージェントに委譲) の途中、
dialogue-engine (56本の choice エッジを持つ最大の example) の移行を担当した
エージェントから、制約なしエッジ (`Choice`) の `Choice::of`/`iter` が返す
順序が `cargo test` の実行のたびに変わる flaky なテスト報告が上がった。
原因は `KeyedTable` (`3fff112` でエッジビュー6型を置き換えたキー付き要素表
共有機構) が素の `HashMap` の薄いラッパーで、`ids`/`iter` の反復順序が
未規定だったこと。freeze 時に始点キーごとの索引 (`{accessor}_from_index`)
を `KeyedTable::iter()` 経由で構築していたため、この未規定の順序がそのまま
索引に伝播し、同一始点から複数の制約なし辺 (平行辺) が出る場合の並びが
プロセスごとに (`HashMap` のハッシュシード次第で) 変わっていた — v3 で
保証されていた「構築時の追加順を保持する」という仕様がランタイム移行の
過程で無自覚に失われていたことになる。この報告を受け、`KeyedTable` の内部を
`Vec<(K, V)>` (挿入順の本体) + `HashMap<K, usize>` (キー→添字の索引) へ
変更し、`get`/`contains_key` の O(1) を保ったまま `ids`/`iter` が挿入順を
返すよう修正した (`975b753`)。`docs/schema_v4.md` に順序保証を仕様として
明記し、平行辺5本以上を張る回帰テスト (builder経由・`graph!`リテラル経由の
両方) を追加した上で、影響を受けうる example (dialogue-engine・org-analyzer・
hello-graph) は `cargo test` を複数回連続実行して非決定性が無いことを確認
している。

コミット: `342ee0b` (v4設計文書) → `3fff112` (ランタイムKeyedTable化)
→ `678727c` (マクロv4実装) → `975b753` (KeyedTable挿入順保持化・順序保証の
発見と修正) → (本セッション: examples/教材/docs移行の一連のコミット、
§4 参照)。

## 3.11 モデリング指針の確定と端点宣言 v4.1 (2026-07-17)

Fudaba #8 の議論をユーザーと実施。経緯の要点 (原文趣旨の引用):

> (Trade 昇格の説明に対し) 正直に言って、afterの書き方も意味不明ですけどね。
> Vertexには 引数 -[関数]-> 帰り値 の構文もあったはずですが、それは今どうなってますか？
> …「矢印というのはそれがどういう意味を持ってるのかで向きの解釈も何もかも変わる、
> したがって言語として記述するときはそこを明確にするようにしないといけないし、
> できるような記法になっていないといけない」

> 私もそれ (端点役割名) は入れたほうがいいと思ってたんだ。

> (flow! について)「原則6に反する」がよくわからない。むしろ入れないといけない
> んじゃないか。脱糖するだけだから問題ない気がする。

> (flow! のスコープ) 計算グラフを見据えてました。
> (役割名の必須/任意) 中間がいいかな。
> もしかして向きのない辺の話？あったほうがいいと思ってました。

決定と成果物:

- **モデリングガイド** (`docs/modeling_guide.md`、Fudaba #8 完了): 大原則
  「同一性+接続性を持つものだけをグラフ要素に」/ 二項関係の 4 分類 (from-to /
  役割名 / 無向 / ノード昇格。判定の核 = 端点の呼び名が向きから導出できるか) /
  ペイロード = 「間柄の属性」だけ / 同種辺の役割差は辺種別に昇格 (可換なら
  分割不要、非可換だけ分割 — reactive-cells の Sub 書き直しで実証。
  `Formula` との二重管理が完全解消し `graph!` から手動 clone も消えた)
- **端点宣言 v4.1** (`docs/edge_endpoints_v4_1.md`、Fudaba #10/#12):
  役割名は任意 (`edge Boss = (subordinate: Employee) -> (superior: Employee)`。
  指定時はアクセサが役割名に置換、where は役割名参照必須、**入次数 each 制約が
  新規に有効** — schema_v4 の保留項目が解消)。無向辺 `--` / 積み荷 `-[X]-`
  (順序なし対、同型端点のみ、役割名不可、自己ループは次数1)。破壊的変更なし
- **flow! の位置づけ訂正** (Fudaba #11): 「原則6違反」というオーケストレータの
  過去の説明は誤用と認め訂正。脱糖のみなら消去可能な拡張。Vertex の設計
  (dataflow_design.md: 矢印は中間値に名前と型を与える) はマクロで括る税だけで
  輸入可能。ユーザーの「計算グラフを見据える」を受け、線引きは「直線 vs DAG」
  でなく「即時実行の脱糖 (fan-out/fan-in 込みで flow!) vs 具象化された計算グラフ
  (ランタイムエンジン)」に更新。仕様詰めは次フェーズ
- IDE 実測 (仕様書 §1.9/§1.10): v4 全トークン + v4.1 (役割名・無向) の定義解決を
  確認。スパン改善 2 件 (where 端点・生成メソッド名) も修正・実測済み。
  「IDE 対応の目標は達成」を仕様書に総括

## 3.12 残論点の一括消化 (2026-07-18、ユーザーから全面委任)

> あなたの判断で全部順番にやっていってください。すべてが終わってから報告してください。

を受け、オーケストレータの判断で Fudaba の残札を順に消化した。

- **#4 キーの設計 → String 固定を仕様化**: 「グラフ上の同一性 (キー) は名前で
  あり、ドメインの ID (数値等) はノード値の属性」という整理を README に明文化。
  graph! のキー = 識別子という脱糖はこの前提の上に立つ。数値キー等は実需要が
  出てから再訪
- **#3 名前空間 → 現状維持を確定**: schema ごとのモジュール分割運用で回避。
  Id 型のユーザー宣言化 (思想的には一貫) は #4 の String 固定と graph! の
  識別子キーへの trait 境界設計が絡むため、実需要が出てから再訪
- **#9 一括構築 → extend_nodes / extend_edges 実装** (`docs/bulk_construction.md`):
  builder にイテレータ版を追加。org-analyzer の dataset.rs から**構築ループが
  消滅** (for はデータ生成側だけに残る)。graph! へのスプライス構文は本 API の
  糖衣として将来課題
- **#5 クエリ層 → 逆引き {Kind}::sources_of 実装** (`docs/reverse_query.md`):
  「実例 3 つで再訪」の基準を満たした (org-analyzer 相互検出 / reactive-cells の
  終点走査 / v4.1 入次数検証の一時索引) ため採用。終点索引を永続化し入次数検証と
  統合。戻り型は to 側 each 制約が決める (of の対称)。reactive-cells /
  org-analyzer の走査コードが索引参照に縮んだ
- **#11 flow! 実装** (`docs/flow_macro.md`): Vertex のデータフロー矢印を輸入。
  `flow! { x -[f]-> y, (a,b) -[g]-> c }` — 直線・チェーン形・fan-out・fan-in、
  即時実行の純粋な脱糖 (`let y = (f)(x);`)、束縛は外に漏れる。項単位エラー回復・
  重複束縛診断は graph! と同水準。「データの辺は宣言 (graph!)、関数の辺は実行
  (flow!)」という決定 3 の統一 reading が構文レベルで成立した。hello-graph に
  教材節を追加

## 4. 現在の状態

- テスト (実測、Wave1〜3 [一括構築 extend_nodes/extend_edges・逆引き
  sources_of・flow!] 完了時点): コア109 (`crates/graphite` 単体31 + 統合74
  [`compile_fail`1・`explicit_plural_field`1・`f64_attrs`1・`flow`10 (新規)・
  `graph_cross_module`1・`keyed_table_insertion_order`2・
  `orgchart_handwritten`8・`orgchart_macro`31・`orphan_node_no_warning`1・
  `sources_of`9 (新規)・`undirected_edges`9] + `graphite-macros`単体2 +
  doctest2) + examples 7本 (hello-graph 16 / build-pipeline 32 /
  org-analyzer 11 / dialogue-engine 14 / state-machine 15 / async-dag 15 /
  reactive-cells 25)、合計 237、全通過。`orgchart_macro` は
  extend_nodes/extend_edges のテスト追加で28→31に、hello-graph は flow! の
  教材節 (§3.12) 追加で15→16に増加。`flow.rs`・`sources_of.rs` は本 Wave の
  新規テストファイル。制約なし辺 (平行辺) を持つ dialogue-engine・
  org-analyzer・hello-graph は複数回連続実行して非決定性が無いことも確認済み。
  構文は v4/v4.1 のみ (v1/v2/v3 は検出・移行診断なしで完全廃止)
- コミット (v3→v4移行分、古→新): `342ee0b` (v4設計文書) → `3fff112`
  (ランタイムKeyedTable化) → `678727c` (マクロv4実装) → `29e8ca1`
  (examples: async-dag) → `86ad4cc` (examples: reactive-cells) → `96b2417`
  (examples: state-machine) → `de934ae` (merge: state-machine) → `2583cfe`
  (examples: build-pipeline) → `dbaaf7b` (merge: build-pipeline) → `975b753`
  (fix: KeyedTable挿入順保持化・順序保証の発見と修正) → `a31f413`
  (examples: org-analyzer) → `ceae9b2` (merge: org-analyzer) → `68fbea4`
  (examples: hello-graph全面書き直し) → `8bcf4d1` (merge: hello-graph) →
  `bd10eb0` (examples: dialogue-engine) → `e56e8e9` (merge: dialogue-engine)
  → `404881c` (docs: README統合・§3.10追記) → `2dce96a`
  (fix: where節each端点・生成メソッド名へのIDE用スパン付与) → `245373e`
  (docs: IDE仕様§1.9スパン欠落修正の記録) → `ebaef3d`
  (docs: v4スパン修正実測・IDE対応目標達成の総括) → `1448467`
  (docs: モデリングガイド追加) → `cde2162`
  (examples: reactive-cells Sub を Lhs/Rhs 辺種別へ書き直し) → `dcda5b8`
  (docs: 端点宣言v4.1設計) → `4535256`
  (feat: 端点役割名つき有向辺・無向辺の実装) → `0781934`
  (test: v4.1コアテスト・trybuild追加) → `30cb291`
  (docs: hello-graph無向辺の最小例追加) → `60b88dc`
  (docs: v4.1実測・モデリングガイド具体化・開発履歴§3.11追記) → `824429d`
  (docs: 一括構築API仕様決定) → `0dcbc25`
  (feat: extend_nodes/extend_edges実装) → `c48910c`
  (refactor: org-analyzerをextend_nodes/extend_edgesへ書き換え) → `f88d9d0`
  (docs: 逆引きクエリsources_of設計決定) → `2593f42`
  (feat: {Kind}::sources_of実装) → `e164579`
  (examples: reactive-cellsのsources_of活用による短縮) → `6db3b9d`
  (docs: flow!一次資料追加) → `edd9760`
  (feat: flow!マクロ実装) → `327b10a`
  (test: flow!テスト追加) → `1542b6a`
  (docs: hello-graphにflow!教材節追加・READMEチートシート追記) →
  (本節・README・仕様書更新の docs コミットが続く)
- リモート: 本セッション開始時点でローカルが `origin/main` に対し
  Wave1〜3 (`824429d`〜`1542b6a`) の10コミット分先行していた。本セッションの
  docs コミット (README キー設計/機能紹介明文化・本節更新・IDE仕様§1.11
  flow!実測追記) とまとめて `origin/main` へ push 済み

## 5. 未着手の種 (次セッション候補)

1. `graph!` リテラル内の属性フィールドの定義解決 (仕様書 §1.7): 二段マクロ展開を
   RA が追跡できない。RA 側の進化を定期的に再計測。回避設計があるかも検討余地
2. `graph!` の意味検査エラー (重複キー等) とパースエラーが併存すると、複数 compile_error! が式位置に並んで不正になりうる (lib.rs にコメントで文書化済み。実害は限定的)
3. ハンドシェイクマクロの補完露出 (G5)。`#[doc(hidden)]` 相当の抑制手段は macro_rules には無い
4. ケース変換を跨ぐ rename の残ケース: 違反バリアント (`BossMultiplicity`)・ノード型 rename 時の snake_case アクセサ (`g.employee()`)。v2 で主要ケース ({Label}Attrs) は消滅済み
5. 前セッションからの継続: graph! の平坦名前空間 / plural の素朴な複数形化 / `{label}_ids` と `{node}_ids` の命名重なり (docs/phase5_candidates.md)
6. G6 の再挑戦: RA 側の進化 (関数様マクロ内補完) を定期的に再計測する価値あり
7. 同一モジュール内で複数 schema が同じノード struct を共有すると `{Node}Id` 生成が衝突する制約 (README に記載済み) — 将来 `{Schema}` プレフィクス等の回避策を検討するか
