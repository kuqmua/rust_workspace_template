#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
)]
#[bounded_string(max = crate::domain_types::RUNNER_MODE_MAX_LEN)]
pub(crate) struct RunnerMode(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::domain_types::RUNNER_MODE_MAX_LEN },
        false,
    >,
);
