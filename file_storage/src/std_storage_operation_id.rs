use super::{FileStoragePathError, MAXIMUM_OPERATION_ID_BYTES};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct StdStorageOperationId(String);
impl TryFrom<String> for StdStorageOperationId {
    type Error = FileStoragePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > MAXIMUM_OPERATION_ID_BYTES {
            return Err(FileStoragePathError::OperationIdInvalid);
        }
        text_policy::domain_types::validate_url_safe_token_part(
            text_policy::domain_types::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::domain_types::UrlSafeTokenPartMaximumBytes::from(
                MAXIMUM_OPERATION_ID_BYTES,
            ),
        )
        .map_err(|_error| FileStoragePathError::OperationIdInvalid)?;
        Ok(Self(value))
    }
}
