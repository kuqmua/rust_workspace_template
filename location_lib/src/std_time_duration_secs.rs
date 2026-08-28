#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct StdTimeDurationSecs(u64);
