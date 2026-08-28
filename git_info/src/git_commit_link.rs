#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::{
    GIT_INFO_STRING_MAX_LEN, GitCommitLinkCow, GitInfoStringTryFromStringError,
    ProjectGitCommitLinkRef,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
    newtype::PartialEqInner,
    newtype::TryFrom,
)]
#[try_from(
    error = GitInfoStringTryFromStringError,
    validator = GitCommitLink::validate
)]
pub struct GitCommitLink(pub(super) String);
impl GitCommitLink {
    #[allow(
        clippy::single_call_fn,
        reason = "derive-generated TryFrom owns the single validation call"
    )]
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
impl From<GitCommitLinkCow> for GitCommitLink {
    fn from(value: GitCommitLinkCow) -> Self {
        Self::try_from(value.0.into_owned()).unwrap_or_else(Self::from)
    }
}
impl From<GitInfoStringTryFromStringError> for GitCommitLink {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl PartialEq<ProjectGitCommitLinkRef> for GitCommitLink {
    fn eq(&self, other: &ProjectGitCommitLinkRef) -> bool {
        self.0 == other.0
    }
}
