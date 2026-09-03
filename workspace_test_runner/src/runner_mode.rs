#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
)]
#[bounded_string(max = crate::domain_types::RUNNER_MODE_MAX_LEN)]
pub(crate) struct RunnerMode(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::domain_types::RUNNER_MODE_MAX_LEN },
        false,
    >,
);
