#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_partial_eq_inner::PartialEqInner,
)]
pub struct GitCommitLink(String);
impl TryFrom<String> for GitCommitLink {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::validate_git_info_string_len::validate_git_info_string_len(value.len())?;
        Ok(Self(value))
    }
}
impl From<crate::git_commit_link_cow::GitCommitLinkCow> for GitCommitLink {
    fn from(value: crate::git_commit_link_cow::GitCommitLinkCow) -> Self {
        Self::try_from(std::borrow::Cow::from(value).into_owned()).unwrap_or_else(Self::from)
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
    fn eq(
        &self,
        project_git_commit_link_ref: &crate::project_git_commit_link_ref::ProjectGitCommitLinkRef,
    ) -> bool {
        self.as_ref() == <&str>::from(*project_git_commit_link_ref)
    }
}
