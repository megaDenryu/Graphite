//! 公開面と判定したパッケージの一覧の整形 (issue #32 検収の是正)。
//!
//! この一覧はそれ自体では違反を検出しない。読み手は、この一覧の件数を内部
//! 領域の件数と合わせて、走査対象のパッケージの総数を突き合わせられる。

use std::fmt::Write;

use crate::inspected_area::InspectedArea;

// 公開面と判定したパッケージ。この型は、表示 (`render`) の直前まで
// `InspectedArea` のまま保持し、綴りへ剥がす場所を表示の境界だけに限る。この型
// は、呼び出し元から受け取った内部領域の件数を使い、総数の突き合わせ行も
// この型の中で組み立てる (内部領域の報告と総数の行が離れて食い違わないように)。
pub(super) struct PublicPackageReport<'a> {
    areas: &'a [InspectedArea],
    internal_count: usize,
}

impl<'a> PublicPackageReport<'a> {
    pub(super) fn new(areas: &'a [InspectedArea], internal_count: usize) -> Self {
        Self {
            areas,
            internal_count,
        }
    }

    pub(super) fn render(&self) -> String {
        let mut text = format!("公開面と判定したパッケージ ({}件):\n", self.areas.len());
        for area in self.areas {
            let _ = writeln!(text, "  {}", area.spelling());
        }
        let _ = writeln!(
            text,
            "領域の総数: 内部領域 {}件 + 公開面 {}件 = {}件",
            self.internal_count,
            self.areas.len(),
            self.internal_count + self.areas.len()
        );
        text
    }
}

#[cfg(test)]
mod tests {
    use super::PublicPackageReport;
    use crate::inspected_area::InspectedArea;

    #[test]
    fn 件数と綴りと総数を出力する() {
        let areas = [
            InspectedArea::at("crates/graphite"),
            InspectedArea::at("crates/other"),
        ];
        let text = PublicPackageReport::new(&areas, 14).render();
        assert!(text.contains("2件"));
        assert!(text.contains("crates/graphite"));
        assert!(text.contains("crates/other"));
        assert!(text.contains("領域の総数: 内部領域 14件 + 公開面 2件 = 16件"));
    }
}
