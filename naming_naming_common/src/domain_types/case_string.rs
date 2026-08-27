use super::CASE_STRING_MAX_LEN;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = CASE_STRING_MAX_LEN)]
pub(super) struct CaseString(pub(super) String);
