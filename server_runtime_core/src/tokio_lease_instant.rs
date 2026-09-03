#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TokioLeaseInstant(tokio::time::Instant);
