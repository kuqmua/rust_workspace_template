#[must_use]
pub fn classify_optional_json_content_type(
    http_content_type_text_ref: crate::http_content_type_text_ref::HttpContentTypeTextRef<'_>,
) -> crate::optional_json_content_type::OptionalJsonContentType {
    let Some(text) = http_content_type_text_ref
        .get()
        .map(str::trim)
        .filter(|text| !text.is_empty())
    else {
        return crate::optional_json_content_type::OptionalJsonContentType::Missing;
    };
    if text.len() > constants_usize::VALUE_4_096 {
        return crate::optional_json_content_type::OptionalJsonContentType::NonJson;
    }
    if text
        .parse::<mime::Mime>()
        .is_ok_and(|media_type| media_type.essence_str() == constants_str::APPLICATION_JSON)
    {
        crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson
    } else {
        crate::optional_json_content_type::OptionalJsonContentType::NonJson
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_optional_json_content_type_distinguishes_missing_and_non_json_values() {
        assert_eq!(
            crate::classify_optional_json_content_type::classify_optional_json_content_type(
                crate::http_content_type_text_ref::HttpContentTypeTextRef::from(None),
            ),
            crate::optional_json_content_type::OptionalJsonContentType::Missing
        );
        assert!(
            [
                (
                    String::new(),
                    crate::optional_json_content_type::OptionalJsonContentType::Missing
                ),
                (
                    [' ', '\t', '\r', '\n'].into_iter().collect::<String>(),
                    crate::optional_json_content_type::OptionalJsonContentType::Missing
                ),
                (
                    constants_str::APPLICATION_JSON.to_ascii_uppercase(),
                    crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson
                ),
                (
                    constants_str::TEST_JSON_CONTENT_TYPE_WITH_CHARSET.to_owned(),
                    crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson
                ),
                (
                    constants_str::APPLICATION_PROBLEM_PLUS_JSON.to_owned(),
                    crate::optional_json_content_type::OptionalJsonContentType::NonJson
                ),
                (
                    constants_str::X.to_owned(),
                    crate::optional_json_content_type::OptionalJsonContentType::NonJson
                ),
            ]
            .into_iter()
            .all(|(text, expected)| {
                crate::classify_optional_json_content_type::classify_optional_json_content_type(
                    crate::http_content_type_text_ref::HttpContentTypeTextRef::from(Some(
                        text.as_str(),
                    )),
                ) == expected
            })
        );
    }

    #[test]
    fn test_json_content_type_size_limit_applies_after_whitespace_trimming() {
        let prefix = format!("{};{}=", constants_str::APPLICATION_JSON, constants_str::X);
        assert!([4095usize, 4096usize, 4097usize].into_iter().all(|length| {
            let mut text = prefix.clone();
            text.push_str(&constants_str::X.repeat(length.saturating_sub(prefix.len())));
            let expected = if length > 4096usize {
                crate::optional_json_content_type::OptionalJsonContentType::NonJson
            } else {
                crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson
            };
            let mut padded = ' '.to_string().repeat(20usize);
            padded.push_str(&text);
            padded.push('\t');
            [text, padded].into_iter().all(|value| {
                crate::classify_optional_json_content_type::classify_optional_json_content_type(
                    crate::http_content_type_text_ref::HttpContentTypeTextRef::from(Some(
                        value.as_str(),
                    )),
                ) == expected
            })
        }));
    }
}
