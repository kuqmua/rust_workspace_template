use super::CLEAN_ANSI_TEXT_MAX_LEN;

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = CLEAN_ANSI_TEXT_MAX_LEN)]
pub(crate) struct CleanAnsiText(pub(super) String);
