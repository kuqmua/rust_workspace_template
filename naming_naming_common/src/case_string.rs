#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

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
#[bounded_string(max = crate::case_string_max_len::CASE_STRING_MAX_LEN)]
pub(super) struct CaseString(pub(super) String);
