#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub(super) enum OtlpExportMode {
    Disabled,
    Enabled,
}

impl From<bool> for OtlpExportMode {
    fn from(value: bool) -> Self {
        if value { Self::Enabled } else { Self::Disabled }
    }
}
