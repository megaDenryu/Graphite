// static_schema!/組織! (static_schema! が生成するmacro_rules!) の正式な
// 利用例 (issue #24 段階2)。組織ドメイン (社員・部署・任命記録・経緯記録) で、
// schema宣言 (static_schema!) とinstance宣言 (schema名そのものを名前にした
// macro_rules!、ここでは 組織!) の2段構えが、役割アクセサ・積み荷アクセサ・
// 無向辺の端点アクセサ・多重度検査・対一意検査とともに機能することを示す。
// 手書き検証の記録は `examples/graphitets-by-hand` (凍結済み) を参照。
//
// static_schema! が生成する macro_rules! は通常のmacro_rules!と同じテキスト
// 順の制約を持つ: `static_schema! { schema 組織 { .. } }` より後ろでしか
// `組織! { .. }` を呼べない (詳細は `docs/static_graph.md` を参照)。
//
// 実行場所: このディレクトリ (examples/static-org) で
//   cargo run
//   cargo test

mod domain;
#[cfg(test)]
mod tests;

use domain::{社員, 社員を作る, 経緯記録, 部署, 任命記録, 名前持ち};
use graphite::static_schema;

// ---------------- schema宣言 ----------------

static_schema! {
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
        edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1;
        edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair;
        edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員);
    }
}

// ---------------- instance宣言 ----------------
//
// 開発部 だけを値なし宣言 (`node 開発部: 部署;`) にして、実行時供給
// (`Nodes::new` への位置引数) を示す。main() とテストの両方から呼ぶため、
// 構築を ノードを組み立てる() へ切り出す。

#[rustfmt::skip]
組織! {
    graph 開発チーム;
    node 太郎 = 社員 { 名前: "太郎".into() };
    node 次郎 = 社員 { 名前: "次郎".into() };
    node 一郎: 社員 = 社員を作る("一郎");
    node 開発部: 部署;
    edge 太郎の所属 = 所属(太郎 -> 開発部);
    edge 次郎の所属 = 所属(次郎 -> 開発部);
    edge 一郎の所属 = 所属(一郎 -> 開発部);
    edge 太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 2020 }]-> 次郎);
    edge 太郎と次郎 = 友人(太郎 -- 次郎);
    edge 太郎と一郎の同僚 = 同僚(太郎 -[経緯記録 { 経緯: "同期入社".into() }]- 一郎);
}

// `{個体名}Ref` は具象ローカルstructなので、利用側はマクロの外から自由に
// メソッドを生やせる (docs/static_graph.md 「生成される名前の公開契約」
// 参照)。あだ名() は 太郎Ref のチェーンの末尾へ通常のメソッドと同じ形で
// 継ぎ足せることを示す。
impl<'a> 太郎Ref<'a> {
    pub(crate) fn あだ名(&self) -> String {
        format!("{}くん", self.entity().名前)
    }
}

/// 値なし宣言 (`node 開発部: 部署;`) の実体は実行時にここで供給する。main()
/// とテストの両方から呼ぶ。
pub(crate) fn ノードを組み立てる() -> Nodes {
    Nodes::new(部署 { 名前: "開発部".into() })
}

fn main() {
    let nodes = ノードを組み立てる();
    let edges = Edges::new(&nodes);
    let g = 開発チーム::new(&nodes, &edges);

    let 太郎の参照 = g.node_refs.太郎;
    println!("太郎の上司: {}", 太郎の参照.太郎の上司().superior().entity().名前());
    println!("太郎の上司の任命日: {}", 太郎の参照.太郎の上司().任命().任命日);
    println!("太郎の上司の所属先: {}", 太郎の参照.太郎の上司().superior().次郎の所属().team().entity().名前());
    println!("太郎の所属先: {}", 太郎の参照.太郎の所属().team().entity().名前());
    println!("太郎の所属元 (辺参照から): {}", g.edge_refs.太郎の所属.member().entity().名前());
    println!("太郎の友人 (無向辺の端点): {} と {}", g.edge_refs.太郎と次郎.甲().entity().名前(), g.edge_refs.太郎と次郎.乙().entity().名前());
    println!(
        "太郎と一郎の同僚関係 (積み荷付き無向辺): {} と {} (経緯: {})",
        g.edge_refs.太郎と一郎の同僚.甲().entity().名前(),
        g.edge_refs.太郎と一郎の同僚.乙().entity().名前(),
        g.edge_refs.太郎と一郎の同僚.経緯().経緯
    );
    println!("次郎の所属先: {}", g.node_refs.次郎.次郎の所属().team().entity().名前());
    println!("上司関係の部下 (辺参照から): {}", g.edge_refs.太郎の上司.subordinate().entity().名前());
    println!("太郎のあだ名 (後付けメソッド): {}", 太郎の参照.あだ名());
    println!("経理チームの花子の所属先 (同一schemaの2つ目のグラフ): {}", 経理チームの花子の所属先を求める());

    // 存在しない辿りがコンパイルエラーになることの実測 (issue #24 段階2):
    // 太郎 は 次郎の所属 辺の端点ではないため、太郎Ref に次郎の所属() メソッド
    // は生成されない。次の1行を実際にアンコメントしてビルドすると、次の
    // エラーになることを確認済み:
    //   error[E0599]: no method named `次郎の所属` found for struct
    //   `太郎Ref<'a>` in the current scope
    //   help: there is a method `太郎の所属` with a similar name
    // 確認後はコメントアウトへ戻してある。
    // 太郎の参照.次郎の所属();

    // 多重度制約違反がコンパイルエラーになることの実測 (issue #24 段階2):
    // 上のinstance宣言から `edge 一郎の所属 = 所属(一郎 -> 開発部);` の1行を
    // 一時的に削除してビルドすると、`each member: 1` 制約 (下限1) に対し
    // 一郎の所属件数が0になり、次のエラーで展開時にコンパイルエラーになる
    // ことを確認済み (const評価時のpanicではなく通常のcompile_error!):
    //   error: 多重度制約違反: `一郎` の `所属` (役割 `member`) の本数が0件で、
    //   範囲 1..1 の外です
    // 確認後は1行を元に戻してある。

    // 対一意制約違反がコンパイルエラーになることの実測 (issue #24 段階2):
    // 上のinstance宣言へ `edge 次郎と太郎 = 友人(次郎 -- 太郎);` (端点の順序を
    // 変えただけの重複) を一時的に追加してビルドすると、無向辺は端点の順序に
    // 依らず正規化して比較するため、次のエラーで展開時にコンパイルエラーに
    // なることを確認済み:
    //   error: 対一意制約違反: 種別 `友人` の辺 `次郎と太郎` は端点の組
    //   (太郎, 次郎) が既出の辺と重複しています
    // 確認後は追加した行を削除してある。
}

/// 同一schemaから `組織!` を2回目に呼んでも、辺値struct群 (`{種別}Edge`) が
/// 重複定義エラーにならないことを示す。辺値struct群は `static_schema!` 側
/// (schema単位、1回だけ展開) へ移してあるため、`組織!` 呼び出しの回数に
/// 依らない。一方 `Nodes`/`Edges`/`NodeRefs`/`EdgeRefs` は `組織!` 呼び出し
/// ごと (instance単位) に固定名で生成されるため、同一スコープで2回展開する
/// と重複定義になる。関数の本体は独立したアイテムスコープを持つため、この
/// 関数の中へ2つ目の呼び出しを閉じ込めて衝突を避ける。
fn 経理チームの花子の所属先を求める() -> String {
    #[rustfmt::skip]
    組織! {
        graph 経理チーム;
        node 花子 = 社員 { 名前: "花子".into() };
        node 総務部 = 部署 { 名前: "総務部".into() };
        edge 花子の所属 = 所属(花子 -> 総務部);
    }

    let nodes = Nodes::new();
    let edges = Edges::new(&nodes);
    let g = 経理チーム::new(&nodes, &edges);
    g.node_refs.花子.花子の所属().team().entity().名前().to_string()
}
