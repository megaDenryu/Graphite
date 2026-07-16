//! reactive-cells — 「リアクティブプログラミングのスパゲッティ」を
//! Graphite (`graph_schema!`/`graph!`) で倒す実証example。
//!
//! 詳しい経緯・設計判断は `README.md` を参照。このファイルは
//! `README.md` の構成 (敵の紹介 → グラフによる再定式化 → 対応表) を
//! そのまま実行可能な物語として再演する。

use reactive_cells::antipattern::{build_diamond_demo, build_infinite_loop_demo};
use reactive_cells::engine::Engine;
use reactive_cells::fixtures::{cyclic_demo_sheet, default_sheet};
use reactive_cells::report;
use reactive_cells::schema::CellId;

fn id(s: &str) -> CellId {
    CellId(s.to_string())
}

fn main() {
    report::print_section("敵1: observer パターンのグリッチ (ダイヤモンド依存)");
    println!(
        "a→b, a→c, b→d, c→d という依存を「値が変わったら購読者へ通知する」\n\
         だけの素朴なセルで組む (b=a*2, c=a+100, d=b+c)。a に 5 を設定する。"
    );
    let diamond = build_diamond_demo(false);
    diamond.trigger(5.0);
    report::print_diamond_demo("結果", &diamond);
    println!(
        "  -> dは2回再計算された。1回目は「bは新しい値(10)・cはまだ古い値(0)」という\n\
         矛盾した中間状態 (d=10) を観測している。最終値(115)は正しいが、\n\
         その値を誰かが1回目のタイミングで読んでいたら間違った値を見ることになる。"
    );

    report::print_section("敵1つづき: 購読登録順を入れ替えると結果の過程が変わる");
    let swapped = build_diamond_demo(true);
    swapped.trigger(5.0);
    report::print_diamond_demo("登録順を入れ替えた結果", &swapped);
    println!("  -> 依存関係は同じなのに、コードの書き方(登録順)次第でグリッチの内容が変わる。");

    report::print_section("敵2: 循環購読は誰も気づかず回り続ける");
    let cap = 200;
    let actual = build_infinite_loop_demo(cap);
    report::print_infinite_loop_demo(cap, actual);

    report::print_section("グラフによる再定式化: ミニスプレッドシートをgraph!で宣言する");
    let sheet = default_sheet().expect("正常なシートは構築に成功するはず");
    println!(
        "セル数 = {}, feedsエッジ数 = {} (依存関係は実行前に一枚で見える構造データ)",
        sheet.cell_ids().count(),
        sheet.feeds().len()
    );
    let mut engine = Engine::new(sheet).expect("循環が無いので構築に成功するはず");
    println!("トポロジカル順序 (これがそのままglitch-freeな再計算順になる):");
    let order: Vec<String> = engine.topological_order().iter().map(|id| id.0.clone()).collect();
    println!("  {}", order.join(" -> "));

    report::print_section("値変更 -> 伝播の物語");
    report::print_set_input("(1) 単価を設定", &id("unit_price"), 1000.0, &engine.set_input(&id("unit_price"), 1000.0));
    report::print_set_input("(2) 数量を設定", &id("quantity"), 3.0, &engine.set_input(&id("quantity"), 3.0));
    report::print_set_input("(3) 税率を設定", &id("tax_rate"), 0.1, &engine.set_input(&id("tax_rate"), 0.1));
    report::print_set_input("(4) 割引率を設定", &id("discount_rate"), 0.05, &engine.set_input(&id("discount_rate"), 0.05));
    report::print_set_input("(5) 配送料を設定", &id("shipping_fee"), 500.0, &engine.set_input(&id("shipping_fee"), 500.0));

    println!("\n現在の値:");
    report::print_engine_snapshot(
        &engine,
        &[
            "unit_price",
            "quantity",
            "tax_rate",
            "discount_rate",
            "shipping_fee",
            "subtotal",
            "discount_amount",
            "tax",
            "adjustment",
            "grand_total",
        ],
    );

    report::print_section("影響範囲だけを再計算する (reachable_fromによる絞り込み)");
    let steps = engine.set_input(&id("tax_rate"), 0.2);
    report::print_set_input("税率を変更", &id("tax_rate"), 0.2, &steps);
    println!(
        "  -> subtotal/discount_amount/他の入力セルはtax_rateから到達不能なので再計算されない\n\
         (このexampleでは実際に{}件だけが再計算された。ダイヤモンド依存を通っても\n\
         各セルはちょうど1回だけ再計算されglitchは起きない — これがグラフによる\n\
         再定式化の核心)。",
        steps.len()
    );

    report::print_section("敵2つづき: 循環の拒否 (構築前にデータ検証で拒否する)");
    let cyclic_sheet = cyclic_demo_sheet().expect("feedsは0..*なので構造としては構築できる");
    println!("cyclic_demo_sheet: 構造としてはSheet::createに成功する (a->b->c->aの循環)。");
    match Engine::new(cyclic_sheet) {
        Ok(_) => println!("  想定外: 循環があるのにEngine::newが成功した"),
        Err(err) => report::print_cycle_error(&err),
    }
    println!(
        "  -> observer パターンなら実行して初めて (無限にnotifyが回って) 気づく循環が、\n\
         Graphite化した依存グラフでは Engine::new の構築時点で具体的な循環パスつきで拒否される。"
    );
}
