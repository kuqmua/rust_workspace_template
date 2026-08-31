#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct RuntimeAdminPasswordHashConcurrency(std::num::NonZeroUsize);
