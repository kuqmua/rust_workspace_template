#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(crate) struct ScaffoldText(pub(super) String);
