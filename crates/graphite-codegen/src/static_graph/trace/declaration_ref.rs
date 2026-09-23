// 意味カードに書く「関係する宣言」への参照 (`宣言:` / `関係する schema
// 宣言:` / `関係する instance 宣言:` の各段落)。ファイルの綴りは
// `crate::schema::codegen::宣言元ファイルの綴り` (動的グラフと共有する
// 「分かっている/分かっていない」判別) から作る。行番号は持たない
// (design_principles.md 原則5.2: 行番号は再生成の対象を広げるため書かない)。
//
// `宣言への参照を試みる` は `宣言元ファイルの綴り` 自身のメソッドにする
// (動的グラフの `crate::schema::codegen::declaration_doc::宣言元ファイルの綴り::宣言への参照`
// と同じ形)。所有者 (`宣言元ファイルの綴り`) を外部引数として受け取る
// static な組み立て関数にしない (issue #41 是正7)。

use crate::schema::codegen::宣言元ファイルの綴り;

pub(crate) struct 宣言への参照 {
    ファイルの綴り: String,
    宣言の形: String,
}

impl 宣言元ファイルの綴り {
    // 宣言元が分かっているときだけ Some を返す。分かっていなければ、
    // その意味カードにはこの段落自体を出さない (`分かっていない` は
    // `static_graph_schema!`/instanceマクロがまだファイル追跡を持たない
    // 段階1の呼び出しが渡す)。
    pub(crate) fn 宣言への参照を試みる(&self, 宣言の形: impl Into<String>) -> Option<宣言への参照> {
        match self {
            Self::パッケージ相対で分かっている(綴り) => {
                Some(宣言への参照 { ファイルの綴り: 綴り.clone(), 宣言の形: 宣言の形.into() })
            }
            Self::分かっていない => None,
        }
    }
}

impl 宣言への参照 {
    pub(crate) fn 段落(&self) -> String {
        format!("宣言: `{}` の `{}`", self.ファイルの綴り, self.宣言の形)
    }

    pub(crate) fn 関係schema段落(&self) -> String {
        format!("関係する schema 宣言: `{}` の `{}`", self.ファイルの綴り, self.宣言の形)
    }

    pub(crate) fn 関係instance段落(&self) -> String {
        format!("関係する instance 宣言: `{}` の `{}`", self.ファイルの綴り, self.宣言の形)
    }
}

// 追跡情報が持つ「関係する宣言」。schema宣言・instance宣言のどちらかに
// 関係するか、どちらとも関係しないかの3択で、同時に両方を持つカードは
// 存在しない (`static_graph::naming` の全呼び出しを確認済み)。従来は
// `関係schema宣言: Option<..>`・`関係instance宣言: Option<..>` の2つの
// Option を並べて持っていたが、これは「両方Some」という起こり得ない組を
// 型の上で表現可能にしていた (issue #41 是正9)。
pub(crate) enum 関係宣言 {
    無し,
    Schema(宣言への参照),
    Instance(宣言への参照),
}
