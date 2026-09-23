//! 追跡対象の静的グラフ宣言 (issue #41 §2)。`TrackedStaticSchema`・
//! `TrackedStaticInstance` は、動的グラフの `TrackedSchema`
//! (crate直下 `lib.rs`) と同じ役割を果たす公開型である。
//!
//! 段階2ではこの2つの型・生成関数を用意し単体試験で確かめるところまでで、
//! `static_graph_schema!`/`__static_graph_impl` の公開の振る舞いをこれへ
//! 切り替えるのは段階3 (cliの探索の切り替えと同時に行う) である。

mod instance;
mod schema;
#[cfg(test)]
mod tests;

pub use instance::{parse_tracked_static_instance, TrackedStaticInstance};
pub use schema::{parse_tracked_static_schema, TrackedStaticSchema};
