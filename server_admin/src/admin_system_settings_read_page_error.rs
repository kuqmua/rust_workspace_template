#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminSystemSettingsReadPageError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    Collection(#[from] server_admin_contract::admin_collection_error::AdminCollectionError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMIN_REPOSITORY_QUERY_FAILED)]
    Query(#[from] crate::sqlx_admin_error::SqlxAdminError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    Serialization(#[from] server_runtime_http::serde_json_error::SerdeJsonError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    StoredValue,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    Text(server_admin_contract::admin_text::AdminTextTryFromStringError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    Total(#[from] pg_crud_common::list_total_error::ListTotalError),
}

impl From<server_admin_contract::admin_text::AdminTextTryFromStringError>
    for AdminSystemSettingsReadPageError
{
    fn from(value: server_admin_contract::admin_text::AdminTextTryFromStringError) -> Self {
        Self::Text(value)
    }
}

impl From<crate::admin_repository_error::AdminRepositoryError>
    for AdminSystemSettingsReadPageError
{
    fn from(value: crate::admin_repository_error::AdminRepositoryError) -> Self {
        match value {
            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue => {
                Self::StoredValue
            }
            crate::admin_repository_error::AdminRepositoryError::Sqlx(sqlx_admin_error) => {
                Self::Query(sqlx_admin_error)
            }
        }
    }
}

impl to_err_string::to_err_string::ToErrString for AdminSystemSettingsReadPageError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
