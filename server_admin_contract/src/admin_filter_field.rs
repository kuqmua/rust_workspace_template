#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(
    max = 63usize,
    chars,
    serde,
    utoipa,
    description = "administrator filter field"
)]
pub struct AdminFilterField(bounded_types::bounded_string::BoundedString<0usize, 63usize, true>);
