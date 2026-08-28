#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = 20usize, min = constants_usize::ONE)]
pub struct TransportIfMatch(String);
