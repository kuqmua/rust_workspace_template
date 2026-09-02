#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct StdTimeDurationSecs(u64);
