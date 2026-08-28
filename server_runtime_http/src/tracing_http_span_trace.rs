#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::Display, newtype::FromInner,
)]
pub(crate) struct TracingHttpSpanTrace(Box<str>);
