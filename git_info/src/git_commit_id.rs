#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Default,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct GitCommitId(String);
impl TryFrom<String> for GitCommitId {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::validate_git_info_string_len::validate_git_info_string_len(value.len())
            .map(|()| Self(value))
    }
}
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
