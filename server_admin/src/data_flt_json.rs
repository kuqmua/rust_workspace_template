#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(super) struct DataFltJson(String);
