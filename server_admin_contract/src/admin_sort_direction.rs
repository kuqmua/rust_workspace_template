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
    naming_macros::EnumWithUnitFieldsToSnakeCaseStr,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminSortDirection {
    #[default]
    Ascending,
    Descending,
}
impl AsRef<str> for AdminSortDirection {
    fn as_ref(&self) -> &str {
        self.as_snake_case_str()
    }
}
