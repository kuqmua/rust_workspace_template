const SC_STRING_MAX_LEN: usize = 1_048_576;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedStringWrapper,
)]
#[bounded_string(max = SC_STRING_MAX_LEN, description = "snake case string")]
pub(crate) struct SnakeCaseString(
    bounded_types::bounded_string::BoundedString<0usize, { SC_STRING_MAX_LEN }, false>,
);
