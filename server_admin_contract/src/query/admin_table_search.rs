#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = 128usize,
    chars,
    serde,
    utoipa,
    description = "administrator table search"
)]
pub struct AdminTableSearch(String);
