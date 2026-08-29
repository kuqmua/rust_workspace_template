#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApiUrlPathSegmentRef<'value_lt>(pub(super) &'value_lt str);

impl<'value_lt> TryFrom<&'value_lt str> for ApiUrlPathSegmentRef<'value_lt> {
    type Error = crate::api_url_build_error::ApiUrlBuildError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.contains('/')
            || matches!(
                value,
                constants_str::catalog::DOT | constants_str::test_fixtures::DOT_DOT
            )
        {
            Err(crate::api_url_build_error::ApiUrlBuildError::InvalidPathSegment)
        } else {
            Ok(Self(value))
        }
    }
}
