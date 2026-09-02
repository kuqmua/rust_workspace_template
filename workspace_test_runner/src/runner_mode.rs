#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = crate::domain_types::RUNNER_MODE_MAX_LEN)]
pub(crate) struct RunnerMode(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::domain_types::RUNNER_MODE_MAX_LEN },
        false,
    >,
);
