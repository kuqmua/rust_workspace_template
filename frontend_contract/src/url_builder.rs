pub use super::api_url::ApiUrl;
pub use super::api_url_build_error::ApiUrlBuildError;
use super::api_url_component_encode_set::API_URL_COMPONENT_ENCODE_SET;
pub use super::api_url_path_segment_ref::ApiUrlPathSegmentRef;
pub use super::api_url_query_component_ref::ApiUrlQueryComponentRef;
#[cfg(test)]
mod tests {
    #[test]
    fn path_and_query_components_are_encoded() {
        let mut url = super::ApiUrl::try_from(String::from(constants_str::TEST_API_URL_BASE))
            .expect("17480cb4 path_and_query_components_are_encoded invariant must hold");
        url.push_path_segment(
            super::ApiUrlPathSegmentRef::try_from(constants_str::TEST_API_URL_SEGMENT)
                .expect("c013abc7 path_and_query_components_are_encoded invariant must hold"),
        );
        url.push_query_pair(
            constants_str::TEST_API_URL_QUERY_NAME.into(),
            constants_str::TEST_API_URL_QUERY_VALUE.into(),
        );
        assert_eq!(url.as_ref(), constants_str::TEST_API_URL_EXPECTED);
    }

    #[test]
    fn traversal_segments_are_rejected() {
        assert_eq!(
            super::ApiUrlPathSegmentRef::try_from(constants_str::DOT_DOT),
            Err(super::ApiUrlBuildError::InvalidPathSegment)
        );
    }
}
