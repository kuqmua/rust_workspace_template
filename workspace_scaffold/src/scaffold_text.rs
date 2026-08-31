#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
    newtype::Display,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(crate) struct ScaffoldText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_16_777_216 },
        false,
    >,
);
