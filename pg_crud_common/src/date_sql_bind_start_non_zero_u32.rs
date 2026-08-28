#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct DateSqlBindStartNonZeroU32(std::num::NonZeroU32);
