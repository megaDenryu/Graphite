// `where each <役割>: N | N..M | N..*` の右辺 (多重度の範囲)。下限は必須、
// 上限は無ければ無制限 (Option::None)。無制限を usize::MAX へ密輸せず、
// 下限 <= 上限 という不変条件を検証付きコンストラクタで保証する。
// 同crateの schema/syntax/each_specification.rs (EachSpec) と同じ設計。

use proc_macro2::Span;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct 多重度範囲 {
    下限: usize,
    上限: Option<usize>,
}

impl 多重度範囲 {
    pub(crate) fn new(下限: usize, 上限: Option<usize>, error_span: Span) -> syn::Result<Self> {
        if let Some(上限値) = 上限 {
            if 下限 > 上限値 {
                return Err(syn::Error::new(
                    error_span,
                    format!("多重度の下限 {下限} は上限 {上限値} 以下でなければなりません"),
                ));
            }
        }
        Ok(Self { 下限, 上限 })
    }

    pub(crate) fn 下限(self) -> usize {
        self.下限
    }

    pub(crate) fn 上限(self) -> Option<usize> {
        self.上限
    }
}
