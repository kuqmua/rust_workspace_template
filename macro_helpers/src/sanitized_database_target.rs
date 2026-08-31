#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::BoundedStringWrapper,
    newtype::Display,
)]
#[bounded_string(max = constants_usize::VALUE_4_096)]
pub struct SanitizedDatabaseTarget(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_4_096 }, false>,
);
