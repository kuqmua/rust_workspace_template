#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(super) struct CommandText(String);
