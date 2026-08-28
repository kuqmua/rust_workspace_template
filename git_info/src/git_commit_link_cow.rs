#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::{GIT_INFO_STRING_MAX_LEN, GitInfoStringTryFromStringError};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde_derive::Deserialize,
    serde_derive::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
    newtype::Display,
)]
#[serde(try_from = "String")]
pub struct GitCommitLinkCow(pub(super) std::borrow::Cow<'static, str>);
impl TryFrom<String> for GitCommitLinkCow {
    type Error = GitInfoStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(std::borrow::Cow::Owned(value))
    }
}
impl TryFrom<std::borrow::Cow<'static, str>> for GitCommitLinkCow {
    type Error = GitInfoStringTryFromStringError;
    fn try_from(value: std::borrow::Cow<'static, str>) -> Result<Self, Self::Error> {
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
impl From<GitInfoStringTryFromStringError> for GitCommitLinkCow {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(std::borrow::Cow::Owned(value.to_string()))
    }
}
