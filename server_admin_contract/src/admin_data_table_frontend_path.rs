#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::Display,
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
