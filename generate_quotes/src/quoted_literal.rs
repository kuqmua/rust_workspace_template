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
#[bounded_string(max = crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN)]
pub struct QuotedLiteral(pub(super) String);
