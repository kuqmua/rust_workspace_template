#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
)]
#[bounded_string(max = 255usize, min = constants_usize::ONE)]
pub struct TransportIdempotencyKey(
    bounded_types::bounded_string::BoundedString<{ constants_usize::ONE }, 255usize, false>,
);
