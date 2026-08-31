#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::BoundedStringWrapper,
    newtype::IntoInnerFrom,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(crate) struct AdminFixtureString(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);
