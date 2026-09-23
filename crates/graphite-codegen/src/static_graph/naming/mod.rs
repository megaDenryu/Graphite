//! 静的グラフの生成名 (issue #41)。`追跡付きの名前` か `内部生成名` だけを
//! 返す。`format_ident!` と `span = ...` はここに集約し、internal/codegen・
//! file 側には書かない (crate直下の `naming/mod.rs` が動的グラフに対して
//! 持つ規則と同じもの、この静的グラフ側はそれとは別の module である)。
//!
//! - `type_names`: `{個体名}Ref`・`{辺名}Ref`・`{種別}Edge` (issue #41 分類A)。
//!   定義箇所 (schemaファイル・instanceファイル自身のmodule内) からの参照
//!   であり、doc付きの `追跡付きの名前` を返す。
//! - `fixed_vocabulary`: `Nodes`/`Edges`/`NodeRefs`/`EdgeRefs`/`Graph`・
//!   `new`・`entity`・`node_refs`/`edge_refs` (分類B、簡潔な意味カード)。
//! - `card_names`: 意味カードの書式を §5.2 の例そのままで固定した4件
//!   (辺アクセサメソッド・役割アクセサ・積み荷アクセサ・`Nodes::new`)。
//! - `reference_paths`: instance側 (instanceファイルの本文・DSLトークンの
//!   錨) からの、別module越しの修飾パス参照 (`{schema名}::{種別}Edge`・
//!   `{グラフ名}::{名前}Ref`)。doc を持たない生の `TokenStream` を返す
//!   (issue #41 是正2)。
//! - `internal_names`: `inline/` が使う内部生成名 (issue #41 是正13)。

mod card_names;
mod field_card_names;
mod fixed_vocabulary;
mod internal_names;
mod reference_paths;
mod tracked_name;
mod type_names;
#[cfg(test)]
mod tests;

pub(crate) use card_names::{
    個体実体所有者構築メソッド名, 役割アクセサの追跡情報を作る, 積み荷アクセサの追跡情報を作る, 辺アクセサメソッドの追跡情報を作る,
};
pub(crate) use field_card_names::{
    edge_refsフィールドの追跡情報を作る, edgesフィールドの追跡情報を作る, node_refsフィールドの追跡情報を作る,
    nodesフィールドの追跡情報を作る,
};
pub(crate) use fixed_vocabulary::{
    個体実体所有者型名, 個体参照フィールド名, 個体参照集合型名, 構築メソッド名, 実体アクセサメソッド名, 辺実体所有者型名,
    辺参照フィールド名, 辺参照集合型名, グラフ型名,
};
pub(crate) use internal_names::{個体供給関数名, 積み荷供給関数名, 錨関数名};
pub(crate) use reference_paths::{個体参照パス, 辺値参照パス, 辺参照パス};
pub(crate) use type_names::{個体参照型名, 辺値型名, 辺参照型名};
