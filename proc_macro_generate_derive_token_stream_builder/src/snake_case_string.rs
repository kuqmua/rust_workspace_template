const SC_STRING_MAX_LEN: usize = 1_048_576;

#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = SC_STRING_MAX_LEN, description = "snake case string")]
pub(crate) struct SnakeCaseString(
    bounded_types::bounded_string::BoundedString<0usize, { SC_STRING_MAX_LEN }, false>,
);
