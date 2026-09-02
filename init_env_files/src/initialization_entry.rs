#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub(crate) struct InitializationEntry {
    keys: crate::env_keys::EnvKeys,
    member: crate::workspace_member::WorkspaceMember,
    #[getters(copy)]
    status: crate::initialization_status::InitializationStatus,
}
impl
    From<(
        crate::env_keys::EnvKeys,
        crate::workspace_member::WorkspaceMember,
        crate::initialization_status::InitializationStatus,
    )> for InitializationEntry
{
    fn from(
        value: (
            crate::env_keys::EnvKeys,
            crate::workspace_member::WorkspaceMember,
            crate::initialization_status::InitializationStatus,
        ),
    ) -> Self {
        let (keys, member, status) = value;
        Self {
            keys,
            member,
            status,
        }
    }
}
