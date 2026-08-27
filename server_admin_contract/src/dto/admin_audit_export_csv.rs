#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(
    max = 262_144usize,
    chars,
    serde,
    utoipa,
    description = "bounded administrator audit CSV export"
)]
pub struct AdminAuditExportCsv(String);
