#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReplaceRolePermissionsOutcome {
    MissingRole,
    StaleAssignment,
    SystemRole,
    UnknownPermission,
    Updated,
}
