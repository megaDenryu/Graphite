// 辺の柄に書かれた積み荷 `[役割名: 型]` の宣言。役割名がフィールド名・
// アクセサ名になり、型が積み荷struct のフィールド型になる。裸のタプルへ
// 分解して運ぶのをやめ、名前付きで持ち歩く。同crateの
// schema/syntax/edge_payload.rs (EdgePayload) と同じ設計。

use proc_macro2::Ident;

#[derive(Clone)]
pub struct 積み荷宣言 {
    pub 役割: Ident,
    pub 型: Ident,
}
