#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
    newtype::Display,
)]
#[bounded_string(max = 16_384usize, chars)]
pub(crate) struct AdminCsrApiUrl(
    bounded_types::bounded_string::BoundedString<0usize, 16_384usize, true>,
);
