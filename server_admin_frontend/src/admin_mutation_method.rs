#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    naming_macros::EnumWithUnitFieldsToUpperSnakeCaseStr,
)]
pub(crate) enum AdminMutationMethod {
    Delete,
    Patch,
    Post,
}

impl AdminMutationMethod {
    pub(crate) const fn get(self) -> &'static str {
        self.as_upper_snake_case_str()
    }
}
