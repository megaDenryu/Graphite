# 生成コードと手書きテンプレートとの差異

> **Development document** — 索引: `docs/README.md`

この文書は、schema生成コードが `crates/graphite/tests/orgchart_handwritten.rs`
の手書きテンプレートと異なる設計判断をした箇所を記録する文書であり、実装が
変わるたびに追随して更新する。

schema生成コードは基本的に `orgchart_handwritten.rs` と同じ形を生成するが、
「任意のノード型・エッジ型の組み合わせ」へ一般化する過程で次の7点が手書き版と
分かれた。

1. **違反 enum は1スキーマにつき1つ生成する** (`{Schema}Violation`)。手書き版は
   `SchemaViolation` という固定名だったが、複数のスキーマを同じモジュールへ
   宣言したときに型名が衝突しないよう、スキーマ名を接頭辞にしている。
2. **違反 enum のバリアントはエッジ単位で型付き生成する**
   (`{Kind}{役割名}EachViolation` / `{Kind}UniquePairViolation` /
   `{Kind}DuplicateKey` / `{Kind}UnknownSource` / `{Kind}UnknownTarget`)。
   手書き版は `MultiplicityViolation { employee: EmployeeId, .. }` という
   スキーマ共通の1バリアントだったが、一般のスキーマでは辺ごとに始点・終点の
   ノード型が異なりうる (例: `A -> B` と `C -> D` が両方 each 違反を起こしうる)。
   辺ごとに専用バリアントを生成することで、型を `String` へ落とさず固定できる
   (「型の strictness」原則。`docs/development/design_principles.md` 原則1)。
   例: `edge BelongsTo = (employee: Employee) -> (department: Department) where each employee: 1;`
   からは `BelongsToEmployeeEachViolation { source: EmployeeId, count: usize }` /
   `BelongsToUnknownSource { edge: BelongsToId, source: EmployeeId }` /
   `BelongsToUnknownTarget { edge: BelongsToId, target: DepartmentId }` /
   `BelongsToDuplicateKey(BelongsToId)` が生成される (辺キー重複と
   `unique pair` 違反は v4 で追加した。`docs/schema_v4.md` §3.1)。
3. **builder のエッジ追加メソッドの引数は `({Kind}Id, {Kind})` である**。手書き版
   は `boss(employee, boss, attrs)` ・ `reports(manager, report)` のように端点を
   直接引数へ取っていたが、v4 では辺そのものが第一級のキー付き要素になった。
   builder のエッジメソッドは常に「辺キー + 名前付きフィールドの辺値」の対を
   取る (`b.boss(OrgChart::BossId("b1".into()), OrgChart::Boss { subordinate: employee_id, superior: boss_id, appointment: attrs })`
   の形)。
4. **内部ストレージの複数形フィールド名は素朴な英語複数形 (`+ "s"`) で固定する**。
   不規則複数形 (`Category` が `Categorys` になる等) には自動対応していない。
   この名前は非公開フィールドで利用者から見えないため機能上の問題は無く、明示
   指定構文も持たない (`docs/graph_splice.md` §3)。生成コードを `cargo expand`
   等で目視するときは注意する。
5. **導出エッジ (`colleagues` 等) はマクロが生成しない**。公開クエリAPIだけで
   導出クエリを書けるため、`impl OrgChart::Graph { .. }` へ普通のメソッドとして
   後から追記する。
6. **ノード値の型・エッジ属性型はいずれも利用者が `graph_schema!` の外で宣言し、
   マクロは参照するだけである**。手書き版は `pub struct Employee { .. }` /
   `pub struct BossAttrs { pub since: i32 }` をテンプレート内へ直接書いていたが、
   マクロはこれらの型を一切生成せず、スキーマ宣言 (`node Employee;` /
   `edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;`)
   に書かれた型をそのまま参照する。derive の要求も無いため、derive するか
   どうかも含めて利用者の自由である。
7. **ノード・エッジのID型は宣言ごとに既定生成と明示指定を選ぶ**。省略時は schema
   module 内へ `{Node}Id` / `{Kind}Id` を生成する。既存型を使う宣言は
   `(id: 型パス)` を付け、`graph!` では `名前 @ ID式 = 値` と書く (詳細は
   `docs/node_id_v4_2.md`)。
