#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Default,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    validator = |value: &str| {
        if value.len() > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong { len: value.len(), max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN })
        } else { Ok(()) }
    }
)]
pub struct GitCommitId(String);
impl From<crate::git_commit_id_ref::GitCommitIdRef<'_>> for GitCommitId {
    fn from(value: crate::git_commit_id_ref::GitCommitIdRef<'_>) -> Self {
        Self::try_from(<&str>::from(value).to_owned()).unwrap_or_else(Self::from)
    }
}
impl From<crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError>
    for GitCommitId
{
    fn from(
        value: crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}
