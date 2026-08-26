//! `graph!` の1回の構築を識別する印の採番を、クレート唯一のカウンタとして所有する。

use std::sync::atomic::{AtomicU64, Ordering};

/// `graph!` の1回の構築を識別する印を採番するための、クレート全体で
/// 唯一のカウンタ。値そのものに意味はなく、二度と同じ値を発行しないことだけ
/// が要件なので、採番の受け渡しに順序保証は要らない。
static 構築印カウンタ: AtomicU64 = AtomicU64::new(0);

/// `graph!` の1回の構築を識別する印を新しく1つ発行する。名前付き位置が
/// 生成元と異なる `Graph` へ [`crate::NamedGraphElement::bind`] されるのを
/// 実行時に検出するために使う。`graph_schema!` が生成する `Builder::new()` が
/// これを呼び、同じ builder から生まれる `Graph` と全ての名前付き位置へ同じ値を
/// 埋め込む。`bind` はこの値を照合し、一致しなければ契約違反として
/// `panic!` する (`# Panics` の考え方は `docs/development/design_principles.md` 原則2 —
/// この違反は builder/Graph の取り違えという呼び出し規約違反であり、通常の
/// ドメインエラーではない)。採番の順序に意味はなく重複しなければよいため
/// `Relaxed` で十分。加算は `checked_add` で行い、カウンタが `u64` の上限に
/// 達して次の値を発行できない場合は無言で一周させず `panic!` する
/// (到達したらバグの様式。実運用で `graph!` を `u64::MAX` 回呼ぶことは
/// 想定していない)。
#[doc(hidden)]
pub fn 次の構築印を発行する() -> u64 {
    構築印カウンタ
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |現在の値| {
            現在の値.checked_add(1)
        })
        .expect("構築印が u64 を使い切りました")
}
