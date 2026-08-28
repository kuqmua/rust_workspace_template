#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct PgTypesModelEntryCount(pub(super) usize);
impl ValidatedGeneratePgTypesConfig {
    #[must_use]
    pub const fn entry_count(&self) -> PgTypesModelEntryCount {
        self.entry_count
    }
}
