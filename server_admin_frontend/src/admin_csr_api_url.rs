#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = 16_384usize, chars)]
pub(in crate::domain_types::start) struct AdminCsrApiUrl(String);
