#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct StdTimeDurationSecs(u64);
