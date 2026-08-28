#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::Display, newtype::FromInner,
)]
pub(in crate::domain_types) struct TracingHttpSpanTrace(Box<str>);
