//! schema宣言 (`schema <名前> { .. }`) の構文解析と検証。`静的グラフ型!` は
//! この構文解析・検証結果を、macro_rules! 転送用に生トークンのまま焼き込む
//! だけで、コード生成はしない (issue #24 段階2で `静的グラフ内部!` へ移した)。

pub(crate) mod input;
mod validate;
