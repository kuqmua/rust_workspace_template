#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator session timestamp"
)]
pub struct AdminSessionTimestamp(String);
