#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(crate) struct HealthCheckSucceeded(bool);
