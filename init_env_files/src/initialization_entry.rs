#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub(crate) struct InitializationEntry {
    pub(super) keys: crate::EnvKeys,
    pub(super) member: crate::WorkspaceMember,
    pub(super) status: crate::InitializationStatus,
}
impl InitializationEntry {
    pub(crate) const fn keys(&self) -> &crate::EnvKeys {
        &self.keys
    }
    pub(crate) const fn member(&self) -> &crate::WorkspaceMember {
        &self.member
    }
    pub(crate) const fn status(&self) -> crate::InitializationStatus {
        self.status
    }
}
