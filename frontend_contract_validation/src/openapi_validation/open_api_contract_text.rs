#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub struct OpenApiContractText(String);
