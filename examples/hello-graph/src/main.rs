//! hello-graph — Graphite (`graph_schema!`/`graph!`) の意味論を確認する
//! 入門用example。
//!
//! **これは教材です。** アプリとしての面白さは無く、「1個ずつ意味論を
//! 確かめる」ことだけが目的です。実践的な使用例は他の3本
//! (`examples/build-pipeline`・`examples/org-analyzer`・
//! `examples/dialogue-engine`) を参照してください。
//!
//! 上から順に読める構成にしています:
//! - §1 ノード型・エッジ属性型の宣言 (普通の struct)
//! - §2 `graph_schema!` によるスキーマ宣言 (ラベルとは何なのかの説明)
//! - §3 `graph!` での構築と、多重度ごとのアクセス方法
//! - §4 「できないこと」— コンパイルエラーになる例と、実際のエラー引用
//!
//! `cargo run` すると §3 の内容が順に表示されます。

// ============================================================
// §1 型宣言 — ノード型・エッジ属性型は普通の Rust struct
// ============================================================
//
// `graph_schema!` はこれらの型を**生成せず、参照するだけ**です
// (README「使用例」節、`docs/edge_syntax_v2.md` 参照)。derive・可視性・
// 追加のメソッドは全部ふつうの Rust の話であり、Graphite 固有のルールは
// ありません。

/// ノード型その1: 社員。
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

/// ノード型その2: チーム。
#[derive(Debug, Clone, PartialEq)]
pub struct Team {
    pub name: String,
}

/// `boss` エッジが辺1本ごとに運ぶペイロード (属性)。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

// ============================================================
// §2 schema 宣言
// ============================================================
//
// `edge From -[label]-> To (多重度);` の読み方:
//
// - `label` は「エッジ種別の宣言」です。struct のフィールド名に相当する
//   1トークンで、ここから `{label}`/`try_{label}`/`{label}_id(s)`/
//   `{label}_pairs` といったアクセサ・builder メソッド・違反 enum の
//   バリアントが機械的に命名・生成されます。つまり `label` は「値」では
//   なく「これから生成される一群のメソッド名の元になる識別子」です。
//   値のように読み書きできる変数ではありません (§4 で実際に確認します)。
// - `label: Type` の `Type` は「辺1本ごとが運ぶペイロードの型」です。
//   属性なしエッジ (`belongs_to`) は何も運ばず、属性ありエッジ
//   (`boss: BossEdge`) は `BossEdge` の値を辺1本ごとに1つ持ちます。
// - 多重度 `(1)`/`(0..1)`/`(0..*)` は矢印の外側に書きます (辺そのものの
//   属性ではなく「本数の制約」だからです)。

#[rustfmt::skip]
graphite::graph_schema! {
    schema Org {
        node Person;
        node Team;

        edge Person -[belongs_to]-> Team (1);
        edge Person -[boss: BossEdge]-> Person (0..1);
        edge Person -[reports]-> Person (0..*);
    }
}

fn main() {
    section3();
}

// ============================================================
// §3 graph! リテラルでの構築と、多重度ごとのアクセス
// ============================================================

fn section3() {
    println!("=== §3 graph! で組み立てて、多重度ごとにアクセスする ===\n");

    #[rustfmt::skip]
    let g = graphite::graph!(Org {
        alice: Person { name: "Alice".into() },
        bob:   Person { name: "Bob".into() },
        carol: Person { name: "Carol".into() },
        eng:   Team { name: "Engineering".into() },

        alice -[belongs_to]-> eng,
        bob   -[belongs_to]-> eng,
        carol -[belongs_to]-> eng,
        bob   -[boss { since: 2021 }]-> alice,
        alice -[reports]-> bob,
        alice -[reports]-> carol,
    })
    .expect("正常なグラフは構築に成功するはず");

    // --- 多重度 (1): 必須1本。アクセサは参照そのものを返す ---
    // `belongs_to` は `edge Person -[belongs_to]-> Team (1)` から生成された
    // アクセサメソッド。属性が無いので戻り値は `&Team` そのもの。
    // 未知キーを渡すとパニックする (`Vec` の `v[i]` と同じ「呼び出し規約
    // 違反」の扱い。`# Panics` に明記されている)。
    let team: &Team = g.belongs_to(&PersonId("alice".to_string()));
    println!("(1)      alice の所属チーム: {}", team.name);

    // パニックしない版 `try_{label}` は `Option` を返す。
    let unknown = g.try_belongs_to(&PersonId("dave".to_string()));
    println!("(1) try  未知キー dave の所属チーム: {unknown:?}");

    // --- 多重度 (0..1): 高々1本。アクセサは Option<(&Node, &Attrs)> ---
    // `boss` は属性つきエッジなので、Option の中身は
    // (相手ノードへの参照, 属性への参照) のタプル。属性の値へは
    // "ふつうのフィールドアクセス" で辿れる (`attrs.since`)。
    if let Some((boss_person, attrs)) = g.boss(&PersonId("bob".to_string())) {
        println!(
            "(0..1)   bob の上司: {} (就任年: {})",
            boss_person.name, attrs.since
        );
    }
    let no_boss = g.boss(&PersonId("alice".to_string()));
    println!("(0..1)   alice の上司: {no_boss:?}");

    // --- 多重度 (0..*): 0本以上。アクセサは Vec ---
    let reports: Vec<&Person> = g.reports(&PersonId("alice".to_string()));
    let names: Vec<&str> = reports.iter().map(|p| p.name.as_str()).collect();
    println!("(0..*)   alice の部下: {names:?}");

    // --- {label}_pairs(): 辺 = (from, to[, attrs]) の組を列挙する ---
    // 属性なしエッジは (&SrcId, &DstId) の2つ組、属性ありエッジは
    // (&SrcId, &DstId, &Attrs) の3つ組になる。
    println!("\nbelongs_to_pairs (2つ組: from, to):");
    for (from, to) in g.belongs_to_pairs() {
        println!("  {from:?} -> {to:?}");
    }
    println!("boss_pairs (3つ組: from, to, attrs):");
    for (from, to, attrs) in g.boss_pairs() {
        println!("  {from:?} -> {to:?} (since={})", attrs.since);
    }

    // --- {node_snake}_ids(): ノード種別ごとのキー列挙 ---
    let mut people: Vec<String> = g.person_ids().map(|id| id.0.clone()).collect();
    people.sort();
    println!("\nperson_ids: {people:?}");
}

// ============================================================
// §4 「できないこと」
// ============================================================
//
// 以下はすべてコメントアウトしてあります。コメントを外して
// `cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50`
// すると、直下に引用したものと同じエラーが実際に出ることを確認できます
// (このファイルに記載のエラー文はすべて実測したもので、書き下ろしでは
// ありません)。

// --- 4.1 ラベルを変数として使おうとする ---
//
// `boss` はスキーマ宣言の中の1トークンであり、実行時に読み書きできる
// 変数ではありません。生成されるのは `g.boss(...)` という**メソッド**
// であって、裸の `boss` という名前の値は存在しません。
//
// fn section4_1(g: &Org) {
//     let _ = boss.since;
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0425]: cannot find value `boss` in this scope
//      --> src\main.rs:166:13
//       |
//   166 |     let _ = boss.since;
//       |             ^^^^ not found in this scope

// --- 4.2 フィールドに直接アクセスしようとする (メソッド呼び出しを忘れる) ---
//
// アクセサは常に「呼び出す」もの (`g.boss(&id)`) であり、`g.boss` という
// フィールドそのものは非公開の内部ストレージ (`HashMap<PersonId, (PersonId, BossEdge)>`)
// です。このファイルは schema 宣言と同じモジュールなので `g.boss` という
// 式自体は private エラーにはなりません (Rust の可視性はモジュール単位
// であり、同一モジュール内では非公開フィールドも見えるため)。しかし
// 中身は `Person` ではなく内部ストレージそのものなので、`Person` として
// 使おうとした瞬間に型不一致になります。括弧を付け忘れて `g.boss` と
// 書くと、素朴には使えません。
//
// fn section4_2(g: &Org) -> Person {
//     g.boss
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0308]: mismatched types
//      --> src\main.rs:184:5
//       |
//   183 | fn section4_2(g: &Org) -> Person {
//       |                           ------ expected `Person` because of return type
//   184 |     g.boss
//       |     ^^^^^^ expected `Person`, found `HashMap<PersonId, (PersonId, BossEdge)>`
//       |
//       = note: expected struct `Person`
//                  found struct `HashMap<PersonId, (PersonId, BossEdge)>`
//
// (`g.boss` という式そのものは同一モジュール内なので private エラーには
// ならず素朴に評価できてしまいますが、その型は `Person` ではなく内部
// ストレージの `HashMap` そのものであることがこの型不一致から分かります。
// つまり「boss というフィールドで社員そのものが手に入る」という誤解は
// この型不一致で正されます。)

// --- 4.3 存在しないエッジラベルを graph! に書く ---
//
// `graph!` は `graph_schema!` が生成したハンドシェイクマクロ経由でラベルを
// 検査するため、未知のラベルには「利用可能なエッジ一覧」付きの
// `compile_error!` が出ます。
//
// fn section4_3() {
//     #[rustfmt::skip]
//     let _ = graphite::graph!(Org {
//         alice: Person { name: "Alice".into() },
//         eng: Team { name: "Engineering".into() },
//         alice -[no_such_label]-> eng,
//     });
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取。ハンドシェイク
// マクロの `compile_error!` と、それに続く rustc 標準の「メソッドが
// 見つからない」エラーの2つが重ねて出ます):
//
//   error: スキーマ Org にエッジ `no_such_label` は存在しません。利用可能: belongs_to, boss, reports
//      --> src\main.rs:63:1
//       |
//    63 | / graphite::graph_schema! {
//    64 | |     schema Org {
//       | ...
//    72 | | }
//       | |_^
//   ...
//   205 |       let _ = graphite::graph!(Org {
//       |  _____________-
//   ...
//   208 | |         alice -[no_such_label]-> eng,
//   209 | |     });
//       | |______- in this macro invocation
//
//   error[E0599]: no method named `no_such_label` found for mutable reference
//   `&mut OrgBuilder` in the current scope
//      --> src\main.rs:208:17
//       |
//   208 | |         alice -[no_such_label]-> eng,
//       | |                -^^^^^^^^^^^^^ method not found in `&mut OrgBuilder`

// --- 4.4 端点の型を間違えたエッジを graph! に書く ---
//
// `belongs_to` は `Person -[belongs_to]-> Team` として宣言されているので、
// from/to は Person/Team でなければなりません。両方を Person にすると
// 型不一致になります。
//
// fn section4_4() {
//     #[rustfmt::skip]
//     let _ = graphite::graph!(Org {
//         alice: Person { name: "Alice".into() },
//         bob: Person { name: "Bob".into() },
//         alice -[belongs_to]-> bob,
//     });
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0308]: mismatched types
//      --> src\main.rs:228:13
//       |
//   228 |       let _ = graphite::graph!(Org {
//       |  _____________^
//   229 | |         alice: Person { name: "Alice".into() },
//   230 | |         bob: Person { name: "Bob".into() },
//   231 | |         alice -[belongs_to]-> bob,
//       | |                 ---------- arguments to this method are incorrect
//   232 | |     });
//       | |______^ expected `TeamId`, found `PersonId`
//       |
//   note: method defined here
//      --> src\main.rs:68:23
//       |
//    68 | |         edge Person -[belongs_to]-> Team (1);
//       | |                       ^^^^^^^^^^

#[cfg(test)]
mod tests {
    use super::*;

    fn build() -> Org {
        #[rustfmt::skip]
        let g = graphite::graph!(Org {
            alice: Person { name: "Alice".into() },
            bob:   Person { name: "Bob".into() },
            carol: Person { name: "Carol".into() },
            eng:   Team { name: "Engineering".into() },

            alice -[belongs_to]-> eng,
            bob   -[belongs_to]-> eng,
            carol -[belongs_to]-> eng,
            bob   -[boss { since: 2021 }]-> alice,
            alice -[reports]-> bob,
            alice -[reports]-> carol,
        });
        g.expect("正常なグラフは構築に成功するはず")
    }

    #[test]
    fn 多重度1のアクセサは参照そのものを返す() {
        let g = build();
        let team = g.belongs_to(&PersonId("alice".to_string()));
        assert_eq!(team.name, "Engineering");
    }

    #[test]
    fn 多重度0か1のアクセサはoptionのタプルを返し属性フィールドへアクセスできる() {
        let g = build();
        let (boss, attrs) = g
            .boss(&PersonId("bob".to_string()))
            .expect("bobの上司はaliceのはず");
        assert_eq!(boss.name, "Alice");
        assert_eq!(attrs.since, 2021);
        assert!(g.boss(&PersonId("alice".to_string())).is_none());
    }

    #[test]
    fn 多重度0以上のアクセサはvecを返す() {
        let g = build();
        let mut names: Vec<&str> = g
            .reports(&PersonId("alice".to_string()))
            .into_iter()
            .map(|p| p.name.as_str())
            .collect();
        names.sort();
        assert_eq!(names, vec!["Bob", "Carol"]);
    }

    #[test]
    fn try_belongs_toは未知キーでnoneを返す() {
        let g = build();
        assert!(g
            .try_belongs_to(&PersonId("dave".to_string()))
            .is_none());
    }

    #[test]
    fn pairsイテレータは3つ組で列挙できる() {
        let g = build();
        let boss_pairs: Vec<(&PersonId, &PersonId, &BossEdge)> = g.boss_pairs().collect();
        assert_eq!(boss_pairs.len(), 1);
        let (from, to, attrs) = boss_pairs[0];
        assert_eq!(*from, PersonId("bob".to_string()));
        assert_eq!(*to, PersonId("alice".to_string()));
        assert_eq!(attrs.since, 2021);
    }
}
