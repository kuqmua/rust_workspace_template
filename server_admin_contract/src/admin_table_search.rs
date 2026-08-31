#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = 128usize,
    chars,
    serde,
    utoipa,
    description = "administrator table search"
)]
pub struct AdminTableSearch(bounded_types::bounded_string::BoundedString<0usize, 128usize, true>);
