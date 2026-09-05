//! `where each <参照名>` がどちらの端点を指すか判定できることを検査する。

use crate::schema::semantic::each制約が指す端点の側を判定する;
use crate::schema::syntax::EdgeDecl;

// `where each <参照名>: ..` の `<参照名>` がどちらの端点を指すか判定できるかを
// 検査する (`schema::semantic::each制約が指す端点の側を判定する` 参照)。
//
// 判定そのものは意味層が持つ。検査側が意味層の判定を呼ぶのは、検査が
// 「意味として成立するか」を問う工程であるため依存の向きとして正しい。
pub fn validate_each_reference(edges: &[EdgeDecl]) -> syn::Result<()> {
    for edge in edges {
        for constraint in &edge.constraints.each {
            each制約が指す端点の側を判定する(edge, &constraint.role)?;
        }
    }
    Ok(())
}
