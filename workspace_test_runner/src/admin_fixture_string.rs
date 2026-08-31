#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::BoundedString,
    newtype::IntoInnerFrom,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(crate) struct AdminFixtureString(String);
