#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = 128usize, min = constants_usize::ONE)]
pub struct TransportRetryAfter(
    bounded_types::bounded_string::BoundedString<{ constants_usize::ONE }, 128usize, false>,
);
