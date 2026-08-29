#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = crate::domain_types::RUNNER_MODE_MAX_LEN)]
pub(crate) struct RunnerMode(String);
