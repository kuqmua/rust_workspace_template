#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_naming::EnumWithUnitFieldsToUpperSnakeCaseStr,
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
