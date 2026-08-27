#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = 32usize,
    chars,
    serde,
    utoipa,
    description = "administrator table sort key"
)]
pub struct AdminTableSortKey(String);
