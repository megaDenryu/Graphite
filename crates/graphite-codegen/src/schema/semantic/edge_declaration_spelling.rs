//! 辺定義から、辺宣言の正規形の綴りを組み立てる。
//!
//! 生成物の doc から宣言元を指すための表示であり、DSL 原文の逐語ではない。
//! 正規形とは、末尾の `;` を書かず、明示ID型のパスを生成 module 内から解決できる
//! 形にし、`where` 節へ `each` を記述順に並べた後ろから `unique pair` を回した
//! 綴りのことである (`unique pair` を後ろへ回すのは、構文モデルが `where` 節の
//! 中での記述順を保たないためである)。
//!
//! 綴りの組み立ては辺定義そのものの読み取りとは別の関心なので、
//! [`super::edge_definition`] から切り出してこのファイルへ置く。辺定義が公開して
//! いる読み取りメソッドだけで組み立てられるため、フィールドへは触れない。

use proc_macro2::Ident;

use super::cardinality::役割の多重度制約;
use super::edge_definition::{辺の向き, 辺定義};
use super::type_path_spelling::型パスの綴りを組み立てる;

impl 辺定義 {
    /// この辺宣言の形
    /// (`edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1`)。
    ///
    /// 端点は添字ハンドルで持つため、ノード定義の列を所有する
    /// [`super::スキーマ定義`] だけが型名を解決して呼べる。
    pub(super) fn 宣言の形(&self, 始点の型名: &Ident, 終点の型名: &Ident) -> String {
        let 種別 = match self.公開id型().明示された型パスの綴り() {
            Some(綴り) => format!("{}(id: {綴り})", self.辺種別名()),
            None => self.辺種別名().to_string(),
        };
        let 柄 = self.柄の綴り();
        let 両端 = match self.向き() {
            辺の向き::有向 { 始点, 終点 } => format!(
                "({}: {始点の型名}) {柄} ({}: {終点の型名})",
                始点.役割名(),
                終点.役割名()
            ),
            辺の向き::無向 { .. } => format!("{始点の型名} {柄} {終点の型名}"),
        };
        format!("edge {種別} = {両端}{}", self.where節の綴り())
    }

    /// 宣言の形へ書く柄 (`->` / `-[役割名: 型]->` / `--` / `-[役割名: 型]-`)。
    fn 柄の綴り(&self) -> String {
        // 積み荷を書くと柄の最初の `-` が角括弧の手前へ移るため、角括弧より
        // 後ろは有向で `->`、無向で `-` になる (`docs/edge_endpoints_v4_1.md` §2)。
        let 角括弧より後ろ = if self.有向か() { "->" } else { "-" };
        match self.積み荷() {
            Some(積み荷) => format!(
                "-[{}: {}]{角括弧より後ろ}",
                積み荷.役割名(),
                型パスの綴りを組み立てる(積み荷.型パス())
            ),
            None if self.有向か() => "->".to_string(),
            None => "--".to_string(),
        }
    }

    /// 宣言の形の末尾へ付ける `where` 節。制約が1つも無ければ空文字になる。
    fn where節の綴り(&self) -> String {
        let mut 制約の列: Vec<String> = self
            .記述順の役割の多重度制約()
            .iter()
            .map(役割の多重度制約::where節での綴り)
            .collect();
        if self.端点対の重複可否().対ごとに1本だけか() {
            制約の列.push("unique pair".to_string());
        }
        if 制約の列.is_empty() {
            return String::new();
        }
        format!(" where {}", 制約の列.join(", "))
    }
}
