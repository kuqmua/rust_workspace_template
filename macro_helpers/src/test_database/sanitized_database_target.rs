#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = constants_usize::VALUE_4_096)]
pub struct SanitizedDatabaseTarget(String);
