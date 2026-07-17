# エッジアクセス API — ビュー方式 (導出名の全廃)

> **[v4 (`docs/schema_v4.md`) で置換済み]** このファイルは歴史的記録として残す。
> ビュー6型 (`EdgeOne`/`EdgeOneWith`/`EdgeOption`/`EdgeOptionWith`/`EdgeMany`/
> `EdgeManyWith`) は v4 で全廃され、型名前空間アクセス (`Kind::of`/`get`/
> `between`/`iter`/`ids`/`len`) に置き換わった。現行のエッジアクセスAPIは
> `docs/schema_v4.md` §3.2 を参照すること。

2026-07-15 セッション2 でのユーザー決定。設計考察はオーケストレータ (本ドキュメント
著者) によるもので、経緯は `docs/dev_history_2026-07-14_session2.md` を参照。

## 1. 動機 (ユーザー指摘の原文趣旨)

> (生成 API の命名規則について) テンプレート文字列みたいなもんじゃないですか?
> 確かに参照ジャンプできますが、これ (教材) を読まないと無理ですし、
> 覚えることが多すぎて複雑すぎると思います。

指摘の通り、旧 API はラベル 1 個から `boss`/`try_boss`/`boss_id`/`try_boss_id`/
`boss_ids`/`boss_pairs` という**名前の文字列連結でメソッド群を合成**しており、
これは本セッションで排除してきた「隠れた導出名」(rename のケース変換の壁、
`{Label}Attrs` 等) と同族の設計欠陥である。命名規則を知らなければ API の存在を
推測できず、補完リストも `ラベル数 × 6` で平坦に膨れる。

## 2. 新 API

**ラベルから生成するメソッドはビューを返す 1 個だけ**にし、操作の語彙は
graphite ランタイムの固定メソッド (全 schema・全ラベル共通) にする:

```rust
// 旧                              // 新
g.belongs_to(&alice)               g.belongs_to().of(&alice)     // (1)    &Team
g.try_belongs_to(&alice)           g.belongs_to().get(&alice)    // (1)    Option<&Team>
g.belongs_to_id(&alice)            g.belongs_to().id_of(&alice)  // (1)    &TeamId
g.try_belongs_to_id(&alice)        g.belongs_to().get_id(&alice) // (1)    Option<&TeamId>
g.boss(&bob)                       g.boss().of(&bob)             // (0..1) Option<(&Person, &BossEdge)>
g.boss_id(&bob)                    g.boss().id_of(&bob)          // (0..1) Option<&PersonId>
g.reports(&alice)                  g.reports().of(&alice)        // (0..*) Vec<&Person>
g.reports_ids(&alice)              g.reports().ids_of(&alice)    // (0..*) Vec<&PersonId>
g.{label}_pairs()                  g.{label}().iter()            // 表全体の走査
```

### 語彙の規則 (覚えるのはこれだけ)

- **`of(&from_id)`** — そのラベルの自然な戻り値。**多重度が型を決める**:
  `(1)` → 参照そのもの (未知キーはパニック、`# Panics` 明記)、
  `(0..1)` → `Option`、`(0..*)` → `Vec`。属性ありエッジは相手が
  `(&To, &Attrs)` のタプルになる。
- **`get(&from_id)`** — `of` の Option 版。**`(1)` のビューにのみ存在する**
  (`(0..1)`/`(0..*)` は `of` が既に全域関数なので生成しない。旧 `try_` が
  多重度(1)にのみ存在したのと同じ理屈)。std の `HashMap::get` と同じ意味論。
- **`id_of` / `get_id` / `ids_of`** — 相手のノード値ではなくキーが欲しいとき。
  `of`/`get` と同じ多重度規則 (`(0..*)` は複数形 `ids_of`)。
- **`iter()`** — 表全体を辺単位で走査。属性なしは `(&FromId, &ToId)` の 2 つ組、
  属性ありは `(&FromId, &ToId, &Attrs)` の 3 つ組 (旧 `_pairs` と同一)。
  `(0..*)` の記述順保証は従来どおり維持。
- **`len()` / `is_empty()`** — 表の辺の本数。

発見可能性: `g.` の補完 = ラベルの短い一覧 (schema そのもの)。`g.boss().` の
補完 = その多重度で可能な操作だけ。**命名規則の暗記も教材の参照も不要になる**。

### メンタルモデルとの一致

hello-graph §2.5 の「ラベル = テーブル名」の比喩がそのまま API になる:
`g.boss()` = 表を取る、`.of(&bob)` = 表を引く、`.iter()` = 表を走査する。

## 3. 実装設計

### 3.1 ビュー型は graphite ランタイムに置く (マクロ生成しない)

多重度 × 属性有無 = 6 種のジェネリックなビュー型を `graphite` クレートに
1 回だけ手書きする (rustdoc もここに 1 回だけ書く):

| 型 | 多重度 | 属性 | 参照する内部表 |
|---|---|---|---|
| `EdgeOne<'g, F, T, To>` | (1) | なし | `HashMap<F, T>` |
| `EdgeOneWith<'g, F, T, To, A>` | (1) | あり | `HashMap<F, (T, A)>` |
| `EdgeOption<'g, F, T, To>` | (0..1) | なし | `HashMap<F, T>` |
| `EdgeOptionWith<'g, F, T, To, A>` | (0..1) | あり | `HashMap<F, (T, A)>` |
| `EdgeMany<'g, F, T, To>` | (0..*) | なし | `HashMap<F, Vec<T>>` |
| `EdgeManyWith<'g, F, T, To, A>` | (0..*) | あり | `HashMap<F, Vec<(T, A)>>` |

(F = FromId、T = ToId、To = 相手ノード値の型、A = 属性型。型名は実装時に
原則3 で微調整してよいが、6 分割・ランタイム側配置・ゼロコストは確定事項)

- 各ビューは「エッジ表への参照」と「相手ノードのストレージへの参照」の
  2 つの `&'g` を持つ (To の解決に必要)。ゼロサイズに近い借用ラッパーで、
  メソッドは全て inline 可能 (原則5)。
- `(0..*)` の記述順保証・`of` のパニック文言 (旧アクセサと同等の情報量) を維持。

### 3.2 マクロが生成するもの

```rust
impl Org {
    /// boss 表 ((0..1)、ペイロード BossEdge) へのビュー。
    pub fn boss(&self) -> graphite::EdgeOptionWith<'_, PersonId, PersonId, Person, BossEdge> {
        graphite::EdgeOptionWith::new(&self.boss, &self.people)
    }
}
```

の薄い 1 メソッドのみ。`try_{label}`/`{label}_id`/`{label}_ids`/`{label}_pairs`
の生成は**全廃** (旧 API の痕跡を残さない。検出・移行診断も設けない —
既定方針どおり素の method-not-found に任せる)。

- スパン規約: `fn boss` の ident はラベルの出現トークンそのまま (従来どおり)。
  導出名が消えたため、**ラベルの F2 rename は完全カスケードになる**
  (`docs/ide_support_spec.md` §1.5 の境界条件がエッジについて無関係になる)。

### 3.3 変更しないもの

- builder API (`b.boss(from, to, attrs)` / 総称 `insert` / 型名メソッド) —
  もともとラベル 1 個 = メソッド 1 個で爆発していない
- ノードアクセサ (`g.person(&id)` / `g.person_ids()`) — 2 個のみで許容範囲。
  ビュー化はエッジで運用してみてから判断する将来課題
- 違反 enum・`create`/`create_collecting`・graph! リテラル構文 (v3)

## 4. 移行対象

- crates/graphite/tests/ の全アクセサ呼び出し・trybuild stderr
- examples 4 本 (hello-graph は §3 クックブックのエッジ節と README チートシートを
  新語彙で書き直し。関数数はむしろ減るはず)
- README (root) の使用例・API 説明
- `docs/ide_support_spec.md` は実装後に定義ジャンプを再計測して追記
