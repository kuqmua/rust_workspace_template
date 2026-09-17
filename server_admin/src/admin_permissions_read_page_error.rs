#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error,
)]
pub enum AdminPermissionsReadPageError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    PermissionId(
        #[from] server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
    ),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    StoredPermission,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    MissingName,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    Collection(#[from] server_admin_contract::admin_collection_error::AdminCollectionError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    Total(#[from] pg_crud_common::list_total_error::ListTotalError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    TotalConversion,
}

impl to_err_string::to_err_string::ToErrString for AdminPermissionsReadPageError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
