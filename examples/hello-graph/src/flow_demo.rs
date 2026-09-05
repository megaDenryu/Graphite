//! §5 `flow!` — 関数の辺 (`graph!` の宣言される辺との対比)。
//!
//! `graph_schema!`/`graph!` の辺 (`edge Kind = ...` / `Kind(from -> to)`) は
//! **宣言**です — 構築 (`create`) 時にまとめて検証されるデータの繋がりで、
//! 矢印の中の値は `graph!` が名前付きフィールドの辺値へ組み立てます。対して
//! `graphite::flow!` (`docs/flow_macro.md`) の矢印 `-[関数式]->` は
//! **実行**です — 書かれた順に `let 束縛名 = (関数式)(始点..);` という
//! ただの関数呼び出しへ即時に脱糖するだけで、スキーマや builder は
//! 一切関与しません。同じ矢印記法 `-[X]->` を「宣言されるデータの辺」(`graph!`)
//! と「即時実行される関数の辺」(`flow!`) という対照的な2つの意味論に使い
//! 分けている、という読み方が両者を統一します — どちらも「ノードは値、
//! 矢印の中の `X` が辺の主役」という同じ形を共有しているのに、`X` が
//! 「運ばれる積み荷の型/値」なのか「今すぐ呼ばれる関数」なのかで意味が
//! 分岐する、という対応です。`flow!` は文位置マクロなので、束縛名は
//! `graph!` のノード/エッジキーのように builder クロージャの中に閉じず、
//! 普通の `let` 束縛としてマクロ呼び出しの後にそのまま見えます。

pub fn section5() {
    println!("\n=== §5 flow!: 関数の辺 (宣言ではなく即時実行) ===\n");

    fn parse(s: &str) -> i32 {
        s.parse().expect("数値のはず")
    }
    fn validate(x: i32) -> bool {
        x >= 0
    }
    fn double(x: i32) -> i32 {
        x * 2
    }
    fn merge(valid: bool, doubled: i32) -> String {
        format!("valid={valid} doubled={doubled}")
    }

    #[rustfmt::skip]
    graphite::flow! {
        "21" -[parse]-> parsed,              // 直線 (1本の矢印)
        parsed -[validate]-> valid,          // fan-out: parsed を2本の矢印に流す
        parsed -[double]-> doubled,
        (valid, doubled) -[merge]-> summary, // fan-in: タプル始点は多引数呼び出しに脱糖
    };
    // parsed/valid/doubled/summary はいずれも flow! の後で普通のローカル
    // 変数として見える (§3 の graph! 左辺名は名前付きwrapperのメソッドとして
    // 完成後に残るが、ローカル変数そのものが外へ漏れるわけではない)。
    println!("(flow!) parsed={parsed} valid={valid} doubled={doubled}");
    println!("(flow!) summary = {summary}");
}
