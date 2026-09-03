#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
#[bounded_string(
    max = 128usize,
    chars,
    serde,
    utoipa,
    description = "administrator table search"
)]
pub struct AdminTableSearch(bounded_types::bounded_string::BoundedString<0usize, 128usize, true>);
