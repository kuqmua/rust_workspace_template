#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::Display,
    proc_macro_newtype::IntoInner,
)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator audit timestamp"
)]
pub struct AdminAuditTimestamp(bounded_types::bounded_string::BoundedString<0usize, 64, true>);
