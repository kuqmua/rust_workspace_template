#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
)]
#[bounded_string(max = 255usize, min = constants_usize::ONE)]
pub struct TransportIdempotencyKey(
    bounded_types::bounded_string::BoundedString<{ constants_usize::ONE }, 255usize, false>,
);
