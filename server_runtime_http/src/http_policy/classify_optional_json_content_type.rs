#[must_use]
pub fn classify_optional_json_content_type(
    value: super::HttpContentTypeTextRef<'_>,
) -> super::OptionalJsonContentType {
    let Some(text) = value.0.map(str::trim).filter(|text| !text.is_empty()) else {
        return super::OptionalJsonContentType::Missing;
    };
    if text.len() > constants_usize::VALUE_4_096 {
        return super::OptionalJsonContentType::NonJson;
    }
    if text
        .parse::<mime::Mime>()
        .is_ok_and(|media_type| media_type.essence_str() == constants_str::APPLICATION_JSON)
    {
        super::OptionalJsonContentType::ApplicationJson
    } else {
        super::OptionalJsonContentType::NonJson
    }
}
