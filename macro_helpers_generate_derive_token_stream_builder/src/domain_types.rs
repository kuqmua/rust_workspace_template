const SC_STRING_MAX_LEN: usize = 1_048_576;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::AsRefStr, newtype::FromInner,
)]
pub(crate) struct ToSnakeCaseInput<'input_lt>(&'input_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SC_STRING_MAX_LEN, description = "snake case string")]
pub(crate) struct SnakeCaseString(String);
