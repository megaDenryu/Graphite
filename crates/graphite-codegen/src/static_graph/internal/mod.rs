//! `__static_graph_impl!` (issue #24 段階2、`#[doc(hidden)]` の内部proc
//! macro、`graphite_macros::__static_graph_impl` から呼ばれる) が持つ、
//! schemaとinstanceの相互検証 (validate module) と意味モデルの組み立て口。
//! `tracked::instance::parse_tracked_static_instance` が、schemaの構文木
//! (macro_rules!転送で生トークンのままspanを保って届いたものを
//! `static_graph::mod` が構造化した後の型) と、解析済みのinstance構文木を
//! この型へ struct literal で渡す (issue #41 段階3。「schema <名前> { .. }
//! instance { .. }」という結合トークン列を直接 `Parse` する経路は、instance
//! 側に `generated = "..."` の切り出しが挟まるため `static_graph::mod` へ
//! 移した)。
//!
//! 検証前に意味モデルを組み立てることを型状態で防ぐ。
//! `静的グラフ内部入力::検証する` は `self` を消費して `検証済み静的グラフ内部入力`
//! を返し、意味モデルを組み立てられるのは検証済みの型だけである
//! (`意味モデル::組み立てる` は `&検証済み静的グラフ内部入力` しか受け取らない)。

mod validate;

use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;
use crate::static_graph::semantic::意味モデル;

pub struct 静的グラフ内部入力 {
    pub schema: 静的グラフ型入力,
    pub instance: 静的グラフ入力,
}

impl 静的グラフ内部入力 {
    // schema単体の構造検証・instance単体の構造検証・両者を突き合わせる相互
    // 検証の3段で行う。相互検証は前の2段が通っている前提 (端点が宣言済み等)
    // に依存するため、この順序を変えない。`self` を消費し、通れば
    // `検証済み静的グラフ内部入力` を返す (型状態)。
    pub fn 検証する(self) -> syn::Result<検証済み静的グラフ内部入力> {
        self.schema.検証する()?;
        self.instance.検証する()?;
        validate::相互検証する(&self.schema, &self.instance)?;
        Ok(検証済み静的グラフ内部入力 { schema: self.schema, instance: self.instance })
    }
}

// 相互検証を通った静的グラフ内部入力。意味モデルを組み立てられるのは
// この型だけであり、`静的グラフ内部入力` (未検証) からは組み立てられない。

pub struct 検証済み静的グラフ内部入力 {
    schema: 静的グラフ型入力,
    instance: 静的グラフ入力,
}

impl 検証済み静的グラフ内部入力 {
    pub(crate) fn schema(&self) -> &静的グラフ型入力 {
        &self.schema
    }

    pub(crate) fn instance(&self) -> &静的グラフ入力 {
        &self.instance
    }

    pub fn 意味モデルを組み立てる(&self) -> 意味モデル {
        意味モデル::組み立てる(self)
    }
}

// 試験専用の組み立て口。schema・instanceの構文木を直接持ち込む試験コードが
// 相互検証を経由せずに意味モデルを組み立てることを防ぎ、検証してから
// 組み立てる手順を1箇所へ集約する。
#[cfg(test)]
pub(crate) fn 検証してから意味モデルを組み立てる(
    schema: 静的グラフ型入力,
    instance: 静的グラフ入力,
) -> 意味モデル {
    静的グラフ内部入力 { schema, instance }
        .検証する()
        .expect("試験入力は検証を通ること")
        .意味モデルを組み立てる()
}
