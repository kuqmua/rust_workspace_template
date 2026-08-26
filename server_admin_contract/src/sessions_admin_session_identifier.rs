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
    description = "administrator session identifier"
)]
pub struct AdminSessionIdentifier(String);
