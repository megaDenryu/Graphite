//! `graph!` リテラルで組み立てる具体的な依存グラフ。
//!
//! [`default_sheet`] が本編のミニスプレッドシート (見積書: 単価・数量・
//! 税率・割引率・配送料 → 小計 → 割引額・税額 → 調整額 → 合計)。
//! [`cyclic_demo_sheet`] は循環デモ専用の壊れたシート。

use crate::schema::{Cell, Feeds, Formula, Sheet, SheetViolation};

/// 本編のミニスプレッドシート。10セル・11本の `Feeds` エッジ。
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
/// ## `.clone()` が多い理由
///
/// `graph!` の脱糖はノード項を先に `let` 束縛へ展開してから、すべての
/// エッジ呼び出し (`from.clone()`/`to.clone()` を自動生成) を後ろへ回す
/// (`docs/schema_v4.md` §2)。そのため `subtotal` のように
/// **後続のセルの式でも、`Feeds` エッジの端点でも両方使う**識別子は、
/// 式の中で使う時点で明示的に `.clone()` しないと後続のエッジ生成コードが
/// 「すでにmoveされた変数」を借用できずコンパイルエラーになる。この
/// exampleでは早見表として「式の中で他セルのキーを使うときは常に
/// `.clone()` する」という一貫ルールにしている。
///
/// ## 辺キーの命名
///
/// v4 の `graph!` はノードキー・エッジキーが単一の平坦な名前空間なので、
/// 全ての `Feeds` エッジに端点から読める一意なキー (`f_<from>_<to>`) を
/// 付けている (連番は避ける)。
#[rustfmt::skip]
pub fn default_sheet() -> Result<Sheet, SheetViolation> {
    graphite::graph!(Sheet {
        unit_price    = Cell { formula: Formula::Input },
        quantity      = Cell { formula: Formula::Input },
        tax_rate      = Cell { formula: Formula::Input },
        discount_rate = Cell { formula: Formula::Input },
        shipping_fee  = Cell { formula: Formula::Input },

        subtotal        = Cell { formula: Formula::Mul(unit_price.clone(), quantity.clone()) },
        discount_amount = Cell { formula: Formula::Mul(subtotal.clone(), discount_rate.clone()) },
        tax             = Cell { formula: Formula::Mul(subtotal.clone(), tax_rate.clone()) },
        adjustment      = Cell { formula: Formula::Sub(tax.clone(), discount_amount.clone()) },
        grand_total     = Cell {
            formula: Formula::Sum(vec![subtotal.clone(), adjustment.clone(), shipping_fee.clone()])
        },

        f_unit_price_subtotal        = Feeds(unit_price -> subtotal),
        f_quantity_subtotal          = Feeds(quantity -> subtotal),
        f_subtotal_discount_amount   = Feeds(subtotal -> discount_amount),
        f_discount_rate_discount_amount = Feeds(discount_rate -> discount_amount),
        f_subtotal_tax               = Feeds(subtotal -> tax),
        f_tax_rate_tax               = Feeds(tax_rate -> tax),
        f_discount_amount_adjustment = Feeds(discount_amount -> adjustment),
        f_tax_adjustment             = Feeds(tax -> adjustment),
        f_subtotal_grand_total       = Feeds(subtotal -> grand_total),
        f_adjustment_grand_total     = Feeds(adjustment -> grand_total),
        f_shipping_fee_grand_total   = Feeds(shipping_fee -> grand_total),
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
        assert_eq!(Feeds::len(&sheet), 11);
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
