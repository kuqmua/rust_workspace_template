#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = 128usize, min = constants_usize::ONE)]
pub struct TransportRetryAfter(String);
