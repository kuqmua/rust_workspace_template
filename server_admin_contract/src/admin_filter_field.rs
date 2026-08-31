#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
    newtype::Display,
)]
#[bounded_string(
    max = 63usize,
    chars,
    serde,
    utoipa,
    description = "administrator filter field"
)]
pub struct AdminFilterField(bounded_types::bounded_string::BoundedString<0usize, 63usize, true>);
