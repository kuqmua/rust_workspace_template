#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::BoundedString,
    newtype::ToErrStringAsRefStr,
)]
#[bounded_string(max = crate::LOC_TEST_TEXT_MAX_LEN)]
#[serde(try_from = "String")]
pub struct LocationTestText(pub(super) String);
