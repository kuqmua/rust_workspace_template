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
pub struct GitCommitId(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN },
        false,
    >,
);
impl TryFrom<String> for GitCommitId {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::try_bounded_git_info_string::try_bounded_git_info_string(value).map(Self)
    }
}
impl TryFrom<crate::git_commit_id_ref::GitCommitIdRef<'_>> for GitCommitId {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(value: crate::git_commit_id_ref::GitCommitIdRef<'_>) -> Result<Self, Self::Error> {
        let text = <&str>::from(value);
        if text.len() > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: text.len(),
                max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
            });
        }
        Self::try_from(text.to_owned())
    }
}
impl From<crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError>
    for GitCommitId
{
    fn from(
        value: crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    ) -> Self {
        Self(bounded_types::bounded_string::BoundedString::from_truncated(value.to_string()))
    }
}
