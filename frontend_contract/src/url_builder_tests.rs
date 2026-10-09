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
    #[test]
    fn test_path_segments_reject_empty_slashes_and_both_traversal_forms() {
        [
            constants_str::EMPTY,
            constants_str::SLASH,
            constants_str::DOT,
            constants_str::DOT_DOT,
            constants_str::TEST_API_URL_BASE,
        ]
        .into_iter()
        .fold((), |(), value| {
            assert_eq!(
                crate::api_url_path_segment_ref::ApiUrlPathSegmentRef::try_from(value),
                Err(crate::api_url_build_error::ApiUrlBuildError::InvalidPathSegment)
            );
        });
    }

    #[test]
    fn test_path_append_reuses_trailing_slash_and_preserves_query_fragment_suffixes() {
        [
            constants_str::EMPTY.to_owned(),
            format!("?{}={}", constants_str::X, constants_str::X),
            format!("#{}?{}", constants_str::X, constants_str::X),
            format!(
                "?{}={}#{}",
                constants_str::X,
                constants_str::X,
                constants_str::X
            ),
        ]
        .into_iter()
        .fold((), |(), suffix| {
            let input = format!("{}/{}", constants_str::TEST_API_URL_BASE, suffix);
            let expected = format!(
                "{}/{}{}",
                constants_str::TEST_API_URL_BASE,
                constants_str::X,
                suffix
            );
            assert!(
                crate::api_url_path_segment_ref::ApiUrlPathSegmentRef::try_from(constants_str::X)
                    .is_ok_and(|segment| {
                        crate::api_url::ApiUrl::try_from(input).is_ok_and(|mut api_url| {
                            api_url
                                .push_path_segment(segment)
                                .is_ok_and(|()| api_url.as_ref() == expected)
                        })
                    })
            );
        });
    }

    #[test]
    fn test_path_and_query_appends_accept_exact_maximum_length() {
        [
            (false, format!("/{}", constants_str::X)),
            (true, format!("?{}={}", constants_str::X, constants_str::X)),
        ]
        .into_iter()
        .fold((), |(), (append_query, suffix)| {
            let maximum = constants_usize::VALUE_1_048_576;
            let original = constants_str::X.repeat(maximum.saturating_sub(suffix.len()));
            let expected = format!("{original}{suffix}");
            assert!(
                crate::api_url::ApiUrl::try_from(original).is_ok_and(|mut api_url| {
                    let appended = if append_query {
                        api_url
                            .push_query_pair(constants_str::X.into(), constants_str::X.into())
                            .is_ok_and(|()| api_url.as_ref() == expected)
                    } else {
                        crate::api_url_path_segment_ref::ApiUrlPathSegmentRef::try_from(
                            constants_str::X,
                        )
                        .is_ok_and(|segment| {
                            api_url
                                .push_path_segment(segment)
                                .is_ok_and(|()| api_url.as_ref() == expected)
                        })
                    };
                    appended && api_url.as_ref().len() == maximum
                })
            );
        });
    }

    #[test]
    fn test_url_append_length_failures_preserve_original_content() {
        [false, true].into_iter().fold((), |(), append_query| {
            let maximum = constants_usize::VALUE_1_048_576;
            assert!(
                crate::api_url::ApiUrl::try_from(constants_str::X.repeat(maximum)).is_ok_and(
                    |mut api_url| {
                        let rejected = if append_query {
                            api_url
                                .push_query_pair(constants_str::X.into(), constants_str::X.into())
                                .is_err_and(|error| {
                                    matches!(
                                        error,
                                        crate::api_url::ApiUrlTryFromStringError::TooLong { .. }
                                    )
                                })
                        } else {
                            crate::api_url_path_segment_ref::ApiUrlPathSegmentRef::try_from(
                                constants_str::X,
                            )
                            .is_ok_and(|segment| {
                                api_url.push_path_segment(segment).is_err_and(|error| {
                                    matches!(
                                        error,
                                        crate::api_url::ApiUrlTryFromStringError::TooLong { .. }
                                    )
                                })
                            })
                        };
                        rejected
                            && api_url.as_ref().len() == maximum
                            && api_url.as_ref().split(constants_str::X).all(str::is_empty)
                    }
                )
            );
        });
    }
    #[test]
    fn test_query_component_encoding_preserves_exact_ascii_escape_policy() {
        (0u8..=127u8).fold((), |(), byte| {
            let component = char::from(byte).to_string();
            let encoded = if byte.is_ascii_alphanumeric()
                || matches!(byte, b'-' | b'.' | b',' | b'_' | b'~')
            {
                component.clone()
            } else {
                format!("{}{byte:02X}", constants_str::VALUE_PERCENT)
            };
            let expected = format!("{}?{encoded}={encoded}", constants_str::TEST_API_URL_BASE);
            assert!(
                crate::api_url::ApiUrl::try_from(constants_str::TEST_API_URL_BASE.to_owned())
                    .is_ok_and(|mut api_url| {
                        api_url
                            .push_query_pair(component.as_str().into(), component.as_str().into())
                            .is_ok_and(|()| api_url.as_ref() == expected)
                    })
            );
        });
    }
}
