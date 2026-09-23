//! 静的グラフの意味モデル (issue #41 段階1)。schemaとinstanceの構文木
//! (`schema::input`・`literal::input`) から、名前解決を1回で済ませた
//! 「個体」「辺種別」「具体辺」を組み立てる。`file` はここで
//! 解決済みの参照だけを読み、種別名・端点名の再引き当て
//! (`.expect(..)` を各所で繰り返すこと) を行わない。
//!
//! 組み立ては schemaとinstanceの相互検証 (`internal::validate::相互検証する`)
//! が通った後にだけ行う (`個体` を端点名で探す等の解決は検証済みである前提
//! で `.expect(..)` を使う。検証前に呼ぶと panic する)。

mod builder;
mod concrete_edge;
mod edge_kind;
mod individual;
#[cfg(test)]
mod tests;

pub(crate) use concrete_edge::{具体辺, 具体辺形状};
pub(crate) use edge_kind::辺種別;
pub(crate) use individual::個体;

use proc_macro2::Ident;

use crate::static_graph::schema::input::静的グラフ型入力;

// schema単体 (instanceを持たない) からschemaの辺種別列だけを組み立てる。
// `TrackedStaticSchema` のschemaファイル生成 (`static_graph::file::schema_file`)
// と、`意味モデルを組み立てる` (instanceも合わせて持つ完全な意味モデル) が
// 共有する (issue #41 段階2)。
pub(crate) fn 辺種別列をschemaから組み立てる(schema: &静的グラフ型入力) -> Vec<辺種別> {
    schema.辺宣言達.iter().map(辺種別::schemaから作る).collect()
}

pub(crate) struct 意味モデル {
    グラフ名: Ident,
    schema名: Ident,
    個体列: Vec<個体>,
    辺種別列: Vec<辺種別>,
    具体辺列: Vec<具体辺>,
}

impl 意味モデル {
    // 唯一の組み立て口。schemaとinstanceを直接受け取らず、相互検証を通った
    // `検証済み静的グラフ内部入力` だけを受け取る。この型は
    // `静的グラフ内部入力::検証する` (`internal::validate::相互検証する` を含む)
    // を経由しないと得られないため、未検証の構文木から意味モデルを組み立てる
    // 経路が構造的に無くなる (端点解決の `.expect(..)` は検証済みである前提
    // に依存するため、これが要る)。中身の解決は builder.rs が持つ。
    pub(crate) fn 組み立てる(
        検証済み: &crate::static_graph::internal::検証済み静的グラフ内部入力,
    ) -> Self {
        builder::意味モデルを組み立てる(検証済み.schema(), 検証済み.instance())
    }

    pub(crate) fn グラフ名(&self) -> &Ident {
        &self.グラフ名
    }

    // instanceが由来するschemaのmodule名 (`組織`)。instanceファイルの本文
    // やDSLトークンの型参照が、schema module越しの修飾パス (`組織::所属Edge`)
    // を組み立てるために使う。
    pub(crate) fn schema名(&self) -> &Ident {
        &self.schema名
    }

    pub(crate) fn 個体列(&self) -> &[個体] {
        &self.個体列
    }

    #[allow(dead_code)]
    pub(crate) fn 辺種別列(&self) -> &[辺種別] {
        &self.辺種別列
    }

    pub(crate) fn 具体辺列(&self) -> &[具体辺] {
        &self.具体辺列
    }

    // instance宣言の形 (`graph 開発チーム`)。`naming::card_names` の
    // `個体実体所有者構築メソッド名` が「関係する instance 宣言」段落に使う。
    pub(crate) fn グラフ宣言の形(&self) -> String {
        format!("graph {}", self.グラフ名)
    }

    // 個体が端点になっている具体辺を、instance宣言順で返す。
    pub(crate) fn この個体が端点になっている具体辺列<'a>(
        &'a self,
        個体名: &'a Ident,
    ) -> impl Iterator<Item = &'a 具体辺> {
        self.具体辺列.iter().filter(move |辺| 辺.端点に含むか(個体名))
    }
}
