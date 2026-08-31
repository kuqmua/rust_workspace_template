#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedStringWrapper,
)]
#[bounded_string(max = crate::domain_types::CLEAN_ANSI_TEXT_MAX_LEN)]
pub(crate) struct CleanAnsiText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::domain_types::CLEAN_ANSI_TEXT_MAX_LEN },
        false,
    >,
);
