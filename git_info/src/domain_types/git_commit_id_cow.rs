use super::{GIT_INFO_STRING_MAX_LEN, GitInfoStringTryFromStringError};

#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr,
)]
pub struct GitCommitIdCow<'commit_lt>(pub(super) std::borrow::Cow<'commit_lt, str>);
impl<'commit_lt> TryFrom<std::borrow::Cow<'commit_lt, str>> for GitCommitIdCow<'commit_lt> {
    type Error = GitInfoStringTryFromStringError;
    fn try_from(value: std::borrow::Cow<'commit_lt, str>) -> Result<Self, Self::Error> {
        if value.len() > GIT_INFO_STRING_MAX_LEN {
            Err(GitInfoStringTryFromStringError::TooLong {
                len: value.len(),
                max: GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl From<GitInfoStringTryFromStringError> for GitCommitIdCow<'_> {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(std::borrow::Cow::Owned(value.to_string()))
    }
}
