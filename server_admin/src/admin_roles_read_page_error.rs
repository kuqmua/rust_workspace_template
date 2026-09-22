#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error,
)]
pub enum AdminRolesReadPageError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    Total(#[from] pg_crud_common::list_total_error::ListTotalError),
}

impl to_err_string::to_err_string::ToErrString for AdminRolesReadPageError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
