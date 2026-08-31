#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = crate::domain_types::CLEAN_ANSI_TEXT_MAX_LEN)]
pub(crate) struct CleanAnsiText(String);
