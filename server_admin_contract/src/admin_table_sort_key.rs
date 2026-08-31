#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = 32usize,
    chars,
    serde,
    utoipa,
    description = "administrator table sort key"
)]
pub struct AdminTableSortKey(bounded_types::bounded_string::BoundedString<0usize, 32usize, true>);
