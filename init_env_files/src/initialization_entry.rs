#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub(crate) struct InitializationEntry {
    pub(super) keys: crate::env_keys::EnvKeys,
    pub(super) member: crate::workspace_member::WorkspaceMember,
    pub(super) status: crate::initialization_status::InitializationStatus,
}
impl InitializationEntry {
    pub(crate) const fn keys(&self) -> &crate::env_keys::EnvKeys {
        &self.keys
    }
    pub(crate) const fn member(&self) -> &crate::workspace_member::WorkspaceMember {
        &self.member
    }
    pub(crate) const fn status(&self) -> crate::initialization_status::InitializationStatus {
        self.status
    }
}
