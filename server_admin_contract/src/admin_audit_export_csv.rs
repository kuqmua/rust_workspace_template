#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(
    max = 262_144usize,
    chars,
    serde,
    utoipa,
    description = "bounded administrator audit CSV export"
)]
pub struct AdminAuditExportCsv(
    bounded_types::bounded_string::BoundedString<0usize, 262_144usize, true>,
);
