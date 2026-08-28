#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Default,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
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
pub struct GitCommitId(pub(super) String);
impl From<crate::git_commit_id_ref::GitCommitIdRef<'_>> for GitCommitId {
    fn from(value: crate::git_commit_id_ref::GitCommitIdRef<'_>) -> Self {
        Self::try_from(value.0.to_owned()).unwrap_or_else(Self::from)
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
