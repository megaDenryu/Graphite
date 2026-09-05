//! 回帰テスト: `KeyedTable` (`crates/graphite/src/keyed_table.rs`) は挿入順を
//! 保持する仕様であり (`docs/schema_v4.md` §3「順序保証」)、これを土台にした
//! 制約なし辺種別の `{Kind}::of`/`iter` は格納順 (構築時の追加順) を保持する
//! はず、という約束を検証する。
//!
//! 発覚の経緯: dialogue-engine 移行中、同一始点から平行辺が複数ある種別
//! (Choice 相当) の `of()`/`iter()` の順序がプロセスごとに変わり、テストが
//! flaky になった。原因は (1) `KeyedTable` の内部が素の `HashMap` で反復順序
//! が未規定だったこと、(2) freeze 時の `from_index` 構築が builder の挿入順
//! ではなく、出来上がった `KeyedTable` (HashMap) の `iter` 順で行われていた
//! こと、の2点。`KeyedTable` を `Vec<(K, V)>` 本体 + `HashMap<K, usize>` 索引
//! の構造に変えることで両方解消される (from_index の構築源である
//! `#accessor.iter()` が挿入順を返すようになるため)。
//!
//! builder 直接経由・`graph!` リテラル経由の両方で確認する。
//!
//! `tests/` は統合テストの根のモジュールディレクトリであり、裸の `mod` はこの
//! ディレクトリ直下を探して cargo が別のテスト対象として組み立てるため、
//! このファイルはモジュールの綴りを `#[path]` で明示する。

#[cfg(test)]
#[path = "keyed_table_insertion_order/tests.rs"]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub struct Speaker {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub text: String,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Dialogue {
    include!("generated/keyed_table_insertion_order_dialogue.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/keyed_table_insertion_order_dialogue.rs";
    schema Dialogue {
        node Speaker;
        node Line;

        // 制約なし (each も unique pair も無し): 同一始点からの平行辺が自由。
        edge Choice = (speaker: Speaker) -> (line: Line);
    }
}

use Dialogue::{Choice, ChoiceId, LineId, SpeakerId};

/// 記述順どおりの `line{i}` テキスト列を作る補助関数。
fn expected_texts(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("line{i}")).collect()
}
