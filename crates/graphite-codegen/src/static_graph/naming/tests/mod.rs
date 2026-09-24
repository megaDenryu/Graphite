// issue #41 段階1の完了条件: 設計書 §5.2 の4つの意味カード (個体参照・辺
// アクセサメソッド・役割アクセサ・固定語彙の構築メソッド) と、追加で
// 用意した積み荷アクセサの意味カードの本文を単体試験で固定する。カードごとに
// 独立したファイルへ分ける (1ファイル100行の原則。それぞれ別の生成物・別の
// naming関数を検査する独立した責務であり、この module本体は例の組み立てだけ
// を共有する)。
//
// 例は設計書 §5.2 と同じ組織schema・開発チームinstanceを使う
// (`examples/static-org` と同じ形)。

mod card1_individual;
mod card2_edge_accessor;
mod card3_role_accessor;
mod card4_fixed_vocabulary;
mod card5_payload_accessor;

use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;
use crate::static_graph::semantic::意味モデル;

pub(super) fn 開発チームの意味モデルを作る() -> 意味モデル {
    let schema: 静的グラフ型入力 = syn::parse2(quote! {
        schema 組織 {
            node 社員;
            node 部署;
            edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
        }
    })
    .unwrap();
    let instance: 静的グラフ入力 = syn::parse2(quote! {
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 次郎 = 社員 { 名前: "次郎".into() };
        node 一郎: 社員 = 社員を作る("一郎");
        node 開発部: 部署;
        edge 太郎の所属 = 所属(太郎 -> 開発部);
        edge 次郎の所属 = 所属(次郎 -> 開発部);
        edge 一郎の所属 = 所属(一郎 -> 開発部);
    })
    .unwrap();
    crate::static_graph::internal::検証してから意味モデルを組み立てる(schema, instance)
}

// この試験群はschemaとinstanceを同じファイル (`src/main.rs`) に書く例
// (`examples/static-org` と同じ形) なので、両方の宣言元が同じ綴りになる。
pub(super) fn src_main() -> 宣言元の対 {
    宣言元の対::new(
        宣言元ファイルの綴り::パッケージ相対で分かっている("src/main.rs".to_string()),
        宣言元ファイルの綴り::パッケージ相対で分かっている("src/main.rs".to_string()),
    )
}
