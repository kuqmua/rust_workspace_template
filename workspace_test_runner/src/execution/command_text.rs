#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(super) struct CommandText(pub(super) String);
