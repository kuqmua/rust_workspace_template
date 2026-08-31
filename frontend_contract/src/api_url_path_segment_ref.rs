#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::GetInner,
)]
pub struct ApiUrlPathSegmentRef<'value_lt>(&'value_lt str);

impl<'value_lt> TryFrom<&'value_lt str> for ApiUrlPathSegmentRef<'value_lt> {
    type Error = crate::api_url_build_error::ApiUrlBuildError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.contains('/')
            || matches!(value, constants_str::DOT | constants_str::DOT_DOT)
        {
            Err(crate::api_url_build_error::ApiUrlBuildError::InvalidPathSegment)
        } else {
            Ok(Self(value))
        }
    }
}
