use super::AdminFrontendPath;

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
impl From<crate::domain_types::AdminDataTable> for AdminDataTableFrontendPath {
    fn from(value: crate::domain_types::AdminDataTable) -> Self {
        Self(format!("{}/{}", AdminFrontendPath::Root.get(), value).into_boxed_str())
    }
}
