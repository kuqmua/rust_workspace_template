#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::Display, newtype::FromInner,
)]
pub struct TracingObservedErrorSpanTrace(Box<str>);
