#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
pub struct AdminDataTableFrontendPath(Box<str>);
impl From<crate::admin_data_table::AdminDataTable> for AdminDataTableFrontendPath {
    fn from(value: crate::admin_data_table::AdminDataTable) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_frontend_path::AdminFrontendPath::Root.get(),
                value
            )
            .into_boxed_str(),
        )
    }
}
