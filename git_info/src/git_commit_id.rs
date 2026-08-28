#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{GIT_INFO_STRING_MAX_LEN, GitCommitIdRef, GitInfoStringTryFromStringError};

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
    error = GitInfoStringTryFromStringError,
    validator = GitCommitId::validate
)]
pub struct GitCommitId(pub(super) String);
impl GitCommitId {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), GitInfoStringTryFromStringError> {
        if value.len() > GIT_INFO_STRING_MAX_LEN {
            Err(GitInfoStringTryFromStringError::TooLong {
                len: value.len(),
                max: GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(())
        }
    }
}
impl From<GitCommitIdRef<'_>> for GitCommitId {
    fn from(value: GitCommitIdRef<'_>) -> Self {
        Self::try_from(value.0.to_owned()).unwrap_or_else(Self::from)
    }
}
impl From<GitInfoStringTryFromStringError> for GitCommitId {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
