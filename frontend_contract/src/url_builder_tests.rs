#[cfg(test)]
mod tests {
    fn build_api_url_with_query_and_fragment()
    -> Result<crate::api_url::ApiUrl, crate::api_url::ApiUrlTryFromStringError> {
        let component = constants_str::TEST_API_URL_QUERY_NAME;
        crate::api_url::ApiUrl::try_from(format!(
            "{}?{}={}#{}",
            constants_str::TEST_API_URL_BASE,
            component,
            component,
            component
        ))
    }
    #[test]
    fn test_path_and_query_components_are_encoded() {
        let mut url =
            crate::api_url::ApiUrl::try_from(String::from(constants_str::TEST_API_URL_BASE))
                .expect(constants_str::DIAGNOSTIC_17480CB4);
        url.push_path_segment(
            crate::api_url_path_segment_ref::ApiUrlPathSegmentRef::try_from(
                constants_str::TEST_API_URL_SEGMENT,
            )
            .expect(constants_str::DIAGNOSTIC_C013ABC7),
        )
        .expect(constants_str::DIAGNOSTIC_8DDC23D4);
        url.push_query_pair(
            constants_str::TEST_API_URL_QUERY_NAME.into(),
            constants_str::TEST_API_URL_QUERY_VALUE.into(),
        )
        .expect(constants_str::DIAGNOSTIC_0E672D91);
        assert_eq!(url.as_ref(), constants_str::TEST_API_URL_EXPECTED);
    }

    #[test]
    fn test_traversal_segments_are_rejected() {
        assert_eq!(
            crate::api_url_path_segment_ref::ApiUrlPathSegmentRef::try_from(constants_str::DOT_DOT),
            Err(crate::api_url_build_error::ApiUrlBuildError::InvalidPathSegment)
        );
    }
    #[test]
    fn test_path_segment_precedes_existing_query_and_fragment() {
        let component = constants_str::TEST_API_URL_QUERY_NAME;
        let expected = format!(
            "{}/{}?{}={}#{}",
            constants_str::TEST_API_URL_BASE,
            component,
            component,
            component,
            component
        );
        let result = crate::api_url_path_segment_ref::ApiUrlPathSegmentRef::try_from(component)
            .map(|api_url_path_segment_ref| -> Result<crate::api_url::ApiUrl, crate::api_url::ApiUrlTryFromStringError> {
                let mut api_url = build_api_url_with_query_and_fragment()?;
                api_url.push_path_segment(api_url_path_segment_ref)?;
                Ok(api_url)
            });
        assert!(matches!(result, Ok(Ok(api_url)) if api_url.as_ref() == expected));
    }

    #[test]
    fn test_query_pair_precedes_fragment_containing_question_mark() {
        let component = constants_str::TEST_API_URL_QUERY_NAME;
        let input = format!(
            "{}#{}?{}",
            constants_str::TEST_API_URL_BASE,
            component,
            component
        );
        let expected = format!(
            "{}?{}={}#{}?{}",
            constants_str::TEST_API_URL_BASE,
            component,
            component,
            component,
            component
        );
        let result = crate::api_url::ApiUrl::try_from(input).and_then(|mut api_url| {
            api_url.push_query_pair(component.into(), component.into())?;
            Ok(api_url)
        });
        assert!(matches!(result, Ok(api_url) if api_url.as_ref() == expected));
    }

    #[test]
    fn test_query_pair_preserves_existing_query_and_fragment() {
        let component = constants_str::TEST_API_URL_QUERY_NAME;
        let expected = format!(
            "{}?{}={}&{}={}#{}",
            constants_str::TEST_API_URL_BASE,
            component,
            component,
            component,
            component,
            component
        );
        let result = build_api_url_with_query_and_fragment().and_then(|mut api_url| {
            api_url.push_query_pair(component.into(), component.into())?;
            Ok(api_url)
        });
        assert!(matches!(result, Ok(api_url) if api_url.as_ref() == expected));
    }
}
