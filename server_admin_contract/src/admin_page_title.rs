#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AdminPageTitle {
    Api,
    Metrics,
    Permissions,
    Profile,
    Roles,
    Sessions,
    Settings,
    Tables,
    Users,
    Version,
}
