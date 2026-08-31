#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_8_192)]
pub struct TransportPath(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_8_192 }, false>,
);
