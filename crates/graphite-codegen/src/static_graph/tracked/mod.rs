//! 追跡対象の静的グラフ宣言。`TrackedStaticSchema`・`TrackedStaticInstance`
//! は、動的グラフの `TrackedSchema` (crate直下 `lib.rs`) と同じ役割を果たす
//! 公開型である。`parse_tracked_static_instance` は `&TrackedStaticSchema`
//! を受け取り、`graphite-cli` の2段階の解決 (静的schema名簿からinstanceを
//! 見つける) から呼べる。`instance展開用に解析する` は
//! `graphite_macros::__static_graph_impl` のその場展開専用の crate内部版
//! であり、schemaの生の構文木だけを受け取る (`static_graph::mod` 参照)。

mod instance;
mod schema;
#[cfg(test)]
mod tests;

pub(crate) use instance::instance展開用に解析する;
pub use instance::{parse_tracked_static_instance, TrackedStaticInstance};
pub use schema::{parse_tracked_static_schema, TrackedStaticSchema};
