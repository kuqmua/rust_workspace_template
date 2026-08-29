#[must_use]
pub fn normalize_identifier_path(
    path: crate::http_request_path_ref::HttpRequestPathRef<'_>,
) -> Option<crate::http_normalized_path::HttpNormalizedPath> {
    if path.0.len() > constants_usize::VALUE_8_192
        || !path.0.bytes().any(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    let normalized = path.0.split('/').enumerate().fold(
        String::with_capacity(path.0.len()),
        |mut normalized, (index, segment)| {
            if index > constants_usize::ZERO {
                normalized.push('/');
            }
            if !segment.is_empty()
                && segment.len() <= constants_usize::TWENTY.saturating_sub(constants_usize::ONE)
                && segment.bytes().all(|byte| byte.is_ascii_digit())
            {
                normalized
                    .push_str(constants_str::test_fixtures::HTTP_NORMALIZED_IDENTIFIER_SEGMENT);
            } else if uuid::Uuid::parse_str(segment)
                .is_ok_and(|value| value.get_version_num() == constants_usize::FOUR)
            {
                normalized.push_str(constants_str::test_fixtures::HTTP_NORMALIZED_UUID_SEGMENT);
            } else {
                normalized.push_str(segment);
            }
            normalized
        },
    );
    if normalized == path.0 {
        None
    } else {
        crate::http_normalized_path::HttpNormalizedPath::try_from(normalized).ok()
    }
}
