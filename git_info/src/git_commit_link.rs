#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_partial_eq_inner::PartialEqInner,
    proc_macro_newtype_try_from::TryFrom,
)]
#[try_from(
    error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    validator = |value: &str| {
        if value.len() > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong { len: value.len(), max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN })
        } else { Ok(()) }
    }
)]
pub struct GitCommitLink(String);
impl From<crate::git_commit_link_cow::GitCommitLinkCow> for GitCommitLink {
    fn from(git_commit_link_cow: crate::git_commit_link_cow::GitCommitLinkCow) -> Self {
        Self::try_from(std::borrow::Cow::from(git_commit_link_cow).into_owned())
            .unwrap_or_else(Self::from)
    }
}
impl From<crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError>
    for GitCommitLink
{
    fn from(
        git_info_string_try_from_string_error: crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    ) -> Self {
        Self(git_info_string_try_from_string_error.to_string())
    }
}
impl PartialEq<crate::project_git_commit_link_ref::ProjectGitCommitLinkRef> for GitCommitLink {
    fn eq(
        &self,
        project_git_commit_link_ref: &crate::project_git_commit_link_ref::ProjectGitCommitLinkRef,
    ) -> bool {
        self.as_ref() == <&str>::from(*project_git_commit_link_ref)
    }
}
