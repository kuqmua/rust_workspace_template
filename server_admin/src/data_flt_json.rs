#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(crate) struct DataFltJson(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);
