#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
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
    error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    validator = |value: &str| {
        if value.len() > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong { len: value.len(), max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN })
        } else { Ok(()) }
    }
)]
pub struct GitCommitLink(pub(super) String);
impl From<crate::git_commit_link_cow::GitCommitLinkCow> for GitCommitLink {
    fn from(value: crate::git_commit_link_cow::GitCommitLinkCow) -> Self {
        Self::try_from(value.0.into_owned()).unwrap_or_else(Self::from)
    }
}
impl From<crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError>
    for GitCommitLink
{
    fn from(
        value: crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}
impl PartialEq<crate::project_git_commit_link_ref::ProjectGitCommitLinkRef> for GitCommitLink {
    fn eq(&self, other: &crate::project_git_commit_link_ref::ProjectGitCommitLinkRef) -> bool {
        self.0 == other.0
    }
}
