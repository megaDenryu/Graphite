//! 静的グラフの生成名 (issue #41)。`追跡付きの名前` か `内部生成名` だけを
//! 返す。`format_ident!` と `span = ...` はここに集約し、`file` 側には
//! 書かない (crate直下の `naming/mod.rs` が動的グラフに対して持つ規則と
//! 同じもの、この静的グラフ側はそれとは別の module である)。
//!
//! - `type_names`: `{個体名}Ref`・`{辺名}Ref`・`{種別}Edge` (issue #41 分類A)。
//!   定義箇所 (schemaファイル・instanceファイル自身のmodule内) からの参照
//!   であり、doc付きの `追跡付きの名前` を返す。
//! - `fixed_vocabulary`: `NodeRefs`/`EdgeRefs`/`Graph`・`entity`・
//!   `node_refs`/`edge_refs` (分類B、簡潔な意味カード。`node_refs`/
//!   `edge_refs`は`Graph`が持つメソッドの名前である)。
//! - `card_names`: 意味カードの書式を §5.2 の例そのままで固定した3件
//!   (辺アクセサメソッド・役割アクセサ・積み荷アクセサ)。
//! - `accessor_card_names`: `NodeRefs`/`EdgeRefs`が個体・具体辺1件ごとに
//!   持つ読み出しメソッドの意味カード。
//! - `construct_fixed_vocabulary`: `construct!` (値ありの個体・積み荷を
//!   差し替えられない、`Graph`を実体化する唯一の入口。`Graph`の内部構築子は
//!   内部専用のC分類であり、`naming::internal_names::内部構築子名` が
//!   名前を持つ)。
//! - `reference_paths`: instance側 (instanceファイルの本文・DSLトークンの
//!   型参照) からの、別module越しの修飾パス参照 (`{schema名}::{種別}Edge`・
//!   `{グラフ名}::{名前}Ref`)。doc を持たない生の `TokenStream` を返す。
//! - `internal_names`: `inline/` が使う内部生成名。
//! - `wiring_names`: `graph` (配線用、docを持たない非公開フィールド・
//!   引数の名前)。
//! - `fingerprint_anchor`: `schema_entry.rs`・`instance_entry.rs` が指紋照合
//!   コードのmoduleパスに使う、`generated = "..."` リテラルのspanを持つ
//!   識別子 (doc を持たない。C分類でもない: 実在するschema名/instance名の
//!   トークンのspanを、別の実在するトークン (`generated` リテラル) の
//!   spanへ付け替えるだけであり、公開APIの名前ではなく診断のspanだけに
//!   使う)。

mod accessor_card_names;
mod card_names;
mod construct_fixed_vocabulary;
mod fingerprint_anchor;
mod fixed_vocabulary;
mod internal_names;
mod reference_paths;
mod tracked_name;
mod type_names;
mod wiring_names;
#[cfg(test)]
mod tests;

pub(crate) use accessor_card_names::{edge_refsメソッドの追跡情報を作る, node_refsメソッドの追跡情報を作る};
pub(crate) use card_names::{役割アクセサの追跡情報を作る, 積み荷アクセサの追跡情報を作る, 辺アクセサメソッドの追跡情報を作る};
pub(crate) use construct_fixed_vocabulary::構築マクロ名;
pub(crate) use fixed_vocabulary::{
    個体参照メソッド名, 個体参照集合型名, 実体アクセサメソッド名, 辺参照メソッド名, 辺参照集合型名, グラフ型名,
};
pub(crate) use fingerprint_anchor::指紋照合パスの起点;
pub(crate) use internal_names::{
    型参照関数名, 個体値マクロ名, 値キャプチャ変数名, 値関数名, 値束縛マクロ名, 内部構築子名, 積み荷値マクロ名,
};
pub(crate) use reference_paths::{個体参照パス, 辺値参照パス, 辺参照パス};
pub(crate) use type_names::{個体参照型名, 辺値型名, 辺参照型名};
pub(crate) use wiring_names::graphフィールド名;
