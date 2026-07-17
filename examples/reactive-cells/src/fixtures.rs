//! `graph!` リテラルで組み立てる具体的な依存グラフ。
//!
//! [`default_sheet`] が本編のミニスプレッドシート (見積書: 単価・数量・
//! 税率・割引率・配送料 → 小計 → 割引額・税額 → 調整額 → 合計)。
//! [`cyclic_demo_sheet`] は循環デモ専用の壊れたシート。

use crate::schema::{Cell, Feeds, Formula, Lhs, Rhs, Sheet, SheetViolation};

/// 本編のミニスプレッドシート。10セル・11本の依存エッジ (`Feeds` 9本 +
/// `Lhs`/`Rhs` 1本ずつ)。
///
/// ## セル構成
///
/// | セル | 種別 | 式 |
/// |---|---|---|
/// | `unit_price`/`quantity`/`tax_rate`/`discount_rate`/`shipping_fee` | 入力 | — |
/// | `subtotal` | 計算 | `unit_price * quantity` |
/// | `discount_amount` | 計算 | `subtotal * discount_rate` |
/// | `tax` | 計算 | `subtotal * tax_rate` |
/// | `adjustment` | 計算 | `tax - discount_amount` |
/// | `grand_total` | 計算 | `subtotal + adjustment + shipping_fee` |
///
/// ## ダイヤモンド依存
///
/// `subtotal` (a) → `discount_amount` (b) → `adjustment` (d)、
/// `subtotal` (a) → `tax` (c) → `adjustment` (d) という
/// `a→b, a→c, b→d, c→d` の形のダイヤモンドがそのまま含まれている
/// (README「グリッチの実演」節、`Engine::set_input` のテスト参照)。
/// `adjustment` は `discount_amount`・`tax` という「`subtotal` を
/// 経由した2つの経路」の両方から到達可能なセルであり、observer パターン
/// で書けば2回再計算され1回目は矛盾した中間状態を観測する典型例になる
/// (`crate::antipattern::build_diamond_demo` が実際にそれを再現する)。
///
/// ## `adjustment` の被減数/減数 — `Lhs`/`Rhs` エッジ (モデリングガイド§5)
///
/// `adjustment = tax - discount_amount` なので、`tax` が被減数
/// (`Lhs(tax -> adjustment)`)、`discount_amount` が減数
/// (`Rhs(discount_amount -> adjustment)`) になる。`Formula::Sub` 自体は
/// どのセルが被減数/減数かを保持せず、この2本のエッジだけが情報源
/// (`src/schema.rs` のモジュール doc、`docs/modeling_guide.md` §5 参照)。
///
/// ## `.clone()` が要らない理由
///
/// `graph!` の脱糖はエッジ呼び出し (`Feeds(from -> to)` 等) の端点を
/// 自動的に `.clone()` する (`docs/schema_v4.md` §2)。以前の設計では
/// `Formula::Mul(subtotal.clone(), ..)` のように**値の式の中**でも
/// セルキーを使っていたため、その使用箇所だけは手動 `.clone()` が
/// 必要だった。`Formula` が `CellId` を保持しなくなった (演算対象は
/// エッジから読む) ことで、セルキーが登場するのは常にエッジ構築の
/// 中だけになり、手動 `.clone()` は一切不要になった。
///
/// ## 辺キーの命名
///
/// v4 の `graph!` はノードキー・エッジキーが単一の平坦な名前空間なので、
/// 全ての依存エッジに端点から読める一意なキー (`f_<from>_<to>` /
/// `l_<from>_<to>` / `r_<from>_<to>`。接頭辞はエッジ種別) を付けている
/// (連番は避ける)。
#[rustfmt::skip]
pub fn default_sheet() -> Result<Sheet, SheetViolation> {
    graphite::graph!(Sheet {
        unit_price    = Cell { formula: Formula::Input },
        quantity      = Cell { formula: Formula::Input },
        tax_rate      = Cell { formula: Formula::Input },
        discount_rate = Cell { formula: Formula::Input },
        shipping_fee  = Cell { formula: Formula::Input },

        subtotal        = Cell { formula: Formula::Mul },
        discount_amount = Cell { formula: Formula::Mul },
        tax             = Cell { formula: Formula::Mul },
        adjustment      = Cell { formula: Formula::Sub },
        grand_total     = Cell { formula: Formula::Sum },

        f_unit_price_subtotal           = Feeds(unit_price -> subtotal),
        f_quantity_subtotal             = Feeds(quantity -> subtotal),
        f_subtotal_discount_amount      = Feeds(subtotal -> discount_amount),
        f_discount_rate_discount_amount = Feeds(discount_rate -> discount_amount),
        f_subtotal_tax                  = Feeds(subtotal -> tax),
        f_tax_rate_tax                  = Feeds(tax_rate -> tax),
        l_tax_adjustment                = Lhs(tax -> adjustment),
        r_discount_amount_adjustment    = Rhs(discount_amount -> adjustment),
        f_subtotal_grand_total          = Feeds(subtotal -> grand_total),
        f_adjustment_grand_total        = Feeds(adjustment -> grand_total),
        f_shipping_fee_grand_total      = Feeds(shipping_fee -> grand_total),
    })
}

/// 循環デモ専用の壊れたシート。3セル `a -> b -> c -> a` の `Feeds`
/// エッジによる循環購読を表す (README「循環の拒否」節)。
///
/// `Feeds` は `where unique pair` のみで循環そのものを禁止する制約は
/// 無いので `graph!`/`Sheet::create` 自体は**構造としては正常に構築
/// できてしまう** (端点は全て宣言済みで、同一対の重複も無い)。循環の
/// 検出は `graphite::Graph::topological_sort`
/// (= [`crate::engine::Engine::new`] が内部で呼ぶ) まで遅延される —
/// これは意図的な設計で、「schema/graph! は構造の整合性だけを見る、
/// 非循環性はドメイン (このexampleでは再計算エンジン) が要求する制約」
/// という責務分離を表す (`README.md`「グラフによる再定式化」節)。
/// このデモは3セルとも `Formula::Input` のままなので、`Lhs`/`Rhs` は
/// 登場しない (被減数/減数の区別が要らない、循環検出だけが目的)。
#[rustfmt::skip]
pub fn cyclic_demo_sheet() -> Result<Sheet, SheetViolation> {
    graphite::graph!(Sheet {
        a = Cell { formula: Formula::Input },
        b = Cell { formula: Formula::Input },
        c = Cell { formula: Formula::Input },

        f_a_b = Feeds(a -> b),
        f_b_c = Feeds(b -> c),
        f_c_a = Feeds(c -> a),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::SheetNode;

    #[test]
    fn default_sheetは10セル11エッジで構築できる() {
        let sheet = default_sheet().expect("正常なシートは構築に成功するはず");
        assert_eq!(Cell::ids(&sheet).count(), 10);
        assert_eq!(Feeds::len(&sheet), 9, "可換な演算(Mul/Sum)の被演算子9本");
        assert_eq!(Lhs::len(&sheet), 1, "adjustmentの被減数(tax)1本");
        assert_eq!(Rhs::len(&sheet), 1, "adjustmentの減数(discount_amount)1本");
    }

    #[test]
    fn cyclic_demo_sheetは構造としては構築に成功する() {
        // 循環そのものはSheet::create/graph!の検証対象外 (端点存在と
        // unique pairだけを見る)。循環検出はEngine::new側の責務。
        let sheet = cyclic_demo_sheet().expect("Feedsはunique pairのみなので循環でも構造検証は通るはず");
        assert_eq!(Cell::ids(&sheet).count(), 3);
        assert_eq!(Feeds::len(&sheet), 3);
    }
}
