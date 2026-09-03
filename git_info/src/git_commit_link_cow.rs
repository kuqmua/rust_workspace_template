#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde_derive::Deserialize,
    serde_derive::Serialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
#[serde(try_from = "String")]
pub struct GitCommitLinkCow(std::borrow::Cow<'static, str>);
impl TryFrom<String> for GitCommitLinkCow {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::try_from(std::borrow::Cow::Owned(string))
    }
}
impl TryFrom<std::borrow::Cow<'static, str>> for GitCommitLinkCow {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(cow: std::borrow::Cow<'static, str>) -> Result<Self, Self::Error> {
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
    for GitCommitLinkCow
{
    fn from(
        git_info_string_try_from_string_error: crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    ) -> Self {
        Self(std::borrow::Cow::Owned(
            git_info_string_try_from_string_error.to_string(),
        ))
    }
}
