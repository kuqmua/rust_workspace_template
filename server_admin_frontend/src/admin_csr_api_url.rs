#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::Display,
)]
#[bounded_string(max = 16_384usize, chars)]
pub(crate) struct AdminCsrApiUrl(
    bounded_types::bounded_string::BoundedString<0usize, 16_384usize, true>,
);
