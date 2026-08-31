#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(super) struct CommandText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_16_777_216 },
        false,
    >,
);
