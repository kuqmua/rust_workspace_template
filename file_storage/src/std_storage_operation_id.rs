#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct StdStorageOperationId(String);
impl TryFrom<String> for StdStorageOperationId {
    type Error = crate::file_storage_path_error::FileStoragePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::domain_types::MAXIMUM_OPERATION_ID_BYTES {
            return Err(crate::file_storage_path_error::FileStoragePathError::OperationIdInvalid);
        }
        text_policy::validate_url_safe_token_part::validate_url_safe_token_part(
            text_policy::url_safe_token_part_ref::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                crate::domain_types::MAXIMUM_OPERATION_ID_BYTES,
            ),
        )
        .map_err(|_error| {
            crate::file_storage_path_error::FileStoragePathError::OperationIdInvalid
        })?;
        Ok(Self(value))
    }
}
