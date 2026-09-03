#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
#[bounded_string(
    max = 32usize,
    chars,
    serde,
    utoipa,
    description = "administrator table sort key"
)]
pub struct AdminTableSortKey(bounded_types::bounded_string::BoundedString<0usize, 32usize, true>);
