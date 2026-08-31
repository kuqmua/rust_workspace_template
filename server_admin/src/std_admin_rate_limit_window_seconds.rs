#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    generate_accessor::Getters,
)]
pub(crate) struct StdAdminRateLimitWindowSeconds(i32);
