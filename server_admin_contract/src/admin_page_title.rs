#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
)]
pub(crate) enum AdminPageTitle {
    Branding,
    Health,
    Api,
    Metrics,
    Rules,
    Profile,
    Roles,
    Sessions,
    Settings,
    Tables,
    Users,
    Version,
}
