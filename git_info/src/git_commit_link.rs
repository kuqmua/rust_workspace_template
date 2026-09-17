#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_partial_eq_inner::PartialEqInner,
)]
pub struct GitCommitLink(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN },
        false,
    >,
);
impl TryFrom<String> for GitCommitLink {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::try_bounded_git_info_string::try_bounded_git_info_string(value).map(Self)
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
        bounded_types::try_from_bounded_error_text::try_from_bounded_error_text(value)
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
