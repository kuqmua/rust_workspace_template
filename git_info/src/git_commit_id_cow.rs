#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct GitCommitIdCow<'commit_lt>(std::borrow::Cow<'commit_lt, str>);
impl<'commit_lt> TryFrom<std::borrow::Cow<'commit_lt, str>> for GitCommitIdCow<'commit_lt> {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(cow: std::borrow::Cow<'commit_lt, str>) -> Result<Self, Self::Error> {
        if cow.len() > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
                len: cow.len(),
                max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(Self(cow))
        }
    }
}
impl From<crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError>
    for GitCommitIdCow<'_>
{
    fn from(
        git_info_string_try_from_string_error: crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    ) -> Self {
        Self(std::borrow::Cow::Owned(
            git_info_string_try_from_string_error.to_string(),
        ))
    }
}
