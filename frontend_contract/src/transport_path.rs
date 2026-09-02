#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_8_192)]
pub struct TransportPath(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_8_192 }, false>,
);
