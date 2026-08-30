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
    newtype::IntoInnerFrom,
)]
#[serde(try_from = "String")]
pub struct GitCommitLinkCow(std::borrow::Cow<'static, str>);
impl TryFrom<String> for GitCommitLinkCow {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(std::borrow::Cow::Owned(value))
    }
}
impl TryFrom<std::borrow::Cow<'static, str>> for GitCommitLinkCow {
    type Error = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError;
    fn try_from(value: std::borrow::Cow<'static, str>) -> Result<Self, Self::Error> {
        if value.len() > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN {
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
                len: value.len(),
                max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl From<crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError>
    for GitCommitLinkCow
{
    fn from(
        value: crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    ) -> Self {
        Self(std::borrow::Cow::Owned(value.to_string()))
    }
}
