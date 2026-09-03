#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_8_192)]
pub struct TransportPath(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_8_192 }, false>,
);
