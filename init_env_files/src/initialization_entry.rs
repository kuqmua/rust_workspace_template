#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub(crate) struct InitializationEntry {
    keys: crate::env_keys::EnvKeys,
    member: crate::workspace_member::WorkspaceMember,
    status: crate::initialization_status::InitializationStatus,
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
impl
    From<(
        crate::env_keys::EnvKeys,
        crate::workspace_member::WorkspaceMember,
        crate::initialization_status::InitializationStatus,
    )> for InitializationEntry
{
    fn from(
        (keys, member, status): (
            crate::env_keys::EnvKeys,
            crate::workspace_member::WorkspaceMember,
            crate::initialization_status::InitializationStatus,
        ),
    ) -> Self {
        Self {
            keys,
            member,
            status,
        }
    }
}
