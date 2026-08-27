#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{EnvKeys, InitializationStatus, WorkspaceMember};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub(crate) struct InitializationEntry {
    pub(super) keys: EnvKeys,
    pub(super) member: WorkspaceMember,
    pub(super) status: InitializationStatus,
}
impl InitializationEntry {
    pub(crate) const fn keys(&self) -> &EnvKeys {
        &self.keys
    }
    pub(crate) const fn member(&self) -> &WorkspaceMember {
        &self.member
    }
    pub(crate) const fn status(&self) -> InitializationStatus {
        self.status
    }
}
