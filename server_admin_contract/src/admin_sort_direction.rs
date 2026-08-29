#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminSortDirection {
    #[default]
    Asc,
    Desc,
}
impl AsRef<str> for AdminSortDirection {
    fn as_ref(&self) -> &str {
        match self {
            Self::Asc => constants_str::catalog::ASC_ALT,
            Self::Desc => constants_str::catalog::DESC_ALT,
        }
    }
}
