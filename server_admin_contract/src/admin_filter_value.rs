#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::Display,
)]
#[bounded_string(
    max = 4096usize,
    chars,
    serde,
    utoipa,
    description = "administrator filter value"
)]
pub struct AdminFilterValue(bounded_types::bounded_string::BoundedString<0usize, 4096usize, true>);
