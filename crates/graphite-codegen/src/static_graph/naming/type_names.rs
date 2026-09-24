// `file` が読む3つの型名。design_principles.md 系譜どおり、
// `format_ident!` はここだけに書く (issue #41 §5.1「コード生成の側から
// format_ident! と span = ... を消す」)。

use quote::format_ident;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::semantic::{個体, 具体辺, 意味モデル, 辺種別};
use crate::static_graph::trace::{名前の由来, 意味項目, 追跡情報構築器};

use super::tracked_name::追跡付きの名前;

// `{個体名}Ref` (issue #41 分類A、由来 = InstanceNode)。instance自身の宣言
// だけを参照するため、schemaの宣言元は要らない。
pub(crate) fn 個体参照型名(
    意味モデル: &意味モデル,
    個体: &個体,
    宣言元: &宣言元の対,
) -> 追跡付きの名前 {
    let ident = format_ident!("{}Ref", 個体.名前(), span = 個体.名前().span());
    let 追跡 = 追跡情報構築器::new(
        名前の由来::InstanceNode { 個体名: 個体.名前().clone() },
        "Graphite 静的グラフの具体個体参照。",
    )
    .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()))
    .意味項目を足す(意味項目::new("個体", 個体.名前()))
    .意味項目を足す(意味項目::new("実体型", 個体.実体型()))
    .宣言を添える(宣言元.instance(), 個体.宣言の形())
    .完成する();
    追跡付きの名前::new(ident, 追跡)
}

// `{辺名}Ref` (issue #41 分類A、由来 = InstanceEdge)。主宣言はinstance自身の
// edge宣言、関係schema宣言はschemaのedge種別宣言であり、schemaとinstanceが
// 別ファイルの場合は宣言元が食い違う。
pub(crate) fn 辺参照型名(
    意味モデル: &意味モデル,
    具体辺: &具体辺,
    宣言元: &宣言元の対,
) -> 追跡付きの名前 {
    let ident = format_ident!("{}Ref", 具体辺.名前(), span = 具体辺.名前().span());
    let 追跡 = 追跡情報構築器::new(
        名前の由来::InstanceEdge { 辺名: 具体辺.名前().clone() },
        "Graphite 静的グラフの具体辺参照。",
    )
    .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()))
    .意味項目を足す(意味項目::new("具体辺", 具体辺.名前()))
    .意味項目を足す(意味項目::new("辺種別", 具体辺.種別().名前()))
    .宣言を添える(宣言元.instance(), 具体辺.宣言の形())
    .関係schema宣言を添える(宣言元.schema(), 具体辺.種別().宣言の形())
    .完成する();
    追跡付きの名前::new(ident, 追跡)
}

// `{種別}Edge` (issue #41 分類A、由来 = SchemaEdge)。schemaだけから決まる
// (instanceを見ない)。instanceはどの辺も`{種別}Edge`を構築せず、DSLの種別
// トークン (`所属(太郎 -> 開発部)`の`所属`等) がF12で着地する先としてだけ
// 使う型アンカーである (`docs/static_graph.md`「生成される名前の公開契約」)。
// 積み荷の有無で概要文を変える (積み荷を持たない種別に「積み荷を持つ」
// と書くのは実態と食い違うため)。
pub(crate) fn 辺値型名(辺種別: &辺種別, 宣言元: &宣言元ファイルの綴り) -> 追跡付きの名前 {
    let ident = format_ident!("{}Edge", 辺種別.名前(), span = 辺種別.名前().span());
    let 概要文 = match 辺種別.積み荷() {
        Some(_) => "Graphite 静的グラフの辺種別を表す型アンカー。端点の役割と、schemaが定めるこの種別の積み荷の形を示す (どのinstanceもこの型を構築しない)。",
        None => "Graphite 静的グラフの辺種別を表す型アンカー。端点の役割の形を示す (どのinstanceもこの型を構築しない)。",
    };
    let 追跡 = 追跡情報構築器::new(名前の由来::SchemaEdge { 種別名: 辺種別.名前().clone() }, 概要文)
        .意味項目を足す(意味項目::new("辺種別", 辺種別.名前()))
        .宣言を添える(宣言元, 辺種別.宣言の形())
        .完成する();
    追跡付きの名前::new(ident, 追跡)
}
